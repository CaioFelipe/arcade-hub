use macroquad::prelude::*;

// Paleta Oficial Safira Escuro + Neo-Retro 1px
pub const BG_COLOR: Color = Color::new(0.035, 0.047, 0.078, 1.0);       // #090C14
pub const SURFACE_COLOR: Color = Color::new(0.063, 0.082, 0.133, 1.0);  // #101522
pub const SURFACE_HOVER: Color = Color::new(0.090, 0.118, 0.184, 1.0);  // #171E2F
pub const BORDER_COLOR: Color = Color::new(0.12, 0.16, 0.23, 1.0);      // #1F2937
pub const BORDER_ACTIVE: Color = Color::new(0.25, 0.45, 0.90, 0.6);

pub const COBALT_ACCENT: Color = Color::new(0.145, 0.388, 0.921, 1.0); // #2563EB
pub const ICE_BLUE: Color = Color::new(0.376, 0.647, 0.980, 1.0);      // #60A5FA
pub const TEXT_PRIMARY: Color = Color::new(0.973, 0.980, 0.988, 1.0);  // #F8FAFC
pub const TEXT_MUTED: Color = Color::new(0.580, 0.639, 0.722, 1.0);    // #94A3B8
pub const EMERALD: Color = Color::new(0.063, 0.725, 0.506, 1.0);       // #10B981
pub const YELLOW_ACCENT: Color = Color::new(0.980, 0.800, 0.082, 1.0); // #FACC15
pub const CORAL: Color = Color::new(0.957, 0.247, 0.369, 1.0);         // #F43F5E

pub struct AppFonts {
    pub regular: Font,
    pub bold: Font,
}

impl AppFonts {
    pub fn load() -> Self {
        let regular_bytes = include_bytes!("../assets/font.ttf");
        let bold_bytes = include_bytes!("../assets/font_bold.ttf");

        let regular = load_ttf_font_from_bytes(regular_bytes).unwrap();
        let bold = load_ttf_font_from_bytes(bold_bytes).unwrap();

        Self { regular, bold }
    }
}

/// Renderiza texto em alta definição com rasterização vetorial nítida (subpixel/supersampled)
pub fn draw_text_ui(
    text: &str,
    x: f32,
    y: f32,
    size: f32,
    color: Color,
    font: &Font,
) {
    let scale_factor = 1.25;
    draw_text_ex(
        text,
        x,
        y,
        TextParams {
            font: Some(font),
            font_size: (size * scale_factor) as u16,
            font_scale: 1.0 / scale_factor,
            font_scale_aspect: 1.0,
            rotation: 0.0,
            color,
        },
    );
}

/// Mede a dimensão exata do texto com a fonte vetorial
pub fn measure_text_ui(text: &str, size: f32, font: &Font) -> TextDimensions {
    let scale_factor = 1.25;
    let mut dim = measure_text(text, Some(font), (size * scale_factor) as u16, 1.0 / scale_factor);
    dim.height = size;
    dim
}

/// Abre uma URL no navegador padrão do Windows sem bibliotecas externas
pub fn open_browser_url(url: &str) {
    if !url.is_empty() && url.starts_with("http") {
        let _ = std::process::Command::new("rundll32")
            .args(["url.dll,FileProtocolHandler", url])
            .spawn();
    }
}

/// Desenha um retângulo com borda de 1px
pub fn draw_card_rect(x: f32, y: f32, w: f32, h: f32, bg: Color, border: Color) {
    draw_rectangle(x, y, w, h, bg);
    draw_rectangle_lines(x, y, w, h, 1.0, border);
}

/// Desenha o grid de coordenadas de fundo
pub fn draw_background_grid(width: f32, height: f32) {
    clear_background(BG_COLOR);
    let step = 48.0;
    let grid_color = Color::new(1.0, 1.0, 1.0, 0.025);

    let mut x = 0.0;
    while x <= width {
        draw_line(x, 0.0, x, height, 1.0, grid_color);
        x += step;
    }

    let mut y = 0.0;
    while y <= height {
        draw_line(0.0, y, width, y, 1.0, grid_color);
        y += step;
    }

    // Pontos estelares sutis
    let dots = [
        (120.0, 80.0, Color::new(0.38, 0.65, 0.98, 0.15)),
        (340.0, 220.0, Color::new(0.98, 0.80, 0.08, 0.12)),
        (850.0, 140.0, Color::new(0.06, 0.73, 0.51, 0.15)),
        (1100.0, 280.0, Color::new(0.38, 0.65, 0.98, 0.18)),
        (720.0, 520.0, Color::new(0.96, 0.25, 0.37, 0.12)),
        (180.0, 680.0, Color::new(0.38, 0.65, 0.98, 0.14)),
        (1040.0, 640.0, Color::new(0.98, 0.80, 0.08, 0.10)),
    ];
    for (dx, dy, color) in dots {
        if dx < width && dy < height {
            draw_circle(dx, dy, 2.0, color);
        }
    }
}

// ==========================================
// ÍCONES VETORIAIS GEOMÉTRICOS (100% NATIVOS)
// ==========================================

