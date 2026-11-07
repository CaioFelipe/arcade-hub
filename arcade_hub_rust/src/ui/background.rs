use macroquad::prelude::*;
use crate::theme::BG_COLOR;

/// Gerador pseudo-aleatório leve (xorshift32) sem dependências externas
struct SimpleRng {
    state: u32,
}

impl SimpleRng {
    fn new(seed: u32) -> Self {
        Self {
            state: if seed == 0 { 0x12345678 } else { seed },
        }
    }

    fn next_u32(&mut self) -> u32 {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 17;
        self.state ^= self.state << 5;
        self.state
    }

    fn next_f32(&mut self) -> f32 {
        (self.next_u32() as f32) / (u32::MAX as f32)
    }

    fn range(&mut self, min: f32, max: f32) -> f32 {
        min + self.next_f32() * (max - min)
    }
}

/// Partícula individual da constelação de fundo
pub struct Particle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub base_vel: Vec2,
    pub radius: f32,
    pub color: Color,
    pub base_alpha: f32,
    pub pulse_phase: f32,
}

/// Sistema de fundo dinâmico: Grid Tático + Holofote + Constelação Reativa
pub struct BackgroundSystem {
    particles: Vec<Particle>,
    time: f32,
}

impl BackgroundSystem {
    pub fn new(width: f32, height: f32) -> Self {
        let mut rng = SimpleRng::new(0xABCD9876);
        let count = 52;
        let mut particles = Vec::with_capacity(count);

        let colors = [
            Color::new(0.38, 0.65, 0.98, 1.0),   // Ice Blue (#60A5FA)
            Color::new(0.145, 0.388, 0.921, 1.0), // Cobalt Accent (#2563EB)
            Color::new(0.063, 0.725, 0.506, 1.0), // Emerald (#10B981)
            Color::new(0.98, 0.80, 0.08, 1.0),   // Yellow Accent (#FACC15)
            Color::new(0.75, 0.82, 0.95, 1.0),   // Slate Luminescence
        ];

        for i in 0..count {
            let vx = rng.range(-15.0, 15.0);
            let vy = rng.range(-15.0, 15.0);
            let color = colors[i % colors.len()];
            let radius = rng.range(1.2, 2.4);
            let base_alpha = rng.range(0.25, 0.60);
            let pulse_phase = rng.range(0.0, std::f32::consts::TAU);

            particles.push(Particle {
                pos: Vec2::new(rng.range(0.0, width), rng.range(0.0, height)),
                vel: Vec2::new(vx, vy),
                base_vel: Vec2::new(vx, vy),
                radius,
                color,
                base_alpha,
                pulse_phase,
            });
        }

        Self {
            particles,
            time: 0.0,
        }
    }

