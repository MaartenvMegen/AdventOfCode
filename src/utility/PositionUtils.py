from functools import lru_cache

import numpy as np

OFFSET = [(1, 1), (0, 1), (1, 0), (-1, 0), (0, -1), (1, -1), (-1, 1), (-1, -1)]

GRID_BLANK = "_"


def distance_between(p1, p2):
    x1, y1 = p1
    x2, y2 = p2
    return np.math.sqrt((x1 - x2) ** 2 + (y1 - y2) ** 2)


def angle_between(origin, point):
    # calculate angle between point and origin with respect to 12 o'clock
    # returns value between 0 and 360
    x1, y1 = point
    x2, y2 = origin
    dX = x2 - x1
    dY = y2 - y1
    rads = np.math.atan2(-dX, dY)
    degrees = np.math.degrees(rads)
    if degrees >= 0:
        return degrees
    else:
        return 360 + degrees


class Point():

    def __init__(self, x, y, symbol = "#"):
        self.loc = (x, y)
        self.symbol = symbol


class Grid():

    def __init__(self):
        self.grid = dict()

    def get_locations(self):
        return self.grid.values()

    def add_location(self, point):
        self.grid[point.loc] = point

    def up(self, current_point):
        return self.get_new_pos(current_point, (0, -1))

    def down(self, current_point):
        return self.get_new_pos(current_point, (0, 1))

    def left(self, current_point):
        return self.get_new_pos(current_point, (-1, 0))

    def right(self, current_point):
        return self.get_new_pos(current_point, (1, 0))

    def up_left(self, current_point):
        return self.get_new_pos(current_point, (-1, -1))

    def up_right(self, current_point):
        return self.get_new_pos(current_point, (1, -1))

    def down_right(self, current_point):
        return self.get_new_pos(current_point, (-1, 1))

    def down_left(self, current_point):
        return self.get_new_pos(current_point, (1, 1))

    def get_direct_neighbours(self, current_point, ignore_symbol):
        for offset in OFFSET:
            yield self.get_new_pos(current_point, offset)

    def get_closest_neighbours(self, current_point, ignore_symbol):
        for offset in OFFSET:
            yield self.get_closest(current_point, ignore_symbol, offset)

    def get_closest(self, current_point, ignore_symbol, offsets):
        x, y = current_point.loc
        x_delta, y_delta = offsets

        x_offset = x+x_delta
        y_offset = y+y_delta
        while True:
            try:
                loc = self.get_point(x_offset, y_offset)
                if loc.symbol != ignore_symbol:
                    return loc
                else:
                    x_offset += x_delta
                    y_offset += y_delta
                    #print(f'next ref {x_offset, y_offset}')
            except:
                return None

    def get_new_pos(self, current_point, offset):
        (x, y) = current_point.loc
        #print(f'checking position: {current_point.loc}')
        (dx, dy) = offset

        new_pos = (x + dx, y+dy)
        if new_pos in self.grid.keys():
            #print(f'found neighbour at {new_pos}')
            return self.grid[new_pos]
        else:
            return None

    def get_point(self, x, y):
        return self.grid[x, y]

    def get_size(self):
        all_locs = list(self.grid.keys())
        x_locs = list(zip(*all_locs))[0]
        y_locs = list(zip(*all_locs))[1]
        x_min, x_max = min(x_locs), max(x_locs)
        y_min, y_max = min(y_locs), max(y_locs)
        return x_min, x_max, y_min, y_max

    def grid_to_string(self):
        all_locs = list(self.grid.keys())
        x_locs = list(zip(*all_locs))[0]
        y_locs = list(zip(*all_locs))[1]
        x_min, x_max = min(x_locs), max(x_locs)
        y_min, y_max = min(y_locs), max(y_locs)
        output = "current board size: x from {} to {}, y from {} to {}\n".format(x_min, x_max, y_min, y_max)
        output += "---------------------------\n"

        for y in range(y_min, y_max + 1):
            x_string = ""
            for x in range(x_min, x_max + 1):
                if (x,y) in self.grid.keys():
                    x_string += self.grid[(x, y)].symbol
                else:
                    x_string += GRID_BLANK
            output += x_string
            output += "\n"
        output += "---------------------------\n"
        return output


    def search_a_b(self, a, b, symbols):
        # do a depth first to get min distance from a -> b
        # symbols are allowed grid points to search over

        search_locs = set()
        search_locs.add(a)
        edge_locs = set([neighbour for neighbour in self.neighbours(a) if neighbour is not None and neighbour.symbol in symbols ])
        length = 1
        while b not in edge_locs:

            search_locs.update(edge_locs)

            edge_locs = [neighbour for lookup in edge_locs for neighbour in self.neighbours(lookup) if
                         neighbour is not None and neighbour.symbol in symbols]
            length += 1
            print(f"current iteration {length}")

        print(f"from {a.symbol} to {b.symbol} took {length} steps")
        return length

