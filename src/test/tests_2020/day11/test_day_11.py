import os
import unittest

from src.utility import lineyielder
from src.utility.PositionUtils import Grid, Point

THIS_DIR = os.path.dirname(os.path.abspath(__file__))

EMPTY_SEAT = "L"
FLOOR = "."
OCCUPIED = "#"


def perform_round(grid, neighbour_func, ignore_symbol, max_neighbours):
    desired_changes = []

    for location in grid.get_locations():
        x, y = location.loc
        symbol = location.symbol
        # room for speedup by only making neighbour func return symbols we are interested in.
        # that would would make the next lines length lookups instead of for loops

        neighbours = neighbour_func(location, ignore_symbol)
        if symbol == EMPTY_SEAT and neighbours:
            for neighbour in neighbours:
                if neighbour and neighbour.symbol == OCCUPIED:
                    break
            else:
                desired_changes.append(Point(x, y, OCCUPIED))
                continue

        if symbol == OCCUPIED and neighbours:
            # as soon as max neighbours is reached break
            neighbour_cnt = 0
            for neighbour in neighbours:
                if neighbour and neighbour.symbol == OCCUPIED:
                    neighbour_cnt += 1
                    if neighbour_cnt >= max_neighbours:
                        desired_changes.append(Point(x, y, EMPTY_SEAT))
                        break

    for changed_pos in desired_changes:
        grid.add_location(changed_pos)
    # print(grid.grid_to_string())

    if desired_changes:
        return True
    else:
        return False


class Day11Tester(unittest.TestCase):

    def test_example_a(self):
        grid = Grid()
        for y, line in enumerate(lineyielder.yield_lines_fp("./example.txt", THIS_DIR)):
            for x, char in enumerate(line):
                grid.add_location(Point(x, y, char))
        print(grid.grid_to_string())

        round_counter = 0
        while perform_round(grid, grid.get_direct_neighbours, ignore_symbol=FLOOR, max_neighbours=4):
            round_counter += 1
        self.assertEqual(37, sum([1 for location in grid.get_locations() if location.symbol == OCCUPIED]))

    def test_part_a(self):
        grid = Grid()
        for y, line in enumerate(lineyielder.yield_lines_fp("./input.txt", THIS_DIR)):
            for x, char in enumerate(line):
                grid.add_location(Point(x, y, char))
        print(grid.grid_to_string())
        round_counter = 0
        while perform_round(grid, grid.get_direct_neighbours, ignore_symbol=FLOOR, max_neighbours=4):
            round_counter += 1
        self.assertEqual(2494, sum([1 for location in grid.get_locations() if location.symbol == OCCUPIED]))

    def test_example_b(self):
        grid = Grid()
        for y, line in enumerate(lineyielder.yield_lines_fp("./example.txt", THIS_DIR)):
            for x, char in enumerate(line):
                grid.add_location(Point(x, y, char))
        print(grid.grid_to_string())

        round_counter = 0
        while perform_round(grid, grid.get_closest_neighbours, ignore_symbol=FLOOR, max_neighbours=5):
            round_counter += 1
        self.assertEqual(26, sum([1 for location in grid.get_locations() if location.symbol == OCCUPIED]))

    def test_b(self):
        grid = Grid()
        for y, line in enumerate(lineyielder.yield_lines_fp("./input.txt", THIS_DIR)):
            for x, char in enumerate(line):
                grid.add_location(Point(x, y, char))
        print(grid.grid_to_string())

        round_counter = 0
        while perform_round(grid, grid.get_closest_neighbours, ignore_symbol=FLOOR, max_neighbours=5):
            round_counter += 1
        self.assertEqual(2306, sum([1 for location in grid.get_locations() if location.symbol == OCCUPIED]))
