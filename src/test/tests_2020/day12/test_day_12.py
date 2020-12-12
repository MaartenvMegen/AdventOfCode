import os
import unittest

from src.utility import lineyielder
from src.utility.PositionUtils import Grid, Point

THIS_DIR = os.path.dirname(os.path.abspath(__file__))


def west(x, y, dir, value):
    print(f"west {value}")
    return x - value, y, dir


def north(x, y, dir, value):
    print(f"north {value}")
    return x, y - value, dir


def east(x, y, dir, value):
    print(f"east {value}")
    return x + value, y, dir


def south(x, y, dir, value):
    print(f"south {value}")
    return x, y + value, dir


def rotate_right(x, y, dir, value):
    new_dir = (dir + value) % 360
    print(f'rotating right from {dir} to {new_dir} using value {value}')

    return x, y, new_dir


def rotate_left(x, y, dir, value):
    # 90 + 360 - 90 = 360
    new_dir = (dir+360-value) % 360
    print(f'rotating left from {dir} to {new_dir} using value {value}')
    return x, y, new_dir


def forward(x, y, dir, value):
    print(f'moving forward in direction: {directions[dir]}')
    return COMMAND[directions[dir]](x, y, dir, value)


directions = {
    90: "E",
    180: "S",
    270: "W",
    0: "N"
}

COMMAND = {
    "F": forward,
    "R": rotate_right,
    "L": rotate_left,
    "N": north,
    "S": south,
    "E": east,
    "W": west
}


def parse_command(command):
    action = command[0]
    value = int(command[1:])
    return action, value


class Day12Tester(unittest.TestCase):

    def test_example_a(self):
        x = 0
        y = 0
        dir = 90

        for line in lineyielder.yield_lines_fp("example.txt", THIS_DIR):
            action, value = parse_command(line)
            x, y, dir = COMMAND[action](x, y, dir, value)
        print(f"ended up at {x} {y} facing {dir}")
        self.assertEqual(25, abs(x)+abs(y))

    def test_a(self):
        x = 0
        y = 0
        dir = 90

        for line in lineyielder.yield_lines_fp("input.txt", THIS_DIR):
            action, value = parse_command(line)
            x, y, dir = COMMAND[action](x, y, dir, value)
            print(f' command resulted in position {x},{y}')
        print(f"ended up at {x} {y} facing {dir}")
        self.assertEqual(1010, abs(x)+abs(y))

