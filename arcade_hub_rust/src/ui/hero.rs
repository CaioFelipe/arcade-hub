use crate::models::GameMeta;
use crate::theme::*;
use macroquad::prelude::*;

pub enum HeroAction {
    None,
    PlayGame(String),
    OpenDetails(String),
}

pub fn draw_hero(
    x: f32,
    y: f32,
    w: f32,
    featured: &GameMeta,
    mouse_pos: Vec2,
    fonts: &AppFonts,
) -> HeroAction {
    let hero_h = 230.0;

    // Fundo do Card Hero
    draw_card_rect(x, y, w, hero_h, SURFACE_COLOR, BORDER_COLOR);

    // Destaque gráfico no fundo
    draw_circle(x + w - 160.0, y + 115.0, 90.0, Color::new(0.14, 0.39, 0.92, 0.08));
    draw_circle(x + w - 160.0, y + 115.0, 50.0, Color::new(0.38, 0.65, 0.98, 0.12));

    // Nave Espacial Vetorial Estilizada no lado direito
    let ship_cx = x + w - 160.0;
    let ship_cy = y + 115.0;
    draw_triangle(
        Vec2::new(ship_cx, ship_cy - 45.0),
        Vec2::new(ship_cx - 35.0, ship_cy + 35.0),
        Vec2::new(ship_cx, ship_cy + 18.0),
        COBALT_ACCENT,
    );
    draw_triangle(
        Vec2::new(ship_cx, ship_cy - 45.0),
        Vec2::new(ship_cx + 35.0, ship_cy + 35.0),
        Vec2::new(ship_cx, ship_cy + 18.0),
        COBALT_ACCENT,
    );
    draw_circle(ship_cx, ship_cy - 8.0, 8.0, ICE_BLUE);
    draw_circle(ship_cx, ship_cy - 8.0, 3.5, TEXT_PRIMARY);

    // Textos do Hero
    let text_x = x + 36.0;
    draw_text_ui("DESTAQUE DA SEMANA", text_x, y + 36.0, 11.0, ICE_BLUE, &fonts.bold);
    draw_text_ui(&featured.title, text_x, y + 74.0, 32.0, TEXT_PRIMARY, &fonts.bold);

    // Descrição resumida
    draw_text_ui(&featured.description, text_x, y + 104.0, 14.0, TEXT_MUTED, &fonts.regular);

    let auth_text = format!("Desenvolvido por: {}  |  Licença: {}", featured.author, featured.license);
    draw_text_ui(&auth_text, text_x, y + 132.0, 12.0, Color::new(0.6, 0.7, 0.8, 0.9), &fonts.regular);

    // Botões de Ação
    let btn_play_w = 150.0;
    let btn_h = 38.0;
    let btn_y = y + 162.0;

    let play_clicked = draw_button_ui(text_x, btn_y, btn_play_w, btn_h, "Jogar Agora", true, mouse_pos, &fonts.bold);
    let details_clicked = draw_button_ui(text_x + btn_play_w + 14.0, btn_y, 130.0, btn_h, "Ficha Técnica", false, mouse_pos, &fonts.regular);

    if play_clicked {
        HeroAction::PlayGame(featured.id.clone())
    } else if details_clicked {
        HeroAction::OpenDetails(featured.id.clone())
    } else {
        HeroAction::None
    }
}
