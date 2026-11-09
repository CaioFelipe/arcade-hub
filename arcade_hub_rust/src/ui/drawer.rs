use crate::models::GameMeta;
use crate::theme::*;
use macroquad::prelude::*;

pub enum DrawerAction {
    None,
    Close,
    Play(String),
}

pub fn draw_details_drawer(
    width: f32,
    height: f32,
    game: &GameMeta,
    mouse_pos: Vec2,
    fonts: &AppFonts,
) -> DrawerAction {
    // Backdrop semitransparente
    draw_rectangle(0.0, 0.0, width, height, Color::new(0.0, 0.0, 0.0, 0.65));

    // Gaveta à direita
    let drawer_w = 460.0_f32.min(width * 0.9);
    let drawer_x = width - drawer_w;

    draw_card_rect(drawer_x, 0.0, drawer_w, height, SURFACE_COLOR, BORDER_COLOR);

    let pad = 28.0;
    let mut y = 32.0;

    // Cabeçalho da Gaveta
    draw_text_ui(&game.title, drawer_x + pad, y + 24.0, 26.0, TEXT_PRIMARY, &fonts.bold);
    draw_text_ui(&game.category, drawer_x + pad, y + 46.0, 13.0, YELLOW_ACCENT, &fonts.regular);

    // Botão Fechar com X Vetorial
    let close_x = width - 40.0;
    let close_y = y + 16.0;
    let is_close_hovered = mouse_pos.x >= close_x - 12.0 && mouse_pos.x <= close_x + 12.0 &&
                           mouse_pos.y >= close_y - 12.0 && mouse_pos.y <= close_y + 12.0;

    draw_icon_close(close_x, close_y, 14.0, if is_close_hovered { CORAL } else { TEXT_MUTED });

    y += 66.0;

    // Ficha Técnica e Créditos Oficiais
    draw_card_rect(drawer_x + pad, y, drawer_w - pad * 2.0, 110.0, BG_COLOR, BORDER_COLOR);
    draw_text_ui(&format!("Autor Original: {}", game.author), drawer_x + pad + 16.0, y + 26.0, 13.0, TEXT_PRIMARY, &fonts.bold);
    draw_text_ui(&format!("Licença de Software: {}", game.license), drawer_x + pad + 16.0, y + 48.0, 12.0, ICE_BLUE, &fonts.regular);
    draw_text_ui(&format!("Coleção: {}", game.collection), drawer_x + pad + 16.0, y + 70.0, 12.0, TEXT_MUTED, &fonts.regular);
    draw_text_ui("Arquitetura: 100% Rust Puro  |  Engine: Macroquad 60+ FPS", drawer_x + pad + 16.0, y + 92.0, 11.0, EMERALD, &fonts.regular);

    y += 122.0;

    // Botão Link direto para o GitHub do Autor
    let gh_btn_w = drawer_w - pad * 2.0;
    let gh_btn_h = 34.0;
    let is_gh_hovered = mouse_pos.x >= drawer_x + pad && mouse_pos.x <= drawer_x + pad + gh_btn_w &&
                        mouse_pos.y >= y && mouse_pos.y <= y + gh_btn_h;

    let gh_bg = if is_gh_hovered { Color::new(0.12, 0.20, 0.35, 1.0) } else { Color::new(0.08, 0.12, 0.22, 1.0) };
    let gh_border = if is_gh_hovered { ICE_BLUE } else { BORDER_COLOR };

    draw_card_rect(drawer_x + pad, y, gh_btn_w, gh_btn_h, gh_bg, gh_border);
    draw_icon_github(drawer_x + pad + 24.0, y + 17.0, 15.0, ICE_BLUE);
    draw_text_ui("Acessar Repositório Oficial no GitHub", drawer_x + pad + 44.0, y + 22.0, 12.0, ICE_BLUE, &fonts.bold);
    draw_icon_external_link(drawer_x + drawer_w - pad - 24.0, y + 17.0, 11.0, ICE_BLUE);

    if is_gh_hovered && is_mouse_button_pressed(MouseButton::Left) {
        open_browser_url(&game.repo_url);
    }

    y += 48.0;

    // Descrição
    draw_text_ui("Descrição da Mecânica", drawer_x + pad, y + 18.0, 15.0, ICE_BLUE, &fonts.bold);
    y += 28.0;

    let desc_len = game.description.len();
    let split_pos = if desc_len > 48 { 48 } else { desc_len };
    let line1 = &game.description[..split_pos];
    let line2 = if desc_len > 48 { &game.description[split_pos..] } else { "" };

    draw_text_ui(line1, drawer_x + pad, y + 14.0, 13.0, Color::new(0.75, 0.80, 0.88, 1.0), &fonts.regular);
    if !line2.is_empty() {
        draw_text_ui(line2, drawer_x + pad, y + 32.0, 13.0, Color::new(0.75, 0.80, 0.88, 1.0), &fonts.regular);
        y += 18.0;
    }
    y += 30.0;

    // Comandos de Controle
    draw_text_ui("Comandos de Controle", drawer_x + pad, y + 18.0, 15.0, ICE_BLUE, &fonts.bold);
    y += 28.0;

    draw_card_rect(drawer_x + pad, y, drawer_w - pad * 2.0, 84.0, BG_COLOR, BORDER_COLOR);
    draw_text_ui(&format!("Movimento: {}", game.controls_move), drawer_x + pad + 16.0, y + 28.0, 13.0, TEXT_PRIMARY, &fonts.bold);
    draw_text_ui(&format!("Ação / Disparo: {}", game.controls_action), drawer_x + pad + 16.0, y + 54.0, 13.0, YELLOW_ACCENT, &fonts.regular);

    // Botão Jogar no Rodapé
    let btn_w = drawer_w - pad * 2.0;
    let play_clicked = draw_button_ui(drawer_x + pad, height - 70.0, btn_w, 42.0, "Iniciar Jogo no Palco", true, mouse_pos, &fonts.bold);

    // Fechar ao clicar fora, no X ou no ESC
    if is_key_pressed(KeyCode::Escape) || (is_close_hovered && is_mouse_button_pressed(MouseButton::Left)) {
        return DrawerAction::Close;
    }

    if is_mouse_button_pressed(MouseButton::Left) && mouse_pos.x < drawer_x {
        return DrawerAction::Close;
    }

    if play_clicked {
        DrawerAction::Play(game.id.clone())
    } else {
        DrawerAction::None
    }
}
