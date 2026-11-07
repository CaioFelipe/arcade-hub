"""
Jogo: Snake Arcade
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
        self.cell_size = 20
        self.grid_w = width // self.cell_size
        self.grid_h = (height - 60) // self.cell_size  # 60px reservado para topo
        self.offset_y = 60
        self.reset()

    def reset(self):
        hub_sdk.set_current_game("snake")
        self.snake = [
            (self.grid_w // 2, self.grid_h // 2),
            (self.grid_w // 2 - 1, self.grid_h // 2),
            (self.grid_w // 2 - 2, self.grid_h // 2)
        ]
        self.direction = (1, 0)
        self.next_direction = (1, 0)
        self.score = 0
        self.level = 1
        self.game_over = False
        self.score_submitted = False
        self.tick_timer = 0
        self.speed = 8  # frames por passo
        self.spawn_food()

    def spawn_food(self):
        while True:
            fx = random.randint(0, self.grid_w - 1)
            fy = random.randint(0, self.grid_h - 1)
            if (fx, fy) not in self.snake:
                self.food = (fx, fy)
                break

    def update(self, keys, events):
        for event in events:
            if event.type == pygame.KEYDOWN:
                if event.key == pygame.K_r and self.game_over:
                    self.reset()
                elif not self.game_over:
                    if (event.key == pygame.K_UP or event.key == pygame.K_w) and self.direction != (0, 1):
                        self.next_direction = (0, -1)
                    elif (event.key == pygame.K_DOWN or event.key == pygame.K_s) and self.direction != (0, -1):
                        self.next_direction = (0, 1)
                    elif (event.key == pygame.K_LEFT or event.key == pygame.K_a) and self.direction != (1, 0):
                        self.next_direction = (-1, 0)
                    elif (event.key == pygame.K_RIGHT or event.key == pygame.K_d) and self.direction != (-1, 0):
                        self.next_direction = (1, 0)

        if self.game_over:
            if not self.score_submitted:
                hub_sdk.submit_score(self.score, self.level, "Jogador Snake")
                self.score_submitted = True
            return

        self.tick_timer += 1
        if self.tick_timer >= self.speed:
            self.tick_timer = 0
            self.direction = self.next_direction

            head_x, head_y = self.snake[0]
            dx, dy = self.direction
            new_head = (head_x + dx, head_y + dy)

            # Colisão com parede
            if new_head[0] < 0 or new_head[0] >= self.grid_w or new_head[1] < 0 or new_head[1] >= self.grid_h:
                self.game_over = True
                return

            # Colisão consigo mesma
            if new_head in self.snake:
                self.game_over = True
                return

            self.snake.insert(0, new_head)

            # Comer alimento
            if new_head == self.food:
                self.score += 10
                self.level = 1 + self.score // 50
                self.speed = max(3, 8 - (self.level // 2))
                self.spawn_food()
            else:
                self.snake.pop()

    def draw(self, surface, font):
        surface.fill((11, 14, 23))

        # Barra Superior
        pygame.draw.rect(surface, (18, 24, 38), (0, 0, self.width, self.offset_y))
        pygame.draw.line(surface, (37, 99, 235), (0, self.offset_y), (self.width, self.offset_y), 2)

        txt_title = font.render("SNAKE ARCADE", True, (96, 165, 250))
        surface.blit(txt_title, (20, 18))

        txt_score = font.render(f"PONTUAÇÃO: {self.score}", True, (248, 250, 252))
        surface.blit(txt_score, (260, 18))

        txt_lvl = font.render(f"NÍVEL: {self.level}", True, (52, 211, 153))
        surface.blit(txt_lvl, (500, 18))

        # Grid sutil
        for gx in range(0, self.width, self.cell_size):
            pygame.draw.line(surface, (17, 24, 39), (gx, self.offset_y), (gx, self.height))
        for gy in range(self.offset_y, self.height, self.cell_size):
            pygame.draw.line(surface, (17, 24, 39), (0, gy), (self.width, gy))

        # Alimento (Maçã)
        fx, fy = self.food
        food_rect = pygame.Rect(fx * self.cell_size + 2, self.offset_y + fy * self.cell_size + 2, self.cell_size - 4, self.cell_size - 4)
        pygame.draw.rect(surface, (239, 68, 68), food_rect, border_radius=4)
        pygame.draw.circle(surface, (254, 202, 202), (food_rect.centerx - 2, food_rect.centery - 2), 2)

        # Cobra
        for idx, (sx, sy) in enumerate(self.snake):
            seg_rect = pygame.Rect(sx * self.cell_size + 1, self.offset_y + sy * self.cell_size + 1, self.cell_size - 2, self.cell_size - 2)
            if idx == 0:
                pygame.draw.rect(surface, (16, 185, 129), seg_rect, border_radius=6)
                # Olhos
                pygame.draw.circle(surface, (11, 14, 23), (seg_rect.centerx - 3, seg_rect.centery - 3), 2)
                pygame.draw.circle(surface, (11, 14, 23), (seg_rect.centerx + 3, seg_rect.centery - 3), 2)
            else:
                color = (5, 150, 105) if idx % 2 == 0 else (16, 185, 129)
                pygame.draw.rect(surface, color, seg_rect, border_radius=4)

        if self.game_over:
            overlay = pygame.Surface((self.width, self.height), pygame.SRCALPHA)
            overlay.fill((0, 0, 0, 180))
            surface.blit(overlay, (0, 0))

            txt_go = font.render("FIM DE JOGO", True, (239, 68, 68))
            txt_restart = font.render("Pressione [R] para jogar novamente ou [ESC] para sair", True, (248, 250, 252))
            surface.blit(txt_go, (self.width // 2 - txt_go.get_width() // 2, self.height // 2 - 30))
            surface.blit(txt_restart, (self.width // 2 - txt_restart.get_width() // 2, self.height // 2 + 10))


if __name__ == "__main__":
    pygame.init()
    width, height = 800, 600
    screen = pygame.display.set_mode((width, height))
    pygame.display.set_caption("Snake Arcade — Arcade Hub")
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
