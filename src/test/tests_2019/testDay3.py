import unittest
from collections import defaultdict
import matplotlib.pyplot as plt

from src.utility.Inputs import line1, line2


class Day3Tester(unittest.TestCase):
    direction = {'R': (1, "x"), "U": (1, "y"), "L": (-1, "x"), "D": (-1, "y")}

    def test_example(self):
        line1 = ["R8", "U5", "L5", "D3"]
        line2 = ["U7", "R6", "D4", "L4"]
        distance = self.get_minimum_distance(line1, line2)
        self.assertEqual(6, distance)

    def test_example2(self):
        line1 = ["R75","D30","R83","U83","L12","D49","R71","U7","L72"]
        line2 = ["U62","R66","U55","R34","D71","R55","D58","R83"]
        distance = self.get_minimum_distance(line1, line2)
        self.assertEqual(159, distance)

    def test_reality(self):
        results = self.get_minimum_distance(line1, line2)

        self.assertNotEqual(14, results, "is the wrong answer")
        self.assertNotEqual(23, results, "is the wrong answer")

    def get_minimum_distance(self, line1, line2):
        steps_to_loc = defaultdict(lambda: defaultdict(int))
        locs, steps_to_loc = self.parse_line(line1, steps_to_loc)
        locs2, steps_to_loc = self.parse_line(line2, steps_to_loc)

        intersections = [loc for loc in locs2 if loc in locs]
        results = [((x, y), abs(x) + abs(y)) for (x, y) in intersections]

        print("smallest nr of steps to an intersection: {}".format(sorted([sum(steps_to_loc[intersection].values()) for intersection in intersections])[0]))
        results = sorted(results, key=lambda loc: loc[1])
        print("smallest distance found for location: {}, distance {}".format(results[0][0], results[0][1]))
        # plt.scatter(*zip(*locs))
        # plt.scatter(*zip(*locs2), color = "red")
        # plt.show()
        return results[0][1]

    def parse_line(self, line1, steps_to_loc):
        locs = set()
        current_pos = (0, 0)
        steps = 0
        for line_spec in line1:
            (increment, axis) = self.direction[line_spec[0]]
            x, y = current_pos
            distance = int(line_spec[1:])
            # print("will make stepsize {} on {} axis over distance {}".format(increment, axis, distance))
            if axis == "x":
                for i in range(0, distance):
                    x = x + increment
                    steps += 1
                    current_pos = (x, y)
                    locs.add(current_pos)
                    steps_to_loc[current_pos]['1'] = steps
            if axis == "y":
                for i in range(0, distance):
                    y = y + increment
                    steps += 1
                    current_pos = (x, y)
                    locs.add(current_pos)
                    steps_to_loc[current_pos]['1'] = steps
        return locs, steps_to_loc


if __name__ == '__main__':
    unittest.main()
