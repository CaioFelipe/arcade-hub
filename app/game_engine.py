"""
game_engine.py - Motor de Jogos In-Window em Memória para o Arcade Hub
Executa todos os 11 jogos do catálogo e jogos da comunidade 100% na mesma janela,
sem abrir subprocessos nem janelas externas do Pygame ou Tkinter.
Renderiza quadros em 800x600 diretamente em JPEG Base64 com alto desempenho.
"""
import os
os.environ["SDL_VIDEODRIVER"] = "dummy"
os.environ["SDL_AUDIODRIVER"] = "dummy"
os.environ["PYGAME_HIDE_SUPPORT_PROMPT"] = "1"

import io
import base64
import math
import random
import pygame

# Inicialização do Pygame em modo headless
pygame.init()

# Paleta Oficial Safira & Neon
PALETTE = {
    "bg": (9, 12, 20),           # Safira Escuro #090C14
    "surface": (17, 24, 39),      # Slate Escuro #111827
    "cobalt": (37, 99, 235),      # Azul Cobalto #2563EB
    "ice": (96, 165, 250),        # Azul Gelo #60A5FA
    "emerald": (16, 185, 129),    # Esmeralda #10B981
    "yellow": (250, 204, 21),     # Amarelo #FACC15
    "coral": (244, 63, 94),       # Coral #F43F5E
    "purple": (168, 85, 247),     # Roxo #A855F7
    "text": (248, 250, 252),      # Branco Pérola #F8FAFC
    "muted": (148, 163, 184),     # Cinza Neutro #94A3B8
    "border": (30, 41, 59),       # Borda Sutil #1E293B
}

def get_keys_set(keys):
    """Normaliza lista de teclas para conjunto em minúsculas."""
    res = set()
    for k in keys:
        if isinstance(k, str):
            res.add(k.lower())
            res.add(k)
    return res

