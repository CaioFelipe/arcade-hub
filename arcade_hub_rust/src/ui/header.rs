use crate::theme::*;
use macroquad::prelude::*;

pub struct HeaderAction {
    pub add_game_clicked: bool,
    pub search_query_changed: bool,
}

pub fn draw_header(
    width: f32,
    search_query: &mut String,
    is_search_active: &mut bool,
    mouse_pos: Vec2,
    fonts: &AppFonts,
) -> HeaderAction {
    let header_h = 64.0;

    // Barra superior
    draw_card_rect(0.0, 0.0, width, header_h, SURFACE_COLOR, BORDER_COLOR);

    // Marca / Logo Vetorial
    let logo_x = 28.0;
    let logo_y = 32.0;
    draw_circle(logo_x, logo_y, 14.0, COBALT_ACCENT);
    draw_circle(logo_x - 4.0, logo_y, 3.5, TEXT_PRIMARY);
    draw_circle(logo_x + 4.0, logo_y, 3.5, YELLOW_ACCENT);

    draw_text_ui("ARCADE HUB", logo_x + 24.0, logo_y + 6.0, 18.0, TEXT_PRIMARY, &fonts.bold);

    // Campo de Busca
    let search_w = (width * 0.28).clamp(220.0, 360.0);
    let search_x = logo_x + 190.0;
    let search_y = 16.0;
    let search_h = 32.0;

    let is_search_hovered = mouse_pos.x >= search_x && mouse_pos.x <= search_x + search_w &&
                            mouse_pos.y >= search_y && mouse_pos.y <= search_y + search_h;

    if is_mouse_button_pressed(MouseButton::Left) {
        *is_search_active = is_search_hovered;
    }

    let search_border = if *is_search_active { ICE_BLUE } else { BORDER_COLOR };
    draw_card_rect(search_x, search_y, search_w, search_h, BG_COLOR, search_border);

    // Ícone de Lupa Vetorial
    let icon_cx = search_x + 16.0;
    let icon_cy = search_y + 16.0;
    draw_circle_lines(icon_cx, icon_cy, 5.0, 1.5, TEXT_MUTED);
    draw_line(icon_cx + 4.0, icon_cy + 4.0, icon_cx + 8.0, icon_cy + 8.0, 1.5, TEXT_MUTED);

    // Texto da busca ou placeholder
    if search_query.is_empty() {
        draw_text_ui("Buscar jogos... (Ctrl+K)", search_x + 30.0, search_y + 21.0, 13.0, TEXT_MUTED, &fonts.regular);
    } else {
        draw_text_ui(search_query.as_str(), search_x + 30.0, search_y + 21.0, 13.0, TEXT_PRIMARY, &fonts.regular);
    }

    // Input de teclado para a busca
    let mut search_changed = false;
    if *is_search_active {
        while let Some(c) = get_char_pressed() {
            if c >= ' ' && c <= '~' {
                search_query.push(c);
                search_changed = true;
            }
        }
        if is_key_pressed(KeyCode::Backspace) && !search_query.is_empty() {
            search_query.pop();
            search_changed = true;
        }
    }

    // Botão "+ Adicionar Jogo"
    let btn_w = 155.0;
    let btn_h = 34.0;
    let btn_x = width - btn_w - 28.0;
    let btn_y = 15.0;

    let add_clicked = draw_button_ui(btn_x, btn_y, btn_w, btn_h, "+ Adicionar Jogo", true, mouse_pos, &fonts.bold);

    HeaderAction {
        add_game_clicked: add_clicked,
        search_query_changed: search_changed,
    }
}
