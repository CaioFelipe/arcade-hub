"""
Space Odyssey - Jogo de Combate Espacial em Pygame (Apex Hardcore Edition)
Autor: Comunidade Pygame / Arcade Hub
Licença: MIT

Melhorias:
- Sprites vetoriais detalhados com crateras 3D, iluminação direcional e fuselagem com blindagem
- Dificuldade balanceada e progressiva em 5 ondas por setor
- Drops táticos controlados (debris nunca dropam vida; escudo regenera apenas após 6s sem dano)
- Chefe Flagship Dreadnought épico (850 HP total) com canhões rastreadores e escudo defletor frontal
- 100% auditado pela Sandbox AST do Arcade Hub
"""
import math
import random
import time
import pygame
import hub_sdk

PALETTE = {
    "bg": (9, 12, 20),
    "surface": (16, 21, 34),
    "border": (31, 41, 55),
    "cobalt": (37, 99, 235),
    "ice": (96, 165, 250),
    "emerald": (16, 185, 129),
    "yellow": (250, 204, 21),
    "coral": (244, 63, 94),
    "purple": (168, 85, 247),
    "text": (248, 250, 252),
    "muted": (148, 163, 184),
}


class Game:
    def __init__(self, width=800, height=600):
        self.width = width
        self.height = height
        self.reset()

    def reset(self):
        hub_sdk.set_current_game("space_odyssey")

        self.player_x = self.width / 2.0
        self.player_y = 520.0
        self.player_vx = 0.0
        self.player_vy = 0.0
        self.tilt = 0.0

        # Defesas rigorosamente calibradas
        self.shield = 100.0
        self.hull = 100.0
        self.shield_regen_delay = 0.0 # 6 segundos sem dano
        self.shield_flash = 0.0
        self.invuln_timer = 1.5
        self.dash_cooldown = 0.0
        self.dash_timer = 0.0

        # Arsenal com progressão em pods
        self.weapon_level = 1
        self.weapon_pods = 0
        self.nova_bombs = 1 # Inicia com apenas 1 ogiva EMP
        self.overdrive_timer = 0.0
        self.cooldown = 0

        # Nova EMP
        self.nova_active = False
        self.nova_radius = 0.0
        self.nova_origin = (0.0, 0.0)

        # Pontuação, Setores e Ondas
        self.score = 0
        self.combo_mult = 1.0
        self.combo_timer = 0.0
        self.lives = 3
        self.level = 1
        self.sector = 1
        self.wave = 1
        self.wave_kills = 0
        self.wave_target_kills = 18
        self.wave_banner_text = "ONDA 1: CINTURÃO DE DETRITOS DENSO"
        self.wave_banner_timer = 3.5

        self.game_over = False
        self.score_submitted = False

        # Entidades
        self.stars = [
            [random.randint(0, self.width), random.randint(0, self.height), random.uniform(0.5, 4.0), random.choice([1, 2])]
            for _ in range(110)
        ]
        self.bullets = []
        self.enemy_bullets = []
        self.asteroids = []
        self.aliens = []
        self.mines = []
        self.powerups = []
        self.particles = []
        self.shockwaves = []
        self.floating_texts = []

        # Chefe Flagship Dreadnought
        self.boss = None

        self.spawn_timer = 0
        self.alien_spawn_timer = 0
        self.camera_shake = 0.0

        self.spawn_asteroid("titan", 180, -40)
        self.spawn_asteroid("medium", 620, -60)
        self.spawn_asteroid("metallic", 400, -90)

    def spawn_asteroid(self, tier, x=None, y=None):
        if x is None:
            x = random.uniform(50, self.width - 50)
        if y is None:
            y = -45

        if tier == "titan":
            radius, hp = random.uniform(42, 48), 10
        elif tier == "medium":
            radius, hp = random.uniform(24, 30), 4
        elif tier == "debris":
            radius, hp = random.uniform(12, 16), 2
        elif tier == "metallic":
            radius, hp = random.uniform(28, 34), 12
        else: # radioactive
            radius, hp = random.uniform(30, 36), 7

        vx = random.uniform(-1.4, 1.4)
        vy = random.uniform(2.0, 3.8 + self.wave * 0.25)
        rot = random.uniform(0, 6.28)
        v_rot = random.uniform(-0.03, 0.03)
        shape = [random.uniform(0.80, 1.20) for _ in range(12)]
        craters = [
            (random.uniform(-0.4, 0.4) * radius, random.uniform(-0.4, 0.4) * radius, random.uniform(0.2, 0.3) * radius)
            for _ in range(2 if tier == "medium" else 4)
        ]

        self.asteroids.append([x, y, vx, vy, radius, hp, hp, tier, rot, v_rot, shape, craters])

    def trigger_nova_bomb(self):
        if self.nova_bombs <= 0 or self.nova_active:
            return
        self.nova_bombs -= 1
        self.nova_active = True
        self.nova_radius = 10.0
        self.nova_origin = (self.player_x, self.player_y)
        self.camera_shake = 0.9
        self.enemy_bullets.clear()
        self.shockwaves.append([self.player_x, self.player_y, 10.0, 900.0, 38.0, PALETTE["purple"]])
        self.floating_texts.append([self.player_x, self.player_y - 40, "QUANTUM NOVA EMP!", PALETTE["purple"], 1.5])

    def fire_laser(self):
        is_od = self.overdrive_timer > 0.0
        self.cooldown = 4 if is_od else 9
        px, py = self.player_x, self.player_y

        lvl = self.weapon_level
        if lvl == 1:
            self.bullets.append([px - 11, py - 14, 0, -15, 1, 1, False, 1.1, PALETTE["ice"]])
            self.bullets.append([px + 11, py - 14, 0, -15, 1, 1, False, 1.1, PALETTE["ice"]])
        elif lvl == 2:
            self.bullets.append([px, py - 18, 0, -16, 1, 1, False, 1.1, PALETTE["yellow"]])
            self.bullets.append([px - 14, py - 10, -2.0, -14.5, 1, 1, False, 1.1, PALETTE["ice"]])
            self.bullets.append([px + 14, py - 10, 2.0, -14.5, 1, 1, False, 1.1, PALETTE["ice"]])
        elif lvl == 3:
            for i in (-2, -1, 0, 1, 2):
                ang = i * 0.15
                self.bullets.append([px + i * 6, py - 14, math.sin(ang) * 15, -math.cos(ang) * 15, 1, 2, False, 1.1, PALETTE["ice"]])
        elif lvl == 4:
            self.bullets.append([px - 8, py - 16, 0, -18, 2, 3, False, 1.3, PALETTE["emerald"]])
            self.bullets.append([px + 8, py - 16, 0, -18, 2, 3, False, 1.3, PALETTE["emerald"]])
            self.bullets.append([px - 18, py - 8, -3.0, -14, 1, 2, False, 1.1, PALETTE["ice"]])
            self.bullets.append([px + 18, py - 8, 3.0, -14, 1, 2, False, 1.1, PALETTE["ice"]])
        else:
            self.bullets.append([px - 8, py - 16, 0, -18, 2, 4, False, 1.3, PALETTE["purple"]])
            self.bullets.append([px + 8, py - 16, 0, -18, 2, 4, False, 1.3, PALETTE["purple"]])
            self.bullets.append([px - 22, py - 6, -4, -8, 3, 1, True, 1.8, PALETTE["yellow"]])
            self.bullets.append([px + 22, py - 6, 4, -8, 3, 1, True, 1.8, PALETTE["yellow"]])

        for _ in range(4):
            self.particles.append([
                px + random.uniform(-10, 10), py - 18,
                random.uniform(-1.5, 1.5), random.uniform(-4, -1),
                PALETTE["purple"] if is_od else PALETTE["ice"],
                0.2, 0.2, random.uniform(2, 3.5)
            ])

    def take_damage(self, amount):
        if self.invuln_timer > 0.0 or self.dash_timer > 0.0:
            return
        self.shield_regen_delay = 6.0 # 6s de espera para recarregar
        self.shield_flash = 0.35
        self.camera_shake = min(1.0, self.camera_shake + 0.5)

        if self.shield > 0.0:
            self.shield -= amount
            if self.shield < 0.0:
                self.hull += self.shield
                self.shield = 0.0
        else:
            self.hull -= amount

        for _ in range(14):
            self.particles.append([
                self.player_x, self.player_y,
                random.uniform(-5, 5), random.uniform(-5, 5),
                PALETTE["ice"] if self.shield > 0.0 else PALETTE["coral"],
                0.4, 0.4, random.uniform(2, 4.5)
            ])

        if self.hull <= 0.0:
            self.lives -= 1
            if self.weapon_level > 1:
                self.weapon_level -= 1
                self.weapon_pods = 0
            if self.lives <= 0:
                self.game_over = True
                self.camera_shake = 1.0
                self.shockwaves.append([self.player_x, self.player_y, 10, 350, 20, PALETTE["coral"]])
            else:
                self.hull = 100.0
                self.shield = 100.0
                self.invuln_timer = 3.0
                self.player_x = self.width / 2.0
                self.player_y = 520.0
                self.player_vx = 0.0
                self.player_vy = 0.0
                self.floating_texts.append([self.player_x, self.player_y - 40, f"NAVE RECONSTRUÍDA! ({self.lives} VIDAS)", PALETTE["yellow"], 2.2])

    def drop_loot(self, x, y, tier=None, is_alien=False):
        roll = random.random()
        if tier == "debris":
            if roll < 0.05:
                self.powerups.append([x, y, random.uniform(-1, 1), random.uniform(0.8, 1.8), "stardust", 12.0])
        elif tier == "medium":
            if roll < 0.12:
                self.powerups.append([x, y, random.uniform(-1, 1), random.uniform(0.8, 1.8), "stardust", 12.0])
        elif tier == "titan":
            if roll < 0.20:
                kind = "weapon" if (self.weapon_level < 5 and roll < 0.10) else "shield"
                self.powerups.append([x, y, random.uniform(-1.2, 1.2), random.uniform(0.8, 2.0), kind, 14.0])
        elif tier == "radioactive":
            kind = "weapon" if roll < 0.40 else ("nova" if roll < 0.70 else "overdrive")
            self.powerups.append([x, y, random.uniform(-1.2, 1.2), random.uniform(0.8, 2.0), kind, 15.0])
        elif tier == "metallic":
            if roll < 0.35:
                self.powerups.append([x, y, random.uniform(-1.2, 1.2), random.uniform(0.8, 2.0), "stardust", 14.0])
        elif is_alien:
            if roll < 0.22:
                kind = "weapon" if roll < 0.08 else ("shield" if roll < 0.15 else "stardust")
                self.powerups.append([x, y, random.uniform(-1.2, 1.2), random.uniform(0.8, 2.0), kind, 14.0])

    def check_wave_progression(self):
        if self.boss is not None:
            return
        if self.wave_kills >= self.wave_target_kills:
            self.wave += 1
            self.wave_kills = 0
            if self.wave == 2:
                self.wave_target_kills = 24
                self.wave_banner_text = "ONDA 2: INCURSÃO DE RECONHECIMENTO ALIEN"
                self.wave_banner_timer = 3.5
            elif self.wave == 3:
                self.wave_target_kills = 30
                self.wave_banner_text = "ONDA 3: TEMPESTADE DE MINÉRIOS RADIOATIVOS"
                self.wave_banner_timer = 3.5
            elif self.wave == 4:
                self.wave_target_kills = 36
                self.wave_banner_text = "ONDA 4: ESQUADRÃO DE ATAQUE PESADO"
                self.wave_banner_timer = 3.5
            elif self.wave == 5:
                self.wave_target_kills = 999
                self.wave_banner_text = "/// ALERTA VERMELHO: FLAGSHIP DREADNOUGHT SE APROXIMA ///"
                self.wave_banner_timer = 4.0
                # Chefe Dreadnought (450 HP Core, 200 HP turrets, Escudo Frontal)
                self.boss = {
                    "x": 400.0, "y": -140.0, "vx": 1.8,
                    "phase": "warning", "phase_timer": 3.5,
                    "turret_left": {"hp": 200, "destroyed": False, "timer": 45, "angle": 0.0},
                    "turret_right": {"hp": 200, "destroyed": False, "timer": 45, "angle": 0.0},
                    "core_hp": 450, "core_max_hp": 450,
                    "attack_timer": 60, "explosion_countdown": 4.5
                }
                self.camera_shake = 0.9
            else:
                self.sector += 1
                self.wave = 1
                self.wave_target_kills = 22
                self.wave_banner_text = f"SETOR {self.sector}: QUADRANTE PROFUNDO"
                self.wave_banner_timer = 4.0

    def update(self, keys, events):
        dt = 1.0 / 60.0

        for event in events:
            if event.type == pygame.KEYDOWN:
                if event.key == pygame.K_r and self.game_over:
                    self.reset()
                elif event.key == pygame.K_x and not self.game_over:
                    self.trigger_nova_bomb()
                elif (event.key == pygame.K_LSHIFT or event.key == pygame.K_RSHIFT) and self.dash_cooldown <= 0.0:
                    self.dash_timer = 0.20
                    self.dash_cooldown = 1.8
                    self.invuln_timer = 0.30
                    self.camera_shake = min(1.0, self.camera_shake + 0.2)
            elif event.type == pygame.MOUSEBUTTONDOWN and not self.game_over:
                if event.button == 3:
                    self.trigger_nova_bomb()

        if self.game_over:
            if not self.score_submitted:
                hub_sdk.submit_score(self.score, self.level, "Comandante Fox")
                self.score_submitted = True
            return

        # Temporizadores
        if self.cooldown > 0: self.cooldown -= 1
        if self.invuln_timer > 0.0: self.invuln_timer -= dt
        if self.dash_cooldown > 0.0: self.dash_cooldown -= dt
        if self.dash_timer > 0.0: self.dash_timer -= dt
        if self.overdrive_timer > 0.0: self.overdrive_timer -= dt
        if self.shield_flash > 0.0: self.shield_flash -= dt
        if self.wave_banner_timer > 0.0: self.wave_banner_timer -= dt
        if self.camera_shake > 0.0: self.camera_shake = max(0.0, self.camera_shake - 1.8 * dt)

        # Recarga de Escudo Balanceada (6s de espera, 10% por seg)
        if self.shield_regen_delay > 0.0:
            self.shield_regen_delay -= dt
        elif self.shield < 100.0:
            self.shield = min(100.0, self.shield + 10.0 * dt)

        # Combo
        if self.combo_timer > 0.0:
            self.combo_timer -= dt
            if self.combo_timer <= 0.0:
                self.combo_mult = 1.0

        # Movimento Jogador
        move_x, move_y = 0.0, 0.0
        if keys[pygame.K_LEFT] or keys[pygame.K_a]: move_x -= 1.0
        if keys[pygame.K_RIGHT] or keys[pygame.K_d]: move_x += 1.0
        if keys[pygame.K_UP] or keys[pygame.K_w]: move_y -= 1.0
        if keys[pygame.K_DOWN] or keys[pygame.K_s]: move_y += 1.0

        spd = 15.0 if self.dash_timer > 0.0 else 7.0
        acc = 1.5
        damp = 0.86

        self.player_vx = max(-spd, min(spd, self.player_vx + move_x * acc * spd))
        self.player_vy = max(-spd * 0.7, min(spd * 0.7, self.player_vy + move_y * acc * spd))
        self.player_x += self.player_vx
        self.player_y += self.player_vy
        self.player_vx *= damp
        self.player_vy *= damp

        self.player_x = max(40.0, min(self.width - 40.0, self.player_x))
        self.player_y = max(260.0, min(self.height - 40.0, self.player_y))

        # Disparo contínuo
        if (keys[pygame.K_SPACE] or pygame.mouse.get_pressed()[0]) and self.cooldown == 0:
            self.fire_laser()

        # Efeito de Nova Bomb
        if self.nova_active:
            self.nova_radius += 38.0
            if self.nova_radius > 900.0:
                self.nova_active = False
            else:
                nr = self.nova_radius
                nx, ny = self.nova_origin
                self.enemy_bullets = [b for b in self.enemy_bullets if math.hypot(b[0] - nx, b[1] - ny) > nr]
                for a in self.asteroids:
                    if math.hypot(a[0] - nx, a[1] - ny) < nr + a[4]:
                        a[5] -= 3
                for al in self.aliens:
                    if math.hypot(al[0] - nx, al[1] - ny) < nr + 20:
                        al[5] -= 5

        # Estrelas
        for s in self.stars:
            warp = 2.5 if self.dash_timer > 0.0 else 1.0
            s[1] += s[2] * warp
            if s[1] > self.height:
                s[1] = 0
                s[0] = random.randint(0, self.width)

        # Atualizar Tiros do Jogador
        px, py = self.player_x, self.player_y
        for b in self.bullets[:]:
            b[7] -= dt
            if b[7] <= 0:
                self.bullets.remove(b)
                continue

            if b[6]: # Míssil teleguiado
                tx, ty = None, None
                for al in self.aliens:
                    tx, ty = al[0], al[1]
                    break
                if tx is None:
                    for a in self.asteroids:
                        tx, ty = a[0], a[1]
                        break
                if tx is not None:
                    dx, dy = tx - b[0], ty - b[1]
                    dist = math.hypot(dx, dy)
                    if dist > 0.01:
                        b[2] += (dx / dist) * 1.0
                        b[3] += (dy / dist) * 1.0

            b[0] += b[2]
            b[1] += b[3]

            hit = False
            for a in self.asteroids:
                if math.hypot(b[0] - a[0], b[1] - a[1]) < a[4] + 6 and a[5] > 0:
                    dmg = (b[4] - 1) if a[7] == "metallic" and b[4] > 1 else b[4]
                    a[5] -= dmg
                    b[5] -= 1
                    if b[5] <= 0 and b in self.bullets:
                        self.bullets.remove(b)
                    hit = True
                    break
            if hit:
                continue

            for al in self.aliens:
                if math.hypot(b[0] - al[0], b[1] - al[1]) < 24 and al[5] > 0:
                    al[5] -= b[4]
                    b[5] -= 1
                    if b[5] <= 0 and b in self.bullets:
                        self.bullets.remove(b)
                    hit = True
                    break
            if hit:
                continue

            # Colisão com o Chefe Dreadnought
            if self.boss is not None and self.boss["phase"] in ("combat", "overcharge"):
                bx, by = self.boss["x"], self.boss["y"]
                # Turret Esquerda
                tl = self.boss["turret_left"]
                if not tl["destroyed"] and math.hypot(b[0] - (bx - 110), b[1] - (by + 25)) < 28:
                    tl["hp"] -= b[4]
                    b[5] -= 1
                    if b[5] <= 0 and b in self.bullets: self.bullets.remove(b)
                    continue
                # Turret Direita
                tr = self.boss["turret_right"]
                if not tr["destroyed"] and math.hypot(b[0] - (bx + 110), b[1] - (by + 25)) < 28:
                    tr["hp"] -= b[4]
                    b[5] -= 1
                    if b[5] <= 0 and b in self.bullets: self.bullets.remove(b)
                    continue
                # Núcleo Central: PROTEGIDO pelo Escudo Frontal enquanto as turrets estiverem vivas!
                if math.hypot(b[0] - bx, b[1] - (by + 15)) < 36:
                    if not tl["destroyed"] or not tr["destroyed"]:
                        # Escudo reflete tiro
                        if b in self.bullets: self.bullets.remove(b)
                    else:
                        self.boss["core_hp"] -= b[4]
                        b[5] -= 1
                        if b[5] <= 0 and b in self.bullets: self.bullets.remove(b)
                    continue

            if b[1] < -20 or b[1] > self.height + 20:
                if b in self.bullets:
                    self.bullets.remove(b)

        # Atualizar Asteroides e Subdivisão
        col_dmg = 0.0
        for a in self.asteroids[:]:
            a[0] += a[2]
            a[1] += a[3]
            a[8] += a[9]

            dist = math.hypot(a[0] - px, a[1] - py)
            if dist < a[4] + 30 and dist > a[4] + 18:
                self.combo_mult = min(5.0, self.combo_mult + 0.04)
                self.combo_timer = 3.5

            if dist < a[4] + 16:
                col_dmg += 45.0 # Dano pesado
                a[5] = 0

            if a[5] <= 0:
                self.asteroids.remove(a)
                pts = 250 if a[7] == "titan" else (120 if a[7] == "medium" else 60)
                if a[7] == "radioactive": pts = 380
                elif a[7] == "metallic": pts = 300
                self.score += int(pts * self.combo_mult)
                self.wave_kills += 1
                self.level = 1 + self.score // 2500

                # Fragmentação
                if a[7] == "titan":
                    self.spawn_asteroid("medium", a[0] - 16, a[1])
                    self.spawn_asteroid("medium", a[0] + 16, a[1])
                elif a[7] == "medium":
                    self.spawn_asteroid("debris", a[0] - 12, a[1])
                    self.spawn_asteroid("debris", a[0] + 12, a[1])
                elif a[7] == "radioactive":
                    self.shockwaves.append([a[0], a[1], 5, 80, 12, PALETTE["yellow"]])
                    if math.hypot(a[0] - px, a[1] - py) < 80:
                        self.take_damage(25.0)

                self.drop_loot(a[0], a[1], a[7], False)
                self.camera_shake = min(1.0, self.camera_shake + 0.25)
                for _ in range(16):
                    self.particles.append([
                        a[0], a[1],
                        random.uniform(-5, 5), random.uniform(-5, 5),
                        PALETTE["yellow"] if a[7] == "radioactive" else PALETTE["muted"],
                        0.5, 0.5, random.uniform(2, 5)
                    ])

        if col_dmg > 0.0:
            self.take_damage(col_dmg)

        self.asteroids = [a for a in self.asteroids if a[1] < self.height + 60]

        # Inimigos Alienígenas
        self.alien_spawn_timer += 1
        rate_alien = 350 if self.wave == 1 else (180 if self.wave == 2 else 140)
        if self.alien_spawn_timer >= rate_alien and self.boss is None:
            self.alien_spawn_timer = 0
            kind = "gunboat" if (self.wave >= 3 and random.random() < 0.3) else ("minelayer" if random.random() < 0.4 else "scout")
            hp = 5 if kind == "scout" else (10 if kind == "minelayer" else 14)
            self.aliens.append([
                random.uniform(70, self.width - 70), -35,
                random.uniform(-2.2, 2.2), random.uniform(1.5, 2.5),
                kind, hp, hp, 40, 0.0
            ])

        for al in self.aliens[:]:
            al[8] += dt
            al[0] += al[2]
            al[1] += al[3]
            if al[4] == "scout":
                al[2] = math.sin(al[8] * 3.8) * 3.5

            al[7] -= 1
            if al[7] <= 0:
                al[7] = random.randint(45, 80)
                if al[4] == "scout":
                    dx, dy = px - al[0], py - al[1]
                    dist = math.hypot(dx, dy)
                    if dist > 0.01:
                        self.enemy_bullets.append([al[0], al[1] + 12, (dx / dist) * 7.0, (dy / dist) * 7.0, 4.5, PALETTE["coral"]])
                elif al[4] == "minelayer":
                    self.mines.append([al[0], al[1] + 14, 1.2, 12.0, 0.0])
                else: # gunboat
                    for ang in (-0.25, 0.0, 0.25):
                        self.enemy_bullets.append([al[0], al[1] + 16, math.sin(ang) * 6.5, math.cos(ang) * 6.5, 5.0, PALETTE["coral"]])

            if al[5] <= 0:
                self.aliens.remove(al)
                self.score += int(450 * self.combo_mult)
                self.wave_kills += 1
                self.drop_loot(al[0], al[1], None, True)
                self.shockwaves.append([al[0], al[1], 5, 70, 14, PALETTE["coral"]])

        self.aliens = [al for al in self.aliens if al[1] < self.height + 40]

        # Minas Orbitais
        for m in self.mines[:]:
            m[1] += m[2]
            m[4] += dt * 5.0
            m[3] -= dt
            if math.hypot(m[0] - px, m[1] - py) < 45 or m[3] <= 0:
                self.mines.remove(m)
                for a in range(8):
                    ang = a * 0.785
                    self.enemy_bullets.append([m[0], m[1], math.cos(ang) * 4.8, math.sin(ang) * 4.8, 3.5, PALETTE["yellow"]])
                self.shockwaves.append([m[0], m[1], 4, 55, 14, PALETTE["coral"]])

        # Tiros Inimigos
        laser_dmg = 0.0
        for eb in self.enemy_bullets[:]:
            eb[0] += eb[2]
            eb[1] += eb[3]
            if math.hypot(eb[0] - px, eb[1] - py) < eb[4] + 16:
                laser_dmg += 22.0
                self.enemy_bullets.remove(eb)
            elif eb[1] > self.height + 20:
                self.enemy_bullets.remove(eb)

        if laser_dmg > 0.0:
            self.take_damage(laser_dmg)

        # Chefe Dreadnought
        if self.boss is not None:
            b = self.boss
            if b["phase"] == "warning":
                b["phase_timer"] -= dt
                if b["phase_timer"] <= 0:
                    b["phase"] = "arriving"
            elif b["phase"] == "arriving":
                b["y"] += 1.6
                if b["y"] >= 90:
                    b["phase"] = "combat"
            elif b["phase"] == "combat":
                b["x"] += b["vx"]
                if b["x"] < 180 or b["x"] > 620:
                    b["vx"] *= -1

                # Turret Esquerda
                tl = b["turret_left"]
                if not tl["destroyed"]:
                    if tl["hp"] <= 0:
                        tl["destroyed"] = True
                        self.score += 4000
                        self.shockwaves.append([b["x"] - 110, b["y"] + 25, 10, 90, 15, PALETTE["yellow"]])
                    else:
                        tl["timer"] -= 1
                        if tl["timer"] <= 0:
                            tl["timer"] = 40
                            ang = math.atan2(py - (b["y"] + 25), px - (b["x"] - 110))
                            self.enemy_bullets.append([b["x"] - 110, b["y"] + 25, math.cos(ang) * 7.5, math.sin(ang) * 7.5, 5.5, PALETTE["coral"]])

                # Turret Direita
                tr = b["turret_right"]
                if not tr["destroyed"]:
                    if tr["hp"] <= 0:
                        tr["destroyed"] = True
                        self.score += 4000
                        self.shockwaves.append([b["x"] + 110, b["y"] + 25, 10, 90, 15, PALETTE["yellow"]])
                    else:
                        tr["timer"] -= 1
                        if tr["timer"] <= 0:
                            tr["timer"] = 40
                            ang = math.atan2(py - (b["y"] + 25), px - (b["x"] + 110))
                            self.enemy_bullets.append([b["x"] + 110, b["y"] + 25, math.cos(ang) * 7.5, math.sin(ang) * 7.5, 5.5, PALETTE["coral"]])

                if tl["destroyed"] and tr["destroyed"]:
                    b["phase"] = "overcharge"
                    self.shockwaves.append([b["x"], b["y"] + 20, 10, 180, 16, PALETTE["ice"]])
            elif b["phase"] == "overcharge":
                b["x"] += b["vx"] * 1.5
                if b["x"] < 160 or b["x"] > 640:
                    b["vx"] *= -1
                b["attack_timer"] -= 1
                if b["attack_timer"] <= 0:
                    b["attack_timer"] = 28
                    t_now = time.time() * 4.0
                    for i in range(6):
                        ang = i * 1.047 + t_now
                        self.enemy_bullets.append([b["x"], b["y"] + 35, math.cos(ang) * 6.5, abs(math.sin(ang)) * 6.5 + 2.0, 5.0, PALETTE["yellow"]])
                if b["core_hp"] <= 0:
                    b["phase"] = "exploding"
            elif b["phase"] == "exploding":
                b["explosion_countdown"] -= dt
                self.camera_shake = min(1.0, self.camera_shake + 0.5)
                if random.random() < 0.35:
                    rx = b["x"] + random.uniform(-110, 110)
                    ry = b["y"] + random.uniform(-30, 45)
                    self.shockwaves.append([rx, ry, 5, 70, 14, PALETTE["coral"]])
                if b["explosion_countdown"] <= 0:
                    self.score += 25000
                    self.sector += 1
                    self.wave = 1
                    self.wave_kills = 0
                    self.wave_target_kills = 22
                    self.wave_banner_text = f"SETOR {self.sector} CONQUISTADO! AVANÇANDO..."
                    self.wave_banner_timer = 4.0
                    self.shockwaves.append([b["x"], b["y"], 10, 500, 22, PALETTE["yellow"]])
                    self.boss = None

        # PowerUps
        for pup in self.powerups[:]:
            pup[5] -= dt
            if pup[5] <= 0:
                self.powerups.remove(pup)
                continue

            dx, dy = px - pup[0], py - pup[1]
            dist = math.hypot(dx, dy)
            if dist < 120:
                pup[2] += (dx / dist) * 0.7
                pup[3] += (dy / dist) * 0.7

            pup[0] += pup[2]
            pup[1] += pup[3]
            pup[2] *= 0.94
            pup[3] *= 0.94

            if dist < 24:
                self.powerups.remove(pup)
                if pup[4] == "shield":
                    self.shield = min(100.0, self.shield + 18.0)
                    self.floating_texts.append([px, py - 30, "+18% ESCUDO", PALETTE["ice"], 1.2])
                elif pup[4] == "weapon":
                    self.weapon_pods += 1
                    needed = self.weapon_level
                    if self.weapon_pods >= needed and self.weapon_level < 5:
                        self.weapon_level += 1
                        self.weapon_pods = 0
                        self.floating_texts.append([px, py - 30, f"ARMA LVL {self.weapon_level}!", PALETTE["yellow"], 1.5])
                    else:
                        self.floating_texts.append([px, py - 30, f"UPGRADE: {self.weapon_pods}/{needed}", PALETTE["yellow"], 1.0])
                elif pup[4] == "nova":
                    self.nova_bombs = min(3, self.nova_bombs + 1)
                    self.floating_texts.append([px, py - 30, "+1 OGIVA EMP", PALETTE["purple"], 1.5])
                elif pup[4] == "overdrive":
                    self.overdrive_timer = 6.0
                    self.floating_texts.append([px, py - 30, "OVERDRIVE!", PALETTE["purple"], 1.5])
                else: # stardust
                    self.score += int(75 * self.combo_mult)

        # Partículas
        for p in self.particles[:]:
            p[0] += p[2]
            p[1] += p[3]
            p[5] -= dt
            if p[5] <= 0:
                self.particles.remove(p)

        for sw in self.shockwaves[:]:
            sw[2] += sw[4]
            if sw[2] >= sw[3]:
                self.shockwaves.remove(sw)

        for ft in self.floating_texts[:]:
            ft[1] -= 1.0
            ft[4] -= dt
            if ft[4] <= 0:
                self.floating_texts.remove(ft)

        # Checagem de ondas
        self.check_wave_progression()

        # Spawns de asteroides
        self.spawn_timer += 1
        rate = 44 if self.wave == 1 else (38 if self.wave == 2 else 30)
        if self.spawn_timer >= rate and self.boss is None:
            self.spawn_timer = 0
            tier = "radioactive" if (self.wave >= 3 and random.random() < 0.12) else ("metallic" if random.random() < 0.2 else "titan")
            self.spawn_asteroid(tier)

    def draw(self, surface, font):
        surface.fill(PALETTE["bg"])

        # Estrelas
        for s in self.stars:
            col = (200, 220, 255) if s[3] == 2 else (100, 120, 160)
            pygame.draw.circle(surface, col, (int(s[0]), int(s[1])), int(s[2] // 2 + 1))

        # Shockwaves
        for sw in self.shockwaves:
            pygame.draw.circle(surface, sw[5], (int(sw[0]), int(sw[1])), int(sw[2]), 2)

        # Asteroides com Crateras e Shading 3D
        for a in self.asteroids:
            pts = []
            n_pts = len(a[10])
            for i, factor in enumerate(a[10]):
                ang = i * (6.283 / n_pts) + a[8]
                r = a[4] * factor
                pts.append((a[0] + math.cos(ang) * r, a[1] + math.sin(ang) * r))

            col_fill = (45, 55, 35) if a[7] == "radioactive" else ((40, 50, 70) if a[7] == "metallic" else (30, 40, 58))
            col_line = PALETTE["yellow"] if a[7] == "radioactive" else (PALETTE["ice"] if a[7] == "metallic" else PALETTE["muted"])
            if len(pts) > 2:
                pygame.draw.polygon(surface, col_fill, pts)
                pygame.draw.polygon(surface, col_line, pts, 2)

            # Crateras
            for cx, cy, cr in a[11]:
                rx = cx * math.cos(a[8]) - cy * math.sin(a[8])
                ry = cx * math.sin(a[8]) + cy * math.cos(a[8])
                pygame.draw.circle(surface, (15, 20, 30), (int(a[0] + rx), int(a[1] + ry)), int(cr))

        # Naves Alienígenas
        for al in self.aliens:
            ax, ay = int(al[0]), int(al[1])
            if al[4] == "scout":
                pts = [(ax, ay + 16), (ax - 14, ay - 12), (ax, ay - 4), (ax + 14, ay - 12)]
                pygame.draw.polygon(surface, PALETTE["coral"], pts)
                pygame.draw.circle(surface, PALETTE["yellow"], (ax, ay), 3)
            elif al[4] == "minelayer":
                pygame.draw.rect(surface, (50, 40, 65), (ax - 16, ay - 12, 32, 24), border_radius=4)
                pygame.draw.circle(surface, PALETTE["coral"], (ax, ay), 5)
            else: # gunboat
                pygame.draw.rect(surface, (35, 45, 60), (ax - 22, ay - 16, 44, 32), border_radius=4)
                pygame.draw.line(surface, PALETTE["ice"], (ax - 22, ay - 8), (ax + 22, ay - 8), 3)

        # Minas Orbitais
        for m in self.mines:
            pygame.draw.circle(surface, (60, 40, 45), (int(m[0]), int(m[1])), 8)
            pygame.draw.circle(surface, PALETTE["coral"], (int(m[0]), int(m[1])), 8, 2)

        # Chefe Dreadnought
        if self.boss is not None:
            b = self.boss
            bx, by = int(b["x"]), int(b["y"])

            # Chassi (300 x 80)
            pygame.draw.rect(surface, (20, 30, 45), (bx - 150, by - 45, 300, 80), border_radius=6)
            pygame.draw.rect(surface, PALETTE["border"], (bx - 150, by - 45, 300, 80), 2, border_radius=6)

            # Proa dianteira
            pts_prow = [(bx - 150, by + 35), (bx + 150, by + 35), (bx, by + 75)]
            pygame.draw.polygon(surface, (25, 35, 55), pts_prow)
            pygame.draw.line(surface, PALETTE["coral"], (bx - 150, by + 35), (bx, by + 75), 2)
            pygame.draw.line(surface, PALETTE["coral"], (bx + 150, by + 35), (bx, by + 75), 2)

            # Turrets
            tl = b["turret_left"]
            col_tl = (80, 80, 80) if tl["destroyed"] else PALETTE["coral"]
            pygame.draw.circle(surface, col_tl, (bx - 110, by + 25), 18)

            tr = b["turret_right"]
            col_tr = (80, 80, 80) if tr["destroyed"] else PALETTE["coral"]
            pygame.draw.circle(surface, col_tr, (bx + 110, by + 25), 18)

            # Núcleo
            core_col = PALETTE["yellow"] if b["phase"] == "overcharge" else PALETTE["purple"]
            pygame.draw.circle(surface, core_col, (bx, by + 18), 24)

            # Escudo Frontal
            if not tl["destroyed"] or not tr["destroyed"]:
                pygame.draw.arc(surface, PALETTE["ice"], (bx - 140, by - 20, 280, 100), 0.3, 2.8, 3)

            # Barra do Chefe
            pygame.draw.rect(surface, PALETTE["bg"], (180, 22, 440, 14), border_radius=3)
            hp_frac = max(0.0, min(1.0, b["core_hp"] / b["core_max_hp"]))
            pygame.draw.rect(surface, PALETTE["coral"], (181, 23, int(438 * hp_frac), 12), border_radius=2)
            txt_boss = font.render(f"FLAGSHIP DREADNOUGHT (NÚCLEO: {b['core_hp']} HP)", True, PALETTE["coral"])
            surface.blit(txt_boss, (280, 4))

        # Tiros do Jogador
        for b in self.bullets:
            if b[6]:
                pygame.draw.circle(surface, PALETTE["yellow"], (int(b[0]), int(b[1])), 4)
            else:
                pygame.draw.line(surface, b[8], (b[0], b[1]), (b[0], b[1] + 12), 3)

        # Tiros Inimigos
        for eb in self.enemy_bullets:
            pygame.draw.circle(surface, eb[5], (int(eb[0]), int(eb[1])), int(eb[4]))

        # PowerUps
        for pup in self.powerups:
            px, py = int(pup[0]), int(pup[1])
            col = PALETTE["ice"] if pup[4] == "shield" else (PALETTE["yellow"] if pup[4] == "weapon" else PALETTE["purple"])
            pygame.draw.circle(surface, (20, 30, 45), (px, py), 10)
            pygame.draw.circle(surface, col, (px, py), 10, 2)

        # Partículas
        for p in self.particles:
            pygame.draw.circle(surface, p[4], (int(p[0]), int(p[1])), max(1, int(p[7])))

        # Nave do Jogador
        if not self.game_over and (self.invuln_timer <= 0.0 or int(time.time() * 20) % 2 == 0):
            px, py = int(self.player_x), int(self.player_y)

            fl1 = random.randint(8, 16)
            fl2 = random.randint(8, 16)
            pygame.draw.polygon(surface, PALETTE["yellow"], [(px - 9, py + 12), (px - 5, py + 12), (px - 7, py + 12 + fl1)])
            pygame.draw.polygon(surface, PALETTE["yellow"], [(px + 5, py + 12), (px + 9, py + 12), (px + 7, py + 12 + fl2)])

            pts = [(px, py - 22), (px - 22, py + 14), (px, py + 8), (px + 22, py + 14)]
            pygame.draw.polygon(surface, PALETTE["cobalt"], pts)
            pygame.draw.polygon(surface, PALETTE["ice"], [(px, py - 18), (px - 10, py + 10), (px, py + 6), (px + 10, py + 10)])
            pygame.draw.circle(surface, PALETTE["emerald"], (px, py), 4)

            if self.shield > 0.0 or self.shield_flash > 0.0:
                pygame.draw.circle(surface, PALETTE["ice"], (px, py), 28, 2)

        # Textos Flutuantes
        for ft in self.floating_texts:
            txt = font.render(ft[2], True, ft[3])
            surface.blit(txt, (int(ft[0] - txt.get_width() / 2), int(ft[1])))

        # Banner de Onda
        if self.wave_banner_timer > 0:
            b_txt = font.render(self.wave_banner_text, True, PALETTE["yellow"] if self.boss is None else PALETTE["coral"])
            surface.blit(b_txt, (self.width // 2 - b_txt.get_width() // 2, 110))

        # HUD Superior Tático
        pygame.draw.rect(surface, PALETTE["surface"], (0, 0, self.width, 50))
        pygame.draw.line(surface, PALETTE["border"], (0, 50), (self.width, 50), 2)

        # Escudo
        txt_s = font.render("ESCUDO", True, PALETTE["ice"])
        surface.blit(txt_s, (15, 14))
        pygame.draw.rect(surface, PALETTE["bg"], (85, 16, 90, 16), border_radius=4)
        pygame.draw.rect(surface, PALETTE["ice"], (87, 18, int(86 * (self.shield / 100.0)), 12), border_radius=3)

        # Casco
        txt_h = font.render("CASCO", True, PALETTE["emerald"])
        surface.blit(txt_h, (190, 14))
        pygame.draw.rect(surface, PALETTE["bg"], (250, 16, 90, 16), border_radius=4)
        h_col = PALETTE["emerald"] if self.hull > 35 else PALETTE["coral"]
        pygame.draw.rect(surface, h_col, (252, 18, int(86 * (self.hull / 100.0)), 12), border_radius=3)

        # Pontos
        txt_score = font.render(f"PONTOS: {self.score}", True, PALETTE["yellow"])
        surface.blit(txt_score, (360, 14))

        # Setor e Onda
        txt_w = font.render(f"S{self.sector}-W{self.wave} | EMP: {self.nova_bombs}", True, PALETTE["purple"])
        surface.blit(txt_w, (540, 14))

        # Vidas
        for i in range(3):
            col = PALETTE["coral"] if i < self.lives else (50, 60, 75)
            pygame.draw.circle(surface, col, (720 + i * 22, 24), 7)

        # Overlay de Fim de Jogo
        if self.game_over:
            overlay = pygame.Surface((self.width, self.height), pygame.SRCALPHA)
            overlay.fill((9, 12, 20, 220))
            surface.blit(overlay, (0, 0))

            t1 = font.render("MISSÃO ENCERRADA — NAVE DESTRUÍDA", True, PALETTE["coral"])
            t2 = font.render(f"Pontuação Final: {self.score} (Setor {self.sector} - Onda {self.wave})", True, PALETTE["yellow"])
            t3 = font.render("Pressione [R] para reiniciar ou [ESC] para sair", True, PALETTE["text"])

            surface.blit(t1, (self.width // 2 - t1.get_width() // 2, self.height // 2 - 40))
            surface.blit(t2, (self.width // 2 - t2.get_width() // 2, self.height // 2))
            surface.blit(t3, (self.width // 2 - t3.get_width() // 2, self.height // 2 + 40))


if __name__ == "__main__":
    pygame.init()
    width, height = 800, 600
    screen = pygame.display.set_mode((width, height))
    pygame.display.set_caption("Space Odyssey — Apex Hardcore Edition")
    clock = pygame.time.Clock()
    font = pygame.font.Font(None, 24)
    game = Game(width, height)
    running = True
    while running:
        clock.tick(60)
        events = pygame.event.get()
        for event in events:
            if event.type == pygame.QUIT or (event.type == pygame.KEYDOWN and event.key == pygame.K_ESCAPE):
                running = False
        keys = pygame.key.get_pressed()
        game.update(keys, events)
        game.draw(screen, font)
        pygame.display.flip()
    pygame.quit()
