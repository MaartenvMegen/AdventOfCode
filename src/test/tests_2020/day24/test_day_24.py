import os
import unittest
from collections import defaultdict

from parsy import regex, decimal_digit, string, whitespace, seq
import logging

OFFSET = [(1, 1), (0, 1), (1, 0), (-1, 0), (0, -1), (-1, -1)]
BLACK = 1
WHITE = 0
THIS_DIR = os.path.dirname(os.path.abspath(__file__))

west = string('w').result((0, 1))
northwest = string('nw').result((1, 1))
northeast = string("ne").result((1, 0))
east = string("e").result((0, -1))
southeast = string("se").result((-1, -1))
southwest = string('sw').result((-1, 0))
direction = northwest | northeast | west | east | southeast | southwest


def to_coord(steps):
    return tuple([sum(coord) for coord in zip(*steps)])


STEPS = direction.many().map(to_coord).sep_by(string("\n"))


def get_black_tile_count(tiles):
    return list(tiles.values()).count(BLACK)


def flip_tiles(coordinates, tiles):
    for coordinate in coordinates:
        if tiles[coordinate] == BLACK:
            tiles[coordinate] = WHITE
        else:
            tiles[coordinate] = BLACK


def get_neighbour_loc(loc, offset, neighbour_type, tiles):
    new_pos = tuple(coordinate + offset for coordinate, offset in zip(loc, offset))
    # print(f'checking neighbour at position: {new_pos}')
    if new_pos in tiles.keys():
        neighbour_symbol = tiles[new_pos]
        if neighbour_symbol == neighbour_type:
            yield new_pos
    elif neighbour_type == WHITE:
        yield new_pos


def get_direct_neighbours(current_point, neighbour_type, tiles):
    for offset in OFFSET:
        yield from get_neighbour_loc(current_point, offset, neighbour_type, tiles)


def run_modifications(tiles):
    white_tiles = set()
    black_tiles = set()
    for loc, color in tiles.items():
        if color == BLACK:
            black_tiles.add(loc)
            [white_tiles.add(loc) for loc in get_direct_neighbours(loc, WHITE, tiles)]
        else:
            white_tiles.add(loc)
    logging.debug(f"current floor contains {len(white_tiles)} white tiles and {len(black_tiles)} black tiles")

    tiles_to_turn_black = set()
    # for white tiles check if it has 2 black neighbours -> turn black
    for tile in white_tiles:
        black_neighbours = list(get_direct_neighbours(tile, BLACK, tiles))
        logging.debug(f"tile {tile} has {len(black_neighbours)} bordering the current white tile")

        if len(black_neighbours) == 2:
            tiles_to_turn_black.add(tile)
    # for black tiles check if it has 0 or > 2 black ones, is so turn white (delete)
    tiles_to_turn_white = set()
    for tile in black_tiles:
        black_neighbours = list(get_direct_neighbours(tile, BLACK, tiles))
        logging.debug(f"tile {tile} has {len(black_neighbours)} bordering the current black tile")
        if len(black_neighbours) == 0 or len(black_neighbours) > 2:
            tiles_to_turn_white.add(tile)
    # apply changes to tiles
    for tile in tiles_to_turn_white:
        logging.debug(f"setting tile {tile} to white")
        del tiles[tile]
    for tile in tiles_to_turn_black:
        logging.debug(f"setting tile {tile} to black")
        tiles[tile] = BLACK


def run_iterations(tiles):
    logging.debug(f"Day 0: {get_black_tile_count(tiles)}")
    for day in range(1, 101):
        run_modifications(tiles)
        logging.debug(f"Day {day}: {get_black_tile_count(tiles)}")


class Day24Tester(unittest.TestCase):

    def test_parser(self):
        self.assertEqual([(0, 0)], STEPS.parse("nwwswee"))
        self.assertEqual([(0, 0), (0, 0)], STEPS.parse("nwwswee\nnwwswee"))

    def test_example_pt1(self):
        tiles = defaultdict(int)
        with open(os.path.join(THIS_DIR, "example.txt"), "r") as file:
            coordinates = STEPS.parse(file.read())

        flip_tiles(coordinates, tiles)
        self.assertEqual(10, get_black_tile_count(tiles))

    def test_pt1(self):
        tiles = defaultdict(int)
        with open(os.path.join(THIS_DIR, "input.txt"), "r") as file:
            coordinates = STEPS.parse(file.read())

        flip_tiles(coordinates, tiles)
        self.assertEqual(424, get_black_tile_count(tiles))

    def test_example_pt2(self):
        logging.basicConfig(format='%(levelname)s:%(message)s', level=logging.INFO)
        tiles = defaultdict(int)
        with open(os.path.join(THIS_DIR, "example.txt"), "r") as file:
            coordinates = STEPS.parse(file.read())

        flip_tiles(coordinates, tiles)
        run_iterations(tiles)

        self.assertEqual(2208, get_black_tile_count(tiles))

    def test_pt2(self):
        logging.basicConfig(format='%(levelname)s:%(message)s', level=logging.INFO)
        tiles = defaultdict(int)
        with open(os.path.join(THIS_DIR, "input.txt"), "r") as file:
            coordinates = STEPS.parse(file.read())

        flip_tiles(coordinates, tiles)
        run_iterations(tiles)

        self.assertEqual(3737, get_black_tile_count(tiles))
