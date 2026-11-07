"""
Jogo: Tron Light Cycles
Autor Original: Grant Jenks (Adaptado para Pygame e Sandbox Segura Arcade Hub)
Licença: Apache 2.0
"""
import random
import pygame
import hub_sdk

class Game:
    def __init__(self, width=800, height=600):
        self.width = width
        self.height = height
        self.offset_y = 60
        self.play_w = width
        self.play_h = height - self.offset_y
        self.reset()

    def reset(self):
        hub_sdk.set_current_game("tron")
        # Moto 1 (Ciano)
        self.p1_x = 150
        self.p1_y = self.offset_y + self.play_h // 2
        self.p1_dir = (1, 0)
        self.p1_trail = set()
        self.p1_trail_list = [(self.p1_x, self.p1_y)]
        self.p1_trail.add((self.p1_x, self.p1_y))

        # Moto 2 (Laranja / IA)
        self.p2_x = self.width - 150
        self.p2_y = self.offset_y + self.play_h // 2
        self.p2_dir = (-1, 0)
        self.p2_trail = set()
        self.p2_trail_list = [(self.p2_x, self.p2_y)]
        self.p2_trail.add((self.p2_x, self.p2_y))

        self.speed = 3
        self.score1 = 0
        self.score2 = 0
        self.score = 0
        self.level = 1
        self.game_over = False
        self.winner = None
        self.score_submitted = False

    def is_safe_move(self, x, y):
        if x < 10 or x >= self.width - 10 or y < self.offset_y + 10 or y >= self.height - 10:
            return False
        if (x, y) in self.p1_trail or (x, y) in self.p2_trail:
            return False
        return True

    def update(self, keys, events):
        for event in events:
            if event.type == pygame.KEYDOWN:
                if event.key == pygame.K_r and self.game_over:
                    self.reset()
                elif not self.game_over:
                    # Controles Jogador 1 (W/A/S/D)
                    if event.key == pygame.K_w and self.p1_dir != (0, 1):
                        self.p1_dir = (0, -1)
                    elif event.key == pygame.K_s and self.p1_dir != (0, -1):
                        self.p1_dir = (0, 1)
                    elif event.key == pygame.K_a and self.p1_dir != (1, 0):
                        self.p1_dir = (-1, 0)
                    elif event.key == pygame.K_d and self.p1_dir != (-1, 0):
                        self.p1_dir = (1, 0)

                    # Controles Jogador 2 (Setas)
                    if event.key == pygame.K_UP and self.p2_dir != (0, 1):
                        self.p2_dir = (0, -1)
                    elif event.key == pygame.K_DOWN and self.p2_dir != (0, -1):
                        self.p2_dir = (0, 1)
                    elif event.key == pygame.K_LEFT and self.p2_dir != (1, 0):
                        self.p2_dir = (-1, 0)
                    elif event.key == pygame.K_RIGHT and self.p2_dir != (-1, 0):
                        self.p2_dir = (1, 0)

        if self.game_over:
            if not self.score_submitted:
                self.score = self.score1 * 250
                hub_sdk.submit_score(self.score, self.level, "Piloto Tron")
                self.score_submitted = True
            return

        # IA suave para P2 caso não haja input do teclado
        if not (keys[pygame.K_UP] or keys[pygame.K_DOWN] or keys[pygame.K_LEFT] or keys[pygame.K_RIGHT]):
            next_p2_x = self.p2_x + self.p2_dir[0] * self.speed * 4
            next_p2_y = self.p2_y + self.p2_dir[1] * self.speed * 4
            if not self.is_safe_move(next_p2_x, next_p2_y):
                possible = [(0, 1), (0, -1), (1, 0), (-1, 0)]
                safe_dirs = []
                for d in possible:
                    if d[0] == -self.p2_dir[0] and d[1] == -self.p2_dir[1]:
                        continue
                    tx = self.p2_x + d[0] * self.speed * 4
                    ty = self.p2_y + d[1] * self.speed * 4
                    if self.is_safe_move(tx, ty):
                        safe_dirs.append(d)
                if safe_dirs:
                    self.p2_dir = random.choice(safe_dirs)

        # Mover P1
        for _ in range(self.speed):
            self.p1_x += self.p1_dir[0]
            self.p1_y += self.p1_dir[1]

            # Verificar colisão P1
            if not self.is_safe_move(self.p1_x, self.p1_y):
                self.game_over = True
                self.winner = "Jogador 2 / IA"
                self.score2 += 1
                break
            self.p1_trail.add((self.p1_x, self.p1_y))
            self.p1_trail_list.append((self.p1_x, self.p1_y))

        # Mover P2
        if not self.game_over:
            for _ in range(self.speed):
                self.p2_x += self.p2_dir[0]
                self.p2_y += self.p2_dir[1]

                # Verificar colisão P2
                if not self.is_safe_move(self.p2_x, self.p2_y):
                    self.game_over = True
                    self.winner = "Jogador 1"
                    self.score1 += 1
                    break
                self.p2_trail.add((self.p2_x, self.p2_y))
                self.p2_trail_list.append((self.p2_x, self.p2_y))

    def draw(self, surface, font):
        surface.fill((8, 12, 20))

        # Grid Cibernético
        grid_s = 40
        for gx in range(0, self.width, grid_s):
            pygame.draw.line(surface, (14, 20, 32), (gx, self.offset_y), (gx, self.height))
        for gy in range(self.offset_y, self.height, grid_s):
            pygame.draw.line(surface, (14, 20, 32), (0, gy), (self.width, gy))

        # Bordas da Arena Neon
        arena_rect = pygame.Rect(8, self.offset_y + 8, self.width - 16, self.play_h - 16)
        pygame.draw.rect(surface, (30, 41, 59), arena_rect, 2)

        # Rastros P1 (Ciano)
        if len(self.p1_trail_list) > 1:
            pygame.draw.lines(surface, (6, 182, 212), False, self.p1_trail_list, 3)

        # Rastros P2 (Laranja)
        if len(self.p2_trail_list) > 1:
            pygame.draw.lines(surface, (249, 115, 22), False, self.p2_trail_list, 3)

        # Cabeça das Motos
        pygame.draw.circle(surface, (165, 243, 252), (self.p1_x, self.p1_y), 5)
        pygame.draw.circle(surface, (254, 215, 170), (self.p2_x, self.p2_y), 5)

        # Barra Superior
        pygame.draw.rect(surface, (18, 24, 38), (0, 0, self.width, self.offset_y))
        pygame.draw.line(surface, (37, 99, 235), (0, self.offset_y), (self.width, self.offset_y), 2)

        txt_title = font.render("TRON LIGHT CYCLES", True, (6, 182, 212))
        surface.blit(txt_title, (20, 18))

        txt_p1 = font.render(f"P1 (W/A/S/D): {self.score1}", True, (6, 182, 212))
        surface.blit(txt_p1, (270, 18))

        txt_p2 = font.render(f"P2 (Setas/IA): {self.score2}", True, (249, 115, 22))
        surface.blit(txt_p2, (500, 18))

        if self.game_over:
            overlay = pygame.Surface((self.width, self.height), pygame.SRCALPHA)
            overlay.fill((0, 0, 0, 180))
            surface.blit(overlay, (0, 0))

            txt_win = font.render(f"COLISÃO! VENCEDOR: {self.winner.upper()}!", True, (250, 204, 21))
            txt_restart = font.render("Pressione [R] para revanche ou [ESC] para sair", True, (248, 250, 252))
            surface.blit(txt_win, (self.width // 2 - txt_win.get_width() // 2, self.height // 2 - 30))
            surface.blit(txt_restart, (self.width // 2 - txt_restart.get_width() // 2, self.height // 2 + 10))


if __name__ == "__main__":
    pygame.init()
    width, height = 800, 600
    screen = pygame.display.set_mode((width, height))
    pygame.display.set_caption("Tron Light Cycles — Arcade Hub")
    clock = pygame.time.Clock()
    font = pygame.font.SysFont("Arial", 18, bold=True)
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
