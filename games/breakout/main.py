"""
Jogo: Breakout Arcade
Autor: Comunidade Pygame (Adaptado para Sandbox Segura Arcade Hub)
Licença: MIT
"""
import random
import pygame
import hub_sdk

class Game:
    def __init__(self, width=800, height=600):
        self.width = width
        self.height = height
        self.reset()

    def reset(self):
        hub_sdk.set_current_game("breakout")
        self.paddle_w = 110
        self.paddle_h = 14
        self.paddle_x = self.width // 2 - self.paddle_w // 2
        self.paddle_y = self.height - 45

        self.ball_r = 8
        self.ball_x = self.width // 2
        self.ball_y = self.height // 2
        self.ball_dx = random.choice([-4.5, 4.5])
        self.ball_dy = -4.5

        # Gerar blocos
        self.rows = 5
        self.cols = 10
        self.block_w = 70
        self.block_h = 22
        self.gap = 8
        total_w = self.cols * self.block_w + (self.cols - 1) * self.gap
        start_x = (self.width - total_w) // 2

        row_colors = [
            (239, 68, 68),   # Vermelho
            (249, 115, 22),  # Laranja
            (234, 179, 8),   # Amarelo
            (52, 211, 153),  # Verde
            (56, 189, 248)   # Ciano
        ]

        self.blocks = []
        for r in range(self.rows):
            for c in range(self.cols):
                bx = start_x + c * (self.block_w + self.gap)
                by = 75 + r * (self.block_h + self.gap)
                rect = pygame.Rect(bx, by, self.block_w, self.block_h)
                self.blocks.append({"rect": rect, "color": row_colors[r % len(row_colors)]})

        self.score = 0
        self.level = 1
        self.game_over = False
        self.victory = False
        self.score_submitted = False

    def update(self, keys, events):
        for event in events:
            if event.type == pygame.KEYDOWN:
                if event.key == pygame.K_r and (self.game_over or self.victory):
                    self.reset()
            elif event.type == pygame.MOUSEMOTION and not self.game_over and not self.victory:
                mx, _ = event.pos
                self.paddle_x = max(10, min(self.width - self.paddle_w - 10, mx - self.paddle_w // 2))

        if self.game_over or self.victory:
            if not self.score_submitted:
                hub_sdk.submit_score(self.score, self.level, "Block Breaker")
                self.score_submitted = True
            return

        # Teclas Setas
        speed = 8
        if (keys[pygame.K_LEFT] or keys[pygame.K_a]) and self.paddle_x > 10:
            self.paddle_x -= speed
        if (keys[pygame.K_RIGHT] or keys[pygame.K_d]) and self.paddle_x < self.width - self.paddle_w - 10:
            self.paddle_x += speed

        # Movimento Bola
        self.ball_x += self.ball_dx
        self.ball_y += self.ball_dy

        # Colisões Laterais
        if self.ball_x <= self.ball_r:
            self.ball_x = self.ball_r
            self.ball_dx *= -1
        elif self.ball_x >= self.width - self.ball_r:
            self.ball_x = self.width - self.ball_r
            self.ball_dx *= -1

        # Colisão Teto
        if self.ball_y <= 55 + self.ball_r:
            self.ball_y = 55 + self.ball_r
            self.ball_dy *= -1

        # Queda (Fim de Jogo)
        if self.ball_y >= self.height:
            self.game_over = True
            return

        # Colisão Raquete
        pad_rect = pygame.Rect(self.paddle_x, self.paddle_y, self.paddle_w, self.paddle_h)
        if pad_rect.collidepoint(self.ball_x, self.ball_y + self.ball_r) and self.ball_dy > 0:
            offset = (self.ball_x - (self.paddle_x + self.paddle_w / 2)) / (self.paddle_w / 2)
            self.ball_dx = offset * 6.5
            self.ball_dy = -abs(self.ball_dy)

        # Colisão Blocos
        ball_rect = pygame.Rect(self.ball_x - self.ball_r, self.ball_y - self.ball_r, self.ball_r * 2, self.ball_r * 2)
        for b in self.blocks[:]:
            if b["rect"].colliderect(ball_rect):
                self.blocks.remove(b)
                self.ball_dy *= -1
                self.score += 50
                if len(self.blocks) == 0:
                    self.victory = True
                break

    def draw(self, surface, font):
        surface.fill((11, 14, 23))

        # Barra Superior
        pygame.draw.rect(surface, (18, 24, 38), (0, 0, self.width, 55))
        pygame.draw.line(surface, (37, 99, 235), (0, 55), (self.width, 55), 2)

        txt_title = font.render("BREAKOUT ARCADE", True, (96, 165, 250))
        surface.blit(txt_title, (20, 16))

        txt_score = font.render(f"PONTUAÇÃO: {self.score}", True, (248, 250, 252))
        surface.blit(txt_score, (270, 16))

        txt_left = font.render(f"BLOCOS: {len(self.blocks)}", True, (52, 211, 153))
        surface.blit(txt_left, (510, 16))

        # Raquete
        pygame.draw.rect(surface, (37, 99, 235), (self.paddle_x, self.paddle_y, self.paddle_w, self.paddle_h), border_radius=5)
        pygame.draw.rect(surface, (96, 165, 250), (self.paddle_x + 6, self.paddle_y + 2, self.paddle_w - 12, 3), border_radius=2)

        # Bola
        pygame.draw.circle(surface, (248, 250, 252), (int(self.ball_x), int(self.ball_y)), self.ball_r)

        # Blocos
        for b in self.blocks:
            pygame.draw.rect(surface, b["color"], b["rect"], border_radius=4)
            pygame.draw.rect(surface, (255, 255, 255), b["rect"], 1, border_radius=4)

        if self.game_over or self.victory:
            overlay = pygame.Surface((self.width, self.height), pygame.SRCALPHA)
            overlay.fill((0, 0, 0, 185))
            surface.blit(overlay, (0, 0))

            msg = "VITÓRIA — TODOS OS BLOCOS DESTRUÍDOS!" if self.victory else "FIM DE JOGO — A ESFERA CAIU!"
            col = (52, 211, 153) if self.victory else (239, 68, 68)
            txt_end = font.render(msg, True, col)
            txt_restart = font.render("Pressione [R] para tentar novamente ou [ESC] para sair", True, (248, 250, 252))
            surface.blit(txt_end, (self.width // 2 - txt_end.get_width() // 2, self.height // 2 - 30))
            surface.blit(txt_restart, (self.width // 2 - txt_restart.get_width() // 2, self.height // 2 + 10))


if __name__ == "__main__":
    pygame.init()
    width, height = 800, 600
    screen = pygame.display.set_mode((width, height))
    pygame.display.set_caption("Breakout Arcade — Arcade Hub")
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
