import itertools
import math
import os
import unittest
from collections import defaultdict

from parsy import regex, decimal_digit, string, whitespace, seq

THIS_DIR = os.path.dirname(os.path.abspath(__file__))

tile_nr = string("Tile ") >> decimal_digit.at_least(1).concat().tag("tile_nr").desc('tile number') << string(
    ":") << whitespace
puzzle_line = regex(r'[#.]+').sep_by(string("\n")).tag('puzzle_lines').desc('puzzle lines') << string("\n\n")
piece = seq(tile_nr, puzzle_line).map(dict)
pieces = piece.at_least(1)

monster = []
monster.append("                  # ")
monster.append("#    ##    ##    ###")
monster.append(" #  #  #  #  #  #   ")

monster_idx = []
for y, line in enumerate(monster):
    for x_index, x in enumerate(line):
        if x == "#":
            monster_idx.append( (x_index,y) )


def strip_border(tile):
    new_tile = [line[1:-1] for line in tile[1:-1]]
    return new_tile


def rotate_tile(tile):
    new_tile = []
    for x in range(len(tile)):
        new_line = [tile[len(tile) - 1 - y][x] for y in range(len(tile))]
        new_tile.append("".join(new_line))
    return new_tile


def rotate_edges(edges):
    return [edges[3], edges[0], edges[1], edges[2]]


def calculate_corner_tiles(edge_tile_map, tile_edge_map):
    corner_tile_nrs = []
    tile_to_unique_edge_map = defaultdict(list)
    # edges with multiples we can ignore
    for tile, current_edges in tile_edge_map.items():
        matches = 0
        for edge in current_edges:
            # print(f'evaluating edge of length {len(edge)} : {edge}')
            # if match exist and is not from own tile
            if (edge[::-1] in edge_tile_map.keys() and edge[::-1] not in current_edges) or len(
                    edge_tile_map[edge]) > 1:
                matches += 1
            else:
                tile_to_unique_edge_map[tile].append(edge)

            if edge[::-1] in edge_tile_map.keys() and edge[::-1] not in current_edges:
                print(f" tile {tile} has a border with rotated tile {edge_tile_map[str(edge[::-1])]}")
                pass
            if len(edge_tile_map[edge]) > 1:
                check_list = edge_tile_map[edge].copy()
                check_list.remove(tile)
                print(f' tile {tile} has a border with flipped tile {check_list}')
        # print(f'tile nr {tile} has {matches} matches')
        if matches == 2:
            corner_tile_nrs.append(int(tile))

    return corner_tile_nrs, tile_to_unique_edge_map


def flip_tile(tile):
    ## takes a list of tile strings and returns a flipped version
    return [line[::-1] for line in tile]


def flip_edges(edges):
    ## flip tile above does a horizontal flip
    # this means all edges need to be reversed and edge 1 and 3 switched
    edges = [edge[::-1] for edge in edges]
    return [edges[0], edges[3], edges[2], edges[1]]


def get_maps(tiles):
    edge_tile_map = defaultdict(list)
    tile_edge_map = {}
    tile_nr_to_tile_image = {}

    for tile in tiles:
        tile_nr = tile['tile_nr']
        tile_nr_to_tile_image[tile_nr] = tile['puzzle_lines']
        north = tile['puzzle_lines'][0]
        east = ""
        west = ""
        # reverse south and west tiles to go around the clock with the edges

        # print(tile['puzzle_lines'])
        for puzzle_line in tile['puzzle_lines']:
            if puzzle_line:
                west += puzzle_line[0]
                east += puzzle_line[-1]
                south = puzzle_line
        south = south[::-1]
        west = west[::-1]
        edge_tile_map[east].append(tile_nr)
        edge_tile_map[west].append(tile_nr)
        edge_tile_map[north].append(tile_nr)
        edge_tile_map[south].append(tile_nr)
        tile_edge_map[tile_nr] = [north, east, south, west]
        # print(north)
        # print(east)
        # print(south)
        # print(west)
        # print('end of tile')
    return edge_tile_map, tile_edge_map, tile_nr_to_tile_image


