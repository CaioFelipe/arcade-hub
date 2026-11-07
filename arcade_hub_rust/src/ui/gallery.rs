use crate::models::GameMeta;
use crate::theme::*;
use macroquad::prelude::*;

pub enum GalleryAction {
    None,
    PlayGame(String),
    OpenDetails(String),
    FilterCategory(String),
    FilterCollection(String),
    OpenUrl(String),
}

pub fn draw_gallery(
    x: f32,
    y: f32,
    w: f32,
    games: &[GameMeta],
    current_category: &str,
    current_collection: &str,
    mouse_pos: Vec2,
    fonts: &AppFonts,
) -> GalleryAction {
    let mut action = GalleryAction::None;
    let mut curr_y = y;

    // Seção de Coleções Open-Source em Destaque
    draw_text_ui("Coleções Abertas & Autores Oficiais no GitHub", x, curr_y + 18.0, 18.0, TEXT_PRIMARY, &fonts.bold);
    draw_text_ui(
        "Projetos originais em Python com link oficial para os repositórios no GitHub dos autores",
        x,
        curr_y + 36.0,
        13.0,
        TEXT_MUTED,
        &fonts.regular,
    );
    curr_y += 50.0;

    let col_w = (w - 28.0) / 3.0;
    let collections = [
        (
            "Grant Jenks Collection",
            "grant_jenks",
            "Free Python Games (Apache 2.0)",
            "Snake, Tron, Cannon, Tiles, Pac-Man",
            "https://github.com/grantjenks/free-python-games",
        ),
        (
            "Al Sweigart Collection",
            "al_sweigart",
            "Invent with Python (Simplified BSD)",
            "Breakout, Four in a Row, Clássicos",
            "https://github.com/asweigart/inventwithpython",
        ),
        (
            "Comunidade Pygame",
            "pygame_community",
            "Comunidade Aberta (LGPL / MIT)",
            "Space Odyssey, Pong, Flappy, Minesweeper",
            "https://github.com/pygame/pygame",
        ),
    ];

    for (idx, (c_name, c_id, c_sub, c_games, c_url)) in collections.iter().enumerate() {
        let cx = x + (idx as f32) * (col_w + 14.0);
        let card_h = 104.0;
        let is_selected = current_collection == *c_id;

        let is_card_hovered = mouse_pos.x >= cx && mouse_pos.x <= cx + col_w &&
                              mouse_pos.y >= curr_y && mouse_pos.y <= curr_y + card_h;

        let bg_color = if is_selected {
            Color::new(0.08, 0.14, 0.25, 1.0)
        } else if is_card_hovered {
            SURFACE_HOVER
        } else {
            SURFACE_COLOR
        };

        let border_color = if is_selected {
            ICE_BLUE
        } else if is_card_hovered {
            BORDER_ACTIVE
        } else {
            BORDER_COLOR
        };

        draw_card_rect(cx, curr_y, col_w, card_h, bg_color, border_color);

        // Ícone GitHub
        draw_icon_github(cx + 26.0, curr_y + 26.0, 16.0, ICE_BLUE);

        // Título e subtexto
        draw_text_ui(c_name, cx + 44.0, curr_y + 26.0, 15.0, TEXT_PRIMARY, &fonts.bold);
        draw_text_ui(c_sub, cx + 44.0, curr_y + 44.0, 11.0, YELLOW_ACCENT, &fonts.regular);
        draw_text_ui(c_games, cx + 18.0, curr_y + 64.0, 11.0, TEXT_MUTED, &fonts.regular);

        // Botão "Abrir GitHub" dentro do card
        let gh_btn_w = 120.0;
        let gh_btn_h = 24.0;
        let gh_btn_x = cx + col_w - gh_btn_w - 12.0;
        let gh_btn_y = curr_y + card_h - 32.0;

        let is_gh_hovered = mouse_pos.x >= gh_btn_x && mouse_pos.x <= gh_btn_x + gh_btn_w &&
                            mouse_pos.y >= gh_btn_y && mouse_pos.y <= gh_btn_y + gh_btn_h;

        let gh_bg = if is_gh_hovered { Color::new(0.18, 0.28, 0.45, 1.0) } else { Color::new(0.10, 0.15, 0.25, 1.0) };
        let gh_border = if is_gh_hovered { ICE_BLUE } else { BORDER_COLOR };
        draw_card_rect(gh_btn_x, gh_btn_y, gh_btn_w, gh_btn_h, gh_bg, gh_border);
        draw_text_ui("GitHub ↗", gh_btn_x + 14.0, gh_btn_y + 16.0, 11.0, ICE_BLUE, &fonts.bold);
        draw_icon_external_link(gh_btn_x + gh_btn_w - 18.0, gh_btn_y + 12.0, 10.0, ICE_BLUE);

        if is_gh_hovered && is_mouse_button_pressed(MouseButton::Left) {
            action = GalleryAction::OpenUrl(c_url.to_string());
        } else if is_card_hovered && is_mouse_button_pressed(MouseButton::Left) {
            // Clicar no card filtra os jogos por essa coleção
            if is_selected {
                action = GalleryAction::FilterCollection("all".into());
            } else {
                action = GalleryAction::FilterCollection(c_id.to_string());
            }
        }
    }
    curr_y += 124.0;

    // Barra de Filtros por Categoria
    let categories = [
        "Todos",
        "Arcade Clássico",
        "Ação & Reflexos",
        "Puzzle & Raciocínio",
        "Estratégia & Tabuleiro",
    ];

    let mut chip_x = x;
    for cat in categories {
        let is_selected = (cat == "Todos" && current_category == "all") || (cat == current_category);
        let label_dim = measure_text_ui(cat, 13.0, &fonts.regular);
        let chip_w = label_dim.width + 24.0;
        let chip_h = 30.0;

        let is_hovered = mouse_pos.x >= chip_x && mouse_pos.x <= chip_x + chip_w &&
                         mouse_pos.y >= curr_y && mouse_pos.y <= curr_y + chip_h;

        let (bg, border, text_col) = if is_selected {
            (COBALT_ACCENT, ICE_BLUE, TEXT_PRIMARY)
        } else if is_hovered {
            (SURFACE_HOVER, BORDER_ACTIVE, TEXT_PRIMARY)
        } else {
            (SURFACE_COLOR, BORDER_COLOR, TEXT_MUTED)
        };

        draw_card_rect(chip_x, curr_y, chip_w, chip_h, bg, border);
        draw_text_ui(
            cat,
            chip_x + 12.0,
            curr_y + (chip_h + label_dim.height) * 0.5 - 2.0,
            13.0,
            text_col,
            if is_selected { &fonts.bold } else { &fonts.regular },
        );

        if is_hovered && is_mouse_button_pressed(MouseButton::Left) {
            action = GalleryAction::FilterCategory(if cat == "Todos" { "all".into() } else { cat.into() });
        }

        chip_x += chip_w + 10.0;
    }
    curr_y += 46.0;

    // Grid de Jogos (Cards Responsivos)
    let card_cols = 3;
    let card_w = (w - (card_cols as f32 - 1.0) * 16.0) / (card_cols as f32);
    let card_h = 168.0;

    for (i, game) in games.iter().enumerate() {
        let row = i / card_cols;
        let col = i % card_cols;
        let gx = x + (col as f32) * (card_w + 16.0);
        let gy = curr_y + (row as f32) * (card_h + 16.0);

        let is_hovered = mouse_pos.x >= gx && mouse_pos.x <= gx + card_w &&
                         mouse_pos.y >= gy && mouse_pos.y <= gy + card_h;

        let border_col = if is_hovered { BORDER_ACTIVE } else { BORDER_COLOR };
        draw_card_rect(gx, gy, card_w, card_h, SURFACE_COLOR, border_col);

        // Ícone decorativo vetorial
        draw_card_rect(gx + 16.0, gy + 16.0, 36.0, 36.0, Color::new(0.08, 0.12, 0.22, 1.0), BORDER_COLOR);
        draw_icon_play(gx + 34.0, gy + 34.0, 14.0, ICE_BLUE);

        // Título e Categoria
        draw_text_ui(&game.title, gx + 62.0, gy + 28.0, 17.0, TEXT_PRIMARY, &fonts.bold);
        draw_text_ui(&game.category, gx + 62.0, gy + 46.0, 11.0, YELLOW_ACCENT, &fonts.regular);

        // Autor
        let auth_str = format!("Por: {}", game.author);
        draw_text_ui(&auth_str, gx + 16.0, gy + 74.0, 12.0, TEXT_MUTED, &fonts.regular);

        // Descrição resumida
        let desc = if game.description.len() > 65 {
            format!("{}...", &game.description[..62])
        } else {
            game.description.clone()
        };
        draw_text_ui(&desc, gx + 16.0, gy + 96.0, 12.0, Color::new(0.70, 0.75, 0.85, 0.9), &fonts.regular);

        // Botões no rodapé do Card
        let btn_play_w = 95.0;
        let btn_h = 28.0;
        let btn_y = gy + card_h - 38.0;

        if draw_button_ui(gx + 16.0, btn_y, btn_play_w, btn_h, "Jogar", true, mouse_pos, &fonts.bold) {
            action = GalleryAction::PlayGame(game.id.clone());
        }

        if draw_button_ui(gx + 16.0 + btn_play_w + 8.0, btn_y, 80.0, btn_h, "Ficha", false, mouse_pos, &fonts.regular) {
            action = GalleryAction::OpenDetails(game.id.clone());
        }
    }

    action
}