/// Ícone Fechar / X vetorial limpo
pub fn draw_icon_close(cx: f32, cy: f32, size: f32, color: Color) {
    let half = size * 0.5;
    draw_line(cx - half, cy - half, cx + half, cy + half, 2.0, color);
    draw_line(cx + half, cy - half, cx - half, cy + half, 2.0, color);
}

/// Ícone Voltar / Seta para a esquerda
pub fn draw_icon_arrow_left(cx: f32, cy: f32, size: f32, color: Color) {
    let half = size * 0.5;
    draw_line(cx - half, cy, cx + half, cy, 2.0, color);
    draw_line(cx - half, cy, cx - half * 0.3, cy - half * 0.7, 2.0, color);
    draw_line(cx - half, cy, cx - half * 0.3, cy + half * 0.7, 2.0, color);
}

/// Ícone Link Externo / GitHub (↗)
pub fn draw_icon_external_link(cx: f32, cy: f32, size: f32, color: Color) {
    let half = size * 0.5;
    // Caixa
    draw_line(cx - half, cy - half * 0.4, cx - half, cy + half, 1.5, color);
    draw_line(cx - half, cy + half, cx + half * 0.4, cy + half, 1.5, color);
    // Seta saindo
    draw_line(cx - half * 0.2, cy + half * 0.2, cx + half, cy - half, 1.5, color);
    draw_line(cx + half * 0.3, cy - half, cx + half, cy - half, 1.5, color);
    draw_line(cx + half, cy - half * 0.3, cx + half, cy - half, 1.5, color);
}

/// Ícone GitHub Emblema
pub fn draw_icon_github(cx: f32, cy: f32, size: f32, color: Color) {
    let r = size * 0.5;
    draw_circle_lines(cx, cy, r, 1.5, color);
    draw_circle(cx - r * 0.3, cy - r * 0.2, 1.5, color);
    draw_circle(cx + r * 0.3, cy - r * 0.2, 1.5, color);
    draw_line(cx - r * 0.4, cy + r * 0.3, cx + r * 0.4, cy + r * 0.3, 1.5, color);
}

/// Ícone Play
pub fn draw_icon_play(x: f32, y: f32, size: f32, color: Color) {
    let half = size * 0.5;
    draw_triangle(
        Vec2::new(x - half * 0.6, y - half),
        Vec2::new(x - half * 0.6, y + half),
        Vec2::new(x + half * 0.8, y),
        color,
    );
}

/// Ícone Pausa
pub fn draw_icon_pause(x: f32, y: f32, size: f32, color: Color) {
    let w = size * 0.25;
    let h = size;
    draw_rectangle(x - size * 0.35, y - h * 0.5, w, h, color);
    draw_rectangle(x + size * 0.10, y - h * 0.5, w, h, color);
}

/// Ícone Reiniciar
pub fn draw_icon_restart(x: f32, y: f32, size: f32, color: Color) {
    let r = size * 0.45;
    draw_circle_lines(x, y, r, 1.5, color);
    draw_triangle(
        Vec2::new(x + r, y - 2.0),
        Vec2::new(x + r + 4.0, y + 4.0),
        Vec2::new(x + r - 4.0, y + 4.0),
        color,
    );
}

/// Ícone Adicionar (+)
pub fn draw_icon_plus(x: f32, y: f32, size: f32, color: Color) {
    let half = size * 0.5;
    draw_line(x - half, y, x + half, y, 2.0, color);
    draw_line(x, y - half, x, y + half, 2.0, color);
}

/// Ícone Escudo / Verificação
pub fn draw_icon_shield(x: f32, y: f32, size: f32, color: Color) {
    let half = size * 0.5;
    draw_triangle(
        Vec2::new(x - half, y - half),
        Vec2::new(x + half, y - half),
        Vec2::new(x, y + half),
        color,
    );
}

/// Botão customizado com fonte vetorial e ícone opcional
pub fn draw_button_ui(
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    label: &str,
    primary: bool,
    mouse_pos: Vec2,
    font: &Font,
) -> bool {
    let is_hovered = mouse_pos.x >= x && mouse_pos.x <= x + w && mouse_pos.y >= y && mouse_pos.y <= y + h;

    let (bg, border, text_col) = if primary {
        if is_hovered {
            (Color::new(0.18, 0.46, 0.98, 1.0), ICE_BLUE, TEXT_PRIMARY)
        } else {
            (COBALT_ACCENT, Color::new(0.20, 0.40, 0.90, 0.8), TEXT_PRIMARY)
        }
    } else {
        if is_hovered {
            (SURFACE_HOVER, BORDER_ACTIVE, TEXT_PRIMARY)
        } else {
            (SURFACE_COLOR, BORDER_COLOR, TEXT_MUTED)
        }
    };

    draw_card_rect(x, y, w, h, bg, border);

    let font_size = 14.0;
    let text_dim = measure_text_ui(label, font_size, font);
    draw_text_ui(
        label,
        x + (w - text_dim.width) * 0.5,
        y + (h + text_dim.height) * 0.5 - 2.0,
        font_size,
        text_col,
        font,
    );

    is_hovered && is_mouse_button_pressed(MouseButton::Left)
}
