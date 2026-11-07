"""
Jogo: Campo Minado
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
        self.cols = 16
        self.rows = 12
        self.num_mines = 22
        self.cell_size = 36
        self.board_w = self.cols * self.cell_size
        self.board_h = self.rows * self.cell_size
        self.offset_x = (width - self.board_w) // 2
        self.offset_y = 100
        self.reset()

    def reset(self):
        hub_sdk.set_current_game("minesweeper")
        # -1 = mina, 0..8 = contagem
        self.grid = [[0 for _ in range(self.cols)] for _ in range(self.rows)]
        # False = coberto, True = revelado
        self.revealed = [[False for _ in range(self.cols)] for _ in range(self.rows)]
        # False = sem bandeira, True = com bandeira
        self.flagged = [[False for _ in range(self.cols)] for _ in range(self.rows)]

        self.mines_placed = False
        self.game_over = False
        self.victory = False
        self.score = 0
        self.level = 1
        self.score_submitted = False
        self.flags_left = self.num_mines

    def place_mines(self, safe_r, safe_c):
        all_cells = [(r, c) for r in range(self.rows) for c in range(self.cols) if (r, c) != (safe_r, safe_c)]
        mine_cells = random.sample(all_cells, self.num_mines)
        for r, c in mine_cells:
            self.grid[r][c] = -1

        # Calcular contagens
        for r in range(self.rows):
            for c in range(self.cols):
                if self.grid[r][c] == -1:
                    continue
                cnt = 0
                for dr in [-1, 0, 1]:
                    for dc in [-1, 0, 1]:
                        nr, nc = r + dr, c + dc
                        if 0 <= nr < self.rows and 0 <= nc < self.cols:
                            if self.grid[nr][nc] == -1:
                                cnt += 1
                self.grid[r][c] = cnt
        self.mines_placed = True

    def reveal(self, r, c):
        if not (0 <= r < self.rows and 0 <= c < self.cols):
            return
        if self.revealed[r][c] or self.flagged[r][c]:
            return

        if not self.mines_placed:
            self.place_mines(r, c)

        self.revealed[r][c] = True

        if self.grid[r][c] == -1:
            self.game_over = True
            self.victory = False
            return

        if self.grid[r][c] == 0:
            for dr in [-1, 0, 1]:
                for dc in [-1, 0, 1]:
                    self.reveal(r + dr, c + dc)

        # Checar vitória
        unrevealed_safe = 0
        for ro in range(self.rows):
            for co in range(self.cols):
                if self.grid[ro][co] != -1 and not self.revealed[ro][co]:
                    unrevealed_safe += 1
        if unrevealed_safe == 0:
            self.victory = True
            self.game_over = True
            self.score = 1500

    def toggle_flag(self, r, c):
        if not (0 <= r < self.rows and 0 <= c < self.cols):
            return
        if self.revealed[r][c]:
            return
        if self.flagged[r][c]:
            self.flagged[r][c] = False
            self.flags_left += 1
        elif self.flags_left > 0:
            self.flagged[r][c] = True
            self.flags_left -= 1

    def update(self, keys, events):
        for event in events:
            if event.type == pygame.KEYDOWN:
                if event.key == pygame.K_r:
                    self.reset()
            elif event.type == pygame.MOUSEBUTTONDOWN and not self.game_over:
                mx, my = event.pos
                c = (mx - self.offset_x) // self.cell_size
                r = (my - self.offset_y) // self.cell_size
                if 0 <= r < self.rows and 0 <= c < self.cols:
                    if event.button == 1:  # Botão Esquerdo
                        self.reveal(r, c)
                    elif event.button == 3:  # Botão Direito
                        self.toggle_flag(r, c)

        if self.game_over and self.victory and not self.score_submitted:
            hub_sdk.submit_score(self.score, self.level, "Varredor de Minas")
            self.score_submitted = True

    def draw(self, surface, font):
        surface.fill((11, 14, 23))

        # Barra Superior
        pygame.draw.rect(surface, (18, 24, 38), (0, 0, self.width, 60))
        pygame.draw.line(surface, (37, 99, 235), (0, 60), (self.width, 60), 2)

        txt_title = font.render("CAMPO MINADO", True, (96, 165, 250))
        surface.blit(txt_title, (20, 18))

        txt_flags = font.render(f"MINAS / BANDEIRAS: {self.flags_left}", True, (244, 63, 94))
        surface.blit(txt_flags, (270, 18))

        txt_inst = font.render("Botão Esq: Revelar • Botão Dir: Bandeira • [R] Reiniciar", True, (148, 163, 184))
        surface.blit(txt_inst, (460, 18))

        # Desenhar Células
        num_colors = {
            1: (59, 130, 246),
            2: (34, 197, 94),
            3: (239, 68, 68),
            4: (168, 85, 247),
            5: (234, 179, 8),
            6: (6, 182, 212),
            7: (244, 114, 182),
            8: (148, 163, 184)
        }

        for r in range(self.rows):
            for c in range(self.cols):
                cx = self.offset_x + c * self.cell_size
                cy = self.offset_y + r * self.cell_size
                rect = pygame.Rect(cx, cy, self.cell_size, self.cell_size)

                if self.revealed[r][c]:
                    pygame.draw.rect(surface, (15, 23, 42), rect)
                    pygame.draw.rect(surface, (30, 41, 59), rect, 1)

                    val = self.grid[r][c]
                    if val == -1:
                        # Mina
                        pygame.draw.circle(surface, (239, 68, 68), rect.center, self.cell_size // 3)
                        pygame.draw.circle(surface, (248, 250, 252), rect.center, 3)
                    elif val > 0:
                        txt_num = font.render(str(val), True, num_colors.get(val, (248, 250, 252)))
                        surface.blit(txt_num, (rect.centerx - txt_num.get_width() // 2, rect.centery - txt_num.get_height() // 2))
                else:
                    # Coberto
                    pygame.draw.rect(surface, (30, 41, 59), rect, border_radius=2)
                    pygame.draw.rect(surface, (51, 65, 85), rect, 1, border_radius=2)

                    if self.flagged[r][c]:
                        # Bandeira
                        pygame.draw.polygon(surface, (239, 68, 68), [
                            (rect.centerx - 4, rect.centery - 8),
                            (rect.centerx + 8, rect.centery - 3),
                            (rect.centerx - 4, rect.centery + 2)
                        ])
                        pygame.draw.line(surface, (248, 250, 252), (rect.centerx - 4, rect.centery - 10), (rect.centerx - 4, rect.centery + 8), 2)

        if self.game_over:
            overlay = pygame.Surface((self.width, self.height), pygame.SRCALPHA)
            overlay.fill((0, 0, 0, 185))
            surface.blit(overlay, (0, 0))

            msg = "VITÓRIA — TODAS AS MINAS NEUTRALIZADAS!" if self.victory else "BOOM! VOCÊ ACIONOU UMA MINA!"
            col = (52, 211, 153) if self.victory else (239, 68, 68)
            txt_end = font.render(msg, True, col)
            txt_restart = font.render("Pressione [R] para jogar novamente ou [ESC] para sair", True, (248, 250, 252))
            surface.blit(txt_end, (self.width // 2 - txt_end.get_width() // 2, self.height // 2 - 30))
            surface.blit(txt_restart, (self.width // 2 - txt_restart.get_width() // 2, self.height // 2 + 10))


if __name__ == "__main__":
    pygame.init()
    width, height = 800, 600
    screen = pygame.display.set_mode((width, height))
    pygame.display.set_caption("Campo Minado — Arcade Hub")
    clock = pygame.time.Clock()
    font = pygame.font.SysFont("Arial", 16, bold=True)
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
