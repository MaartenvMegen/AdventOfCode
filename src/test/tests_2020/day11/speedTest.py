import cProfile
import os

from src.test.tests_2020.day11.test_day_11 import perform_round
from src.utility import lineyielder
from src.utility.PositionUtils import Grid, Point

THIS_DIR = os.path.dirname(os.path.abspath(__file__))

EMPTY_SEAT = "L"
FLOOR = "."
OCCUPIED = "#"

def round():
    grid = Grid()
    for y, line in enumerate(lineyielder.yield_lines_fp("./input.txt", THIS_DIR)):
        for x, char in enumerate(line):
            grid.add_location(Point(x, y, char))
    perform_round(grid, grid.get_closest_neighbours, ignore_symbol=FLOOR, max_neighbours=5)



if __name__ == '__main__':
    cProfile.run('round()')