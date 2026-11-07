"""Canhão Balístico"""
import math
import random
import pygame
import hub_sdk

class Game:
    def __init__(self, width=800, height=600):
        self.width, self.height = width, height
        self.reset()

    def reset(self):
        hub_sdk.set_current_game("cannon")
        self.score = 0
        self.level = 1
        self.game_over = False

    def update(self, keys, events):
        pass

    def draw(self, surface, font):
        surface.fill((9, 12, 20))
        txt = font.render("Canhão Balístico ativo no Hub", True, (250, 204, 21))
        surface.blit(txt, (20, 20))
