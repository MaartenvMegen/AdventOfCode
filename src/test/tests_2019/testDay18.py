import os
import unittest

WALL = "#"

MAZE_CHAR = "@"

THIS_DIR = os.path.dirname(os.path.abspath(__file__))

maze = {}
# object is key, distance to origin is value

interesting_locs = {}
analyzed_locs = set()

class Day18Tester(unittest.TestCase):

    def test_something(self):
        with open(os.path.join(THIS_DIR, "day18input.txt"), "r") as file:
            y = 0
            for line in file.readlines():
                x = 0
                for character in line:
                    maze[ (x,y)] = character
                    x += 1
                y+=1


        self.parse_maze(maze)
        print('getting there')

    def parse_maze(self, maze):
        starting_point = ()
        for location, character in maze.items():
            if character == MAZE_CHAR:
                print('found maze at location: {}'.format(location))
                starting_point = location
                break

        # first do one iteration of bfs
        locs = self.get_neigbour_locs(starting_point)
        for loc in locs:
            if not maze[loc] == WALL and loc not in analyzed_locs:
                analyzed_locs.add(loc)



    def get_neigbour_locs(self, loc):
        (x,y) = loc
        locs = []
        locs.append( (x+1, y))
        locs.append( (x, y+1))
        locs.append( (x-1, y))
        locs.append( (x, y-1))

if __name__ == '__main__':
    unittest.main()
