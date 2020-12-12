import math
import os
import unittest

from src.utility import lineyielder
from src.utility.PositionUtils import Grid, Point

THIS_DIR = os.path.dirname(os.path.abspath(__file__))


class Ship():
    def __init__(self, x, y, wx, wy):
        self.x = x
        self.y = y
        self.wx = wx
        self.wy = wy

        self.COMMAND = {
            "F": self.forward,
            "R": self.right,
            "L": self.left,
            "N": self.north,
            "S": self.south,
            "E": self.east,
            "W": self.west
        }

    def west(self, value):
        self.wx -= value

    def east(self, value):
        self.wx += value

    def north(self, value):
        self.wy -= value

    def south(self, value):
        self.wy += value

    def left(self, value):
        self.rotate_wp("L", value)

    def right(self, value):
        self.rotate_wp("R", value)

    def rotate_wp(self, direction, degrees, ):
        """rotate a point around the origin (0, 0)."""

        # with workaround because the ships coordinates system is mirrored on the y axis (up is negative)
        if direction == "R":
            degrees = -degrees

        radians = math.radians(degrees)
        xx = self.wx * math.cos(radians) + self.wy * math.sin(radians)
        yy = -self.wx * math.sin(radians) + self.wy * math.cos(radians)
        self.wx = int(round(xx))
        self.wy = int(round(yy))

    def forward(self, value):
        # multiply waypoints by value for position offset
        x_offset = self.wx * value
        y_offset = self.wy * value
        self.x += x_offset
        self.y += y_offset


def parse_command(command):
    action = command[0]
    value = int(command[1:])
    return action, value


class Day12Tester(unittest.TestCase):

    def test_b_example(self):
        ship = Ship(0, 0, 10, -1)

        for line in lineyielder.yield_lines_fp("example.txt", THIS_DIR):
            action, value = parse_command(line)
            #print(f'command {action} value {value}')
            ship.COMMAND[action](value)

            #print(f' command resulted in position {ship.x},{ship.y} with waypoint {ship.wx}, {ship.wy}')
        print(f"ended up at {ship.x} {ship.y} facing {dir}")
        self.assertEqual(286, abs(ship.x) + abs(ship.y))

    def test_b_input(self):
        ship = Ship(0, 0, 10, -1)

        for line in lineyielder.yield_lines_fp("input.txt", THIS_DIR):
            action, value = parse_command(line)
            # print(f'command {action} value {value}')
            ship.COMMAND[action](value)
            # print(f' command resulted in position {ship.x},{ship.y} with waypoint {ship.wx}, {ship.wy}')
        print(f"ended up at {ship.x} {ship.y}")
        self.assertEqual(52742, abs(ship.x) + abs(ship.y))

    def test_rotate(self):
        # faces x in east and y in north
        ship = Ship(0, 0, 1, -1)
        # rotate right should make it east by y and south by x
        ship.rotate_wp("R", 90)
        self.assertEqual(1, ship.wx)
        self.assertEqual(1, ship.wy)

        ship = Ship(0, 0, 1, -1)
        # rotate right should make it east by y and south by x
        ship.rotate_wp("L", 90)
        self.assertEqual(-1, ship.wx)
        self.assertEqual(-1, ship.wy)