# ==========================================
# 1. SPACE ODYSSEY (APEX OVERHAUL)
# ==========================================
class SpaceOdysseyGame:
    def __init__(self):
        self.player_x = 400.0
        self.player_y = 510.0
        self.player_vx = 0.0
        self.player_vy = 0.0
        self.lasers = []       # [x, y, vx, vy, dmg, pierce, is_missile, life, color]
        self.enemy_lasers = [] # [x, y, vx, vy, radius, color]
        self.asteroids = []    # [x, y, vx, vy, r, hp, max_hp, tier, shape]
        self.aliens = []       # [x, y, vx, vy, kind, hp, timer, age]
        self.mines = []        # [x, y, vy, timer]
        self.powerups = []     # [x, y, vx, vy, kind, life]
        self.particles = []    # [x, y, vx, vy, color, life, max_life, size]
        self.shockwaves = []   # [x, y, r, max_r, spd, color]
        self.stars = [
            [random.randint(0, 800), random.randint(0, 600), random.uniform(0.5, 4.0), random.choice([1, 2])]
            for _ in range(80)
        ]

        self.shield = 100.0
        self.hull = 100.0
        self.shield_regen_delay = 0.0 # 6s de espera
        self.shield_flash = 0.0
        self.dash_cooldown = 0.0
        self.dash_timer = 0.0
        self.invuln_timer = 1.5

        self.weapon_level = 1
        self.weapon_pods = 0
        self.nova_bombs = 1 # Começa com apenas 1 ogiva EMP
        self.overdrive_timer = 0.0
        self.nova_active = False
        self.nova_radius = 0.0

        self.cooldown = 0
        self.score = 0
        self.lives = 3
        self.level = 1
        self.sector = 1
        self.wave = 1
        self.wave_kills = 0
        self.wave_target_kills = 18
        self.combo_mult = 1.0
        self.combo_timer = 0.0
        self.game_over = False
        self.tick_count = 0
        self.boss = None

        self.spawn_asteroid("titan", 220, -30)
        self.spawn_asteroid("medium", 580, -40)

    def spawn_asteroid(self, tier, x=None, y=None):
        if x is None:
            x = random.uniform(50, 750)
        if y is None:
            y = -40
        if tier == "titan":
            r, hp = random.uniform(38, 46), 5
        elif tier == "medium":
            r, hp = random.uniform(22, 28), 2
        elif tier == "debris":
            r, hp = random.uniform(10, 14), 1
        else: # radioactive
            r, hp = random.uniform(28, 34), 4
        vx = random.uniform(-1.0, 1.0)
        vy = random.uniform(1.8, 3.2)
        shape = [random.uniform(0.8, 1.2) for _ in range(8)]
        self.asteroids.append({"x": x, "y": y, "vx": vx, "vy": vy, "r": r, "hp": hp, "max_hp": hp, "tier": tier, "shape": shape})

    def trigger_nova(self):
        if self.nova_bombs <= 0 or self.nova_active:
            return
        self.nova_bombs -= 1
        self.nova_active = True
        self.nova_radius = 10.0
        self.enemy_lasers.clear()
        self.shockwaves.append([self.player_x, self.player_y, 10.0, 900.0, 36.0, PALETTE["purple"]])

    def fire_laser(self):
        is_od = self.overdrive_timer > 0.0
        self.cooldown = 4 if is_od else 8
        px, py = self.player_x, self.player_y
        lvl = self.weapon_level

        if lvl == 1:
            self.lasers.append([px - 11, py - 14, 0, -15, 1, 1, False, 1.2, PALETTE["ice"]])
            self.lasers.append([px + 11, py - 14, 0, -15, 1, 1, False, 1.2, PALETTE["ice"]])
        elif lvl == 2:
            self.lasers.append([px, py - 18, 0, -16, 1, 1, False, 1.2, PALETTE["yellow"]])
            self.lasers.append([px - 14, py - 10, -2.2, -14.5, 1, 1, False, 1.2, PALETTE["ice"]])
            self.lasers.append([px + 14, py - 10, 2.2, -14.5, 1, 1, False, 1.2, PALETTE["ice"]])
        elif lvl == 3:
            for i in (-2, -1, 0, 1, 2):
                ang = i * 0.16
                self.lasers.append([px + i * 6, py - 14, math.sin(ang) * 15, -math.cos(ang) * 15, 1, 2, False, 1.2, PALETTE["ice"]])
        elif lvl == 4:
            self.lasers.append([px - 8, py - 16, 0, -18, 2, 4, False, 1.4, PALETTE["emerald"]])
            self.lasers.append([px + 8, py - 16, 0, -18, 2, 4, False, 1.4, PALETTE["emerald"]])
            self.lasers.append([px - 20, py - 8, -3.5, -14, 1, 2, False, 1.2, PALETTE["ice"]])
            self.lasers.append([px + 20, py - 8, 3.5, -14, 1, 2, False, 1.2, PALETTE["ice"]])
        else:
            self.lasers.append([px - 8, py - 16, 0, -18, 2, 5, False, 1.4, PALETTE["purple"]])
            self.lasers.append([px + 8, py - 16, 0, -18, 2, 5, False, 1.4, PALETTE["purple"]])
            self.lasers.append([px - 22, py - 6, -4, -8, 3, 1, True, 2.0, PALETTE["yellow"]])
            self.lasers.append([px + 22, py - 6, 4, -8, 3, 1, True, 2.0, PALETTE["yellow"]])

    def take_damage(self, amount):
        if self.invuln_timer > 0.0 or self.dash_timer > 0.0:
            return
        self.shield_regen_delay = 6.0 # 6s de espera para recarga
        self.shield_flash = 0.35
        if self.shield > 0.0:
            self.shield -= amount
            if self.shield < 0.0:
                self.hull += self.shield
                self.shield = 0.0
        else:
            self.hull -= amount

        for _ in range(16):
            self.particles.append([
                self.player_x, self.player_y,
                random.uniform(-5, 5), random.uniform(-5, 5),
                PALETTE["ice"] if self.shield > 0.0 else PALETTE["coral"],
                20, 20, random.uniform(2, 5)
            ])

        if self.hull <= 0.0:
            self.lives -= 1
            if self.weapon_level > 1:
                self.weapon_level -= 1
                self.weapon_pods = 0
            if self.lives <= 0:
                self.game_over = True
            else:
                self.hull = 100.0
                self.shield = 100.0
                self.invuln_timer = 3.0
                self.player_x = 400.0
                self.player_y = 510.0
                self.player_vx = 0.0
                self.player_vy = 0.0

    def update(self, keys, events):
        if self.game_over:
            return
        self.tick_count += 1
        kset = get_keys_set(keys)
        dt = 1.0 / 60.0

        # Timers
        if self.cooldown > 0: self.cooldown -= 1
        if self.invuln_timer > 0.0: self.invuln_timer -= dt
        if self.dash_cooldown > 0.0: self.dash_cooldown -= dt
        if self.dash_timer > 0.0: self.dash_timer -= dt
        if self.overdrive_timer > 0.0: self.overdrive_timer -= dt
        if self.shield_flash > 0.0: self.shield_flash -= dt

        # Recarga de Escudo
        if self.shield_regen_delay > 0.0:
            self.shield_regen_delay -= dt
        elif self.shield < 100.0:
            self.shield = min(100.0, self.shield + 10.0 * dt)

        # Combo
        if self.combo_timer > 0.0:
            self.combo_timer -= dt
            if self.combo_timer <= 0.0:
                self.combo_mult = 1.0

        # Controles
        move_x, move_y = 0.0, 0.0
        if "arrowleft" in kset or "a" in kset: move_x -= 1.0
        if "arrowright" in kset or "d" in kset: move_x += 1.0
        if "arrowup" in kset or "w" in kset: move_y -= 1.0
        if "arrowdown" in kset or "s" in kset: move_y += 1.0

        # Dash (Shift)
        if ("shift" in kset or "leftshift" in kset or "rightshift" in kset) and self.dash_cooldown <= 0.0:
            self.dash_timer = 0.22
            self.dash_cooldown = 1.4
            self.invuln_timer = 0.35

        # Nova Bomb (X)
        if "x" in kset:
            self.trigger_nova()

        # Mouse / Clique
        for ev in events:
            if ev.get("type") == "mousemove":
                self.player_x = max(30, min(770, ev.get("x", self.player_x)))
            elif ev.get("type") == "click":
                self.fire_laser()

        spd = 16.0 if self.dash_timer > 0.0 else 7.5
        acc = 1.6
        damp = 0.86
        self.player_vx = max(-spd, min(spd, self.player_vx + move_x * acc * spd))
        self.player_vy = max(-spd * 0.7, min(spd * 0.7, self.player_vy + move_y * acc * spd))
        self.player_x += self.player_vx
        self.player_y += self.player_vy
        self.player_vx *= damp
        self.player_vy *= damp
        self.player_x = max(40.0, min(760.0, self.player_x))
        self.player_y = max(280.0, min(560.0, self.player_y))

        # Disparo
        if (" " in kset or "space" in kset or "enter" in kset) and self.cooldown == 0:
            self.fire_laser()

        # Efeito Nova Bomb
        if self.nova_active:
            self.nova_radius += 38.0
            if self.nova_radius > 900.0:
                self.nova_active = False
            else:
                for a in self.asteroids:
                    if math.hypot(a["x"] - self.player_x, a["y"] - self.player_y) < self.nova_radius + a["r"]:
                        a["hp"] -= 4
                for al in self.aliens:
                    if math.hypot(al["x"] - self.player_x, al["y"] - self.player_y) < self.nova_radius + 20:
                        al["hp"] -= 6

        # Estrelas
        for s in self.stars:
            warp = 2.5 if self.dash_timer > 0.0 else 1.0
            s[1] += s[2] * warp
            if s[1] > 600:
                s[1] = 0
                s[0] = random.randint(0, 800)

        # Atualizar Lasers do Jogador
        new_lasers = []
        for l in self.lasers:
            l[7] -= dt
            if l[7] <= 0:
                continue
            if l[6]: # Míssil
                for a in self.asteroids:
                    dx, dy = a["x"] - l[0], a["y"] - l[1]
                    dist = math.hypot(dx, dy)
                    if dist > 0.01:
                        l[2] += (dx / dist) * 0.9
                        l[3] += (dy / dist) * 0.9
                    break
            l[0] += l[2]
            l[1] += l[3]

            hit = False
            for a in self.asteroids:
                if math.hypot(l[0] - a["x"], l[1] - a["y"]) < a["r"] + 6 and a["hp"] > 0:
                    a["hp"] -= l[4]
                    l[5] -= 1
                    if l[5] <= 0:
                        hit = True
                    break
            if not hit and l[1] > -20:
                new_lasers.append(l)
        self.lasers = new_lasers

        # Spawn Asteroides
        spawn_rate = max(18, 48 - self.level * 3)
        if self.tick_count % spawn_rate == 0:
            tier = "radioactive" if random.random() < 0.1 else ("titan" if random.random() < 0.35 else "medium")
            self.spawn_asteroid(tier)

        # Atualizar Asteroides e Fragmentação
        new_asteroids = []
        col_dmg = 0.0
        for ast in self.asteroids:
            ast["x"] += ast["vx"]
            ast["y"] += ast["vy"]

            dist = math.hypot(ast["x"] - self.player_x, ast["y"] - self.player_y)
            if dist < ast["r"] + 16:
                col_dmg += 45.0
                ast["hp"] = 0

            if ast["hp"] <= 0:
                pts = 250 if ast["tier"] == "titan" else (120 if ast["tier"] == "medium" else 60)
                if ast["tier"] == "radioactive": pts = 400
                self.score += int(pts * self.combo_mult)
                self.level = 1 + self.score // 2000

                # Fragmentação
                if ast["tier"] == "titan":
                    self.spawn_asteroid("medium", ast["x"] - 15, ast["y"])
                    self.spawn_asteroid("medium", ast["x"] + 15, ast["y"])
                elif ast["tier"] == "medium":
                    self.spawn_asteroid("debris", ast["x"] - 10, ast["y"])
                    self.spawn_asteroid("debris", ast["x"] + 10, ast["y"])

                # Partículas
                for _ in range(14):
                    self.particles.append([
                        ast["x"], ast["y"],
                        random.uniform(-5, 5), random.uniform(-5, 5),
                        PALETTE["yellow"] if ast["tier"] == "radioactive" else PALETTE["ice"],
                        25, 25, random.uniform(2, 5)
                    ])
                continue

            if ast["y"] < 630:
                new_asteroids.append(ast)
        self.asteroids = new_asteroids

        if col_dmg > 0.0:
            self.take_damage(col_dmg)

        # Partículas
        for p in self.particles[:]:
            p[0] += p[2]
            p[1] += p[3]
            p[5] -= 1
            if p[5] <= 0:
                self.particles.remove(p)

        for sw in self.shockwaves[:]:
            sw[2] += sw[4]
            if sw[2] >= sw[3]:
                self.shockwaves.remove(sw)

    def draw(self, surf, fonts):
        surf.fill(PALETTE["bg"])

        # Estrelas
        for sx, sy, spd, lyr in self.stars:
            b = min(255, int(spd * 60 + 80))
            surf.fill((b, b, b), (int(sx), int(sy), 2, 2))

        # Shockwaves
        for sw in self.shockwaves:
            pygame.draw.circle(surf, sw[5], (int(sw[0]), int(sw[1])), int(sw[2]), 2)

        # Partículas
        for p in self.particles:
            pygame.draw.circle(surf, p[4], (int(p[0]), int(p[1])), max(1, int(p[7])))

        # Asteroides
        for ast in self.asteroids:
            col_edge = PALETTE["yellow"] if ast["tier"] == "radioactive" else PALETTE["border"]
            pygame.draw.circle(surf, col_edge, (int(ast["x"]), int(ast["y"])), int(ast["r"]))
            pygame.draw.circle(surf, (71, 85, 105), (int(ast["x"]), int(ast["y"])), max(2, int(ast["r"] - 2)))

        # Lasers
        for l in self.lasers:
            if l[6]: # Míssil
                pygame.draw.circle(surf, PALETTE["yellow"], (int(l[0]), int(l[1])), 4)
            else:
                pygame.draw.line(surf, l[8], (l[0], l[1]), (l[0], l[1] + 12), 3)

        # Nave do Jogador
        if not self.game_over or (self.tick_count // 5) % 2 == 0:
            px, py = int(self.player_x), int(self.player_y)
            # Propulsores duplos
            fl1 = random.randint(8, 16)
            fl2 = random.randint(8, 16)
            pygame.draw.polygon(surf, PALETTE["yellow"], [(px - 8, py + 12), (px - 4, py + 12), (px - 6, py + 12 + fl1)])
            pygame.draw.polygon(surf, PALETTE["yellow"], [(px + 4, py + 12), (px + 8, py + 12), (px + 6, py + 12 + fl2)])

            # Fuselagem
            pts = [(px, py - 20), (px - 18, py + 14), (px, py + 8), (px + 18, py + 14)]
            pygame.draw.polygon(surf, PALETTE["cobalt"], pts)
            pygame.draw.polygon(surf, PALETTE["ice"], pts, 2)
            pygame.draw.circle(surf, PALETTE["emerald"], (px, py), 4)

            # Escudo
            if self.shield > 0.0 or self.shield_flash > 0.0:
                pygame.draw.circle(surf, PALETTE["ice"], (px, py), 26, 2)

        # HUD Superior Tático
        pygame.draw.rect(surf, PALETTE["surface"], (0, 0, 800, 50))
        pygame.draw.line(surf, PALETTE["border"], (0, 50), (800, 50), 2)

        # Escudo
        txt_s = fonts["sm"].render("ESCUDO", True, PALETTE["ice"])
        surf.blit(txt_s, (20, 16))
        pygame.draw.rect(surf, PALETTE["bg"], (85, 18, 90, 14), border_radius=3)
        pygame.draw.rect(surf, PALETTE["ice"], (86, 19, int(88 * (self.shield / 100.0)), 12), border_radius=2)

        # Casco
        txt_h = fonts["sm"].render("CASCO", True, PALETTE["emerald"])
        surf.blit(txt_h, (190, 16))
        pygame.draw.rect(surf, PALETTE["bg"], (245, 18, 90, 14), border_radius=3)
        h_col = PALETTE["emerald"] if self.hull > 35 else PALETTE["coral"]
        pygame.draw.rect(surf, h_col, (246, 19, int(88 * (self.hull / 100.0)), 12), border_radius=2)

        # Pontos
        txt_score = fonts["md"].render(f"PONTOS: {self.score}", True, PALETTE["yellow"])
        surf.blit(txt_score, (360, 12))

        # Nível & EMP
        txt_w = fonts["sm"].render(f"NÍVEL: {self.level} | EMP: {self.nova_bombs}", True, PALETTE["purple"])
        surf.blit(txt_w, (550, 16))

        # Vidas
        for i in range(3):
            col = PALETTE["coral"] if i < self.lives else (50, 60, 75)
            pygame.draw.circle(surf, col, (720 + i * 22, 25), 7)



# ==========================================
# 2. SNAKE ARCADE
# ==========================================
class SnakeGame:
    def __init__(self):
        self.cell = 20
        self.grid_w = 40
        self.grid_h = 30
        self.snake = [(20, 15), (19, 15), (18, 15)]
        self.dir = (1, 0)
        self.next_dir = (1, 0)
        self.apple = self.spawn_apple()
        self.score = 0
        self.level = 1
        self.game_over = False
        self.tick_timer = 0
        self.speed = 5

    def spawn_apple(self):
        while True:
            pos = (random.randint(1, self.grid_w - 2), random.randint(1, self.grid_h - 2))
            if pos not in self.snake:
                return pos

    def update(self, keys, events):
        if self.game_over:
            return
        kset = get_keys_set(keys)

        for ev in events:
            if ev.get("type") == "click":
                # Direção pelo clique em relação à cabeça
                head = self.snake[0]
                cx, cy = head[0] * self.cell + 10, head[1] * self.cell + 10
                mx, my = ev.get("x", 400), ev.get("y", 300)
                dx, dy = mx - cx, my - cy
                if abs(dx) > abs(dy):
                    if dx > 0 and self.dir != (-1, 0): self.next_dir = (1, 0)
                    elif dx < 0 and self.dir != (1, 0): self.next_dir = (-1, 0)
                else:
                    if dy > 0 and self.dir != (0, -1): self.next_dir = (0, 1)
                    elif dy < 0 and self.dir != (0, 1): self.next_dir = (0, -1)

        if ("arrowup" in kset or "w" in kset) and self.dir != (0, 1):
            self.next_dir = (0, -1)
        elif ("arrowdown" in kset or "s" in kset) and self.dir != (0, -1):
            self.next_dir = (0, 1)
        elif ("arrowleft" in kset or "a" in kset) and self.dir != (1, 0):
            self.next_dir = (-1, 0)
        elif ("arrowright" in kset or "d" in kset) and self.dir != (-1, 0):
            self.next_dir = (1, 0)

        self.tick_timer += 1
        curr_speed = max(2, self.speed - (self.score // 50))
        if self.tick_timer >= curr_speed:
            self.tick_timer = 0
            self.dir = self.next_dir
            hx, hy = self.snake[0]
            dx, dy = self.dir
            new_head = ((hx + dx) % self.grid_w, (hy + dy) % self.grid_h)

            # Colisão consigo mesmo
            if new_head in self.snake:
                self.game_over = True
                return

            self.snake.insert(0, new_head)
            if new_head == self.apple:
                self.score += 10
                self.level = 1 + self.score // 50
                self.apple = self.spawn_apple()
            else:
                self.snake.pop()

    def draw(self, surf, fonts):
        surf.fill(PALETTE["bg"])

        # Grid suave
        for x in range(0, 800, self.cell):
            pygame.draw.line(surf, (15, 23, 42), (x, 0), (x, 600), 1)
        for y in range(0, 600, self.cell):
            pygame.draw.line(surf, (15, 23, 42), (0, y), (800, y), 1)

        # Maçã com brilho
        ax, ay = self.apple[0] * self.cell, self.apple[1] * self.cell
        pulse = math.sin(pygame.time.get_ticks() * 0.006) * 2
        pygame.draw.circle(surf, PALETTE["coral"], (ax + 10, ay + 10), int(8 + pulse))
        pygame.draw.circle(surf, (255, 255, 255), (ax + 7, ay + 7), 2)
        pygame.draw.rect(surf, PALETTE["emerald"], (ax + 9, ay + 2, 2, 4))

        # Cobra
        for i, (sx, sy) in enumerate(self.snake):
            rect = (sx * self.cell + 1, sy * self.cell + 1, self.cell - 2, self.cell - 2)
            if i == 0:
                pygame.draw.rect(surf, PALETTE["ice"], rect, border_radius=6)
                # Olhos
                dx, dy = self.dir
                ox, oy = (sx * self.cell + 10 + dx * 4, sy * self.cell + 10 + dy * 4)
                pygame.draw.circle(surf, PALETTE["bg"], (int(ox), int(oy)), 3)
            else:
                # Gradiente para cauda
                factor = max(0.4, 1.0 - (i / max(1, len(self.snake))))
                col = (int(PALETTE["cobalt"][0] * factor), int(PALETTE["cobalt"][1] * factor), int(PALETTE["cobalt"][2] * factor))
                pygame.draw.rect(surf, col, rect, border_radius=4)

        # HUD
        surf.blit(fonts["md"].render(f"PONTUAÇÃO: {self.score}", True, PALETTE["yellow"]), (20, 20))
        surf.blit(fonts["sm"].render(f"VELOCIDADE: {self.level}x", True, PALETTE["emerald"]), (20, 50))


# ==========================================
# 3. PACMAN RETRÔ
# ==========================================
class PacmanGame:
    def __init__(self):
        self.rows = 15
        self.cols = 20
        self.cell_w = 40
        self.cell_h = 40
        # 1 = Parede, 0 = Pílula, 2 = Energizer, 3 = Vazio
        self.grid = [
            [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
            [1,2,0,0,0,0,1,0,0,0,0,0,0,1,0,0,0,0,2,1],
            [1,0,1,1,0,0,1,0,1,1,1,1,0,1,0,0,1,1,0,1],
            [1,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,1],
            [1,0,0,0,1,1,0,1,1,3,3,1,1,0,1,1,0,0,0,1],
            [1,1,1,0,1,0,0,1,3,3,3,3,1,0,0,1,0,1,1,1],
            [3,3,1,0,0,0,0,1,3,3,3,3,1,0,0,0,0,1,3,3],
            [1,1,1,0,1,0,0,1,1,1,1,1,1,0,0,1,0,1,1,1],
            [1,0,0,0,1,1,0,0,0,0,0,0,0,0,1,1,0,0,0,1],
            [1,0,1,0,0,0,0,1,1,0,0,1,1,0,0,0,0,1,0,1],
            [1,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,1],
            [1,2,0,0,0,1,1,0,1,1,1,1,0,1,1,0,0,0,2,1],
            [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
            [3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3],
            [3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3],
        ]
        self.px, self.py = 10.0, 8.0
        self.pdir = (0, 0)
        self.next_dir = (0, 0)
        self.score = 0
        self.lives = 3
        self.level = 1
        self.game_over = False
        self.frightened_timer = 0
        self.mouth_angle = 0
        self.ghosts = [
            {"x": 9.0, "y": 6.0, "dir": (-1, 0), "col": PALETTE["coral"]},
            {"x": 10.0, "y": 6.0, "dir": (1, 0), "col": (236, 72, 153)},
            {"x": 9.0, "y": 5.0, "dir": (0, -1), "col": PALETTE["ice"]},
            {"x": 10.0, "y": 5.0, "dir": (0, 1), "col": PALETTE["yellow"]}
        ]

    def update(self, keys, events):
        if self.game_over:
            return
        kset = get_keys_set(keys)

        for ev in events:
            if ev.get("type") == "click":
                mx, my = ev.get("x", 400), ev.get("y", 300)
                cx, cy = self.px * self.cell_w + 20, self.py * self.cell_h + 20
                if abs(mx - cx) > abs(my - cy):
                    self.next_dir = (1, 0) if mx > cx else (-1, 0)
                else:
                    self.next_dir = (0, 1) if my > cy else (0, -1)

        if "arrowup" in kset or "w" in kset: self.next_dir = (0, -1)
        elif "arrowdown" in kset or "s" in kset: self.next_dir = (0, 1)
        elif "arrowleft" in kset or "a" in kset: self.next_dir = (-1, 0)
        elif "arrowright" in kset or "d" in kset: self.next_dir = (1, 0)

        # Pacman move
        spd = 0.15
        nx = self.px + self.next_dir[0] * spd
        ny = self.py + self.next_dir[1] * spd
        gx, gy = int(round(nx)), int(round(ny))
        if 0 <= gy < 13 and 0 <= gx < 20 and self.grid[gy][gx] != 1:
            self.pdir = self.next_dir

        self.px += self.pdir[0] * spd
        self.py += self.pdir[1] * spd

        # Coletar pontos
        cgx, cgy = int(round(self.px)), int(round(self.py))
        if 0 <= cgy < 13 and 0 <= cgx < 20:
            if self.grid[cgy][cgx] == 0:
                self.grid[cgy][cgx] = 3
                self.score += 10
            elif self.grid[cgy][cgx] == 2:
                self.grid[cgy][cgx] = 3
                self.score += 50
                self.frightened_timer = 200

        if self.frightened_timer > 0:
            self.frightened_timer -= 1

        # Animação da boca
        self.mouth_angle = abs(math.sin(pygame.time.get_ticks() * 0.015)) * 40

        # Mover Fantasmas
        gspd = 0.08 if self.frightened_timer == 0 else 0.04
        dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)]
        for g in self.ghosts:
            g["x"] += g["dir"][0] * gspd
            g["y"] += g["dir"][1] * gspd
            gx, gy = int(round(g["x"])), int(round(g["y"]))
            if random.random() < 0.05 or self.grid[gy][gx] == 1:
                valid_dirs = []
                for d in dirs:
                    tx, ty = int(round(g["x"] + d[0])), int(round(g["y"] + d[1]))
                    if 0 <= ty < 13 and 0 <= tx < 20 and self.grid[ty][tx] != 1:
                        valid_dirs.append(d)
                if valid_dirs:
                    g["dir"] = random.choice(valid_dirs)

            # Colisão com Pacman
            dist = math.hypot(g["x"] - self.px, g["y"] - self.py)
            if dist < 0.7:
                if self.frightened_timer > 0:
                    self.score += 200
                    g["x"], g["y"] = 9.5, 6.0
                else:
                    self.lives -= 1
                    self.px, self.py = 10.0, 8.0
                    self.pdir = (0, 0)
                    if self.lives <= 0:
                        self.game_over = True

    def draw(self, surf, fonts):
        surf.fill(PALETTE["bg"])

        # Labirinto
        for r in range(13):
            for c in range(20):
                val = self.grid[r][c]
                rx, ry = c * self.cell_w, r * self.cell_h
                if val == 1:
                    pygame.draw.rect(surf, PALETTE["cobalt"], (rx + 2, ry + 2, self.cell_w - 4, self.cell_h - 4), border_radius=6)
                elif val == 0:
                    pygame.draw.circle(surf, PALETTE["yellow"], (rx + 20, ry + 20), 3)
                elif val == 2:
                    p = math.sin(pygame.time.get_ticks() * 0.01) * 2
                    pygame.draw.circle(surf, (255, 255, 255), (rx + 20, ry + 20), int(6 + p))

        # Pacman
        pcx, pcy = int(self.px * self.cell_w + 20), int(self.py * self.cell_h + 20)
        pygame.draw.circle(surf, PALETTE["yellow"], (pcx, pcy), 14)
        # Recorte da boca
        angle_rad = math.radians(self.mouth_angle)
        dx, dy = (self.pdir[0], self.pdir[1]) if self.pdir != (0, 0) else (1, 0)
        base_angle = math.atan2(dy, dx)
        p1 = (pcx, pcy)
        p2 = (pcx + int(16 * math.cos(base_angle + angle_rad)), pcy + int(16 * math.sin(base_angle + angle_rad)))
        p3 = (pcx + int(16 * math.cos(base_angle - angle_rad)), pcy + int(16 * math.sin(base_angle - angle_rad)))
        pygame.draw.polygon(surf, PALETTE["bg"], [p1, p2, p3])

        # Fantasmas
        for g in self.ghosts:
            gcx, gcy = int(g["x"] * self.cell_w + 20), int(g["y"] * self.cell_h + 20)
            col = (59, 130, 246) if self.frightened_timer > 0 else g["col"]
            # Corpo do fantasma
            pygame.draw.circle(surf, col, (gcx, gcy - 2), 12)
            pygame.draw.rect(surf, col, (gcx - 12, gcy - 2, 24, 14))
            # Olhos
            pygame.draw.circle(surf, (255, 255, 255), (gcx - 4, gcy - 4), 4)
            pygame.draw.circle(surf, (255, 255, 255), (gcx + 4, gcy - 4), 4)
            pygame.draw.circle(surf, (15, 23, 42), (gcx - 4, gcy - 4), 2)
            pygame.draw.circle(surf, (15, 23, 42), (gcx + 4, gcy - 4), 2)

        # Barra inferior de HUD
        pygame.draw.rect(surf, PALETTE["surface"], (0, 520, 800, 80))
        surf.blit(fonts["md"].render(f"PONTOS: {self.score}", True, PALETTE["yellow"]), (30, 545))
        surf.blit(fonts["sm"].render("VIDAS:", True, PALETTE["text"]), (600, 550))
        for i in range(3):
            col = PALETTE["yellow"] if i < self.lives else (50, 60, 75)
            pygame.draw.circle(surf, col, (680 + i * 26, 560), 8)


# ==========================================
# 4. FLAPPY BIRD / JETPACK
# ==========================================
class FlappyGame:
    def __init__(self):
        self.bird_y = 300.0
        self.bird_vy = 0.0
        self.pipes = []
        self.score = 0
        self.level = 1
        self.game_over = False
        self.tick_count = 0
        self.spawn_pipe()

    def spawn_pipe(self):
        gap_h = 160
        top_h = random.randint(80, 360)
        self.pipes.append({
            "x": 820.0,
            "top_h": top_h,
            "gap": gap_h,
            "scored": False
        })

    def update(self, keys, events):
        if self.game_over:
            return
        self.tick_count += 1
        kset = get_keys_set(keys)

        flap = False
        for ev in events:
            if ev.get("type") in ("click", "mousedown"):
                flap = True

        if " " in kset or "space" in kset or "arrowup" in kset or "w" in kset:
            if self.tick_count % 3 == 0:  # anti-spam suave
                flap = True

        if flap:
            self.bird_vy = -7.5

        # Física
        self.bird_vy += 0.45
        self.bird_y += self.bird_vy

        # Spawn pipes
        if self.tick_count % 85 == 0:
            self.spawn_pipe()

        # Atualizar pipes
        bird_box = pygame.Rect(180, int(self.bird_y) - 14, 28, 28)
        for p in self.pipes[:]:
            p["x"] -= 4.0
            # Pontuação
            if not p["scored"] and p["x"] < 180:
                p["scored"] = True
                self.score += 1
                if self.score % 10 == 0:
                    self.level += 1

            # Colisão
            top_rect = pygame.Rect(int(p["x"]), 0, 64, p["top_h"])
            bot_rect = pygame.Rect(int(p["x"]), p["top_h"] + p["gap"], 64, 600 - (p["top_h"] + p["gap"]))
            if bird_box.colliderect(top_rect) or bird_box.colliderect(bot_rect):
                self.game_over = True

            if p["x"] < -80:
                self.pipes.remove(p)

        # Colisão com chão e teto
        if self.bird_y > 570 or self.bird_y < 10:
            self.game_over = True

    def draw(self, surf, fonts):
        surf.fill(PALETTE["bg"])

        # Céu / Horizonte retrô
        for i in range(12):
            col = (12 + i * 2, 18 + i * 3, 34 + i * 4)
            pygame.draw.rect(surf, col, (0, i * 50, 800, 50))

        # Canos / Pilares
        for p in self.pipes:
            px = int(p["x"])
            # Superior
            pygame.draw.rect(surf, PALETTE["cobalt"], (px, 0, 64, p["top_h"]))
            pygame.draw.rect(surf, PALETTE["ice"], (px - 4, p["top_h"] - 20, 72, 20), border_radius=4)
            # Inferior
            by = p["top_h"] + p["gap"]
            pygame.draw.rect(surf, PALETTE["cobalt"], (px, by, 64, 600 - by))
            pygame.draw.rect(surf, PALETTE["ice"], (px - 4, by, 72, 20), border_radius=4)

        # Chão
        pygame.draw.rect(surf, PALETTE["surface"], (0, 580, 800, 20))
        pygame.draw.line(surf, PALETTE["emerald"], (0, 580), (800, 580), 2)

        # Pássaro / Pod
        by = int(self.bird_y)
        pygame.draw.circle(surf, PALETTE["yellow"], (194, by), 14)
        pygame.draw.circle(surf, PALETTE["coral"], (204, by + 2), 6)  # Bico
        pygame.draw.circle(surf, (255, 255, 255), (198, by - 4), 4)   # Olho
        pygame.draw.circle(surf, (15, 23, 42), (200, by - 4), 2)
        # Asa inclinada com base na velocidade
        wing_dy = int(max(-8, min(8, self.bird_vy)))
        pygame.draw.polygon(surf, PALETTE["ice"], [(186, by), (176, by + wing_dy), (184, by + 6)])

        # HUD
        surf.blit(fonts["lg"].render(str(self.score), True, (255, 255, 255)), (390, 40))


# ==========================================
# 5. PONG CLÁSSICO
# ==========================================
class PongGame:
    def __init__(self):
        self.p1_y = 250.0
        self.p2_y = 250.0
        self.ball_x = 400.0
        self.ball_y = 300.0
        self.ball_vx = 6.0 * random.choice([-1, 1])
        self.ball_vy = random.uniform(-3, 3)
        self.trail = []
        self.score = 0  # Pontos P1
        self.score_p2 = 0
        self.level = 1
        self.game_over = False

    def update(self, keys, events):
        if self.game_over:
            return
        kset = get_keys_set(keys)

        for ev in events:
            if ev.get("type") == "mousemove":
                self.p1_y = max(40, min(480, ev.get("y", 300) - 45))

        # Controle P1
        if "arrowup" in kset or "w" in kset:
            self.p1_y = max(20, self.p1_y - 8)
        if "arrowdown" in kset or "s" in kset:
            self.p1_y = min(490, self.p1_y + 8)

        # IA P2 responsiva
        target_y = self.ball_y - 45
        diff = target_y - self.p2_y
        self.p2_y += max(-6.5, min(6.5, diff * 0.12))

        # Atualizar rastro
        self.trail.append((self.ball_x, self.ball_y))
        if len(self.trail) > 6:
            self.trail.pop(0)

        # Mover Bola
        self.ball_x += self.ball_vx
        self.ball_y += self.ball_vy

        # Rebater Teto / Chão
        if self.ball_y <= 10:
            self.ball_y = 10
            self.ball_vy *= -1
        elif self.ball_y >= 590:
            self.ball_y = 590
            self.ball_vy *= -1

        # Rebater Raquete P1 (Esquerda x=40)
        if self.ball_x <= 54 and self.ball_vx < 0:
            if self.p1_y - 10 <= self.ball_y <= self.p1_y + 100:
                self.ball_vx = abs(self.ball_vx) * 1.05
                offset = (self.ball_y - (self.p1_y + 45)) / 45.0
                self.ball_vy = offset * 6.5
                self.score += 10

        # Rebater Raquete P2 (Direita x=746)
        if self.ball_x >= 746 and self.ball_vx > 0:
            if self.p2_y - 10 <= self.ball_y <= self.p2_y + 100:
                self.ball_vx = -abs(self.ball_vx) * 1.05
                offset = (self.ball_y - (self.p2_y + 45)) / 45.0
                self.ball_vy = offset * 6.5

        # Ponto
        if self.ball_x < 0:
            self.score_p2 += 1
            self.reset_ball(1)
        elif self.ball_x > 800:
            self.score += 100
            self.reset_ball(-1)

        if self.score_p2 >= 7:
            self.game_over = True

    def reset_ball(self, dir_x):
        self.ball_x = 400.0
        self.ball_y = 300.0
        self.ball_vx = 6.0 * dir_x
        self.ball_vy = random.uniform(-3, 3)
        self.trail = []

    def draw(self, surf, fonts):
        surf.fill(PALETTE["bg"])

        # Linha central tracejada
        for y in range(0, 600, 30):
            pygame.draw.line(surf, (30, 41, 59), (400, y), (400, y + 16), 3)

        # Rastro da bola
        for i, (tx, ty) in enumerate(self.trail):
            rad = max(2, int(8 * (i / len(self.trail))))
            pygame.draw.circle(surf, PALETTE["ice"], (int(tx), int(ty)), rad)

        # Bola
        pygame.draw.circle(surf, (255, 255, 255), (int(self.ball_x), int(self.ball_y)), 9)
        pygame.draw.circle(surf, PALETTE["yellow"], (int(self.ball_x), int(self.ball_y)), 6)

        # Raquetes
        pygame.draw.rect(surf, PALETTE["cobalt"], (40, int(self.p1_y), 14, 90), border_radius=6)
        pygame.draw.rect(surf, PALETTE["coral"], (746, int(self.p2_y), 14, 90), border_radius=6)

        # Placar grande
        surf.blit(fonts["lg"].render(str(self.score), True, PALETTE["ice"]), (320, 30))
        surf.blit(fonts["lg"].render(str(self.score_p2), True, PALETTE["coral"]), (440, 30))


# ==========================================
# 6. BREAKOUT ARCADE
# ==========================================
class BreakoutGame:
    def __init__(self):
        self.paddle_x = 350.0
        self.paddle_w = 110
        self.ball_x = 400.0
        self.ball_y = 520.0
        self.ball_vx = 5.0
        self.ball_vy = -5.0
        self.attached = True
        self.bricks = []
        self.particles = []
        self.score = 0
        self.lives = 3
        self.level = 1
        self.game_over = False
        self.init_bricks()

    def init_bricks(self):
        colors = [PALETTE["coral"], PALETTE["yellow"], PALETTE["emerald"], PALETTE["ice"], PALETTE["cobalt"]]
        self.bricks = []
        for r in range(5):
            for c in range(10):
                self.bricks.append({
                    "rect": pygame.Rect(35 + c * 74, 60 + r * 28, 66, 22),
                    "color": colors[r],
                    "points": (5 - r) * 20
                })

    def update(self, keys, events):
        if self.game_over:
            return
        kset = get_keys_set(keys)

        for ev in events:
            if ev.get("type") == "mousemove":
                self.paddle_x = max(20, min(780 - self.paddle_w, ev.get("x", 400) - self.paddle_w // 2))
            elif ev.get("type") == "click" and self.attached:
                self.attached = False

        if "arrowleft" in kset or "a" in kset:
            self.paddle_x = max(20, self.paddle_x - 8)
        if "arrowright" in kset or "d" in kset:
            self.paddle_x = min(780 - self.paddle_w, self.paddle_x + 8)

        if (" " in kset or "space" in kset or "enter" in kset) and self.attached:
            self.attached = False

        if self.attached:
            self.ball_x = self.paddle_x + self.paddle_w / 2
            self.ball_y = 530.0
            return

        # Movimento da bola
        self.ball_x += self.ball_vx
        self.ball_y += self.ball_vy

        # Paredes
        if self.ball_x <= 10:
            self.ball_x = 10
            self.ball_vx *= -1
        elif self.ball_x >= 790:
            self.ball_x = 790
            self.ball_vx *= -1
        if self.ball_y <= 10:
            self.ball_y = 10
            self.ball_vy *= -1

        # Raquete
        paddle_rect = pygame.Rect(int(self.paddle_x), 545, self.paddle_w, 18)
        ball_rect = pygame.Rect(int(self.ball_x) - 7, int(self.ball_y) - 7, 14, 14)
        if ball_rect.colliderect(paddle_rect) and self.ball_vy > 0:
            self.ball_vy = -abs(self.ball_vy)
            offset = (self.ball_x - (self.paddle_x + self.paddle_w / 2)) / (self.paddle_w / 2)
            self.ball_vx = offset * 6.5

        # Colisão com Tijolos
        for b in self.bricks[:]:
            if b["rect"].colliderect(ball_rect):
                self.bricks.remove(b)
                self.ball_vy *= -1
                self.score += b["points"]
                for _ in range(8):
                    self.particles.append([
                        self.ball_x, self.ball_y,
                        random.uniform(-3, 3), random.uniform(-3, 3),
                        b["color"], random.randint(10, 20)
                    ])
                break

        # Queda ao fundo
        if self.ball_y > 610:
            self.lives -= 1
            if self.lives <= 0:
                self.game_over = True
            else:
                self.attached = True

        # Vitória da fase
        if not self.bricks:
            self.level += 1
            self.init_bricks()
            self.attached = True

        # Atualizar partículas
        for p in self.particles[:]:
            p[0] += p[2]
            p[1] += p[3]
            p[5] -= 1
            if p[5] <= 0:
                self.particles.remove(p)

    def draw(self, surf, fonts):
        surf.fill(PALETTE["bg"])

        # Tijolos
        for b in self.bricks:
            pygame.draw.rect(surf, b["color"], b["rect"], border_radius=4)
            pygame.draw.rect(surf, (255, 255, 255), b["rect"], 1, border_radius=4)

        # Partículas
        for px, py, _, _, col, life in self.particles:
            pygame.draw.circle(surf, col, (int(px), int(py)), max(1, life // 4))

        # Raquete
        pygame.draw.rect(surf, PALETTE["cobalt"], (int(self.paddle_x), 545, self.paddle_w, 18), border_radius=8)
        pygame.draw.rect(surf, PALETTE["ice"], (int(self.paddle_x) + 4, 547, self.paddle_w - 8, 4), border_radius=2)

        # Bola
        pygame.draw.circle(surf, (255, 255, 255), (int(self.ball_x), int(self.ball_y)), 8)
        pygame.draw.circle(surf, PALETTE["yellow"], (int(self.ball_x), int(self.ball_y)), 5)

        # HUD
        surf.blit(fonts["md"].render(f"PONTOS: {self.score}", True, PALETTE["yellow"]), (20, 15))
        surf.blit(fonts["sm"].render(f"FASE {self.level}", True, PALETTE["ice"]), (400, 15))
        for i in range(3):
            col = PALETTE["coral"] if i < self.lives else (50, 60, 75)
            pygame.draw.circle(surf, col, (730 + i * 22, 25), 7)


# ==========================================
# 7. CANHÃO BALÍSTICO
# ==========================================
class CannonGame:
    def __init__(self):
        self.angle = 45.0  # Graus
        self.power = 18.0
        self.projectiles = []
        self.targets = []
        self.particles = []
        self.score = 0
        self.level = 1
        self.game_over = False
        self.tick_count = 0
        self.spawn_targets()

    def spawn_targets(self):
        self.targets = []
        for i in range(4):
            self.targets.append({
                "x": random.randint(400, 750),
                "y": random.randint(100, 380),
                "vy": random.choice([-1.5, 1.5]),
                "r": random.randint(18, 28),
                "col": random.choice([PALETTE["coral"], PALETTE["yellow"], PALETTE["ice"]])
            })

    def update(self, keys, events):
        if self.game_over:
            return
        self.tick_count += 1
        kset = get_keys_set(keys)

        for ev in events:
            if ev.get("type") == "mousemove":
                mx, my = ev.get("x", 400), ev.get("y", 300)
                dx, dy = mx - 80, 520 - my
                if dx > 0 and dy > 0:
                    self.angle = math.degrees(math.atan2(dy, dx))
                    self.power = max(10, min(26, math.hypot(dx, dy) * 0.05))
            elif ev.get("type") == "click":
                self.fire()

        if "arrowup" in kset or "w" in kset:
            self.angle = min(85.0, self.angle + 1.5)
        if "arrowdown" in kset or "s" in kset:
            self.angle = max(15.0, self.angle - 1.5)
        if "arrowright" in kset or "d" in kset:
            self.power = min(28.0, self.power + 0.3)
        if "arrowleft" in kset or "a" in kset:
            self.power = max(8.0, self.power - 0.3)

        if (" " in kset or "space" in kset or "enter" in kset) and self.tick_count % 15 == 0:
            self.fire()

        # Atualizar Projéteis
        for proj in self.projectiles[:]:
            proj["x"] += proj["vx"]
            proj["y"] += proj["vy"]
            proj["vy"] += 0.45  # Gravidade
            # Trilha
            self.particles.append([proj["x"], proj["y"], 0, 0, PALETTE["muted"], 8])

            # Colisão com Alvos
            for t in self.targets[:]:
                dist = math.hypot(proj["x"] - t["x"], proj["y"] - t["y"])
                if dist < t["r"] + 8:
                    if proj in self.projectiles:
                        self.projectiles.remove(proj)
                    self.targets.remove(t)
                    self.score += 150
                    for _ in range(20):
                        self.particles.append([
                            t["x"], t["y"],
                            random.uniform(-5, 5), random.uniform(-5, 5),
                            t["col"], random.randint(15, 30)
                        ])
                    break

            if proj["y"] > 560 or proj["x"] > 820:
                if proj in self.projectiles:
                    self.projectiles.remove(proj)

        # Mover Alvos
        for t in self.targets:
            t["y"] += t["vy"]
            if t["y"] < 80 or t["y"] > 450:
                t["vy"] *= -1

        if not self.targets:
            self.level += 1
            self.spawn_targets()

        # Partículas
        for p in self.particles[:]:
            p[0] += p[2]
            p[1] += p[3]
            p[5] -= 1
            if p[5] <= 0:
                self.particles.remove(p)

    def fire(self):
        rad = math.radians(self.angle)
        vx = math.cos(rad) * self.power
        vy = -math.sin(rad) * self.power
        bx = 80 + math.cos(rad) * 45
        by = 520 - math.sin(rad) * 45
        self.projectiles.append({"x": bx, "y": by, "vx": vx, "vy": vy})

    def draw(self, surf, fonts):
        surf.fill(PALETTE["bg"])

        # Terreno
        pygame.draw.rect(surf, PALETTE["surface"], (0, 540, 800, 60))
        pygame.draw.line(surf, PALETTE["emerald"], (0, 540), (800, 540), 2)

        # Alvos
        for t in self.targets:
            pygame.draw.circle(surf, t["col"], (int(t["x"]), int(t["y"])), t["r"])
            pygame.draw.circle(surf, (255, 255, 255), (int(t["x"]), int(t["y"])), max(2, t["r"] // 2))

        # Partículas
        for px, py, _, _, col, life in self.particles:
            pygame.draw.circle(surf, col, (int(px), int(py)), max(1, life // 3))

        # Projéteis
        for proj in self.projectiles:
            pygame.draw.circle(surf, PALETTE["yellow"], (int(proj["x"]), int(proj["y"])), 6)

        # Canhão
        rad = math.radians(self.angle)
        end_x = 80 + math.cos(rad) * 45
        end_y = 520 - math.sin(rad) * 45
        pygame.draw.line(surf, PALETTE["cobalt"], (80, 520), (int(end_x), int(end_y)), 12)
        pygame.draw.circle(surf, PALETTE["ice"], (80, 520), 18)

        # HUD
        surf.blit(fonts["md"].render(f"PONTOS: {self.score}", True, PALETTE["yellow"]), (20, 20))
        surf.blit(fonts["sm"].render(f"ÂNGULO: {int(self.angle)}°  FORÇA: {int(self.power)}", True, PALETTE["ice"]), (20, 55))


# ==========================================
# 8. TRON LIGHT CYCLES
# ==========================================
class TronGame:
    def __init__(self):
        self.w = 800
        self.h = 600
        self.grid = set()
        self.p1_x = 150
        self.p1_y = 300
        self.p1_dir = (2, 0)
        self.p1_next = (2, 0)

        self.p2_x = 650
        self.p2_y = 300
        self.p2_dir = (-2, 0)

        self.score = 0
        self.level = 1
        self.game_over = False

    def update(self, keys, events):
        if self.game_over:
            return
        kset = get_keys_set(keys)

        for ev in events:
            if ev.get("type") == "click":
                mx, my = ev.get("x", 400), ev.get("y", 300)
                dx, dy = mx - self.p1_x, my - self.p1_y
                if abs(dx) > abs(dy):
                    if dx > 0 and self.p1_dir != (-2, 0): self.p1_next = (2, 0)
                    elif dx < 0 and self.p1_dir != (2, 0): self.p1_next = (-2, 0)
                else:
                    if dy > 0 and self.p1_dir != (0, -2): self.p1_next = (0, 2)
                    elif dy < 0 and self.p1_dir != (0, 2): self.p1_next = (0, -2)

        if ("arrowup" in kset or "w" in kset) and self.p1_dir != (0, 2): self.p1_next = (0, -2)
        elif ("arrowdown" in kset or "s" in kset) and self.p1_dir != (0, -2): self.p1_next = (0, 2)
        elif ("arrowleft" in kset or "a" in kset) and self.p1_dir != (2, 0): self.p1_next = (-2, 0)
        elif ("arrowright" in kset or "d" in kset) and self.p1_dir != (-2, 0): self.p1_next = (2, 0)

        self.p1_dir = self.p1_next

        # IA simples para P2 (desvia de obstáculos)
        p2_next_pos = (self.p2_x + self.p2_dir[0] * 5, self.p2_y + self.p2_dir[1] * 5)
        if (p2_next_pos in self.grid or p2_next_pos[0] < 40 or p2_next_pos[0] > 760 or
            p2_next_pos[1] < 40 or p2_next_pos[1] > 560 or random.random() < 0.02):
            choices = [(2, 0), (-2, 0), (0, 2), (0, -2)]
            valid = []
            for c in choices:
                if c[0] == -self.p2_dir[0] and c[1] == -self.p2_dir[1]:
                    continue
                test_pos = (self.p2_x + c[0] * 6, self.p2_y + c[1] * 6)
                if 40 < test_pos[0] < 760 and 40 < test_pos[1] < 560 and test_pos not in self.grid:
                    valid.append(c)
            if valid:
                self.p2_dir = random.choice(valid)

        # Mover P1 e P2 (2 passos por tick para suavidade)
        for _ in range(2):
            self.p1_x += self.p1_dir[0]
            self.p1_y += self.p1_dir[1]
            p1_pos = (self.p1_x, self.p1_y)

            self.p2_x += self.p2_dir[0]
            self.p2_y += self.p2_dir[1]
            p2_pos = (self.p2_x, self.p2_y)

            # Colisão P1
            if (p1_pos in self.grid or self.p1_x < 20 or self.p1_x > 780 or
                self.p1_y < 20 or self.p1_y > 580):
                self.game_over = True
                return

            # Colisão P2
            if (p2_pos in self.grid or self.p2_x < 20 or self.p2_x > 780 or
                self.p2_y < 20 or self.p2_y > 580):
                self.score += 500
                self.game_over = True
                return

            self.grid.add(p1_pos)
            self.grid.add(p2_pos)
            self.score += 1

    def draw(self, surf, fonts):
        surf.fill(PALETTE["bg"])

        # Grid Cibernético
        for x in range(0, 800, 40):
            pygame.draw.line(surf, (15, 23, 42), (x, 0), (x, 600), 1)
        for y in range(0, 600, 40):
            pygame.draw.line(surf, (15, 23, 42), (0, y), (800, y), 1)

        # Bordas
        pygame.draw.rect(surf, PALETTE["cobalt"], (18, 18, 764, 564), 2)

        # Rastros
        for gx, gy in self.grid:
            surf.set_at((gx, gy), PALETTE["ice"])

        # Posição P1
        pygame.draw.circle(surf, PALETTE["ice"], (self.p1_x, self.p1_y), 5)
        # Posição P2
        pygame.draw.circle(surf, PALETTE["coral"], (self.p2_x, self.p2_y), 5)

        # HUD
        surf.blit(fonts["md"].render(f"PONTUAÇÃO: {self.score}", True, PALETTE["yellow"]), (30, 30))


# ==========================================
# 9. QUEBRA-CABEÇA 15 (TILES)
# ==========================================
class TilesGame:
    def __init__(self):
        self.board = list(range(1, 16)) + [0]
        self.score = 0  # Movimentos
        self.level = 1
        self.game_over = False
        self.won = False
        self.shuffle_board()

    def shuffle_board(self):
        # Embaralha com movimentos válidos garantindo solvabilidade
        for _ in range(120):
            empty_idx = self.board.index(0)
            er, ec = empty_idx // 4, empty_idx % 4
            adj = []
            if er > 0: adj.append((er - 1) * 4 + ec)
            if er < 3: adj.append((er + 1) * 4 + ec)
            if ec > 0: adj.append(er * 4 + (ec - 1))
            if ec < 3: adj.append(er * 4 + (ec + 1))
            swap_idx = random.choice(adj)
            self.board[empty_idx], self.board[swap_idx] = self.board[swap_idx], self.board[empty_idx]
        self.score = 0
        self.won = False

    def slide(self, target_idx):
        empty_idx = self.board.index(0)
        er, ec = empty_idx // 4, empty_idx % 4
        tr, tc = target_idx // 4, target_idx % 4
        if abs(er - tr) + abs(ec - tc) == 1:
            self.board[empty_idx], self.board[target_idx] = self.board[target_idx], self.board[empty_idx]
            self.score += 1
            if self.board == list(range(1, 16)) + [0]:
                self.won = True
                self.game_over = True

    def update(self, keys, events):
        if self.won:
            return
        kset = get_keys_set(keys)
        empty_idx = self.board.index(0)
        er, ec = empty_idx // 4, empty_idx % 4

        for ev in events:
            if ev.get("type") == "click":
                mx, my = ev.get("x", 0), ev.get("y", 0)
                ox, oy = 220, 120
                if ox <= mx < ox + 360 and oy <= my < oy + 360:
                    tc = (mx - ox) // 90
                    tr = (my - oy) // 90
                    self.slide(tr * 4 + tc)

        # Setas direcionais
        if ("arrowup" in kset or "w" in kset) and er < 3:
            self.slide((er + 1) * 4 + ec)
        elif ("arrowdown" in kset or "s" in kset) and er > 0:
            self.slide((er - 1) * 4 + ec)
        elif ("arrowleft" in kset or "a" in kset) and ec < 3:
            self.slide(er * 4 + (ec + 1))
        elif ("arrowright" in kset or "d" in kset) and ec > 0:
            self.slide(er * 4 + (ec - 1))

    def draw(self, surf, fonts):
        surf.fill(PALETTE["bg"])

        # Tabuleiro 4x4 centralizado (x=220, y=120, tam=360)
        ox, oy = 220, 120
        pygame.draw.rect(surf, PALETTE["surface"], (ox - 8, oy - 8, 376, 376), border_radius=12)

        for i, val in enumerate(self.board):
            r, c = i // 4, i % 4
            tx, ty = ox + c * 90, oy + r * 90
            if val != 0:
                pygame.draw.rect(surf, PALETTE["cobalt"], (tx + 3, ty + 3, 84, 84), border_radius=8)
                pygame.draw.rect(surf, PALETTE["ice"], (tx + 3, ty + 3, 84, 84), 1, border_radius=8)
                t_surf = fonts["lg"].render(str(val), True, (255, 255, 255))
                surf.blit(t_surf, (tx + 45 - t_surf.get_width() // 2, ty + 45 - t_surf.get_height() // 2))

        # HUD
        surf.blit(fonts["md"].render(f"MOVIMENTOS: {self.score}", True, PALETTE["yellow"]), (300, 50))
        if self.won:
            surf.blit(fonts["lg"].render("PARABÉNS! QUEBRA-CABEÇA RESOLVIDO!", True, PALETTE["emerald"]), (140, 520))


# ==========================================
# 10. CAMPO MINADO (MINESWEEPER)
# ==========================================
class MinesweeperGame:
    def __init__(self):
        self.rows = 10
        self.cols = 10
        self.mines_count = 15
        self.mines = set()
        self.revealed = set()
        self.flags = set()
        self.score = 0
        self.level = 1
        self.game_over = False
        self.won = False
        self.first_click = True

    def init_mines(self, safe_r, safe_c):
        all_cells = [(r, c) for r in range(self.rows) for c in range(self.cols) if (r, c) != (safe_r, safe_c)]
        self.mines = set(random.sample(all_cells, self.mines_count))

    def count_neighbors(self, r, c):
        cnt = 0
        for dr in (-1, 0, 1):
            for dc in (-1, 0, 1):
                if (dr, dc) != (0, 0) and (r + dr, c + dc) in self.mines:
                    cnt += 1
        return cnt

    def reveal(self, r, c):
        if (r, c) in self.revealed or (r, c) in self.flags:
            return
        if self.first_click:
            self.init_mines(r, c)
            self.first_click = False

        self.revealed.add((r, c))
        self.score += 10

        if (r, c) in self.mines:
            self.game_over = True
            return

        if self.count_neighbors(r, c) == 0:
            for dr in (-1, 0, 1):
                for dc in (-1, 0, 1):
                    nr, nc = r + dr, c + dc
                    if 0 <= nr < self.rows and 0 <= nc < self.cols:
                        self.reveal(nr, nc)

        # Checar vitória
        if len(self.revealed) == (self.rows * self.cols) - self.mines_count:
            self.won = True
            self.game_over = True

    def update(self, keys, events):
        if self.game_over:
            return
        ox, oy = 210, 110
        cw = 38

        for ev in events:
            etype = ev.get("type")
            mx, my = ev.get("x", 0), ev.get("y", 0)
            if ox <= mx < ox + self.cols * cw and oy <= my < oy + self.rows * cw:
                c = (mx - ox) // cw
                r = (my - oy) // cw
                if etype == "click":
                    button = ev.get("button", 0)
                    if button == 2:  # Botão direito
                        if (r, c) not in self.revealed:
                            if (r, c) in self.flags: self.flags.remove((r, c))
                            else: self.flags.add((r, c))
                    else:
                        self.reveal(r, c)
                elif etype == "contextmenu":
                    if (r, c) not in self.revealed:
                        if (r, c) in self.flags: self.flags.remove((r, c))
                        else: self.flags.add((r, c))

    def draw(self, surf, fonts):
        surf.fill(PALETTE["bg"])

        ox, oy = 210, 110
        cw = 38
        pygame.draw.rect(surf, PALETTE["surface"], (ox - 8, oy - 8, self.cols * cw + 16, self.rows * cw + 16), border_radius=10)

        num_colors = {
            1: PALETTE["ice"], 2: PALETTE["emerald"], 3: PALETTE["coral"],
            4: PALETTE["purple"], 5: PALETTE["yellow"], 6: (236, 72, 153)
        }

        for r in range(self.rows):
            for c in range(self.cols):
                rx, ry = ox + c * cw, oy + r * cw
                rect = (rx + 2, ry + 2, cw - 4, cw - 4)
                if (r, c) in self.revealed:
                    if (r, c) in self.mines:
                        pygame.draw.rect(surf, PALETTE["coral"], rect, border_radius=4)
                        pygame.draw.circle(surf, (0, 0, 0), (rx + cw // 2, ry + cw // 2), 7)
                    else:
                        pygame.draw.rect(surf, (26, 36, 56), rect, border_radius=4)
                        cnt = self.count_neighbors(r, c)
                        if cnt > 0:
                            col = num_colors.get(cnt, (255, 255, 255))
                            t_surf = fonts["md"].render(str(cnt), True, col)
                            surf.blit(t_surf, (rx + cw // 2 - t_surf.get_width() // 2, ry + cw // 2 - t_surf.get_height() // 2))
                else:
                    pygame.draw.rect(surf, PALETTE["cobalt"], rect, border_radius=4)
                    if (r, c) in self.flags:
                        pygame.draw.circle(surf, PALETTE["yellow"], (rx + cw // 2, ry + cw // 2), 6)

        # HUD
        remaining_mines = max(0, self.mines_count - len(self.flags))
        surf.blit(fonts["md"].render(f"MINAS: {remaining_mines}", True, PALETTE["coral"]), (220, 50))
        surf.blit(fonts["md"].render(f"PONTOS: {self.score}", True, PALETTE["yellow"]), (460, 50))
        if self.won:
            surf.blit(fonts["lg"].render("CAMPO DESMINADO COM SUCESSO!", True, PALETTE["emerald"]), (180, 520))


# ==========================================
# 11. CONECTE QUATRO (CONNECT)
# ==========================================
class ConnectGame:
    def __init__(self):
        self.cols = 7
        self.rows = 6
        self.board = [[0 for _ in range(self.cols)] for _ in range(self.rows)]
        self.current_player = 1  # 1 = Azul, 2 = Vermelho (IA)
        self.selected_col = 3
        self.score = 0
        self.level = 1
        self.game_over = False
        self.winner = 0

    def drop_disc(self, col):
        if col < 0 or col >= self.cols or self.board[0][col] != 0:
            return False
        for r in range(self.rows - 1, -1, -1):
            if self.board[r][col] == 0:
                self.board[r][col] = self.current_player
                if self.check_win(r, col, self.current_player):
                    self.winner = self.current_player
                    self.game_over = True
                    if self.winner == 1:
                        self.score += 1000
                else:
                    self.current_player = 2 if self.current_player == 1 else 1
                    if self.current_player == 2 and not self.game_over:
                        self.ai_move()
                return True
        return False

    def ai_move(self):
        valid_cols = [c for c in range(self.cols) if self.board[0][c] == 0]
        if valid_cols:
            self.drop_disc(random.choice(valid_cols))

    def check_win(self, r, c, p):
        dirs = [(0, 1), (1, 0), (1, 1), (1, -1)]
        for dr, dc in dirs:
            count = 1
            # Direção positiva
            for step in range(1, 4):
                nr, nc = r + dr * step, c + dc * step
                if 0 <= nr < self.rows and 0 <= nc < self.cols and self.board[nr][nc] == p:
                    count += 1
                else: break
            # Direção negativa
            for step in range(1, 4):
                nr, nc = r - dr * step, c - dc * step
                if 0 <= nr < self.rows and 0 <= nc < self.cols and self.board[nr][nc] == p:
                    count += 1
                else: break
            if count >= 4:
                return True
        return False

    def update(self, keys, events):
        if self.game_over:
            return
        kset = get_keys_set(keys)

        ox, cw = 175, 65
        for ev in events:
            if ev.get("type") == "mousemove":
                mx = ev.get("x", 400)
                if ox <= mx < ox + self.cols * cw:
                    self.selected_col = (mx - ox) // cw
            elif ev.get("type") == "click":
                mx = ev.get("x", 400)
                if ox <= mx < ox + self.cols * cw:
                    self.drop_disc((mx - ox) // cw)

        if "arrowleft" in kset or "a" in kset:
            self.selected_col = max(0, self.selected_col - 1)
        elif "arrowright" in kset or "d" in kset:
            self.selected_col = min(self.cols - 1, self.selected_col + 1)
        elif " " in kset or "space" in kset or "enter" in kset:
            self.drop_disc(self.selected_col)

    def draw(self, surf, fonts):
        surf.fill(PALETTE["bg"])

        ox, oy = 175, 140
        cw = 65
        # Indicador de coluna do jogador
        if not self.game_over and self.current_player == 1:
            ix = ox + self.selected_col * cw + cw // 2
            pygame.draw.polygon(surf, PALETTE["ice"], [(ix, oy - 20), (ix - 12, oy - 35), (ix + 12, oy - 35)])

        # Chassis do tabuleiro
        pygame.draw.rect(surf, PALETTE["cobalt"], (ox - 10, oy - 10, self.cols * cw + 20, self.rows * cw + 20), border_radius=16)

        for r in range(self.rows):
            for c in range(self.cols):
                cx, cy = ox + c * cw + cw // 2, oy + r * cw + cw // 2
                val = self.board[r][c]
                if val == 0:
                    pygame.draw.circle(surf, PALETTE["bg"], (cx, cy), 24)
                elif val == 1:
                    pygame.draw.circle(surf, PALETTE["ice"], (cx, cy), 24)
                    pygame.draw.circle(surf, (255, 255, 255), (cx, cy), 16)
                elif val == 2:
                    pygame.draw.circle(surf, PALETTE["coral"], (cx, cy), 24)
                    pygame.draw.circle(surf, (255, 255, 255), (cx, cy), 16)

        # HUD
        turn_str = "SUA VEZ (AZUL)" if self.current_player == 1 else "VEZ DA IA (VERMELHO)"
        turn_col = PALETTE["ice"] if self.current_player == 1 else PALETTE["coral"]
        surf.blit(fonts["md"].render(turn_str, True, turn_col), (310, 45))
        if self.game_over:
            res_str = "VITÓRIA DO JOGADOR AZUL!" if self.winner == 1 else "A IA VENCEU A PARTIDA!"
            res_col = PALETTE["emerald"] if self.winner == 1 else PALETTE["coral"]
            surf.blit(fonts["lg"].render(res_str, True, res_col), (200, 545))


# ==========================================
# WRAPPER PARA JOGOS DA COMUNIDADE
# ==========================================
class CommunityGameWrapper:
    def __init__(self, game_id, game_path):
        from app import security_runner
        self.game_class = security_runner.load_verified_game_class(game_id, game_path)
        self.game_instance = self.game_class()

    def update(self, keys, events):
        if hasattr(self.game_instance, "update"):
            self.game_instance.update(keys, events)

    def draw(self, surf, fonts):
        if hasattr(self.game_instance, "draw"):
            self.game_instance.draw(surf, fonts["md"])

    @property
    def score(self):
        return getattr(self.game_instance, "score", 0)

    @property
    def level(self):
        return getattr(self.game_instance, "level", 1)

    @property
    def game_over(self):
        return getattr(self.game_instance, "game_over", False)

    @game_over.setter
    def game_over(self, val):
        if hasattr(self.game_instance, "game_over"):
            self.game_instance.game_over = val


# ==========================================
# MOTOR PRINCIPAL IN-WINDOW
# ==========================================
class InWindowGameRunner:
    def __init__(self):
        self.surface = pygame.Surface((800, 600))
        # NUNCA use SysFont no Windows 11!
        self.fonts = {
            "sm": pygame.font.Font(None, 20),
            "md": pygame.font.Font(None, 28),
            "lg": pygame.font.Font(None, 42),
            "xl": pygame.font.Font(None, 56)
        }
        self.font = self.fonts["md"]
        self.game_id = None
        self.game = None
        self.paused = False
        self.score = 0
        self.level = 1
        self.game_over = False

    def create_game_instance(self, game_id: str):
        if game_id == "space_odyssey": return SpaceOdysseyGame()
        elif game_id == "snake": return SnakeGame()
        elif game_id == "pacman": return PacmanGame()
        elif game_id == "flappy": return FlappyGame()
        elif game_id == "pong": return PongGame()
        elif game_id == "breakout": return BreakoutGame()
        elif game_id == "cannon": return CannonGame()
        elif game_id == "tron": return TronGame()
        elif game_id == "tiles": return TilesGame()
        elif game_id == "minesweeper": return MinesweeperGame()
        elif game_id == "connect": return ConnectGame()
        else:
            # Jogo comunitário
            import sys
            if getattr(sys, 'frozen', False):
                base_dir = os.path.dirname(sys.executable)
            else:
                base_dir = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
            game_path = os.path.join(base_dir, "games", game_id, "main.py")
            if os.path.exists(game_path):
                return CommunityGameWrapper(game_id, game_path)
            raise ValueError(f"Jogo '{game_id}' não catalogado.")

    def start_game(self, game_id: str) -> dict:
        self.game_id = game_id
        self.game = self.create_game_instance(game_id)
        self.paused = False
        self.game_over = False
        self.score = 0
        self.level = 1
        return self.step([], [])

    def step(self, keys: list[str], events: list[dict]) -> dict:
        if not self.game:
            self.surface.fill(PALETTE["bg"])
            txt = self.fonts["md"].render("Nenhum jogo em execução", True, PALETTE["muted"])
            self.surface.blit(txt, (400 - txt.get_width() // 2, 290))
            return self._encode_frame()

        kset = get_keys_set(keys)

        # Checar reinício
        restart_triggered = False
        for ev in events:
            if ev.get("type") == "restart":
                restart_triggered = True

        if "r" in kset or restart_triggered:
            self.game = self.create_game_instance(self.game_id)
            self.game_over = False
            self.paused = False

        # Checar pause
        for ev in events:
            if ev.get("type") == "pause":
                self.paused = not self.paused

        if not self.paused and not getattr(self.game, "game_over", False):
            self.game.update(keys, events)

        # Sincronizar variáveis de estado
        self.score = getattr(self.game, "score", 0)
        self.level = getattr(self.game, "level", 1)
        self.game_over = getattr(self.game, "game_over", False)

        # Renderizar cena
        self.game.draw(self.surface, self.fonts)

        # Overlay de Pausa
        if self.paused and not self.game_over:
            overlay = pygame.Surface((800, 600), pygame.SRCALPHA)
            overlay.fill((9, 12, 20, 190))
            self.surface.blit(overlay, (0, 0))

            box_rect = pygame.Rect(180, 240, 440, 110)
            pygame.draw.rect(self.surface, PALETTE["surface"], box_rect, border_radius=12)
            pygame.draw.rect(self.surface, PALETTE["ice"], box_rect, 2, border_radius=12)

            t1 = self.fonts["lg"].render("JOGO PAUSADO", True, (255, 255, 255))
            t2 = self.fonts["sm"].render("Pressione [P] ou clique em Continuar", True, PALETTE["muted"])
            self.surface.blit(t1, (400 - t1.get_width() // 2, 260))
            self.surface.blit(t2, (400 - t2.get_width() // 2, 310))

        # Overlay de Fim de Jogo
        if self.game_over:
            overlay = pygame.Surface((800, 600), pygame.SRCALPHA)
            overlay.fill((9, 12, 20, 200))
            self.surface.blit(overlay, (0, 0))

            box_rect = pygame.Rect(150, 220, 500, 150)
            pygame.draw.rect(self.surface, PALETTE["surface"], box_rect, border_radius=16)
            pygame.draw.rect(self.surface, PALETTE["coral"], box_rect, 2, border_radius=16)

            t1 = self.fonts["xl"].render("FIM DE JOGO", True, PALETTE["coral"])
            t2 = self.fonts["md"].render(f"Pontuação Final: {self.score}", True, PALETTE["yellow"])
            t3 = self.fonts["sm"].render("Pressione [R] para Jogar Novamente", True, PALETTE["text"])

            self.surface.blit(t1, (400 - t1.get_width() // 2, 240))
            self.surface.blit(t2, (400 - t2.get_width() // 2, 290))
            self.surface.blit(t3, (400 - t3.get_width() // 2, 330))

        return self._encode_frame()

    def toggle_pause(self) -> dict:
        self.paused = not self.paused
        return {"status": "ok", "paused": self.paused}

    def stop_game(self) -> dict:
        last_score = self.score
        last_game = self.game_id
        self.game = None
        self.game_id = None
        self.paused = False
        self.game_over = False
        return {"status": "ok", "last_game": last_game, "last_score": last_score}

    def _encode_frame(self) -> dict:
        buf = io.BytesIO()
        pygame.image.save(self.surface, buf, "JPEG")
        buf.seek(0)
        b64 = base64.b64encode(buf.read()).decode("ascii")
        data_url = f"data:image/jpeg;base64,{b64}"
        return {
            "status": "ok",
            "game_id": self.game_id,
            "frame": data_url,
            "score": self.score,
            "level": self.level,
            "game_over": self.game_over,
            "paused": self.paused
        }

# Instância global reutilizável para o servidor
runner = InWindowGameRunner()
