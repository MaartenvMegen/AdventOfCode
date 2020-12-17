import itertools
import os
import unittest
from collections import defaultdict

from src.utility import lineyielder

INACTIVE = "."
ACTIVE = "#"
THIS_DIR = os.path.dirname(os.path.abspath(__file__))


def get_nd_offsets(n):
    ofsets = -1, 0, 1
    for permutation in itertools.product(ofsets, repeat=n):
        if permutation != tuple([0] * n):
            yield permutation


class GridN():
    def __init__(self, n):
        self.grid = defaultdict()
        self.dimensions = n

    def add_location(self, loc, symbol):
        self.grid[loc] = symbol

    def get_direct_neighbours(self, current_point, neighbour_type, offsets):
        for offset in offsets:
            yield from self.get_neighbour(current_point, offset, neighbour_type)

    def get_neighbour(self, loc, offset, neighbour_type):
        new_pos = tuple(coordinate + offset for coordinate, offset in zip(loc, offset))
        # print(f'checking neighbour at position: {new_pos}')
        if new_pos in self.grid.keys():
            neighbour_symbol = self.grid[new_pos]
            if neighbour_symbol == neighbour_type:
                yield neighbour_symbol

    def get_size(self):
        all_locs = list(self.grid.keys())
        all_locs = list(zip(*all_locs))

        min_max_per_dimension = []

        for n in range(0, self.dimensions):
            min_loc = min(all_locs[n])
            max_loc = max(all_locs[n])
            min_max_per_dimension.append((min_loc, max_loc))

        return min_max_per_dimension

    def grid_to_string(self):
        min_max_per_dimension = self.get_size()

        # Extract first 2 dimensions and plot those for each of the other dimensions
        n_range = []
        for n in range(0, self.dimensions):
            n_range.append(range(min_max_per_dimension[n][0], min_max_per_dimension[n][1] + 1))

        output = "---------BOARD-------------\n"
        for dim, (min, max) in enumerate(min_max_per_dimension):
            output += f"dimension {dim} has range {min} - {max}\n"
        output += "---------------------------\n"

        for other_dims in itertools.product(*n_range[2:]):
            if other_dims:
                output += f"dimension {other_dims}\n"
            for y in n_range[1]:
                for x in n_range[0]:
                    if (x, y, *other_dims) in self.grid.keys():
                        output += self.grid[(x, y, *other_dims)]
                    else:
                        output += INACTIVE
                output += "\n"
            output += "---------------------------\n"
        return output


def cycle(grid):
    min_max_per_dimension = grid.get_size()
    changed_coordinates = {}
    offsets = list(get_nd_offsets(grid.dimensions))

    n_range = []
    for n in range(0, grid.dimensions):
        n_range.append(range(min_max_per_dimension[n][0] - 1, min_max_per_dimension[n][1] + 2))

    for coordinate in itertools.product(*n_range):
        if coordinate in grid.grid.keys():
            own_symbol = grid.grid[coordinate]
        else:
            own_symbol = INACTIVE
        # print(f"evaluating position {option} current symbol {own_symbol}")
        active_neighbours = len(list(grid.get_direct_neighbours(coordinate, ACTIVE, offsets)))
        if own_symbol == ACTIVE and not 2 <= active_neighbours <= 3:
            changed_coordinates[coordinate] = INACTIVE
        if own_symbol == INACTIVE and active_neighbours == 3:
            changed_coordinates[coordinate] = ACTIVE

    grid.grid.update(changed_coordinates)


def get_active_cubes(grid):
    return len([1 for cube in grid.grid.values() if cube == ACTIVE])


class Day17Tester(unittest.TestCase):

    def test_example_b(self):
        grid = GridN(4)
        for y, line in enumerate(lineyielder.yield_lines_fp("./example.txt", THIS_DIR)):
            for x, char in enumerate(line):
                grid.add_location((x, y, 0, 0), char)

        for i in range(0, 6):
            #print(f'running cycle {i}')
            cycle(grid)
            # print(grid.grid_to_string())
        self.assertEqual(848, get_active_cubes(grid))

    def test_b(self):
        grid = GridN(4)
        for y, line in enumerate(lineyielder.yield_lines_fp("./input.txt", THIS_DIR)):
            for x, char in enumerate(line):
                grid.add_location((x, y, 0, 0), char)

        for i in range(0, 6):
            #print(f'running cycle {i}')
            cycle(grid)
        self.assertEqual(1972, get_active_cubes(grid))

    def test_a(self):
        grid = GridN(3)
        for y, line in enumerate(lineyielder.yield_lines_fp("./input.txt", THIS_DIR)):
            for x, char in enumerate(line):
                grid.add_location((x, y, 0), char)

        for _ in range(0, 6):
            cycle(grid)
        self.assertEqual(295, get_active_cubes(grid))

    def test_example_a(self):
        grid = GridN(3)
        for y, line in enumerate(lineyielder.yield_lines_fp("./example.txt", THIS_DIR)):
            for x, char in enumerate(line):
                grid.add_location((x, y, 0), char)

        for _ in range(0, 6):
            cycle(grid)
        self.assertEqual(112, get_active_cubes(grid))

    def test_2d(self):
        grid = GridN(2)
        grid.add_location( (0,1) , "#")
        grid.add_location( (1,0) , "#")
        print(grid.grid_to_string())