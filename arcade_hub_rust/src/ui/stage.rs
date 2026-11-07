use crate::theme::*;
use macroquad::prelude::*;

pub enum StageAction {
    None,
    BackToGallery,
    TogglePause,
    Restart,
}

pub fn draw_stage_header(
    width: f32,
    title: &str,
    category: &str,
    author: &str,
    score: u32,
    is_paused: bool,
    mouse_pos: Vec2,
    fonts: &AppFonts,
) -> StageAction {
    let bar_h = 56.0;

    // Barra Superior do Console
    draw_card_rect(0.0, 0.0, width, bar_h, SURFACE_COLOR, BORDER_COLOR);

    // Botão Voltar (ESC) com ícone de seta vetorial
    let back_w = 175.0;
    let back_h = 32.0;
    let back_x = 20.0;
    let back_y = 12.0;

    let is_back_hovered = mouse_pos.x >= back_x && mouse_pos.x <= back_x + back_w &&
                          mouse_pos.y >= back_y && mouse_pos.y <= back_y + back_h;

    let (bg, border, text_col) = if is_back_hovered {
        (SURFACE_HOVER, BORDER_ACTIVE, TEXT_PRIMARY)
    } else {
        (SURFACE_COLOR, BORDER_COLOR, TEXT_MUTED)
    };
    draw_card_rect(back_x, back_y, back_w, back_h, bg, border);
    draw_icon_arrow_left(back_x + 18.0, back_y + 16.0, 14.0, text_col);
    draw_text_ui("Voltar à Galeria (ESC)", back_x + 34.0, back_y + 21.0, 12.0, text_col, &fonts.bold);
    let back_clicked = is_back_hovered && is_mouse_button_pressed(MouseButton::Left);

    // Título do Jogo e Autor
    let title_x = 215.0;
    draw_text_ui(title, title_x, 24.0, 16.0, TEXT_PRIMARY, &fonts.bold);
    draw_text_ui(
        &format!("{}  |  Autor: {}", category, author),
        title_x,
        42.0,
        11.0,
        TEXT_MUTED,
        &fonts.regular,
    );

    // Direita: Placar e Botões
    let mut right_x = width - 20.0;

    // Botão Reiniciar (R)
    let r_w = 110.0;
    right_x -= r_w;
    let restart_clicked = draw_button_ui(right_x, 12.0, r_w, back_h, "Reiniciar (R)", false, mouse_pos, &fonts.regular);

    // Botão Pausar (P)
    let p_w = 100.0;
    right_x -= p_w + 10.0;
    let pause_label = if is_paused { "Retomar (P)" } else { "Pausar (P)" };
    let pause_clicked = draw_button_ui(right_x, 12.0, p_w, back_h, pause_label, false, mouse_pos, &fonts.regular);

    // Placar ao Vivo
    let score_str = format!("PONTOS: {}", score);
    let s_dim = measure_text_ui(&score_str, 15.0, &fonts.bold);
    right_x -= s_dim.width + 30.0;
    draw_card_rect(right_x, 12.0, s_dim.width + 20.0, back_h, BG_COLOR, BORDER_COLOR);
    draw_text_ui(&score_str, right_x + 10.0, 32.0, 15.0, YELLOW_ACCENT, &fonts.bold);

    // Teclas globais
    if is_key_pressed(KeyCode::Escape) || back_clicked {
        StageAction::BackToGallery
    } else if is_key_pressed(KeyCode::P) || pause_clicked {
        StageAction::TogglePause
    } else if is_key_pressed(KeyCode::R) || restart_clicked {
        StageAction::Restart
    } else {
        StageAction::None
    }
}
