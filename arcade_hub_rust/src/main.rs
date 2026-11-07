#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(dead_code)]

mod theme;
mod models;
mod games;
mod ui;

use models::{get_default_catalog, GameMeta, ScoresDB};
use games::{create_game, ArcadeGame};
use theme::*;
use ui::header::draw_header;
use ui::hero::{draw_hero, HeroAction};
use ui::gallery::{draw_gallery, GalleryAction};
use ui::drawer::{draw_details_drawer, DrawerAction};
use ui::add_modal::{draw_add_modal, AddModalAction, AddModalState};
use ui::stage::{draw_stage_header, StageAction};
use ui::background::BackgroundSystem;
use macroquad::prelude::*;

enum AppState {
    Gallery,
    Playing {
        game_id: String,
        game: Box<dyn ArcadeGame>,
        is_paused: bool,
    },
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Arcade Hub".to_string(),
        window_width: 1280,
        window_height: 820,
        high_dpi: true,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let fonts = AppFonts::load();
    let mut catalog = get_default_catalog();
    let mut scores_db = ScoresDB::load();
    let mut bg_system = BackgroundSystem::new(1280.0, 820.0);

    let mut state = AppState::Gallery;
    let mut search_query = String::new();
    let mut is_search_active = false;
    let mut current_category = "all".to_string();
    let mut current_collection = "all".to_string();

    let mut active_drawer_game: Option<GameMeta> = None;
    let mut show_add_modal = false;
    let mut add_modal_state = AddModalState::default();

    let mut scroll_y = 0.0f32;
    let mut toast_message: Option<(String, f32)> = None;

