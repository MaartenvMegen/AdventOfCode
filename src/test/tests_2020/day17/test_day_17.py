import os
import re
import unittest
from collections import defaultdict
from math import prod

from src.utility import lineyielder

INACTIVE = "."
ACTIVE = "#"
THIS_DIR = os.path.dirname(os.path.abspath(__file__))

OFFSET = [(1, 1, 0),
          (0, 1, 0),
          (1, 0, 0),
          (-1, 0, 0),
          (0, -1, 0),
          (1, -1, 0),
          (-1, 1, 0),
          (-1, -1, 0),

          (1, 1, 1),
          (0, 1, 1),
          (1, 0, 1),
          (-1, 0, 1),
          (0, -1, 1),
          (1, -1, 1),
          (-1, 1, 1),
          (-1, -1, 1),
          (0, 0, 1),

          (1, 1, -1),
          (0, 1, -1),
          (1, 0, -1),
          (-1, 0, -1),
          (0, -1, -1),
          (1, -1, -1),
          (-1, 1, -1),
          (-1, -1, -1),
          (0, 0, -1)

          ]


class Grid3d():
    def __init__(self):
        self.grid = defaultdict()

    def add_location(self, loc, symbol):
        # print(f'adding loc {loc} for symbol {symbol}')
        self.grid[loc] = symbol

    def get_direct_neighbours(self, current_point, neighbour_type):
        for offset in OFFSET:
            yield from self.get_neighbour(current_point, offset, neighbour_type)

    def get_neighbour(self, loc, offset, neighbour_type):
        (x, y, z) = loc
        (dx, dy, dz) = offset
        new_pos = (x + dx, y + dy, z + dz)
        #print(f'checking neighbour at position: {new_pos}')
        if new_pos in self.grid.keys():
            neighbour_symbol = self.grid[new_pos]
            if neighbour_symbol == neighbour_type:
                yield neighbour_symbol

    def get_size(self):
        all_locs = list(self.grid.keys())
        x_locs = list(zip(*all_locs))[0]
        y_locs = list(zip(*all_locs))[1]
        z_locs = list(zip(*all_locs))[2]
        x_min, x_max = min(x_locs), max(x_locs)
        y_min, y_max = min(y_locs), max(y_locs)
        z_min, z_max = min(z_locs), max(z_locs)

        return x_min, x_max, y_min, y_max, z_min, z_max

    def grid_to_string(self):
        x_min, x_max, y_min, y_max, z_min, z_max = self.get_size()
        output = "current board size: x from {} to {}, y from {} to {}, z from {} to {}\n".format(x_min, x_max, y_min,
                                                                                                  y_max, z_min, z_max)
        output += "---------------------------\n"
        for z in range(z_min, z_max + 1):
            output += f"\nz={z}\n"
            for y in range(y_min, y_max + 1):
                x_string = ""
                for x in range(x_min, x_max + 1):
                    if (x, y, z) in self.grid.keys():
                        x_string += self.grid[(x, y, z)]
                    else:
                        x_string += INACTIVE
                output += x_string
                output += "\n"
        output += "---------------------------\n"
        return output


def cycle(grid):
    x_min, x_max, y_min, y_max, z_min, z_max = grid.get_size()
    # loop through grid and its edges
    changed_locs = {}

    for z in range(z_min - 1, z_max + 2):
        for y in range(y_min - 1, y_max + 2):
            for x in range(x_min - 1, x_max + 2):
                if (x,y,z) in grid.grid.keys():
                    own_symbol = grid.grid[x,y,z]
                else:
                    own_symbol = INACTIVE

                active_neighbours = len(list(grid.get_direct_neighbours( (x,y,z), ACTIVE)))
                if own_symbol == ACTIVE and not 2 <= active_neighbours <= 3:
                    changed_locs[ x,y,z ] = INACTIVE
                if own_symbol == INACTIVE and active_neighbours == 3:
                    changed_locs[x,y,z] = ACTIVE

    for loc, symbol in changed_locs.items():
        grid.grid[loc] = symbol


def get_active_cubes(grid):
    return len([1 for cube in grid.grid.values() if cube == ACTIVE])


class Day17Tester(unittest.TestCase):

    def test_example_a(self):
        grid = Grid3d()
        for y, line in enumerate(lineyielder.yield_lines_fp("./example.txt", THIS_DIR)):
            for x, char in enumerate(line):
                grid.add_location((x, y, 0), char)
        # print(grid.grid[(1, 0, 0)])
        #print(list(grid.get_direct_neighbours((2, 2, 0), "#")))

        #print(grid.grid_to_string())
        for _ in range(0,6):
            cycle(grid)
        #print(grid.grid_to_string())
        self.assertEqual(112, get_active_cubes(grid))

    def test_a(self):
        grid = Grid3d()
        for y, line in enumerate(lineyielder.yield_lines_fp("./input.txt", THIS_DIR)):
            for x, char in enumerate(line):
                grid.add_location((x, y, 0), char)
        # print(grid.grid[(1, 0, 0)])
        # print(list(grid.get_direct_neighbours((2, 2, 0), "#")))

        # print(grid.grid_to_string())
        for _ in range(0, 6):
            cycle(grid)
        # print(grid.grid_to_string())
        self.assertEqual(112, get_active_cubes(grid))