class Day20Tester(unittest.TestCase):

    def test_parser(self):
        print(tile_nr.parse("Tile 2971:\n"))
        print(puzzle_line.parse("..#.#....#\n..#..\n\n"))
        print(piece.parse("Tile 2971:\n..#.#....#\n..#..\n\n"))
        print(pieces.parse("Tile 2971:\n..#.#....#\n..#..\n\nTile 2972:\n..#.#....#\n..#..\n\n"))

    def test_example_a(self):
        with open(os.path.join(THIS_DIR, "example.txt")) as f:
            data = f.read()
            tiles = pieces.parse(data)

        edge_tile_map, tile_edge_map, tile_nr_to_tile_details = get_maps(tiles)

        corner_tiles, _ = calculate_corner_tiles(edge_tile_map, tile_edge_map)
        answer = math.prod(corner_tiles)
        self.assertEqual(20899048083289, answer)

    def test_flip_both(self):
        # confirm both flip horizontally
        tile = ["123", "456", "789"]
        edges = ["123", "369", "987", "741"]

        self.print_test_tile(edges, tile)

        tile = flip_tile(tile)
        edges = flip_edges(edges)

        self.print_test_tile(edges, tile)

    def print_test_tile(self, edges, tile):
        print(f"  {edges[0]}")
        print('')
        for index, tile in enumerate(tile):
            x_str = edges[3][-(index+1)] + " " + tile + " " + edges[1][index]
            print(x_str)
        print("")
        print(f"  {edges[2][::-1]}")

    def test_rotate_both(self):
        # confirm both rotate right
        # 123       741
        # 456 ->    852
        # 789       963
        tile = ["123", "456", "789"]
        edges = ["123", "369", "987", "741"]
        self.print_test_tile(edges, tile)
        edges = rotate_edges(edges)
        tile = rotate_tile(tile)
        self.print_test_tile(edges, tile)

    def test_flip_tile(self):
        tile = ["123", "456", "789"]
        self.assertEqual(['321', '654', '987'], flip_tile(tile))

    def test_rotate_tile(self):
        tile = ["123", "456", "789"]
        rotated = rotate_tile(tile)
        self.assertEqual(["741", "852", "963"], rotated)

    def test_rotate_edges(self):
        edges = ["...", "###", ".#.", "#.#"]
        rotated = rotate_edges(edges)
        self.assertEqual(["#.#", "...", "###", ".#."], rotated)

    def test_flip_edges(self):
        edges = ["..#", "abc", "##.", "def"]
        flipped = flip_edges(edges)
        self.assertEqual(['#..', 'fed', '.##', 'cba'], flipped)

    def test_strip_border(self):
        tile = ["123", "456", "789"]
        new_tile = strip_border(tile)
        self.assertEqual(["5"], new_tile)

    def test_a(self):
        with open(os.path.join(THIS_DIR, "input.txt")) as f:
            data = f.read()
            tiles = pieces.parse(data)

        edge_tile_map, tile_edge_map, _ = get_maps(tiles)

        corner_tiles, _ = calculate_corner_tiles(edge_tile_map, tile_edge_map)

        answer = math.prod(corner_tiles)
        self.assertEqual(32287787075651, answer)

    def test_search_monster(self):
        # now search for monsters

        image = []
        image.append("                     ")
        image.append("                   # ")
        image.append(" #    ##    ##    ###")
        image.append("  #  #  #  #  #  #   ")
        image.append("                     ")
        image.append("                     ")

        # for each monster index check if its offset from x and y coord is '#' if so monster detected
        # now modify the image to change these places to "O"?

        count = self.detect_monsters(image, monster, monster_idx)
        print(count)

    def detect_monsters(self, image, monster, monster_idx):
        count = 0
        for offset_y in range(len(image) - len(monster) + 1):
            for offset_x in range(len(image[0]) - len(monster[0]) + 1):
                monster_found = all([image[y + offset_y][x + offset_x] == "#" for x, y in monster_idx])
                if monster_found:
                    count += 1
                    print("help a seamonsster!")
        return count

    def test_example_b(self):
        with open(os.path.join(THIS_DIR, "example.txt")) as f:
            data = f.read()
            tiles = pieces.parse(data)

        edge_tile_map, tile_edge_map, tile_nr_to_tile_details = get_maps(tiles)
        # determine which tiles are in a corner
        corner_tile_nrs, tile_to_unique_edge_map = calculate_corner_tiles(edge_tile_map, tile_edge_map)

        # Start with random corner edge
        tile = str(corner_tile_nrs[0])
        self.align_starting_tile_top_left(tile, tile_edge_map, tile_to_unique_edge_map,
                                          tile_nr_to_tile_details)

        tile_location_map = dict()
        tile_location_map[(0, 0)] = tile
        grid_size = int(math.sqrt(len(tiles)))

        # now the other tiles
        self.locate_tile_and_determine_orientation(edge_tile_map, grid_size, tile, tile_edge_map, tile_location_map,
                                                   tile_nr_to_tile_details)

        # cut of the edges
        for tile, details in tile_nr_to_tile_details.items():
            tile_nr_to_tile_details[tile] = strip_border(details)

        # combine into one image
        complete_image = self.create_image(corner_tile_nrs, grid_size, tile_location_map, tile_nr_to_tile_details,
                                           tiles)

        # get monsters and calculate roughness
        monster_count = self.detect_monsters(complete_image, monster, monster_idx)
        answer = self.calculate_roughness(complete_image, monster_count)
        print(answer)

    def test_b(self):
        with open(os.path.join(THIS_DIR, "input.txt")) as f:
            data = f.read()
            tiles = pieces.parse(data)
        print(f'found {len(tiles)} tiles')
        edge_tile_map, tile_edge_map, tile_nr_to_tile_details = get_maps(tiles)
        # determine which tiles are in a corner
        corner_tile_nrs, tile_to_unique_edge_map = calculate_corner_tiles(edge_tile_map, tile_edge_map)
        print(f"corner tiles : {corner_tile_nrs}")
        # Start with random corner edge
        tile = str(corner_tile_nrs[0])
        self.align_starting_tile_top_left(tile, tile_edge_map, tile_to_unique_edge_map,
                                          tile_nr_to_tile_details)

        tile_location_map = dict()
        tile_location_map[(0, 0)] = tile
        grid_size = int(math.sqrt(len(tiles)))

        # now the other tiles
        self.locate_tile_and_determine_orientation(edge_tile_map, grid_size, tile, tile_edge_map, tile_location_map,
                                                   tile_nr_to_tile_details)

        # cut of the edges
        for tile, details in tile_nr_to_tile_details.items():
            tile_nr_to_tile_details[tile] = strip_border(details)

        # combine into one image
        complete_image = self.create_image(corner_tile_nrs, grid_size, tile_location_map, tile_nr_to_tile_details,
                                           tiles)

        # get monsters and calculate roughness
        monster_count= 0
        iterations = 0
        while monster_count == 0 and iterations < 3:
            for options in range(4):
                monster_count = self.detect_monsters(complete_image, monster, monster_idx)
                if monster_count > 0:
                    break
                complete_image = rotate_tile(complete_image)
            complete_image = flip_tile(complete_image)
            iterations += 1

        answer = self.calculate_roughness(complete_image, monster_count)
        print(answer)

    def locate_tile_and_determine_orientation(self, edge_tile_map, grid_size, tile, tile_edge_map, tile_location_map,
                                              tile_nr_to_tile_details):
        east_tile = tile
        south_tile = tile
        for y in range(0, grid_size):
            for x in range(1, grid_size):
                print(f'looking for the east neighbour of tile {east_tile} in x {x} y {y}')
                east_tile = self.neighbour_for_tile(1, edge_tile_map, east_tile, tile_edge_map, tile_nr_to_tile_details,
                                                    tile_location_map, x, y)
            print(f'looking for the south neighbour of tile {south_tile} tile in x {0} y {y + 1}')
            south_tile = self.neighbour_for_tile(2, edge_tile_map, south_tile, tile_edge_map, tile_nr_to_tile_details,
                                                 tile_location_map, 0, y + 1)
            east_tile = south_tile

    def calculate_roughness(self, complete_image, monster_count):
        roughness = sum([1 for line in complete_image for char in line if char == "#"])
        answer = roughness - len(monster_idx) * monster_count
        return answer

    def create_image(self, corner_tile_nrs, grid_size, tile_location_map, tile_nr_to_tile_details, tiles):
        tile_length = len(tile_nr_to_tile_details[str(corner_tile_nrs[0])])
        row_length = grid_size * tile_length
        print(
            f'printing array of size {grid_size} by {grid_size} total tiles {len(tiles)} each tile has length {tile_length}')
        print('found the following tild ids:')
        for y in range(grid_size):
            x_str = ""
            for x in range(grid_size):
                x_str += tile_location_map[(x, y)]
                x_str += " "
            print(x_str)
        # self.print_tile(tile_edge_map, tile_nr_to_tile_details, '1951')
        complete_image = []
        for y in range(row_length):
            x_str = ""
            y_grid = y // tile_length
            # print(f'row {y} in tile {y_grid}')
            inner_y = y % tile_length
            # if inner_y == 0:
            #     print("")

            for x_grid in range(grid_size):
                x_str += tile_nr_to_tile_details[tile_location_map[(x_grid, y_grid)]][inner_y]
                # x_str += " "

            print(x_str)
            complete_image.append(x_str)
        return complete_image

    def print_tile(self, tile_edge_map, tile_nr_to_tile_details, tile_to_print):
        print(f'detailed look at tile {tile_to_print}')
        tile_edge = tile_edge_map[tile_to_print]
        details = tile_nr_to_tile_details[tile_to_print]
        print(f"  {tile_edge[0]}")
        print("")
        for index, line in enumerate(details):
            x_str = ""
            x_str += tile_edge[3][-(index+1)]
            x_str += " "
            x_str += line
            x_str += " "
            x_str += tile_edge[1][index]
            print(x_str)
        print('')
        print(f"  {tile_edge[2][::-1]}")

    def neighbour_for_tile(self, orientation, edge_tile_map, tile, tile_edge_map, tile_nr_to_tile_details,
                           tile_location_map, x, y):

        edge = tile_edge_map[tile][orientation]
        flip = False
        rotations = 0
        neighbour_tile = None
        desired_edge = edge[::-1]

        flipped_tiles = [flipped_tile for flipped_tile in edge_tile_map[edge] if flipped_tile != tile]
        if desired_edge in edge_tile_map:
            non_flipped_tiles = [non_flip_tile for non_flip_tile in edge_tile_map[desired_edge] if non_flip_tile != tile]
        else:
            non_flipped_tiles = []

        if flipped_tiles:
            # found flipped tile
            neighbour_tile = flipped_tiles[0]
            flip = True
        elif non_flipped_tiles:
            without_itself = set(edge_tile_map[desired_edge]) - set([tile])
            print(without_itself)
            neighbour_tile = without_itself.pop()
            print(neighbour_tile)

        if neighbour_tile:
            # print(f'Found {neighbour_tile} as neighbour for tile {tile}')
            edges = tile_edge_map[neighbour_tile]
            opposite_edge_nr = 3 if orientation == 1 else 0
            if flip:
                edges = flip_edges(edges)
                tile_nr_to_tile_details[neighbour_tile] = flip_tile(tile_nr_to_tile_details[neighbour_tile])
            while not edges[opposite_edge_nr] == desired_edge:
                # print('rotating')
                edges = rotate_edges(edges)
                tile_nr_to_tile_details[neighbour_tile] = rotate_tile(tile_nr_to_tile_details[neighbour_tile])
                rotations += 1
            # print(f'tile has correct orientation with flip {flip} and rotations {rotations}')
            tile_edge_map[neighbour_tile] = edges
            tile_location_map[(x, y)] = neighbour_tile
            return neighbour_tile
        else:
            print('no neighbour found, reached an edge')
            return None


    def align_starting_tile_top_left(self, tile, tile_edge_map, tile_to_unique_edge_map, tile_nr_to_tile_details):
        edges_for_top_left = tile_edge_map[tile]
        unique_edges = tile_to_unique_edge_map[tile]
        rotations = 0
        while not (edges_for_top_left[0] in unique_edges and edges_for_top_left[3] in unique_edges):
            print(f'searching correct orientation, current north edge = {edges_for_top_left[0]}')
            edges_for_top_left = rotate_edges(edges_for_top_left)
            tile_nr_to_tile_details[tile] = rotate_tile(tile_nr_to_tile_details[tile])
            rotations += 1
        tile_edge_map[tile] = edges_for_top_left
        print(f"tile has correct orientation with north edge {edges_for_top_left[0]}")
        # tile_transformation_map[tile] = {"rotations": rotations, "flip": False}
