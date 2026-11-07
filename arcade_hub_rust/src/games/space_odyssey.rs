use super::ArcadeGame;
use crate::theme::*;
use macroquad::prelude::*;

// =========================================================================
// 1. ESTRUTURAS DE DADOS & ENUMS
// =========================================================================

#[derive(Clone, Copy, PartialEq)]
pub enum AsteroidTier {
    Titan,       // Raio 42..48, 10 HP, subdivide em 2 Medium
    Medium,      // Raio 24..30, 4 HP, subdivide em 2 Debris
    Debris,      // Raio 12..16, 2 HP, rápido e errático
    Radioactive, // Raio 30..36, 7 HP, veios de magma neon, explode em choque
    Metallic,    // Raio 28..34, 12 HP, armadura densa de ferro estelar
}

struct Crater {
    rel_x: f32,
    rel_y: f32,
    radius: f32,
}

struct Asteroid {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    radius: f32,
    hp: i32,
    max_hp: i32,
    tier: AsteroidTier,
    rotation: f32,
    v_rot: f32,
    vertex_offsets: Vec<f32>,
    craters: Vec<Crater>,
}

impl Asteroid {
    fn new(x: f32, y: f32, vx: f32, vy: f32, tier: AsteroidTier) -> Self {
        let (radius, hp) = match tier {
            AsteroidTier::Titan => (rand::gen_range(42.0, 48.0), 10),
            AsteroidTier::Medium => (rand::gen_range(24.0, 30.0), 4),
            AsteroidTier::Debris => (rand::gen_range(12.0, 16.0), 2),
            AsteroidTier::Radioactive => (rand::gen_range(30.0, 36.0), 7),
            AsteroidTier::Metallic => (rand::gen_range(28.0, 34.0), 12),
        };

        let mut vertex_offsets = Vec::with_capacity(12);
        for _ in 0..12 {
            vertex_offsets.push(rand::gen_range(0.80, 1.20));
        }

        // Gera 2 a 4 crateras com profundidade
        let num_craters = match tier {
            AsteroidTier::Debris => 1,
            AsteroidTier::Medium => 2,
            _ => 4,
        };
        let mut craters = Vec::with_capacity(num_craters);
        for _ in 0..num_craters {
            let cr_dist: f32 = rand::gen_range(0.15, 0.60) * radius;
            let cr_ang: f32 = rand::gen_range(0.0, 6.28);
            craters.push(Crater {
                rel_x: cr_ang.cos() * cr_dist,
                rel_y: cr_ang.sin() * cr_dist,
                radius: rand::gen_range(0.18, 0.32) * radius,
            });
        }

        Self {
            x,
            y,
            vx,
            vy,
            radius,
            hp,
            max_hp: hp,
            tier,
            rotation: rand::gen_range(0.0, 6.28),
            v_rot: rand::gen_range(-0.03, 0.03),
            vertex_offsets,
            craters,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum AlienType {
    Scout,     // Incursão de alta velocidade com tiros quádruplos
    MineLayer, // Drone industrial blindado com rotor giratório
    Gunboat,   // Canhoneira blindada com escudo frontal de plasma
}

struct AlienShip {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    ship_type: AlienType,
    hp: i32,
    max_hp: i32,
    fire_timer: u32,
    age: f32,
}

struct ProximityMine {
    x: f32,
    y: f32,
    vy: f32,
    timer: f32,
    pulse: f32,
}

struct Laser {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    damage: i32,
    pierce: i32,
    is_missile: bool,
    target_pos: Option<Vec2>,
    life: f32,
    color: Color,
}

struct EnemyLaser {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    radius: f32,
    color: Color,
    is_torpedo: bool,
}

#[derive(Clone, Copy, PartialEq)]
pub enum BossPhase {
    Warning,
    Arriving,
    Phase1Turrets,    // Canhões laterais rastreiam o jogador + Escudo frontal ativo
    Phase2Carrier,    // Lança esquadrilhas de caças escolta
    Phase3Overcharge, // Escudo frontal cai, canhão central carrega mega-feixe
    Exploding,
    Defeated,
}

struct BossTurret {
    rel_x: f32,
    rel_y: f32,
    angle: f32,
    hp: i32,
    max_hp: i32,
    destroyed: bool,
    fire_timer: u32,
}

struct DreadnoughtBoss {
    x: f32,
    y: f32,
    vx: f32,
    phase: BossPhase,
    phase_timer: f32,
    turret_left: BossTurret,
    turret_right: BossTurret,
    core_hp: i32,
    core_max_hp: i32,
    attack_timer: u32,
    beam_charge_timer: f32,
    beam_active_timer: f32,
    explosion_countdown: f32,
}

#[derive(Clone, Copy, PartialEq)]
pub enum PowerUpType {
    Shield,    // +18% de Escudo (raro)
    Weapon,    // +1 Ponto de Arsenal
    NovaBomb,  // +1 Ogiva EMP
    Overdrive, // 6s de Hiper-Fogo
    Stardust,  // Cristais táticos
}

struct PowerUp {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    kind: PowerUpType,
    life: f32,
    anim: f32,
}

struct Particle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    color: Color,
    life: f32,
    max_life: f32,
    size: f32,
}

struct Shockwave {
    x: f32,
    y: f32,
    radius: f32,
    max_radius: f32,
    speed: f32,
    color: Color,
}

struct FloatingText {
    x: f32,
    y: f32,
    text: String,
    color: Color,
    life: f32,
}

struct Star {
    x: f32,
    y: f32,
    speed: f32,
    size: f32,
    brightness: f32,
    layer: u8,
}

struct Afterimage {
    x: f32,
    y: f32,
    tilt: f32,
    alpha: f32,
}

// =========================================================================
// 2. MÁQUINA DE COMBATE PRINCIPAL: SPACE ODYSSEY (APEX HARDCORE)
// =========================================================================

pub struct SpaceOdyssey {
    // Jogador e Física
    player_x: f32,
    player_y: f32,
    player_vx: f32,
    player_vy: f32,
    tilt: f32,
    afterimages: Vec<Afterimage>,

    // Defesas Equilibradas
    shield: f32,
    hull: f32,
    shield_regen_delay: f32, // 6.0 segundos sem dano
    shield_hit_flash: f32,
    invuln_timer: f32,
    dash_cooldown: f32,
    dash_timer: f32,

    // Arsenal com Progressão em Pods Acumulativos
    weapon_level: u32,
    weapon_pods_collected: u32,
    nova_bombs: u32,
    overdrive_timer: f32,
    cooldown: u32,

    // Super-Arma Nova EMP
    nova_active: bool,
    nova_radius: f32,
    nova_origin: Vec2,

    // Pontuação e Combos
    score: u32,
    combo_multiplier: f32,
    combo_timer: f32,
    lives: i32,
    game_over: bool,

    // Campanha de Ondas e Setor
    sector: u32,
    wave: u32,
    wave_kills: u32,
    wave_target_kills: u32,
    wave_banner_timer: f32,
    wave_banner_text: String,

    // Chefe Dreadnought
    boss: Option<DreadnoughtBoss>,

    // Entidades
    stars: Vec<Star>,
    lasers: Vec<Laser>,
    enemy_lasers: Vec<EnemyLaser>,
    asteroids: Vec<Asteroid>,
    aliens: Vec<AlienShip>,
    mines: Vec<ProximityMine>,
    powerups: Vec<PowerUp>,
    particles: Vec<Particle>,
    shockwaves: Vec<Shockwave>,
    floating_texts: Vec<FloatingText>,

    // Timers e Câmera
    spawn_timer: u32,
    alien_spawn_timer: u32,
    camera_shake: f32,
    graze_spark_timer: f32,
}

impl SpaceOdyssey {
    pub fn new() -> Self {
        let mut stars = Vec::with_capacity(130);
        for _ in 0..130 {
            let layer = rand::gen_range(0, 3) as u8;
            let (speed, size, brightness) = match layer {
                0 => (rand::gen_range(0.3, 0.8), 1.0, 0.40),
                1 => (rand::gen_range(1.0, 2.0), 1.5, 0.70),
                _ => (rand::gen_range(2.8, 5.0), 2.0, 0.95),
            };
            stars.push(Star {
                x: rand::gen_range(0.0, 800.0),
                y: rand::gen_range(0.0, 600.0),
                speed,
                size,
                brightness,
                layer,
            });
        }

        let mut game = Self {
            player_x: 400.0,
            player_y: 520.0,
            player_vx: 0.0,
            player_vy: 0.0,
            tilt: 0.0,
            afterimages: Vec::new(),

            shield: 100.0,
            hull: 100.0,
            shield_regen_delay: 0.0,
            shield_hit_flash: 0.0,
            invuln_timer: 1.5,
            dash_cooldown: 0.0,
            dash_timer: 0.0,

            weapon_level: 1,
            weapon_pods_collected: 0,
            nova_bombs: 1, // Começa apenas com 1 ogiva EMP (recompensa por sobrevivência)
            overdrive_timer: 0.0,
            cooldown: 0,

            nova_active: false,
            nova_radius: 0.0,
            nova_origin: Vec2::ZERO,

            score: 0,
            combo_multiplier: 1.0,
            combo_timer: 0.0,
            lives: 3,
            game_over: false,

            sector: 1,
            wave: 1,
            wave_kills: 0,
            wave_target_kills: 18,
            wave_banner_timer: 3.5,
            wave_banner_text: "ONDA 1: CINTURÃO DE DETRITOS DENSO".to_string(),

            boss: None,

            stars,
            lasers: Vec::new(),
            enemy_lasers: Vec::new(),
            asteroids: Vec::new(),
            aliens: Vec::new(),
            mines: Vec::new(),
            powerups: Vec::new(),
            particles: Vec::new(),
            shockwaves: Vec::new(),
            floating_texts: Vec::new(),

            spawn_timer: 0,
            alien_spawn_timer: 0,
            camera_shake: 0.0,
            graze_spark_timer: 0.0,
        };

        // Spawns iniciais variados com trajetórias em ângulo
        game.spawn_asteroid_initial();
        game
    }

    fn spawn_asteroid_initial(&mut self) {
        self.asteroids.push(Asteroid::new(180.0, -40.0, 0.8, 2.2, AsteroidTier::Titan));
        self.asteroids.push(Asteroid::new(620.0, -60.0, -0.7, 2.4, AsteroidTier::Medium));
        self.asteroids.push(Asteroid::new(400.0, -100.0, 0.4, 2.8, AsteroidTier::Metallic));
    }

    fn add_shake(&mut self, amount: f32) {
        self.camera_shake = (self.camera_shake + amount).min(1.0);
    }

    fn trigger_nova_bomb(&mut self) {
        if self.nova_bombs == 0 || self.nova_active {
            return;
        }
        self.nova_bombs -= 1;
        self.nova_active = true;
        self.nova_radius = 10.0;
        self.nova_origin = Vec2::new(self.player_x, self.player_y);
        self.add_shake(0.9);

        // Limpa projéteis inimigos na tela
        self.enemy_lasers.clear();

        self.shockwaves.push(Shockwave {
            x: self.player_x,
            y: self.player_y,
            radius: 10.0,
            max_radius: 900.0,
            speed: 38.0,
            color: Color::new(0.65, 0.35, 0.98, 0.9),
        });

        self.floating_texts.push(FloatingText {
            x: self.player_x,
            y: self.player_y - 40.0,
            text: "QUANTUM NOVA EMP!".to_string(),
            color: Color::new(0.85, 0.45, 1.0, 1.0),
            life: 1.5,
        });
    }

    fn fire_primary(&mut self) {
        let is_overdrive = self.overdrive_timer > 0.0;
        let base_delay = if is_overdrive { 4 } else { 9 };
        self.cooldown = base_delay;

        let px = self.player_x;
        let py = self.player_y;

        match self.weapon_level {
            1 => {
                // Dual Blasters
                self.lasers.push(Laser {
                    x: px - 11.0, y: py - 14.0, vx: 0.0, vy: -15.0,
                    damage: 1, pierce: 1, is_missile: false, target_pos: None, life: 1.1, color: ICE_BLUE,
                });
                self.lasers.push(Laser {
                    x: px + 11.0, y: py - 14.0, vx: 0.0, vy: -15.0,
                    damage: 1, pierce: 1, is_missile: false, target_pos: None, life: 1.1, color: ICE_BLUE,
                });
            }
            2 => {
                // Triple Vulcan
                self.lasers.push(Laser {
                    x: px, y: py - 18.0, vx: 0.0, vy: -16.0,
                    damage: 1, pierce: 1, is_missile: false, target_pos: None, life: 1.1, color: YELLOW_ACCENT,
                });
                self.lasers.push(Laser {
                    x: px - 14.0, y: py - 10.0, vx: -2.0, vy: -14.5,
                    damage: 1, pierce: 1, is_missile: false, target_pos: None, life: 1.1, color: ICE_BLUE,
                });
                self.lasers.push(Laser {
                    x: px + 14.0, y: py - 10.0, vx: 2.0, vy: -14.5,
                    damage: 1, pierce: 1, is_missile: false, target_pos: None, life: 1.1, color: ICE_BLUE,
                });
            }
            3 => {
                // Quintuple Scatter
                for i in -2..=2 {
                    let angle = (i as f32) * 0.15;
                    self.lasers.push(Laser {
                        x: px + (i as f32) * 6.0, y: py - 14.0,
                        vx: angle.sin() * 15.0, vy: -angle.cos() * 15.0,
                        damage: 1, pierce: 2, is_missile: false, target_pos: None, life: 1.1, color: ICE_BLUE,
                    });
                }
            }
            4 => {
                // Photon Lance
                self.lasers.push(Laser {
                    x: px - 8.0, y: py - 16.0, vx: 0.0, vy: -18.0,
                    damage: 2, pierce: 3, is_missile: false, target_pos: None, life: 1.3, color: EMERALD,
                });
                self.lasers.push(Laser {
                    x: px + 8.0, y: py - 16.0, vx: 0.0, vy: -18.0,
                    damage: 2, pierce: 3, is_missile: false, target_pos: None, life: 1.3, color: EMERALD,
                });
                self.lasers.push(Laser {
                    x: px - 18.0, y: py - 8.0, vx: -3.0, vy: -14.0,
                    damage: 1, pierce: 2, is_missile: false, target_pos: None, life: 1.1, color: ICE_BLUE,
                });
                self.lasers.push(Laser {
                    x: px + 18.0, y: py - 8.0, vx: 3.0, vy: -14.0,
                    damage: 1, pierce: 2, is_missile: false, target_pos: None, life: 1.1, color: ICE_BLUE,
                });
            }
            _ => {
                // Hyper Swarm
                self.lasers.push(Laser {
                    x: px - 8.0, y: py - 16.0, vx: 0.0, vy: -18.0,
                    damage: 2, pierce: 4, is_missile: false, target_pos: None, life: 1.3, color: Color::new(0.65, 0.45, 1.0, 1.0),
                });
                self.lasers.push(Laser {
                    x: px + 8.0, y: py - 16.0, vx: 0.0, vy: -18.0,
                    damage: 2, pierce: 4, is_missile: false, target_pos: None, life: 1.3, color: Color::new(0.65, 0.45, 1.0, 1.0),
                });
                let target = self.find_nearest_target();
                self.lasers.push(Laser {
                    x: px - 22.0, y: py - 6.0, vx: -4.0, vy: -8.0,
                    damage: 3, pierce: 1, is_missile: true, target_pos: target, life: 1.8, color: YELLOW_ACCENT,
                });
                self.lasers.push(Laser {
                    x: px + 22.0, y: py - 6.0, vx: 4.0, vy: -8.0,
                    damage: 3, pierce: 1, is_missile: true, target_pos: target, life: 1.8, color: YELLOW_ACCENT,
                });
            }
        }

        // Muzzle Flash
        for _ in 0..4 {
            self.particles.push(Particle {
                x: px + rand::gen_range(-10.0, 10.0),
                y: py - 18.0,
                vx: rand::gen_range(-1.5, 1.5),
                vy: rand::gen_range(-4.0, -1.0),
                color: if is_overdrive { Color::new(0.7, 0.4, 1.0, 1.0) } else { ICE_BLUE },
                life: rand::gen_range(0.12, 0.22),
                max_life: 0.22,
                size: rand::gen_range(2.0, 3.5),
            });
        }
    }

    fn find_nearest_target(&self) -> Option<Vec2> {
        let mut closest: Option<(f32, Vec2)> = None;
        let px = self.player_x;
        let py = self.player_y;

        for alien in &self.aliens {
            let dist_sq = (alien.x - px).powi(2) + (alien.y - py).powi(2);
            if closest.is_none() || dist_sq < closest.unwrap().0 {
                closest = Some((dist_sq, Vec2::new(alien.x, alien.y)));
            }
        }
        if closest.is_none() {
            for ast in &self.asteroids {
                let dist_sq = (ast.x - px).powi(2) + (ast.y - py).powi(2);
                if closest.is_none() || dist_sq < closest.unwrap().0 {
                    closest = Some((dist_sq, Vec2::new(ast.x, ast.y)));
                }
            }
        }
        closest.map(|(_, pos)| pos)
    }

    fn take_player_damage(&mut self, amount: f32) {
        if self.invuln_timer > 0.0 || self.dash_timer > 0.0 {
            return;
        }

        self.shield_regen_delay = 6.0; // 6s fora de combate para começar a recarregar
        self.shield_hit_flash = 0.35;
        self.add_shake(0.5);

        if self.shield > 0.0 {
            self.shield -= amount;
            if self.shield < 0.0 {
                let overflow = -self.shield;
                self.shield = 0.0;
                self.hull -= overflow;
            }
        } else {
            self.hull -= amount;
        }

        for _ in 0..14 {
            self.particles.push(Particle {
                x: self.player_x,
                y: self.player_y,
                vx: rand::gen_range(-5.0, 5.0),
                vy: rand::gen_range(-5.0, 5.0),
                color: if self.shield > 0.0 { ICE_BLUE } else { CORAL },
                life: rand::gen_range(0.2, 0.45),
                max_life: 0.45,
                size: rand::gen_range(2.0, 4.5),
            });
        }

        if self.hull <= 0.0 {
            self.lives -= 1;
            // Penalidade de morte: Perde 1 nível de arma
            if self.weapon_level > 1 {
                self.weapon_level -= 1;
                self.weapon_pods_collected = 0;
            }
            if self.lives <= 0 {
                self.game_over = true;
                self.add_shake(1.0);
                self.shockwaves.push(Shockwave {
                    x: self.player_x,
                    y: self.player_y,
                    radius: 10.0,
                    max_radius: 350.0,
                    speed: 20.0,
                    color: CORAL,
                });
            } else {
                self.hull = 100.0;
                self.shield = 100.0;
                self.invuln_timer = 3.0;
                self.player_x = 400.0;
                self.player_y = 520.0;
                self.player_vx = 0.0;
                self.player_vy = 0.0;
                self.floating_texts.push(FloatingText {
                    x: 400.0,
                    y: 480.0,
                    text: format!("NAVE RECUPERADA! (VIDAS RESTANTES: {})", self.lives),
                    color: YELLOW_ACCENT,
                    life: 2.2,
                });
            }
        }
    }

    /// Sistema de Drops Rígido e Balanceado (Sem facilidades excessivas)
    fn drop_loot(&mut self, x: f32, y: f32, tier: Option<AsteroidTier>, is_alien: bool) {
        let roll = rand::gen_range(0.0, 1.0);

        match tier {
            Some(AsteroidTier::Debris) => {
                // Detritos NUNCA dropam itens, apenas chance mínima de Stardust
                if roll < 0.05 {
                    self.powerups.push(PowerUp { x, y, vx: rand::gen_range(-1.0, 1.0), vy: rand::gen_range(0.8, 1.8), kind: PowerUpType::Stardust, life: 12.0, anim: 0.0 });
                }
            }
            Some(AsteroidTier::Medium) => {
                if roll < 0.12 {
                    self.powerups.push(PowerUp { x, y, vx: rand::gen_range(-1.0, 1.0), vy: rand::gen_range(0.8, 1.8), kind: PowerUpType::Stardust, life: 12.0, anim: 0.0 });
                }
            }
            Some(AsteroidTier::Titan) => {
                // Titans tem 20% de chance de cápsula tática
                if roll < 0.20 {
                    let kind = if self.weapon_level < 5 && roll < 0.10 { PowerUpType::Weapon } else { PowerUpType::Shield };
                    self.powerups.push(PowerUp { x, y, vx: rand::gen_range(-1.2, 1.2), vy: rand::gen_range(0.8, 2.0), kind, life: 14.0, anim: 0.0 });
                }
            }
            Some(AsteroidTier::Radioactive) => {
                // Radioativos sempre dropam cápsula de valor (mas explodem perto do player!)
                let kind = if roll < 0.40 { PowerUpType::Weapon } else if roll < 0.70 { PowerUpType::NovaBomb } else { PowerUpType::Overdrive };
                self.powerups.push(PowerUp { x, y, vx: rand::gen_range(-1.2, 1.2), vy: rand::gen_range(0.8, 2.0), kind, life: 15.0, anim: 0.0 });
            }
            Some(AsteroidTier::Metallic) => {
                if roll < 0.35 {
                    self.powerups.push(PowerUp { x, y, vx: rand::gen_range(-1.2, 1.2), vy: rand::gen_range(0.8, 2.0), kind: PowerUpType::Stardust, life: 14.0, anim: 0.0 });
                }
            }
            None => {
                if is_alien {
                    if roll < 0.22 {
                        let kind = if roll < 0.08 { PowerUpType::Weapon } else if roll < 0.15 { PowerUpType::Shield } else { PowerUpType::Stardust };
                        self.powerups.push(PowerUp { x, y, vx: rand::gen_range(-1.2, 1.2), vy: rand::gen_range(0.8, 2.0), kind, life: 14.0, anim: 0.0 });
                    }
                }
            }
        }
    }

    fn check_wave_progression(&mut self) {
        if self.boss.is_some() {
            return;
        }

        if self.wave_kills >= self.wave_target_kills {
            self.wave += 1;
            self.wave_kills = 0;

            match self.wave {
                2 => {
                    self.wave_target_kills = 24;
                    self.wave_banner_text = "ONDA 2: INCURSÃO DE RECONHECIMENTO ALIEN".to_string();
                    self.wave_banner_timer = 3.5;
                }
                3 => {
                    self.wave_target_kills = 30;
                    self.wave_banner_text = "ONDA 3: TEMPESTADE DE MINÉRIOS RADIOATIVOS".to_string();
                    self.wave_banner_timer = 3.5;
                }
                4 => {
                    self.wave_target_kills = 36;
                    self.wave_banner_text = "ONDA 4: ESQUADRÃO DE ATAQUE PESADO".to_string();
                    self.wave_banner_timer = 3.5;
                }
                5 => {
                    // ONDA DO CHEFE COLOSSAL DREADNOUGHT!
                    self.wave_target_kills = 999;
                    self.wave_banner_text = "/// ALERTA VERMELHO: FLAGSHIP DREADNOUGHT SE APROXIMA ///".to_string();
                    self.wave_banner_timer = 4.0;
                    self.boss = Some(DreadnoughtBoss {
                        x: 400.0,
                        y: -140.0,
                        vx: 1.8,
                        phase: BossPhase::Warning,
                        phase_timer: 3.5,
                        turret_left: BossTurret { rel_x: -110.0, rel_y: 25.0, angle: 0.0, hp: 200, max_hp: 200, destroyed: false, fire_timer: 50 },
                        turret_right: BossTurret { rel_x: 110.0, rel_y: 25.0, angle: 0.0, hp: 200, max_hp: 200, destroyed: false, fire_timer: 50 },
                        core_hp: 450,
                        core_max_hp: 450,
                        attack_timer: 60,
                        beam_charge_timer: 0.0,
                        beam_active_timer: 0.0,
                        explosion_countdown: 4.5,
                    });
                    self.add_shake(0.9);
                }
                _ => {
                    // Novo Setor!
                    self.sector += 1;
                    self.wave = 1;
                    self.wave_target_kills = 22;
                    self.wave_banner_text = format!("SETOR {}: QUADRANTE PROFUNDO", self.sector);
                    self.wave_banner_timer = 4.0;
                }
            }
        }
    }
}

// =========================================================================
// 3. IMPLEMENTAÇÃO DO TRAIT ARCADEMAME (UPDATE, DRAW, RESET)
// =========================================================================

impl ArcadeGame for SpaceOdyssey {
    fn update(&mut self) {
        if self.game_over {
            return;
        }

        let dt = 1.0 / 60.0;

        // 1. Atualizar Temporizadores
        if self.cooldown > 0 { self.cooldown -= 1; }
        if self.invuln_timer > 0.0 { self.invuln_timer -= dt; }
        if self.dash_cooldown > 0.0 { self.dash_cooldown -= dt; }
        if self.dash_timer > 0.0 { self.dash_timer -= dt; }
        if self.overdrive_timer > 0.0 { self.overdrive_timer -= dt; }
        if self.shield_hit_flash > 0.0 { self.shield_hit_flash -= dt; }
        if self.graze_spark_timer > 0.0 { self.graze_spark_timer -= dt; }
        if self.wave_banner_timer > 0.0 { self.wave_banner_timer -= dt; }

        // Recarga de Escudo Balanceada (6s fora de combate, 10% por segundo)
        if self.shield_regen_delay > 0.0 {
            self.shield_regen_delay -= dt;
        } else if self.shield < 100.0 {
            self.shield = (self.shield + 10.0 * dt).min(100.0);
        }

        // Multiplicador de Combo Decay
        if self.combo_timer > 0.0 {
            self.combo_timer -= dt;
            if self.combo_timer <= 0.0 {
                self.combo_multiplier = 1.0;
            }
        }

        // Câmera Shake Decay
        if self.camera_shake > 0.0 {
            self.camera_shake = (self.camera_shake - 1.8 * dt).max(0.0);
        }

        // 2. Controles do Jogador
        let mut move_x = 0.0;
        let mut move_y = 0.0;

        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) { move_x -= 1.0; }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) { move_x += 1.0; }
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) { move_y -= 1.0; }
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) { move_y += 1.0; }

        // Dash com Shift
        if (is_key_pressed(KeyCode::LeftShift) || is_key_pressed(KeyCode::RightShift)) && self.dash_cooldown <= 0.0 {
            self.dash_timer = 0.20;
            self.dash_cooldown = 1.8;
            self.invuln_timer = 0.30;
            self.add_shake(0.2);
            for i in 1..=3 {
                self.afterimages.push(Afterimage {
                    x: self.player_x - self.player_vx * (i as f32) * 1.5,
                    y: self.player_y - self.player_vy * (i as f32) * 1.5,
                    tilt: self.tilt,
                    alpha: 0.6 / (i as f32),
                });
            }
        }

        let speed = if self.dash_timer > 0.0 { 15.0 } else { 7.0 };
        let accel = 1.5;
        let damping = 0.86;

        self.player_vx = (self.player_vx + move_x * accel * speed).clamp(-speed, speed);
        self.player_vy = (self.player_vy + move_y * accel * speed).clamp(-speed * 0.7, speed * 0.7);

        self.player_x += self.player_vx;
        self.player_y += self.player_vy;

        self.player_vx *= damping;
        self.player_vy *= damping;

        self.player_x = self.player_x.clamp(40.0, 760.0);
        self.player_y = self.player_y.clamp(260.0, 560.0);

        let target_tilt = (self.player_vx / speed).clamp(-1.0, 1.0);
        self.tilt += (target_tilt - self.tilt) * 0.22;

        self.afterimages.retain_mut(|img| {
            img.alpha -= 3.5 * dt;
            img.alpha > 0.0
        });

        // 3. Disparos
        if (is_key_down(KeyCode::Space) || is_mouse_button_down(MouseButton::Left)) && self.cooldown == 0 {
            self.fire_primary();
        }

        if is_key_pressed(KeyCode::X) || is_mouse_button_pressed(MouseButton::Right) {
            self.trigger_nova_bomb();
        }

        // 4. Expansão da Nova Bomb
        if self.nova_active {
            self.nova_radius += 38.0;
            if self.nova_radius > 900.0 {
                self.nova_active = false;
            } else {
                let nr = self.nova_radius;
                let no = self.nova_origin;
                self.enemy_lasers.retain(|el| (Vec2::new(el.x, el.y) - no).length() > nr);

                for ast in &mut self.asteroids {
                    if (Vec2::new(ast.x, ast.y) - no).length() < nr + ast.radius {
                        ast.hp -= 3;
                    }
                }
                for alien in &mut self.aliens {
                    if (Vec2::new(alien.x, alien.y) - no).length() < nr + 20.0 {
                        alien.hp -= 5;
                    }
                }
                if let Some(boss) = &mut self.boss {
                    boss.turret_left.hp -= 2;
                    boss.turret_right.hp -= 2;
                    if boss.turret_left.destroyed && boss.turret_right.destroyed {
                        boss.core_hp -= 3;
                    }
                }
            }
        }

        // 5. Atualizar Fundo Estelar
        for s in &mut self.stars {
            let warp_factor = if self.dash_timer > 0.0 { 2.5 } else { 1.0 };
            s.y += s.speed * warp_factor;
            if s.y > 600.0 {
                s.y = 0.0;
                s.x = rand::gen_range(0.0, 800.0);
            }
        }

        // 6. Atualizar Lasers do Jogador
        let mut new_asteroids_to_spawn = Vec::new();

        self.lasers.retain_mut(|laser| {
            laser.life -= dt;
            if laser.life <= 0.0 {
                return false;
            }

            if laser.is_missile {
                if let Some(target) = laser.target_pos {
                    let to_target = (target - Vec2::new(laser.x, laser.y)).normalize_or_zero();
                    laser.vx += to_target.x * 1.0;
                    laser.vy += to_target.y * 1.0;
                    let spd = Vec2::new(laser.vx, laser.vy).length();
                    if spd > 14.0 {
                        let norm = Vec2::new(laser.vx, laser.vy).normalize();
                        laser.vx = norm.x * 14.0;
                        laser.vy = norm.y * 14.0;
                    }
                }
            }

            laser.x += laser.vx;
            laser.y += laser.vy;

            // Colisão com Asteroides
            for ast in &mut self.asteroids {
                let dist = (Vec2::new(laser.x, laser.y) - Vec2::new(ast.x, ast.y)).length();
                if dist < ast.radius + 6.0 && ast.hp > 0 {
                    let dmg = if ast.tier == AsteroidTier::Metallic { (laser.damage.max(2) - 1).max(1) } else { laser.damage };
                    ast.hp -= dmg;
                    laser.pierce -= 1;
                    return laser.pierce > 0;
                }
            }

            // Colisão com Naves Alienígenas
            for alien in &mut self.aliens {
                let dist = (Vec2::new(laser.x, laser.y) - Vec2::new(alien.x, alien.y)).length();
                if dist < 24.0 && alien.hp > 0 {
                    alien.hp -= laser.damage;
                    laser.pierce -= 1;
                    return laser.pierce > 0;
                }
            }

            // Colisão com Minas
            for mine in &mut self.mines {
                let dist = (Vec2::new(laser.x, laser.y) - Vec2::new(mine.x, mine.y)).length();
                if dist < 16.0 {
                    mine.timer = -1.0;
                    laser.pierce -= 1;
                    return laser.pierce > 0;
                }
            }

            // Colisão com Chefe Dreadnought (com Escudo Frontal Defletor!)
            if let Some(boss) = &mut self.boss {
                if boss.phase == BossPhase::Phase1Turrets || boss.phase == BossPhase::Phase2Carrier || boss.phase == BossPhase::Phase3Overcharge {
                    // Turret Esquerda
                    if !boss.turret_left.destroyed {
                        let tpos = Vec2::new(boss.x + boss.turret_left.rel_x, boss.y + boss.turret_left.rel_y);
                        if (Vec2::new(laser.x, laser.y) - tpos).length() < 28.0 {
                            boss.turret_left.hp -= laser.damage;
                            laser.pierce -= 1;
                            return laser.pierce > 0;
                        }
                    }
                    // Turret Direita
                    if !boss.turret_right.destroyed {
                        let tpos = Vec2::new(boss.x + boss.turret_right.rel_x, boss.y + boss.turret_right.rel_y);
                        if (Vec2::new(laser.x, laser.y) - tpos).length() < 28.0 {
                            boss.turret_right.hp -= laser.damage;
                            laser.pierce -= 1;
                            return laser.pierce > 0;
                        }
                    }

                    // Núcleo Central: PROTEGIDO pelo Escudo Frontal enquanto as turrets estiverem vivas!
                    let cpos = Vec2::new(boss.x, boss.y + 15.0);
                    if (Vec2::new(laser.x, laser.y) - cpos).length() < 36.0 {
                        if !boss.turret_left.destroyed || !boss.turret_right.destroyed {
                            // Deflete o tiro com centelhas!
                            laser.pierce = 0;
                            return false;
                        } else {
                            // Dano direto no Reator Central!
                            boss.core_hp -= laser.damage;
                            laser.pierce -= 1;
                            return laser.pierce > 0;
                        }
                    }
                }
            }

            laser.y > -20.0 && laser.y < 620.0 && laser.x > -20.0 && laser.x < 820.0
        });

        // 7. Atualizar Asteroides e Fragmentação
        let mut destroyed_asteroids = Vec::new();
        let mut asteroid_collision_damage = 0.0f32;
        let px = self.player_x;
        let py = self.player_y;

        for (i, ast) in self.asteroids.iter_mut().enumerate() {
            ast.x += ast.vx;
            ast.y += ast.vy;
            ast.rotation += ast.v_rot;

            // Mecânica de Graze
            let pdist = (Vec2::new(ast.x, ast.y) - Vec2::new(px, py)).length();
            if pdist < ast.radius + 30.0 && pdist > ast.radius + 18.0 {
                self.combo_multiplier = (self.combo_multiplier + 0.04).min(5.0);
                self.combo_timer = 3.5;
                self.graze_spark_timer = 0.2;
            }

            // Colisão com Jogador (Dano pesado: 45 dano!)
            if pdist < ast.radius + 16.0 {
                asteroid_collision_damage += 45.0;
                ast.hp = 0;
            }

            if ast.hp <= 0 {
                destroyed_asteroids.push(i);
            }
        }

        if asteroid_collision_damage > 0.0 {
            self.take_player_damage(asteroid_collision_damage);
        }

        // Processar Destruição de Asteroides
        for &idx in destroyed_asteroids.iter().rev() {
            if idx < self.asteroids.len() {
                let ast = self.asteroids.remove(idx);
                let pts = match ast.tier {
                    AsteroidTier::Titan => 250,
                    AsteroidTier::Medium => 120,
                    AsteroidTier::Debris => 60,
                    AsteroidTier::Radioactive => 380,
                    AsteroidTier::Metallic => 300,
                };
                let gained = (pts as f32 * self.combo_multiplier) as u32;
                self.score += gained;
                self.wave_kills += 1;

                // Efeito de Estilhaços e Explosão Radioativa
                let pcolor = match ast.tier {
                    AsteroidTier::Radioactive => YELLOW_ACCENT,
                    AsteroidTier::Metallic => Color::new(0.70, 0.75, 0.85, 1.0),
                    _ => Color::new(0.55, 0.62, 0.75, 1.0),
                };
                for _ in 0..16 {
                    self.particles.push(Particle {
                        x: ast.x, y: ast.y,
                        vx: rand::gen_range(-5.0, 5.0), vy: rand::gen_range(-5.0, 5.0),
                        color: pcolor,
                        life: rand::gen_range(0.3, 0.7),
                        max_life: 0.7,
                        size: rand::gen_range(2.0, 5.0),
                    });
                }
                self.add_shake(0.25);

                // Subdivisão Balística
                match ast.tier {
                    AsteroidTier::Titan => {
                        new_asteroids_to_spawn.push(Asteroid::new(ast.x - 16.0, ast.y, -1.8, ast.vy * 1.15, AsteroidTier::Medium));
                        new_asteroids_to_spawn.push(Asteroid::new(ast.x + 16.0, ast.y, 1.8, ast.vy * 1.15, AsteroidTier::Medium));
                    }
                    AsteroidTier::Medium => {
                        new_asteroids_to_spawn.push(Asteroid::new(ast.x - 12.0, ast.y, -2.5, ast.vy * 1.25, AsteroidTier::Debris));
                        new_asteroids_to_spawn.push(Asteroid::new(ast.x + 12.0, ast.y, 2.5, ast.vy * 1.25, AsteroidTier::Debris));
                    }
                    AsteroidTier::Radioactive => {
                        // Choque expansivo que fere o jogador se estiver próximo (Risco vs Recompensa!)
                        self.shockwaves.push(Shockwave { x: ast.x, y: ast.y, radius: 5.0, max_radius: 80.0, speed: 12.0, color: YELLOW_ACCENT });
                        let p_dist = (Vec2::new(ast.x, ast.y) - Vec2::new(self.player_x, self.player_y)).length();
                        if p_dist < 80.0 {
                            self.take_player_damage(25.0);
                        }
                    }
                    _ => {}
                }

                // Drops táticos rigorosamente controlados
                self.drop_loot(ast.x, ast.y, Some(ast.tier), false);
            }
        }
        self.asteroids.extend(new_asteroids_to_spawn);
        self.asteroids.retain(|a| a.y < 650.0);

        // 8. Atualizar Naves Alienígenas
        self.alien_spawn_timer += 1;
        let alien_rate = match self.wave {
            1 => 350,
            2 => 180,
            3 => 160,
            4 => 120,
            _ => 200,
        };

        if self.alien_spawn_timer >= alien_rate && self.boss.is_none() {
            self.alien_spawn_timer = 0;
            let kind = if self.wave >= 3 && rand::gen_range(0, 3) == 0 {
                AlienType::Gunboat
            } else if rand::gen_range(0, 2) == 0 {
                AlienType::MineLayer
            } else {
                AlienType::Scout
            };

            let hp = match kind {
                AlienType::Scout => 5,
                AlienType::MineLayer => 10,
                AlienType::Gunboat => 14,
            };

            self.aliens.push(AlienShip {
                x: rand::gen_range(70.0, 730.0),
                y: -35.0,
                vx: rand::gen_range(-2.2, 2.2),
                vy: rand::gen_range(1.5, 2.5),
                ship_type: kind,
                hp,
                max_hp: hp,
                fire_timer: 35,
                age: 0.0,
            });
        }

        let mut destroyed_aliens = Vec::new();
        for (i, alien) in self.aliens.iter_mut().enumerate() {
            alien.age += dt;
            alien.x += alien.vx;
            alien.y += alien.vy;

            if alien.ship_type == AlienType::Scout {
                alien.vx = (alien.age * 3.8).sin() * 3.5;
            }

            alien.fire_timer -= 1;
            if alien.fire_timer == 0 {
                alien.fire_timer = rand::gen_range(45, 80);
                match alien.ship_type {
                    AlienType::Scout => {
                        // Disparos duplos mirando no vetor do jogador
                        let to_p = (Vec2::new(px, py) - Vec2::new(alien.x, alien.y)).normalize_or_zero();
                        self.enemy_lasers.push(EnemyLaser {
                            x: alien.x, y: alien.y + 12.0, vx: to_p.x * 7.0, vy: to_p.y * 7.0, radius: 4.0, color: CORAL, is_torpedo: false,
                        });
                    }
                    AlienType::MineLayer => {
                        self.mines.push(ProximityMine {
                            x: alien.x, y: alien.y + 14.0, vy: 1.2, timer: 12.0, pulse: 0.0,
                        });
                    }
                    AlienType::Gunboat => {
                        // Rajada pesada de 3 tiros em dispersão
                        for ang in [-0.25f32, 0.0f32, 0.25f32] {
                            self.enemy_lasers.push(EnemyLaser {
                                x: alien.x, y: alien.y + 16.0, vx: ang.sin() * 6.5, vy: ang.cos() * 6.5, radius: 5.0, color: CORAL, is_torpedo: false,
                            });
                        }
                    }
                }
            }

            if alien.hp <= 0 {
                destroyed_aliens.push(i);
            }
        }

        for &idx in destroyed_aliens.iter().rev() {
            if idx < self.aliens.len() {
                let alien = self.aliens.remove(idx);
                self.score += (450.0 * self.combo_multiplier) as u32;
                self.wave_kills += 1;
                self.drop_loot(alien.x, alien.y, None, true);
                self.shockwaves.push(Shockwave {
                    x: alien.x, y: alien.y, radius: 5.0, max_radius: 70.0, speed: 14.0, color: CORAL,
                });
            }
        }
        self.aliens.retain(|a| a.y < 640.0 && a.x > -50.0 && a.x < 850.0);

        // 9. Atualizar Minas Orbitais
        self.mines.retain_mut(|mine| {
            mine.y += mine.vy;
            mine.pulse += dt * 5.0;
            mine.timer -= dt;

            let dist = (Vec2::new(mine.x, mine.y) - Vec2::new(px, py)).length();
            if dist < 45.0 || mine.timer <= 0.0 {
                // Detonação em 8 estilhaços radiais
                for a in 0..8 {
                    let ang = (a as f32) * 0.785;
                    self.enemy_lasers.push(EnemyLaser {
                        x: mine.x, y: mine.y,
                        vx: ang.cos() * 4.8, vy: ang.sin() * 4.8,
                        radius: 3.5, color: YELLOW_ACCENT, is_torpedo: false,
                    });
                }
                self.shockwaves.push(Shockwave {
                    x: mine.x, y: mine.y, radius: 4.0, max_radius: 55.0, speed: 14.0, color: CORAL,
                });
                return false;
            }
            mine.y < 620.0
        });

        // 10. Atualizar Projéteis Inimigos
        let mut enemy_laser_damage = 0.0f32;
        self.enemy_lasers.retain_mut(|el| {
            el.x += el.vx;
            el.y += el.vy;

            let dist = (Vec2::new(el.x, el.y) - Vec2::new(px, py)).length();
            if dist < el.radius + 16.0 {
                enemy_laser_damage += if el.is_torpedo { 35.0 } else { 22.0 };
                return false;
            }
            el.y > -20.0 && el.y < 620.0 && el.x > -20.0 && el.x < 820.0
        });

        if enemy_laser_damage > 0.0 {
            self.take_player_damage(enemy_laser_damage);
        }

        // 11. CHEFE DREADNOUGHT FLAGSHIP (BATALHA ÉPICA MULTI-FASES)
        if let Some(boss) = &mut self.boss {
            match boss.phase {
                BossPhase::Warning => {
                    boss.phase_timer -= dt;
                    if boss.phase_timer <= 0.0 {
                        boss.phase = BossPhase::Arriving;
                    }
                }
                BossPhase::Arriving => {
                    boss.y += 1.6;
                    if boss.y >= 90.0 {
                        boss.phase = BossPhase::Phase1Turrets;
                    }
                }
                BossPhase::Phase1Turrets => {
                    boss.x += boss.vx;
                    if boss.x < 180.0 || boss.x > 620.0 {
                        boss.vx *= -1.0;
                    }

                    // Rastreamento Angular das Baterias até o Jogador
                    let p_vec = Vec2::new(px, py);
                    let t_pos_l = Vec2::new(boss.x + boss.turret_left.rel_x, boss.y + boss.turret_left.rel_y);
                    let t_pos_r = Vec2::new(boss.x + boss.turret_right.rel_x, boss.y + boss.turret_right.rel_y);
                    boss.turret_left.angle = (p_vec.y - t_pos_l.y).atan2(p_vec.x - t_pos_l.x);
                    boss.turret_right.angle = (p_vec.y - t_pos_r.y).atan2(p_vec.x - t_pos_r.x);

                    // Disparos da Turret Esquerda
                    if !boss.turret_left.destroyed {
                        if boss.turret_left.hp <= 0 {
                            boss.turret_left.destroyed = true;
                            self.score += 4000;
                            self.shockwaves.push(Shockwave { x: t_pos_l.x, y: t_pos_l.y, radius: 10.0, max_radius: 90.0, speed: 15.0, color: YELLOW_ACCENT });
                        } else {
                            boss.turret_left.fire_timer -= 1;
                            if boss.turret_left.fire_timer == 0 {
                                boss.turret_left.fire_timer = 40;
                                let a = boss.turret_left.angle;
                                self.enemy_lasers.push(EnemyLaser { x: t_pos_l.x, y: t_pos_l.y, vx: a.cos() * 7.5, vy: a.sin() * 7.5, radius: 5.5, color: CORAL, is_torpedo: false });
                            }
                        }
                    }

                    // Disparos da Turret Direita
                    if !boss.turret_right.destroyed {
                        if boss.turret_right.hp <= 0 {
                            boss.turret_right.destroyed = true;
                            self.score += 4000;
                            self.shockwaves.push(Shockwave { x: t_pos_r.x, y: t_pos_r.y, radius: 10.0, max_radius: 90.0, speed: 15.0, color: YELLOW_ACCENT });
                        } else {
                            boss.turret_right.fire_timer -= 1;
                            if boss.turret_right.fire_timer == 0 {
                                boss.turret_right.fire_timer = 40;
                                let a = boss.turret_right.angle;
                                self.enemy_lasers.push(EnemyLaser { x: t_pos_r.x, y: t_pos_r.y, vx: a.cos() * 7.5, vy: a.sin() * 7.5, radius: 5.5, color: CORAL, is_torpedo: false });
                            }
                        }
                    }

                    // Se ambas as baterias caírem, o escudo frontal colapsa e entra em Sobrecarga!
                    if boss.turret_left.destroyed && boss.turret_right.destroyed {
                        boss.phase = BossPhase::Phase3Overcharge;
                        self.shockwaves.push(Shockwave { x: boss.x, y: boss.y + 20.0, radius: 10.0, max_radius: 180.0, speed: 16.0, color: ICE_BLUE });
                    }
                }
                BossPhase::Phase2Carrier => {}
                BossPhase::Phase3Overcharge => {
                    boss.x += boss.vx * 1.5;
                    if boss.x < 160.0 || boss.x > 640.0 { boss.vx *= -1.0; }

                    boss.attack_timer -= 1;
                    if boss.attack_timer == 0 {
                        boss.attack_timer = 28;
                        // Varredura rotativa de plasma do núcleo desprotegido
                        let t = get_time() as f32 * 4.0;
                        for i in 0..6 {
                            let ang = (i as f32) * 1.047 + t;
                            self.enemy_lasers.push(EnemyLaser {
                                x: boss.x, y: boss.y + 35.0,
                                vx: ang.cos() * 6.5, vy: ang.sin().abs() * 6.5 + 2.0,
                                radius: 5.0, color: YELLOW_ACCENT, is_torpedo: false,
                            });
                        }
                    }

                    if boss.core_hp <= 0 {
                        boss.phase = BossPhase::Exploding;
                    }
                }
                BossPhase::Exploding => {
                    boss.explosion_countdown -= dt;
                    self.camera_shake = (self.camera_shake + 0.5).min(1.0);
                    if rand::gen_range(0, 3) == 0 {
                        let rx = boss.x + rand::gen_range(-110.0, 110.0);
                        let ry = boss.y + rand::gen_range(-30.0, 45.0);
                        self.shockwaves.push(Shockwave { x: rx, y: ry, radius: 5.0, max_radius: 70.0, speed: 14.0, color: CORAL });
                    }
                    if boss.explosion_countdown <= 0.0 {
                        boss.phase = BossPhase::Defeated;
                        self.score += 25000;
                        self.sector += 1;
                        self.wave = 1;
                        self.wave_kills = 0;
                        self.wave_target_kills = 22;
                        self.wave_banner_text = format!("SETOR {} CONQUISTADO! AVANÇANDO...", self.sector);
                        self.wave_banner_timer = 4.0;
                        self.shockwaves.push(Shockwave { x: boss.x, y: boss.y, radius: 10.0, max_radius: 500.0, speed: 22.0, color: YELLOW_ACCENT });
                    }
                }
                BossPhase::Defeated => {}
            }
        }

        if let Some(b) = &self.boss {
            if b.phase == BossPhase::Defeated {
                self.boss = None;
            }
        }

        // 12. Atualizar PowerUps e Atração Magnética
        self.powerups.retain_mut(|pup| {
            pup.life -= dt;
            pup.anim += dt * 3.0;
            if pup.life <= 0.0 { return false; }

            // Tractor Beam (alcance de 120px)
            let to_p = Vec2::new(px - pup.x, py - pup.y);
            let dist = to_p.length();
            if dist < 120.0 {
                let dir = to_p.normalize_or_zero();
                pup.vx += dir.x * 0.7;
                pup.vy += dir.y * 0.7;
            }

            pup.x += pup.vx;
            pup.y += pup.vy;
            pup.vx *= 0.94;
            pup.vy *= 0.94;

            // Coleta
            if dist < 24.0 {
                match pup.kind {
                    PowerUpType::Shield => {
                        self.shield = (self.shield + 18.0).min(100.0); // +18% equilibrado
                        self.floating_texts.push(FloatingText { x: px, y: py - 30.0, text: "+18% ESCUDO".to_string(), color: ICE_BLUE, life: 1.2 });
                    }
                    PowerUpType::Weapon => {
                        self.weapon_pods_collected += 1;
                        let needed = self.weapon_level; // Requer N pods para atingir nível N+1
                        if self.weapon_pods_collected >= needed && self.weapon_level < 5 {
                            self.weapon_level += 1;
                            self.weapon_pods_collected = 0;
                            self.floating_texts.push(FloatingText { x: px, y: py - 30.0, text: format!("CANHÃO NÍVEL {}!", self.weapon_level), color: YELLOW_ACCENT, life: 1.5 });
                        } else {
                            self.floating_texts.push(FloatingText { x: px, y: py - 30.0, text: format!("UPGRADE: {}/{}", self.weapon_pods_collected, needed), color: YELLOW_ACCENT, life: 1.0 });
                        }
                    }
                    PowerUpType::NovaBomb => {
                        self.nova_bombs = (self.nova_bombs + 1).min(3);
                        self.floating_texts.push(FloatingText { x: px, y: py - 30.0, text: "+1 OGIVA EMP".to_string(), color: Color::new(0.7, 0.4, 1.0, 1.0), life: 1.5 });
                    }
                    PowerUpType::Overdrive => {
                        self.overdrive_timer = 6.0;
                        self.floating_texts.push(FloatingText { x: px, y: py - 30.0, text: "OVERDRIVE!".to_string(), color: Color::new(0.8, 0.4, 1.0, 1.0), life: 1.5 });
                    }
                    PowerUpType::Stardust => {
                        self.score += (75.0 * self.combo_multiplier) as u32;
                    }
                }
                return false;
            }
            pup.y < 620.0
        });

        // 13. Partículas e Shockwaves
        self.particles.retain_mut(|p| {
            p.x += p.vx;
            p.y += p.vy;
            p.life -= dt;
            p.life > 0.0
        });

        self.shockwaves.retain_mut(|sw| {
            sw.radius += sw.speed;
            sw.radius < sw.max_radius
        });

        self.floating_texts.retain_mut(|ft| {
            ft.y -= 1.0;
            ft.life -= dt;
            ft.life > 0.0
        });

        // 14. Checagem de Progressão da Onda
        self.check_wave_progression();

        // 15. Spawn de Asteroides conforme Onda
        self.spawn_timer += 1;
        let rate = match self.wave {
            1 => 44,
            2 => 38,
            3 => 32,
            4 => 26,
            _ => 36,
        };

        if self.spawn_timer >= rate && self.boss.is_none() {
            self.spawn_timer = 0;
            let tier = if self.wave >= 3 && rand::gen_range(0, 8) == 0 {
                AsteroidTier::Radioactive
            } else if self.wave >= 2 && rand::gen_range(0, 5) == 0 {
                AsteroidTier::Metallic
            } else if rand::gen_range(0, 3) == 0 {
                AsteroidTier::Titan
            } else {
                AsteroidTier::Medium
            };

            self.asteroids.push(Asteroid::new(
                rand::gen_range(40.0, 760.0),
                -45.0,
                rand::gen_range(-1.4, 1.4),
                rand::gen_range(2.0, 3.8 + (self.wave as f32) * 0.25),
                tier,
            ));
        }
    }

    fn draw(&self, screen_w: f32, screen_h: f32) {
        let shake_x = if self.camera_shake > 0.0 { rand::gen_range(-1.0, 1.0) * self.camera_shake * 12.0 } else { 0.0 };
        let shake_y = if self.camera_shake > 0.0 { rand::gen_range(-1.0, 1.0) * self.camera_shake * 12.0 } else { 0.0 };

        let ox = (screen_w - 800.0) * 0.5 + shake_x;
        let oy = (screen_h - 600.0) * 0.5 + 24.0 + shake_y;

        // Fundo do Palco
        draw_rectangle(ox, oy, 800.0, 600.0, BG_COLOR);
        draw_rectangle_lines(ox, oy, 800.0, 600.0, 1.5, BORDER_COLOR);

        // 1. Estrelas de Fundo (Parallax)
        for s in &self.stars {
            let col = match s.layer {
                0 => Color::new(0.35, 0.40, 0.55, s.brightness),
                1 => Color::new(0.45, 0.65, 0.90, s.brightness),
                _ => Color::new(0.95, 0.98, 1.0, s.brightness),
            };
            if s.layer == 2 {
                draw_line(ox + s.x, oy + s.y, ox + s.x, oy + s.y + 4.0, s.size, col);
            } else {
                draw_rectangle(ox + s.x, oy + s.y, s.size, s.size, col);
            }
        }

        // 2. Shockwaves
        for sw in &self.shockwaves {
            let alpha = (1.0 - sw.radius / sw.max_radius).max(0.0);
            let mut c = sw.color;
            c.a = alpha;
            draw_circle_lines(ox + sw.x, oy + sw.y, sw.radius, 2.5, c);
        }

        // 3. Asteroides com Texturas de Crateras, Sombras e Iluminação Direcional 3D
        for ast in &self.asteroids {
            let n_verts = ast.vertex_offsets.len();
            let center = Vec2::new(ox + ast.x, oy + ast.y);

            // Facetas do corpo rochoso com sombreamento direcional
            for i in 0..n_verts {
                let a1 = (i as f32) * (6.283 / n_verts as f32) + ast.rotation;
                let a2 = ((i + 1) as f32) * (6.283 / n_verts as f32) + ast.rotation;
                let r1 = ast.radius * ast.vertex_offsets[i];
                let r2 = ast.radius * ast.vertex_offsets[(i + 1) % n_verts];

                let p1 = Vec2::new(center.x + a1.cos() * r1, center.y + a1.sin() * r1);
                let p2 = Vec2::new(center.x + a2.cos() * r2, center.y + a2.sin() * r2);

                // Cálculo de iluminação solar vinda do topo-esquerdo (-0.7, -0.7)
                let mid_ang = (a1 + a2) * 0.5;
                let light_factor = ((-mid_ang.cos() - mid_ang.sin()) * 0.5).clamp(0.0, 1.0);

                let (base_r, base_g, base_b) = match ast.tier {
                    AsteroidTier::Radioactive => (0.16 + light_factor * 0.15, 0.22 + light_factor * 0.20, 0.10),
                    AsteroidTier::Metallic => (0.22 + light_factor * 0.28, 0.26 + light_factor * 0.30, 0.36 + light_factor * 0.35),
                    AsteroidTier::Titan => (0.14 + light_factor * 0.18, 0.17 + light_factor * 0.18, 0.24 + light_factor * 0.20),
                    _ => (0.11 + light_factor * 0.14, 0.14 + light_factor * 0.16, 0.20 + light_factor * 0.22),
                };

                let edge_col = match ast.tier {
                    AsteroidTier::Radioactive => YELLOW_ACCENT,
                    AsteroidTier::Metallic => Color::new(0.65, 0.80, 0.98, 1.0),
                    AsteroidTier::Titan => Color::new(0.45, 0.55, 0.70, 1.0),
                    _ => Color::new(0.32, 0.40, 0.52, 1.0),
                };

                draw_triangle(center, p1, p2, Color::new(base_r, base_g, base_b, 1.0));
                draw_line(p1.x, p1.y, p2.x, p2.y, 1.5, edge_col);
            }

            // Crateras com Profundidade e Borda Elevada Iluminada
            for cr in &ast.craters {
                let rot_x = cr.rel_x * ast.rotation.cos() - cr.rel_y * ast.rotation.sin();
                let rot_y = cr.rel_x * ast.rotation.sin() + cr.rel_y * ast.rotation.cos();
                let cx = center.x + rot_x;
                let cy = center.y + rot_y;

                // Fundo escuro da cratera
                draw_circle(cx, cy, cr.radius, Color::new(0.06, 0.08, 0.13, 1.0));
                // Borda iluminada virada para a luz
                draw_arc(cx, cy, 6, cr.radius, -135.0, 90.0, 1.2, Color::new(0.45, 0.55, 0.70, 0.8));
            }

            // Veios de Magma Pulsante para Asteroides Radioativos
            if ast.tier == AsteroidTier::Radioactive {
                let pulse = (get_time() as f32 * 4.0).sin() * 0.3 + 0.7;
                draw_circle_lines(center.x, center.y, ast.radius * 0.55, 1.5, Color::new(0.98, 0.80, 0.08, pulse));
            }
        }

        // 4. Caças Alienígenas com Sprites Vetoriais Detalhados
        for alien in &self.aliens {
            let ax = ox + alien.x;
            let ay = oy + alien.y;
            match alien.ship_type {
                AlienType::Scout => {
                    // Caça Predador V-Wing
                    draw_triangle(Vec2::new(ax, ay + 16.0), Vec2::new(ax - 14.0, ay - 12.0), Vec2::new(ax, ay - 4.0), Color::new(0.18, 0.08, 0.14, 1.0));
                    draw_triangle(Vec2::new(ax, ay + 16.0), Vec2::new(ax + 14.0, ay - 12.0), Vec2::new(ax, ay - 4.0), Color::new(0.18, 0.08, 0.14, 1.0));
                    // Bordas nítidas coral
                    draw_line(ax, ay + 16.0, ax - 14.0, ay - 12.0, 1.8, CORAL);
                    draw_line(ax, ay + 16.0, ax + 14.0, ay - 12.0, 1.8, CORAL);
                    // Sensor ótico ciclope vermelho
                    draw_circle(ax, ay, 3.5, YELLOW_ACCENT);
                }
                AlienType::MineLayer => {
                    // Drone Industrial com Rotor Giratório
                    let rot_ang = get_time() as f32 * 5.0;
                    draw_poly(ax, ay, 8, 16.0, rot_ang, Color::new(0.18, 0.12, 0.24, 1.0));
                    draw_poly_lines(ax, ay, 8, 16.0, rot_ang, 1.5, Color::new(0.75, 0.40, 0.95, 1.0));
                    draw_circle(ax, ay, 5.0, CORAL);
                }
                AlienType::Gunboat => {
                    // Canhoneira Blindada com Escudo Frontal de Plasma
                    draw_rectangle(ax - 22.0, ay - 16.0, 44.0, 32.0, Color::new(0.16, 0.20, 0.30, 1.0));
                    draw_rectangle_lines(ax - 22.0, ay - 16.0, 44.0, 32.0, 1.5, ICE_BLUE);
                    // Canhões duplos
                    draw_line(ax - 12.0, ay + 16.0, ax - 12.0, ay + 24.0, 2.5, CORAL);
                    draw_line(ax + 12.0, ay + 16.0, ax + 12.0, ay + 24.0, 2.5, CORAL);
                    // Escudo frontal translúcido
                    draw_arc(ax, ay - 8.0, 8, 26.0, 20.0, 140.0, 2.5, Color::new(0.38, 0.65, 0.98, 0.7));
                }
            }
        }

        // 5. Minas Orbitais
        for mine in &self.mines {
            let mx = ox + mine.x;
            let my = oy + mine.y;
            let pulse_r = 8.0 + (mine.pulse).sin() * 2.0;
            // Espigões de detecção
            for i in 0..8 {
                let a = (i as f32) * 0.785;
                draw_line(mx, my, mx + a.cos() * 14.0, my + a.sin() * 14.0, 1.5, CORAL);
            }
            draw_circle(mx, my, pulse_r, Color::new(0.25, 0.10, 0.15, 1.0));
            draw_circle_lines(mx, my, pulse_r, 1.5, CORAL);
            draw_circle(mx, my, 2.5, YELLOW_ACCENT);
        }

        // 6. CHEFE COLOSSAL DREADNOUGHT FLAGSHIP (SPRITE VETORIAL MASSIVO)
        if let Some(boss) = &self.boss {
            let bx = ox + boss.x;
            let by = oy + boss.y;

            // Chassi Blindado Principal (300px x 90px)
            draw_rectangle(bx - 150.0, by - 45.0, 300.0, 80.0, Color::new(0.08, 0.12, 0.20, 1.0));
            draw_rectangle_lines(bx - 150.0, by - 45.0, 300.0, 80.0, 2.0, BORDER_COLOR);

            // Proa Angular Dianteira
            draw_triangle(Vec2::new(bx - 150.0, by + 35.0), Vec2::new(bx + 150.0, by + 35.0), Vec2::new(bx, by + 75.0), Color::new(0.10, 0.15, 0.25, 1.0));
            draw_line(bx - 150.0, by + 35.0, bx, by + 75.0, 2.0, CORAL);
            draw_line(bx + 150.0, by + 35.0, bx, by + 75.0, 2.0, CORAL);

            // Ponte de Comando Superior com Janelas de Batalha
            draw_rectangle(bx - 60.0, by - 65.0, 120.0, 22.0, Color::new(0.14, 0.18, 0.28, 1.0));
            draw_rectangle_lines(bx - 60.0, by - 65.0, 120.0, 22.0, 1.5, ICE_BLUE);
            for w in -2..=2 {
                draw_rectangle(bx + (w as f32) * 18.0 - 4.0, by - 58.0, 8.0, 6.0, CORAL);
            }

            // Bateria / Turret Esquerda (com Rotação Ativa virada para o Jogador)
            let tl_x = bx + boss.turret_left.rel_x;
            let tl_y = by + boss.turret_left.rel_y;
            let t_col_l = if boss.turret_left.destroyed { Color::new(0.3, 0.3, 0.3, 1.0) } else { CORAL };
            draw_circle(tl_x, tl_y, 18.0, Color::new(0.14, 0.18, 0.28, 1.0));
            draw_circle_lines(tl_x, tl_y, 18.0, 2.0, t_col_l);
            if !boss.turret_left.destroyed {
                let a = boss.turret_left.angle;
                draw_line(tl_x, tl_y, tl_x + a.cos() * 26.0, tl_y + a.sin() * 26.0, 4.0, CORAL);
            }

            // Bateria / Turret Direita
            let tr_x = bx + boss.turret_right.rel_x;
            let tr_y = by + boss.turret_right.rel_y;
            let t_col_r = if boss.turret_right.destroyed { Color::new(0.3, 0.3, 0.3, 1.0) } else { CORAL };
            draw_circle(tr_x, tr_y, 18.0, Color::new(0.14, 0.18, 0.28, 1.0));
            draw_circle_lines(tr_x, tr_y, 18.0, 2.0, t_col_r);
            if !boss.turret_right.destroyed {
                let a = boss.turret_right.angle;
                draw_line(tr_x, tr_y, tr_x + a.cos() * 26.0, tr_y + a.sin() * 26.0, 4.0, CORAL);
            }

            // Núcleo do Reator Central
            let core_col = if boss.phase == BossPhase::Phase3Overcharge {
                YELLOW_ACCENT
            } else {
                Color::new(0.65, 0.35, 1.0, 1.0)
            };
            draw_circle(bx, by + 18.0, 24.0, core_col);
            draw_circle_lines(bx, by + 18.0, 24.0, 2.5, TEXT_PRIMARY);

            // Escudo Frontal Defletor (Se as turrets ainda estiverem ativas)
            if !boss.turret_left.destroyed || !boss.turret_right.destroyed {
                let pulse_sc = (get_time() as f32 * 4.0).sin() * 0.2 + 0.6;
                draw_arc(bx, by + 20.0, 12, 140.0, 20.0, 140.0, 3.5, Color::new(0.38, 0.65, 0.98, pulse_sc));
                draw_text("ESCUDO FRONTAL ATIVO", bx - 75.0, by + 90.0, 12.0, ICE_BLUE);
            }

            // Barra de Vida do Chefe
            let bar_w = 440.0;
            let bar_x = ox + 180.0;
            let bar_y = oy + 22.0;
            draw_card_rect(bar_x, bar_y, bar_w, 14.0, BG_COLOR, BORDER_COLOR);
            let hp_frac = (boss.core_hp as f32 / boss.core_max_hp as f32).clamp(0.0, 1.0);
            draw_rectangle(bar_x + 1.0, bar_y + 1.0, (bar_w - 2.0) * hp_frac, 12.0, CORAL);
            draw_text(&format!("FLAGSHIP DREADNOUGHT (NÚCLEO: {} HP)", boss.core_hp), bar_x + 105.0, bar_y - 6.0, 13.0, CORAL);
        }

        // 7. Cápsulas Táticas Flutuantes
        for pup in &self.powerups {
            let px = ox + pup.x;
            let py = oy + pup.y;
            let (bg_col, label) = match pup.kind {
                PowerUpType::Shield => (ICE_BLUE, "S"),
                PowerUpType::Weapon => (YELLOW_ACCENT, "W"),
                PowerUpType::NovaBomb => (Color::new(0.75, 0.45, 1.0, 1.0), "B"),
                PowerUpType::Overdrive => (Color::new(0.9, 0.3, 1.0, 1.0), "O"),
                PowerUpType::Stardust => (EMERALD, "*"),
            };
            let bob = (pup.anim).sin() * 3.0;
            draw_poly(px, py + bob, 6, 12.0, pup.anim, Color::new(0.08, 0.12, 0.18, 0.95));
            draw_poly_lines(px, py + bob, 6, 12.0, pup.anim, 1.5, bg_col);
            draw_text(label, px - 4.0, py + bob + 4.0, 12.0, bg_col);
        }

        // 8. Lasers do Jogador
        for l in &self.lasers {
            if l.is_missile {
                draw_circle(ox + l.x, oy + l.y, 4.5, YELLOW_ACCENT);
                draw_line(ox + l.x, oy + l.y, ox + l.x - l.vx * 1.5, oy + l.y - l.vy * 1.5, 2.5, CORAL);
            } else {
                draw_line(ox + l.x, oy + l.y, ox + l.x, oy + l.y + 14.0, 3.5, l.color);
                draw_line(ox + l.x, oy + l.y + 2.0, ox + l.x, oy + l.y + 10.0, 1.5, TEXT_PRIMARY);
            }
        }

        // 9. Lasers Inimigos
        for el in &self.enemy_lasers {
            draw_circle(ox + el.x, oy + el.y, el.radius, el.color);
            draw_circle(ox + el.x, oy + el.y, el.radius * 0.5, TEXT_PRIMARY);
        }

        // 10. Partículas
        for p in &self.particles {
            let alpha = (p.life / p.max_life).clamp(0.0, 1.0);
            let mut col = p.color;
            col.a = alpha;
            draw_circle(ox + p.x, oy + p.y, p.size * alpha, col);
        }

        // 11. NAVE DO JOGADOR — "STARFIGHTER APEX" (SPRITE DE ALTA FIDELIDADE)
        if !self.game_over && (self.invuln_timer <= 0.0 || (get_time() * 20.0) as i32 % 2 == 0) {
            let px = ox + self.player_x;
            let py = oy + self.player_y;

            // Fantasmas de Afterimage do Dash
            for img in &self.afterimages {
                let ix = ox + img.x;
                let iy = oy + img.y;
                let ghost_col = Color::new(0.38, 0.65, 0.98, img.alpha);
                draw_triangle(Vec2::new(ix, iy - 20.0), Vec2::new(ix - 20.0, iy + 14.0), Vec2::new(ix, iy + 8.0), ghost_col);
                draw_triangle(Vec2::new(ix, iy - 20.0), Vec2::new(ix + 20.0, iy + 14.0), Vec2::new(ix, iy + 8.0), ghost_col);
            }

            // Propulsores Iônicos Duplos (Chama Tri-Camada)
            let fl1 = rand::gen_range(8.0, 18.0);
            let fl2 = rand::gen_range(8.0, 18.0);
            // Chama Externa Âmbar
            draw_triangle(Vec2::new(px - 11.0, py + 14.0), Vec2::new(px - 3.0, py + 14.0), Vec2::new(px - 7.0, py + 14.0 + fl1), YELLOW_ACCENT);
            draw_triangle(Vec2::new(px + 3.0, py + 14.0), Vec2::new(px + 11.0, py + 14.0), Vec2::new(px + 7.0, py + 14.0 + fl2), YELLOW_ACCENT);
            // Núcleo Branco Interno
            draw_triangle(Vec2::new(px - 9.0, py + 14.0), Vec2::new(px - 5.0, py + 14.0), Vec2::new(px - 7.0, py + 14.0 + fl1 * 0.5), TEXT_PRIMARY);
            draw_triangle(Vec2::new(px + 5.0, py + 14.0), Vec2::new(px + 9.0, py + 14.0), Vec2::new(px + 7.0, py + 14.0 + fl2 * 0.5), TEXT_PRIMARY);

            // Fuselagem com Projeção e Inclinação Dinâmica (Tilt)
            let tilt_offset = self.tilt * 6.0;
            let nose = Vec2::new(px + tilt_offset, py - 22.0);
            let wing_l = Vec2::new(px - 22.0 + tilt_offset * 0.4, py + 14.0);
            let wing_r = Vec2::new(px + 22.0 + tilt_offset * 0.4, py + 14.0);
            let keel = Vec2::new(px + tilt_offset, py + 8.0);

            // Asas Delta Principais (Azul Cobalto)
            draw_triangle(nose, wing_l, keel, COBALT_ACCENT);
            draw_triangle(nose, wing_r, keel, COBALT_ACCENT);

            // Placas de Blindagem Interna (Azul Gelo)
            draw_triangle(nose, Vec2::new(px - 9.0, py + 10.0), keel, ICE_BLUE);
            draw_triangle(nose, Vec2::new(px + 9.0, py + 10.0), keel, ICE_BLUE);

            // Winglets / Estabilizadores Laterais com LEDs de Navegação
            draw_circle(wing_l.x, wing_l.y, 2.5, ICE_BLUE);
            draw_circle(wing_r.x, wing_r.y, 2.5, EMERALD);

            // Canópi da Cabine com Reflexo Especular
            draw_circle(px + tilt_offset, py - 3.0, 5.0, Color::new(0.06, 0.15, 0.30, 1.0));
            draw_circle(px + tilt_offset - 1.5, py - 4.5, 2.0, TEXT_PRIMARY);

            // Escudo de Força Hexagonal Dinâmico
            if self.shield > 0.0 || self.shield_hit_flash > 0.0 {
                let shield_alpha = if self.shield_hit_flash > 0.0 { 0.85 } else { (self.shield / 100.0) * 0.28 };
                draw_poly_lines(px, py, 6, 28.0, get_time() as f32 * 1.5, 1.8, Color::new(0.38, 0.65, 0.98, shield_alpha));
            }

            // Drones no Overdrive
            if self.overdrive_timer > 0.0 {
                let t = get_time() as f32 * 6.0;
                let d1x = px + (t).cos() * 34.0;
                let d1y = py + (t).sin() * 20.0;
                let d2x = px + (t + 3.14).cos() * 34.0;
                let d2y = py + (t + 3.14).sin() * 20.0;
                draw_circle(d1x, d1y, 4.0, Color::new(0.7, 0.4, 1.0, 1.0));
                draw_circle(d2x, d2y, 4.0, Color::new(0.7, 0.4, 1.0, 1.0));
            }
        }

        // 12. Textos Flutuantes
        for ft in &self.floating_texts {
            draw_text(&ft.text, ox + ft.x, oy + ft.y, 14.0, ft.color);
        }

        // 13. Banner de Onda / Setor
        if self.wave_banner_timer > 0.0 {
            let alpha = (self.wave_banner_timer / 3.5).clamp(0.0, 1.0);
            let b_col = if self.boss.is_some() { CORAL } else { YELLOW_ACCENT };
            let mut c = b_col;
            c.a = alpha;
            let font_sz = 18.0;
            let dim = measure_text(&self.wave_banner_text, None, font_sz as u16, 1.0);
            draw_text(&self.wave_banner_text, ox + (800.0 - dim.width) * 0.5, oy + 120.0, font_sz, c);
        }

        // 14. HUD TÁTICO IN-GAME (Superior)
        let hud_y = oy + 12.0;

        // Barra de Escudo
        draw_text("ESCUDO", ox + 20.0, hud_y + 12.0, 11.0, ICE_BLUE);
        draw_card_rect(ox + 75.0, hud_y + 2.0, 95.0, 12.0, BG_COLOR, BORDER_COLOR);
        let s_frac = (self.shield / 100.0).clamp(0.0, 1.0);
        draw_rectangle(ox + 76.0, hud_y + 3.0, 93.0 * s_frac, 10.0, ICE_BLUE);

        // Barra de Casco
        draw_text("CASCO", ox + 185.0, hud_y + 12.0, 11.0, EMERALD);
        draw_card_rect(ox + 230.0, hud_y + 2.0, 95.0, 12.0, BG_COLOR, BORDER_COLOR);
        let h_frac = (self.hull / 100.0).clamp(0.0, 1.0);
        let h_col = if h_frac > 0.35 { EMERALD } else { CORAL };
        draw_rectangle(ox + 231.0, hud_y + 3.0, 93.0 * h_frac, 10.0, h_col);

        // Nível de Arma com Indicador de Pods
        let w_name = match self.weapon_level {
            1 => "DUAL",
            2 => "TRIPLE",
            3 => "SCATTER",
            4 => "LANCE",
            _ => "HYPER",
        };
        draw_text(&format!("ARMA: LVL {} ({})", self.weapon_level, w_name), ox + 345.0, hud_y + 12.0, 11.0, YELLOW_ACCENT);

        // Setor e Onda
        draw_text(&format!("SETOR {} - ONDA {}", self.sector, self.wave), ox + 490.0, hud_y + 12.0, 11.0, TEXT_PRIMARY);

        // Ogivas Nova EMP
        draw_text("EMP (X):", ox + 630.0, hud_y + 12.0, 11.0, Color::new(0.75, 0.45, 1.0, 1.0));
        for i in 0..3 {
            let col = if i < self.nova_bombs { Color::new(0.75, 0.45, 1.0, 1.0) } else { Color::new(0.2, 0.2, 0.28, 0.5) };
            draw_circle(ox + 690.0 + (i as f32) * 14.0, hud_y + 8.0, 4.5, col);
        }

        // Multiplicador de Combo
        if self.combo_multiplier > 1.0 {
            let combo_str = format!("{:.1}x COMBO!", self.combo_multiplier);
            draw_text(&combo_str, ox + 20.0, oy + 580.0, 15.0, YELLOW_ACCENT);
        }

        // Overlay de Fim de Jogo
        if self.game_over {
            draw_rectangle(ox, oy, 800.0, 600.0, Color::new(0.035, 0.047, 0.078, 0.88));
            let box_w = 480.0;
            let box_h = 240.0;
            let bx = ox + (800.0 - box_w) * 0.5;
            let by = oy + (600.0 - box_h) * 0.5;

            draw_card_rect(bx, by, box_w, box_h, SURFACE_COLOR, CORAL);
            draw_text("MISSÃO ENCERRADA — NAVE DESTRUÍDA", bx + 45.0, by + 40.0, 18.0, CORAL);
            draw_text(&format!("PONTUAÇÃO FINAL: {}", self.score), bx + 130.0, by + 85.0, 16.0, YELLOW_ACCENT);
            draw_text(&format!("SETOR ALCANÇADO: {}   |   ONDA: {}", self.sector, self.wave), bx + 125.0, by + 125.0, 13.0, TEXT_MUTED);
            draw_text("Pressione [R] para reiniciar a missão", bx + 115.0, by + 175.0, 13.0, TEXT_PRIMARY);
            draw_text("ou [ESC] para retornar ao Hub", bx + 145.0, by + 200.0, 11.0, TEXT_MUTED);
        }
    }

    fn reset(&mut self) {
        *self = Self::new();
    }

    fn get_score(&self) -> u32 {
        self.score
    }

    fn is_game_over(&self) -> bool {
        self.game_over
    }

    fn get_title(&self) -> &str {
        "Space Odyssey"
    }
}
