use crate::models::GameMeta;
use crate::theme::*;
use macroquad::prelude::*;

pub enum AddModalAction {
    None,
    Close,
    Save(GameMeta),
}

pub struct AddModalState {
    pub title: String,
    pub author: String,
    pub category: String,
    pub description: String,
    pub controls_move: String,
    pub controls_action: String,
    pub active_field: usize, // 0: title, 1: author, 2: desc, 3: controls
}

impl Default for AddModalState {
    fn default() -> Self {
        Self {
            title: String::new(),
            author: String::new(),
            category: "Ação & Reflexos".into(),
            description: String::new(),
            controls_move: "Setas ou WASD".into(),
            controls_action: "Espaço ou Clique".into(),
            active_field: 0,
        }
    }
}

pub fn draw_add_modal(
    width: f32,
    height: f32,
    state: &mut AddModalState,
    mouse_pos: Vec2,
    fonts: &AppFonts,
) -> AddModalAction {
    // Backdrop
    draw_rectangle(0.0, 0.0, width, height, Color::new(0.0, 0.0, 0.0, 0.70));

    let modal_w = 560.0_f32.min(width * 0.9);
    let modal_h = 490.0_f32.min(height * 0.9);
    let modal_x = (width - modal_w) * 0.5;
    let modal_y = (height - modal_h) * 0.5;

    draw_card_rect(modal_x, modal_y, modal_w, modal_h, SURFACE_COLOR, BORDER_COLOR);

    // Header do Modal
    let pad = 28.0;
    let mut y = modal_y + 24.0;
    draw_text_ui("Estúdio de Criação", modal_x + pad, y + 14.0, 11.0, ICE_BLUE, &fonts.bold);
    draw_text_ui("Adicionar Novo Jogo ao Hub", modal_x + pad, y + 38.0, 22.0, TEXT_PRIMARY, &fonts.bold);

    // Botão Fechar com X Vetorial
    let close_x = modal_x + modal_w - 32.0;
    let close_y = y + 16.0;
    let is_close_hovered = mouse_pos.x >= close_x - 12.0 && mouse_pos.x <= close_x + 12.0 &&
                           mouse_pos.y >= close_y - 12.0 && mouse_pos.y <= close_y + 12.0;

    draw_icon_close(close_x, close_y, 14.0, if is_close_hovered { CORAL } else { TEXT_MUTED });

    y += 65.0;

    // Captura digitação no campo ativo
    while let Some(c) = get_char_pressed() {
        if c >= ' ' && c <= '~' {
            match state.active_field {
                0 => state.title.push(c),
                1 => state.author.push(c),
                2 => state.description.push(c),
                3 => state.controls_move.push(c),
                _ => {}
            }
        }
    }
    if is_key_pressed(KeyCode::Backspace) {
        match state.active_field {
            0 => { state.title.pop(); }
            1 => { state.author.pop(); }
            2 => { state.description.pop(); }
            3 => { state.controls_move.pop(); }
            _ => {}
        }
    }
    if is_key_pressed(KeyCode::Tab) {
        state.active_field = (state.active_field + 1) % 4;
    }

    // Campos de Texto
    let field_w = modal_w - pad * 2.0;
    let field_h = 36.0;

    for idx in 0..4 {
        let (label, val_str) = match idx {
            0 => ("Nome do Jogo", state.title.as_str()),
            1 => ("Autor / Desenvolvedor", state.author.as_str()),
            2 => ("Descrição Resumida", state.description.as_str()),
            3 => ("Comandos de Controle", state.controls_move.as_str()),
            _ => ("", ""),
        };

        draw_text_ui(label, modal_x + pad, y + 14.0, 12.0, TEXT_MUTED, &fonts.regular);
        y += 20.0;

        let is_active = state.active_field == idx;
        let is_hovered = mouse_pos.x >= modal_x + pad && mouse_pos.x <= modal_x + pad + field_w &&
                         mouse_pos.y >= y && mouse_pos.y <= y + field_h;

        if is_hovered && is_mouse_button_pressed(MouseButton::Left) {
            state.active_field = idx;
        }

        let border = if is_active { ICE_BLUE } else { BORDER_COLOR };
        draw_card_rect(modal_x + pad, y, field_w, field_h, BG_COLOR, border);

        let display_val = if val_str.is_empty() && !is_active {
            "Clique para preencher..."
        } else {
            val_str
        };
        let val_col = if val_str.is_empty() && !is_active { Color::new(0.4, 0.45, 0.55, 0.6) } else { TEXT_PRIMARY };
        draw_text_ui(display_val, modal_x + pad + 12.0, y + 23.0, 13.0, val_col, &fonts.regular);

        y += field_h + 16.0;
    }

    // Botões no rodapé
    let btn_save_w = 190.0;
    let btn_h = 38.0;
    let btn_y = modal_y + modal_h - 58.0;

    let save_clicked = draw_button_ui(
        modal_x + modal_w - pad - btn_save_w,
        btn_y,
        btn_save_w,
        btn_h,
        "Salvar no Hub",
        true,
        mouse_pos,
        &fonts.bold,
    );

    let cancel_clicked = draw_button_ui(
        modal_x + pad,
        btn_y,
        110.0,
        btn_h,
        "Cancelar",
        false,
        mouse_pos,
        &fonts.regular,
    );

    if is_key_pressed(KeyCode::Escape) || cancel_clicked || (is_close_hovered && is_mouse_button_pressed(MouseButton::Left)) {
        return AddModalAction::Close;
    }

    if save_clicked && !state.title.trim().is_empty() {
        let slug = state.title.to_lowercase().replace(' ', "_");
        let new_meta = GameMeta {
            id: slug,
            title: state.title.clone(),
            author: if state.author.trim().is_empty() { "Comunidade".into() } else { state.author.clone() },
            category: state.category.clone(),
            description: if state.description.trim().is_empty() { "Jogo comunitário cadastrado no Arcade Hub.".into() } else { state.description.clone() },
            controls_move: state.controls_move.clone(),
            controls_action: state.controls_action.clone(),
            collection: "Comunidade Local".into(),
            license: "MIT".into(),
            repo_url: "#".into(),
            featured: false,
        };
        return AddModalAction::Save(new_meta);
    }

    AddModalAction::None
}
