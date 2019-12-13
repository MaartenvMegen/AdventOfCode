from enum import Enum
from collections import defaultdict


class ObjectDetails(Enum):
    EMPTY = 0
    WALL = 1
    BLOCK = 2
    HORIZONTAL_PADDLE = 3
    BALL = 4


render_info = {ObjectDetails.EMPTY: " ", ObjectDetails.WALL: "#", ObjectDetails.BLOCK: "*",
               ObjectDetails.HORIZONTAL_PADDLE: "-", ObjectDetails.BALL: "o"}


class Screen:
    def __init__(self):
        self.screen_info = defaultdict(lambda: defaultdict(lambda: ObjectDetails))

    def get_blocks(self):
        blocks = 0
        for y, line_info in self.screen_info.items():
            for x, details in line_info.items():
                if details == details.BLOCK:
                    blocks += 1
        print("found {} blocks".format(blocks))

    def display(self, x, y, details):
        if self.screen_info[y][x] == ObjectDetails.BLOCK and details == ObjectDetails.EMPTY:
            print("removed a block")
        self.screen_info[y][x] = details

    def render(self):
        for y, line in self.screen_info.items():
            line_string = "-{}-".format(y)
            for x, details in line.items():
                line_string += render_info[details]
            print(line_string)
