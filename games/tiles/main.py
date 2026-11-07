"""
Jogo: Quebra-Cabeça 15
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
        self.grid_size = 4
        self.tile_dim = 100
        self.gap = 8
        board_total_w = self.grid_size * self.tile_dim + (self.grid_size - 1) * self.gap
        self.board_x = (width - board_total_w) // 2
        self.board_y = 100
        self.reset()

    def reset(self):
        hub_sdk.set_current_game("tiles")
        # Estado resolvido
        self.board = [[r * 4 + c + 1 for c in range(4)] for r in range(4)]
        self.board[3][3] = 0  # 0 é o espaço vazio
        self.empty_pos = (3, 3)

        # Embaralhamento garantidamente solucionável
        for _ in range(120):
            er, ec = self.empty_pos
            neighbors = []
            for dr, dc in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
                nr, nc = er + dr, ec + dc
                if 0 <= nr < 4 and 0 <= nc < 4:
                    neighbors.append((nr, nc))
            sr, sc = random.choice(neighbors)
            self.board[er][ec], self.board[sr][sc] = self.board[sr][sc], self.board[er][ec]
            self.empty_pos = (sr, sc)

        self.moves = 0
        self.score = 0
        self.level = 1
        self.victory = False
        self.game_over = False
        self.score_submitted = False

    def is_solved(self):
        expected = 1
        for r in range(4):
            for c in range(4):
                if r == 3 and c == 3:
                    return self.board[r][c] == 0
                if self.board[r][c] != expected:
                    return False
                expected += 1
        return True

    def move_tile(self, r, c):
        er, ec = self.empty_pos
        if (abs(r - er) == 1 and c == ec) or (abs(c - ec) == 1 and r == er):
            self.board[er][ec], self.board[r][c] = self.board[r][c], self.board[er][ec]
            self.empty_pos = (r, c)
            self.moves += 1
            if self.is_solved():
                self.victory = True
                self.game_over = True
                self.score = max(100, 2000 - self.moves * 15)

    def update(self, keys, events):
        for event in events:
            if event.type == pygame.KEYDOWN:
                if event.key == pygame.K_r:
                    self.reset()
                elif not self.game_over:
                    er, ec = self.empty_pos
                    if event.key == pygame.K_UP and er < 3:
                        self.move_tile(er + 1, ec)
                    elif event.key == pygame.K_DOWN and er > 0:
                        self.move_tile(er - 1, ec)
                    elif event.key == pygame.K_LEFT and ec < 3:
                        self.move_tile(er, ec + 1)
                    elif event.key == pygame.K_RIGHT and ec > 0:
                        self.move_tile(er, ec - 1)
            elif event.type == pygame.MOUSEBUTTONDOWN and not self.game_over:
                if event.button == 1:
                    mx, my = event.pos
                    # Checar se clicou em um bloco
                    for r in range(4):
                        for c in range(4):
                            tx = self.board_x + c * (self.tile_dim + self.gap)
                            ty = self.board_y + r * (self.tile_dim + self.gap)
                            rect = pygame.Rect(tx, ty, self.tile_dim, self.tile_dim)
                            if rect.collidepoint(mx, my):
                                self.move_tile(r, c)
                                break

        if self.game_over and self.victory and not self.score_submitted:
            hub_sdk.submit_score(self.score, self.level, "Mestre dos Blocos")
            self.score_submitted = True

    def draw(self, surface, font):
        surface.fill((11, 14, 23))

        # Barra Superior
        pygame.draw.rect(surface, (18, 24, 38), (0, 0, self.width, 60))
        pygame.draw.line(surface, (37, 99, 235), (0, 60), (self.width, 60), 2)

        txt_title = font.render("QUEBRA-CABEÇA 15", True, (96, 165, 250))
        surface.blit(txt_title, (20, 18))

        txt_moves = font.render(f"MOVIMENTOS: {self.moves}", True, (248, 250, 252))
        surface.blit(txt_moves, (270, 18))

        txt_inst = font.render("Clique ou Setas para mover • [R] Embaralhar", True, (148, 163, 184))
        surface.blit(txt_inst, (450, 18))

        # Fundo do Tabuleiro
        board_w = 4 * self.tile_dim + 3 * self.gap + 16
        pygame.draw.rect(surface, (15, 23, 42), (self.board_x - 8, self.board_y - 8, board_w, board_w), border_radius=12)
        pygame.draw.rect(surface, (30, 41, 59), (self.board_x - 8, self.board_y - 8, board_w, board_w), 2, border_radius=12)

        # Desenhar Blocos
        for r in range(4):
            for c in range(4):
                val = self.board[r][c]
                tx = self.board_x + c * (self.tile_dim + self.gap)
                ty = self.board_y + r * (self.tile_dim + self.gap)
                if val != 0:
                    rect = pygame.Rect(tx, ty, self.tile_dim, self.tile_dim)
                    # Cor gradiente
                    pygame.draw.rect(surface, (30, 58, 138), rect, border_radius=8)
                    pygame.draw.rect(surface, (59, 130, 246), rect, 2, border_radius=8)

                    txt_num = font.render(str(val), True, (248, 250, 252))
                    surface.blit(txt_num, (tx + self.tile_dim // 2 - txt_num.get_width() // 2, ty + self.tile_dim // 2 - txt_num.get_height() // 2))
                else:
                    # Espaço Vazio
                    rect = pygame.Rect(tx, ty, self.tile_dim, self.tile_dim)
                    pygame.draw.rect(surface, (18, 24, 38), rect, border_radius=8)

        if self.victory:
            overlay = pygame.Surface((self.width, self.height), pygame.SRCALPHA)
            overlay.fill((0, 0, 0, 190))
            surface.blit(overlay, (0, 0))

            txt_vic = font.render("PARABÉNS! QUEBRA-CABEÇA RESOLVIDO!", True, (52, 211, 153))
            txt_stat = font.render(f"Completado em {self.moves} movimentos! Pontos: {self.score}", True, (248, 250, 252))
            txt_restart = font.render("Pressione [R] para jogar novamente ou [ESC] para sair", True, (148, 163, 184))
            surface.blit(txt_vic, (self.width // 2 - txt_vic.get_width() // 2, self.height // 2 - 40))
            surface.blit(txt_stat, (self.width // 2 - txt_stat.get_width() // 2, self.height // 2))
            surface.blit(txt_restart, (self.width // 2 - txt_restart.get_width() // 2, self.height // 2 + 35))


if __name__ == "__main__":
    pygame.init()
    width, height = 800, 600
    screen = pygame.display.set_mode((width, height))
    pygame.display.set_caption("Quebra-Cabeça 15 — Arcade Hub")
    clock = pygame.time.Clock()
    font = pygame.font.SysFont("Arial", 22, bold=True)
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
