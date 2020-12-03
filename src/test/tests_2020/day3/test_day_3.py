import math
import os
import unittest

from src.utility import lineyielder
from src.utility.PositionUtils import Grid, Point

THIS_DIR = os.path.dirname(os.path.abspath(__file__))

TREE = '#'


def get_trees(x_offset, grid, stepsize):
    current_x = 0
    trees = 0

    x_min, x_max, y_min, y_max = grid.get_size()
    x_size = x_max - x_min

    for y in range(y_min + 1, y_max + 1, stepsize):
        current_x = (current_x + x_offset) % (x_size + 1)
        if grid.get_point(current_x, y).symbol == TREE:
            trees += 1
        else:
            pass
    return trees


def get_grid(filename):
    grid = Grid()
    for y, line in enumerate(lineyielder.yield_lines_fp(filename, THIS_DIR)):
        for x, symbol in enumerate(line):
            grid.add_location(Point(x, y, symbol))
    return grid


class Day3Tester(unittest.TestCase):

    def test_part_a_example(self):
        grid = get_grid('example.txt')

        tree_count = get_trees(3, grid, 1)
        self.assertEqual(7, tree_count)

    def test_part_a(self):
        grid = get_grid('input.txt')

        tree_count = get_trees(3, grid, 1)
        self.assertEqual(176, tree_count)

    def test_part_b_example(self):
        grid = get_grid('example.txt')

        tree_list = []
        for x_offset, yoffset in zip([1, 3, 5, 7, 1], [1, 1, 1, 1, 2]):
            tree_count = get_trees(x_offset, grid, yoffset)
            tree_list.append(tree_count)
        self.assertEqual(336, math.prod(tree_list))

    def test_part_b(self):
        grid = get_grid('input.txt')

        tree_list = []
        for x_offset, yoffset in zip([1, 3, 5, 7, 1], [1, 1, 1, 1, 2]):
            tree_count = get_trees(x_offset, grid, yoffset)
            tree_list.append(tree_count)

        self.assertEqual(5872458240, math.prod(tree_list))

