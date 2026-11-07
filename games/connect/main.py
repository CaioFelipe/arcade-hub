"""
Jogo: Conecte Quatro
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
        self.cols = 7
        self.rows = 6
        self.cell_size = 72
        self.board_w = self.cols * self.cell_size
        self.board_h = self.rows * self.cell_size
        self.offset_x = (width - self.board_w) // 2
        self.offset_y = 110
        self.reset()

    def reset(self):
        hub_sdk.set_current_game("connect")
        # 0 = vazio, 1 = jogador 1 (Amarelo), 2 = jogador 2 / IA (Vermelho)
        self.board = [[0 for _ in range(self.cols)] for _ in range(self.rows)]
        self.turn = 1
        self.winner = 0
        self.winning_cells = []
        self.game_over = False
        self.score = 0
        self.level = 1
        self.score_submitted = False
        self.hover_col = 3

    def drop_disc(self, col):
        if not (0 <= col < self.cols):
            return False
        # Procurar linha mais baixa livre
        target_r = -1
        for r in range(self.rows - 1, -1, -1):
            if self.board[r][col] == 0:
                target_r = r
                break
        if target_r == -1:
            return False

        self.board[target_r][col] = self.turn
        if self.check_win(target_r, col, self.turn):
            self.game_over = True
            self.winner = self.turn
            if self.winner == 1:
                self.score = 1000
        elif all(self.board[0][c] != 0 for c in range(self.cols)):
            self.game_over = True
            self.winner = -1  # Empate
        else:
            self.turn = 2 if self.turn == 1 else 1
            if self.turn == 2 and not self.game_over:
                self.play_ai_turn()
        return True

    def play_ai_turn(self):
        # IA: 1) Ganhar se puder, 2) Bloquear vitória do P1, 3) Coluna aleatória válida
        valid_cols = [c for c in range(self.cols) if self.board[0][c] == 0]
        if not valid_cols:
            return

        chosen_col = None
        # Checar se IA ganha
        for c in valid_cols:
            r = max(row for row in range(self.rows) if self.board[row][c] == 0)
            self.board[r][c] = 2
            if self.check_win(r, c, 2):
                chosen_col = c
                self.board[r][c] = 0
                break
            self.board[r][c] = 0

        # Checar se precisa bloquear P1
        if chosen_col is None:
            for c in valid_cols:
                r = max(row for row in range(self.rows) if self.board[row][c] == 0)
                self.board[r][c] = 1
                if self.check_win(r, c, 1):
                    chosen_col = c
                    self.board[r][c] = 0
                    break
                self.board[r][c] = 0

        if chosen_col is None:
            # Preferir colunas centrais
            center_order = [3, 2, 4, 1, 5, 0, 6]
            for c in center_order:
                if c in valid_cols:
                    chosen_col = c
                    break

        r = max(row for row in range(self.rows) if self.board[row][chosen_col] == 0)
        self.board[r][chosen_col] = 2
        if self.check_win(r, chosen_col, 2):
            self.game_over = True
            self.winner = 2
        elif all(self.board[0][c] != 0 for c in range(self.cols)):
            self.game_over = True
            self.winner = -1
        else:
            self.turn = 1

    def check_win(self, r, c, player):
        directions = [(0, 1), (1, 0), (1, 1), (1, -1)]
        for dr, dc in directions:
            cells = [(r, c)]
            # Direção positiva
            cr, cc = r + dr, c + dc
            while 0 <= cr < self.rows and 0 <= cc < self.cols and self.board[cr][cc] == player:
                cells.append((cr, cc))
                cr += dr
                cc += dc
            # Direção negativa
            cr, cc = r - dr, c - dc
            while 0 <= cr < self.rows and 0 <= cc < self.cols and self.board[cr][cc] == player:
                cells.append((cr, cc))
                cr -= dr
                cc -= dc
            if len(cells) >= 4:
                self.winning_cells = cells
                return True
        return False

    def update(self, keys, events):
        for event in events:
            if event.type == pygame.KEYDOWN:
                if event.key == pygame.K_r:
                    self.reset()
                elif not self.game_over and self.turn == 1:
                    if pygame.K_1 <= event.key <= pygame.K_7:
                        self.drop_disc(event.key - pygame.K_1)
            elif event.type == pygame.MOUSEMOTION:
                mx, my = event.pos
                col = (mx - self.offset_x) // self.cell_size
                if 0 <= col < self.cols:
                    self.hover_col = col
            elif event.type == pygame.MOUSEBUTTONDOWN and not self.game_over:
                if event.button == 1 and self.turn == 1:
                    mx, my = event.pos
                    col = (mx - self.offset_x) // self.cell_size
                    if 0 <= col < self.cols:
                        self.drop_disc(col)

        if self.game_over and self.winner == 1 and not self.score_submitted:
            hub_sdk.submit_score(self.score, self.level, "Estrategista 4")
            self.score_submitted = True

    def draw(self, surface, font):
        surface.fill((11, 14, 23))

        # Barra Superior
        pygame.draw.rect(surface, (18, 24, 38), (0, 0, self.width, 60))
        pygame.draw.line(surface, (37, 99, 235), (0, 60), (self.width, 60), 2)

        txt_title = font.render("CONECTE QUATRO", True, (96, 165, 250))
        surface.blit(txt_title, (20, 18))

        turn_txt = "SEU TURNO (AMARELO)" if self.turn == 1 else "TURNO DA CPU (VERMELHO)"
        turn_col = (250, 204, 21) if self.turn == 1 else (239, 68, 68)
        txt_turn = font.render(turn_txt, True, turn_col)
        surface.blit(txt_turn, (260, 18))

        txt_inst = font.render("Clique na coluna ou [1..7] • [R] Reiniciar", True, (148, 163, 184))
        surface.blit(txt_inst, (520, 18))

        # Indicador de Coluna no Topo
        if not self.game_over and self.turn == 1:
            hx = self.offset_x + self.hover_col * self.cell_size + self.cell_size // 2
            pygame.draw.circle(surface, (250, 204, 21), (hx, self.offset_y - 25), 18)

        # Grade Azul do Tabuleiro
        board_rect = pygame.Rect(self.offset_x - 10, self.offset_y - 10, self.board_w + 20, self.board_h + 20)
        pygame.draw.rect(surface, (30, 58, 138), board_rect, border_radius=12)
        pygame.draw.rect(surface, (59, 130, 246), board_rect, 2, border_radius=12)

        # Células / Fichas
        for r in range(self.rows):
            for c in range(self.cols):
                cx = self.offset_x + c * self.cell_size + self.cell_size // 2
                cy = self.offset_y + r * self.cell_size + self.cell_size // 2
                val = self.board[r][c]

                color = (15, 23, 42)
                if val == 1:
                    color = (250, 204, 21)
                elif val == 2:
                    color = (239, 68, 68)

                pygame.draw.circle(surface, color, (cx, cy), self.cell_size // 2 - 6)

                # Destaque de vitória
                if (r, c) in self.winning_cells:
                    pygame.draw.circle(surface, (52, 211, 153), (cx, cy), self.cell_size // 2 - 4, 3)

        if self.game_over:
            overlay = pygame.Surface((self.width, self.height), pygame.SRCALPHA)
            overlay.fill((0, 0, 0, 185))
            surface.blit(overlay, (0, 0))

            if self.winner == 1:
                msg = "PARABÉNS! VOCÊ CONECTOU QUATRO!"
                col = (52, 211, 153)
            elif self.winner == 2:
                msg = "A CPU CONECTOU QUATRO!"
                col = (239, 68, 68)
            else:
                msg = "EMPATE NO TABULEIRO!"
                col = (250, 204, 21)

            txt_end = font.render(msg, True, col)
            txt_restart = font.render("Pressione [R] para jogar novamente ou [ESC] para sair", True, (248, 250, 252))
            surface.blit(txt_end, (self.width // 2 - txt_end.get_width() // 2, self.height // 2 - 30))
            surface.blit(txt_restart, (self.width // 2 - txt_restart.get_width() // 2, self.height // 2 + 10))


if __name__ == "__main__":
    pygame.init()
    width, height = 800, 600
    screen = pygame.display.set_mode((width, height))
    pygame.display.set_caption("Conecte Quatro — Arcade Hub")
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