    /// Atualiza simulação física e renderiza o fundo completo
    pub fn update_and_draw(&mut self, width: f32, height: f32, mouse_pos: Vec2, dt: f32) {
        let dt = dt.min(0.05);
        self.time += dt;

        // 1. Limpeza de canvas com Safira Profundo
        clear_background(BG_COLOR);

        // 2. Grid de Coordenadas de 1px com Holofote Reativo
        let step = 48.0;
        let base_grid_color = Color::new(1.0, 1.0, 1.0, 0.022);
        let spot_radius = 260.0_f32;
        let spot_radius_sq = spot_radius * spot_radius;

        // Linhas de grade estáticas sutis
        let mut x = 0.0;
        while x <= width {
            draw_line(x, 0.0, x, height, 1.0, base_grid_color);
            x += step;
        }

        let mut y = 0.0;
        while y <= height {
            draw_line(0.0, y, width, y, 1.0, base_grid_color);
            y += step;
        }

        // Iluminação aditiva e cruzes táticas próximas ao cursor
        let mouse_inside = mouse_pos.x >= 0.0 && mouse_pos.x <= width && mouse_pos.y >= 0.0 && mouse_pos.y <= height;
        if mouse_inside {
            // Auras suaves concêntricas
            draw_circle(mouse_pos.x, mouse_pos.y, 220.0, Color::new(0.145, 0.388, 0.921, 0.022));
            draw_circle(mouse_pos.x, mouse_pos.y, 130.0, Color::new(0.380, 0.650, 0.980, 0.038));
            draw_circle(mouse_pos.x, mouse_pos.y, 50.0, Color::new(0.380, 0.650, 0.980, 0.060));

            let min_ix = ((mouse_pos.x - spot_radius) / step).floor().max(0.0) as i32;
            let max_ix = ((mouse_pos.x + spot_radius) / step).ceil().min(width / step) as i32;
            let min_iy = ((mouse_pos.y - spot_radius) / step).floor().max(0.0) as i32;
            let max_iy = ((mouse_pos.y + spot_radius) / step).ceil().min(height / step) as i32;

            for ix in min_ix..=max_ix {
                let px = ix as f32 * step;
                for iy in min_iy..=max_iy {
                    let py = iy as f32 * step;
                    let dx = px - mouse_pos.x;
                    let dy = py - mouse_pos.y;
                    let dsq = dx * dx + dy * dy;

                    if dsq < spot_radius_sq {
                        let d = dsq.sqrt();
                        let factor = (1.0 - d / spot_radius).powi(2);

                        // Micro-cruz tática na interseção
                        let arm = 3.5 * factor;
                        let line_c = Color::new(0.38, 0.65, 0.98, factor * 0.40);
                        draw_line(px - arm, py, px + arm, py, 1.0, line_c);
                        draw_line(px, py - arm, px, py + arm, 1.0, line_c);

                        // Ponto central de coordenadas
                        let dot_c = Color::new(0.97, 0.98, 1.0, factor * 0.60);
                        draw_circle(px, py, 1.0 + factor * 0.8, dot_c);
                    }
                }
            }
        }

        // 3. Física de Partículas e Repulsão Elástica do Cursor
        let mouse_repel_dist = 145.0_f32;
        let mouse_repel_sq = mouse_repel_dist * mouse_repel_dist;

        for p in self.particles.iter_mut() {
            if mouse_inside {
                let to_mouse = p.pos - mouse_pos;
                let dsq = to_mouse.length_squared();

                if dsq < mouse_repel_sq && dsq > 1.0 {
                    let dist = dsq.sqrt();
                    let norm = to_mouse / dist;
                    let force = (1.0 - dist / mouse_repel_dist) * 110.0;
                    p.vel += norm * force * dt;
                }
            }

            // Inércia e retorno suave à velocidade orbital base
            p.vel = p.vel * 0.95 + p.base_vel * 0.05;
            p.pos += p.vel * dt;

            // Envoltório contínuo nas bordas da janela
            let margin = 20.0;
            if p.pos.x < -margin { p.pos.x = width + margin; }
            if p.pos.x > width + margin { p.pos.x = -margin; }
            if p.pos.y < -margin { p.pos.y = height + margin; }
            if p.pos.y > height + margin { p.pos.y = -margin; }
        }

        // 4. Linhas de Constelação entre Partículas Próximas
        let conn_dist = 95.0_f32;
        let conn_dist_sq = conn_dist * conn_dist;
        let n = self.particles.len();

        for i in 0..n {
            let pi = self.particles[i].pos;

            for j in (i + 1)..n {
                let pj = self.particles[j].pos;
                let dx = pi.x - pj.x;
                let dy = pi.y - pj.y;
                let dsq = dx * dx + dy * dy;

                if dsq < conn_dist_sq {
                    let d = dsq.sqrt();
                    let alpha = (1.0 - d / conn_dist) * 0.13;
                    draw_line(pi.x, pi.y, pj.x, pj.y, 1.0, Color::new(0.25, 0.45, 0.90, alpha));
                }
            }

            // Feixe de conexão translúcido entre o cursor e a partícula próxima
            if mouse_inside {
                let to_mouse = pi - mouse_pos;
                let dsq = to_mouse.length_squared();
                if dsq < mouse_repel_sq {
                    let d = dsq.sqrt();
                    let alpha = (1.0 - d / mouse_repel_dist) * 0.25;
                    draw_line(mouse_pos.x, mouse_pos.y, pi.x, pi.y, 1.0, Color::new(0.38, 0.65, 0.98, alpha));
                }
            }
        }

        // 5. Renderização dos Nós Estelares
        for p in self.particles.iter() {
            let pulse = ((self.time * 2.2 + p.pulse_phase).sin() * 0.5 + 0.5) * 0.25;
            let current_alpha = (p.base_alpha + pulse).clamp(0.15, 0.85);

            // Halo suave
            draw_circle(
                p.pos.x,
                p.pos.y,
                p.radius * 2.2,
                Color::new(p.color.r, p.color.g, p.color.b, current_alpha * 0.22),
            );
            // Núcleo nítido
            draw_circle(
                p.pos.x,
                p.pos.y,
                p.radius,
                Color::new(p.color.r, p.color.g, p.color.b, current_alpha),
            );
        }
    }
}
