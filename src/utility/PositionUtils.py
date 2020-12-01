import numpy as np

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

    def neighbours(self, current_point):
        return [self.up(current_point), self.down(current_point), self.left(current_point), self.right(current_point) ]

    def get_new_pos(self, current_point, offset):
        (x, y) = current_point.loc
        (dx, dy) = offset

        new_pos = (x + dx, y+dy)
        if new_pos in self.grid.keys():
            return self.grid[new_pos]
        else:
            return None

    def grid_to_string(self):
        all_locs = list(self.grid.keys())
        x_locs = list(zip(*all_locs))[0]
        y_locs = list(zip(*all_locs))[1]
        x_min, x_max = min(x_locs), max(x_locs)
        y_min, y_max = min(y_locs), max(y_locs)
        print("current board size {} {} to {} {}".format(x_min, y_min, x_max, y_max))
        print("---------------------------")

        for y in range(y_min, y_max + 1):
            x_string = ""
            for x in range(x_min, x_max + 1):
                if (x,y) in self.grid.keys():
                    x_string += self.grid[(x, y)].symbol
                else:
                    x_string += GRID_BLANK
            print(x_string)
        print("---------------------------")


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