    loop {
        let sw = screen_width();
        let sh = screen_height();
        let mouse_pos = Vec2::new(mouse_position().0, mouse_position().1);

        // Fundo interativo reativo: Grid de Coordenadas + Holofote + Constelação de Partículas
        bg_system.update_and_draw(sw, sh, mouse_pos, get_frame_time());

        // Atualiza temporizador de toast
        if let Some((_, timer)) = &mut toast_message {
            *timer -= get_frame_time();
            if *timer <= 0.0 {
                toast_message = None;
            }
        }

        let mut next_state: Option<AppState> = None;

        match &mut state {
            AppState::Gallery => {
                // Scroll do mouse
                let wheel = mouse_wheel().1;
                if wheel != 0.0 && active_drawer_game.is_none() && !show_add_modal {
                    scroll_y = (scroll_y + wheel * 45.0).clamp(-400.0, 0.0);
                }

                // Atalho Ctrl+K para busca
                if (is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl)) && is_key_pressed(KeyCode::K) {
                    is_search_active = true;
                }

                let content_x = (sw - 1180.0_f32).max(40.0) * 0.5;
                let content_w = 1180.0_f32.min(sw - 40.0);
                let start_y = 80.0 + scroll_y;

                // 1. Hero do Jogo em Destaque
                let featured_game = catalog.iter().find(|g| g.featured).unwrap_or(&catalog[0]);
                let hero_action = draw_hero(content_x, start_y, content_w, featured_game, mouse_pos, &fonts);
                match hero_action {
                    HeroAction::PlayGame(id) => {
                        if let Some(new_game) = create_game(&id) {
                            next_state = Some(AppState::Playing {
                                game_id: id,
                                game: new_game,
                                is_paused: false,
                            });
                        }
                    }
                    HeroAction::OpenDetails(id) => {
                        active_drawer_game = catalog.iter().find(|g| g.id == id).cloned();
                    }
                    HeroAction::None => {}
                }

                // 2. Filtro dos jogos por busca, categoria e coleção
                let filtered_games: Vec<GameMeta> = catalog
                    .iter()
                    .filter(|g| {
                        let matches_cat = current_category == "all" || g.category == current_category;
                        let matches_col = match current_collection.as_str() {
                            "grant_jenks" => g.collection.contains("Grant Jenks"),
                            "al_sweigart" => g.collection.contains("Al Sweigart"),
                            "pygame_community" => g.collection.contains("Comunidade Pygame") || g.collection.contains("Comunidade Arcade"),
                            _ => true,
                        };
                        let q = search_query.to_lowercase();
                        let matches_query = q.is_empty()
                            || g.title.to_lowercase().contains(&q)
                            || g.author.to_lowercase().contains(&q)
                            || g.category.to_lowercase().contains(&q);
                        matches_cat && matches_col && matches_query
                    })
                    .cloned()
                    .collect();

                // 3. Galeria e Coleções
                let gallery_action = draw_gallery(
                    content_x,
                    start_y + 254.0,
                    content_w,
                    &filtered_games,
                    &current_category,
                    &current_collection,
                    mouse_pos,
                    &fonts,
                );

                match gallery_action {
                    GalleryAction::PlayGame(id) => {
                        if let Some(new_game) = create_game(&id) {
                            next_state = Some(AppState::Playing {
                                game_id: id,
                                game: new_game,
                                is_paused: false,
                            });
                        }
                    }
                    GalleryAction::OpenDetails(id) => {
                        active_drawer_game = catalog.iter().find(|g| g.id == id).cloned();
                    }
                    GalleryAction::FilterCategory(cat) => {
                        current_category = cat;
                    }
                    GalleryAction::FilterCollection(col) => {
                        current_collection = col;
                    }
                    GalleryAction::OpenUrl(url) => {
                        open_browser_url(&url);
                        toast_message = Some(("Abrindo repositório oficial no navegador...".into(), 2.5));
                    }
                    GalleryAction::None => {}
                }

                // 4. Header Fixo no Topo
                let header_act = draw_header(sw, &mut search_query, &mut is_search_active, mouse_pos, &fonts);
                if header_act.add_game_clicked {
                    show_add_modal = true;
                    add_modal_state = AddModalState::default();
                }

                // 5. Drawer de Detalhes
                if let Some(game) = &active_drawer_game {
                    match draw_details_drawer(sw, sh, game, mouse_pos, &fonts) {
                        DrawerAction::Close => {
                            active_drawer_game = None;
                        }
                        DrawerAction::Play(id) => {
                            active_drawer_game = None;
                            if let Some(new_game) = create_game(&id) {
                                next_state = Some(AppState::Playing {
                                    game_id: id,
                                    game: new_game,
                                    is_paused: false,
                                });
                            }
                        }
                        DrawerAction::None => {}
                    }
                }

                // 6. Modal de Adicionar Jogo
                if show_add_modal {
                    match draw_add_modal(sw, sh, &mut add_modal_state, mouse_pos, &fonts) {
                        AddModalAction::Close => {
                            show_add_modal = false;
                        }
                        AddModalAction::Save(new_game) => {
                            let title = new_game.title.clone();
                            catalog.insert(0, new_game);
                            show_add_modal = false;
                            toast_message = Some((format!("Jogo '{}' adicionado ao catálogo!", title), 3.0));
                        }
                        AddModalAction::None => {}
                    }
                }
            }

            AppState::Playing {
                game_id,
                game,
                is_paused,
            } => {
                let meta = catalog.iter().find(|g| g.id == *game_id);
                let title = meta.map(|m| m.title.as_str()).unwrap_or("Jogo Arcade");
                let category = meta.map(|m| m.category.as_str()).unwrap_or("Arcade");
                let author = meta.map(|m| m.author.as_str()).unwrap_or("Comunidade");

                // Barra Superior do Console no Palco Integrado
                let stage_act = draw_stage_header(
                    sw,
                    title,
                    category,
                    author,
                    game.get_score(),
                    *is_paused,
                    mouse_pos,
                    &fonts,
                );

                match stage_act {
                    StageAction::BackToGallery => {
                        let final_score = game.get_score();
                        if final_score > 0 {
                            scores_db.add_score(game_id, final_score);
                            toast_message = Some((format!("Placar de {} pontos salvo com sucesso!", final_score), 3.0));
                        }
                        next_state = Some(AppState::Gallery);
                    }
                    StageAction::TogglePause => {
                        *is_paused = !*is_paused;
                    }
                    StageAction::Restart => {
                        game.reset();
                    }
                    StageAction::None => {}
                }

                // Se não estiver pausado e não for sair, avança a física e lógica do jogo
                if !*is_paused && next_state.is_none() {
                    game.update();
                }

                // Renderiza a cena do jogo
                game.draw(sw, sh);

                // Overlay de Jogo Pausado
                if *is_paused && !game.is_game_over() {
                    draw_rectangle(0.0, 56.0, sw, sh - 56.0, Color::new(0.035, 0.047, 0.078, 0.75));
                    let box_w = 420.0;
                    let box_h = 100.0;
                    let bx = (sw - box_w) * 0.5;
                    let by = (sh - box_h) * 0.5;
                    draw_card_rect(bx, by, box_w, box_h, SURFACE_COLOR, ICE_BLUE);

                    let p_dim = measure_text_ui("JOGO PAUSADO", 26.0, &fonts.bold);
                    draw_text_ui("JOGO PAUSADO", (sw - p_dim.width) * 0.5, by + 42.0, 26.0, TEXT_PRIMARY, &fonts.bold);

                    let sub_dim = measure_text_ui("Pressione [P] ou clique em Retomar para continuar", 13.0, &fonts.regular);
                    draw_text_ui("Pressione [P] ou clique em Retomar para continuar", (sw - sub_dim.width) * 0.5, by + 74.0, 13.0, TEXT_MUTED, &fonts.regular);
                }

                // Overlay de Fim de Jogo
                if game.is_game_over() {
                    let box_w = 460.0;
                    let box_h = 130.0;
                    let bx = (sw - box_w) * 0.5;
                    let by = (sh - box_h) * 0.5;
                    draw_card_rect(bx, by, box_w, box_h, SURFACE_COLOR, CORAL);

                    let go_dim = measure_text_ui("FIM DE JOGO", 32.0, &fonts.bold);
                    draw_text_ui("FIM DE JOGO", (sw - go_dim.width) * 0.5, by + 46.0, 32.0, CORAL, &fonts.bold);

                    let score_msg = format!("Pontuação Final: {}", game.get_score());
                    let s_dim = measure_text_ui(&score_msg, 16.0, &fonts.bold);
                    draw_text_ui(&score_msg, (sw - s_dim.width) * 0.5, by + 78.0, 16.0, YELLOW_ACCENT, &fonts.bold);

                    let r_msg = "Pressione [R] para reiniciar ou [ESC] para voltar à galeria";
                    let r_dim = measure_text_ui(r_msg, 12.0, &fonts.regular);
                    draw_text_ui(r_msg, (sw - r_dim.width) * 0.5, by + 104.0, 12.0, TEXT_PRIMARY, &fonts.regular);

                    if is_key_pressed(KeyCode::R) {
                        game.reset();
                    }
                }
            }
        }

        if let Some(s) = next_state {
            state = s;
        }

        // Notificação Toast
        if let Some((msg, _)) = &toast_message {
            let t_dim = measure_text_ui(msg, 13.0, &fonts.bold);
            let tw = t_dim.width + 36.0;
            let th = 36.0;
            let tx = (sw - tw) * 0.5;
            let ty = sh - 56.0;

            draw_card_rect(tx, ty, tw, th, Color::new(0.08, 0.14, 0.24, 0.95), ICE_BLUE);
            draw_circle(tx + 18.0, ty + 18.0, 4.0, EMERALD);
            draw_text_ui(msg, tx + 30.0, ty + 23.0, 13.0, TEXT_PRIMARY, &fonts.bold);
        }

        next_frame().await;
    }
}
