import unittest

from src.utility.PositionUtils import angle_between, Grid, Point


class PositionUtilTester(unittest.TestCase):

    def test_angle(self):
        self.assertEqual(0, angle_between((15, 3), (15, 1)), "up is not 0 degrees")
        self.assertEqual(90, angle_between((15, 3), (17, 3)), 'right is not 90 degrees')
        self.assertEqual(180, angle_between((15, 3), (15, 5)), "down is not 180 degrees")
        self.assertEqual(270, angle_between((15, 3), (13, 3)), "left is not 270 degrees")

    def test_grid(self):
        grid = Grid()

        point_0_0 = Point(0, 0, "#")
        grid.add_location(point_0_0)
        point_0_1 = Point(0, 1, "#")
        point_0_2 = Point(0, 2, "o")
        grid.add_location(point_0_1)
        grid.add_location(point_0_2)

        grid.add_location(Point(0, 2, "#"))
        grid.add_location(Point(1, 0, "#"))
        grid.add_location(Point(2, 0, "*"))
        grid.grid_to_string()

        self.assertEqual(point_0_0, grid.up(point_0_1))

    def test_search(self):
        grid = Grid()

        point_0_0 = Point(0, 0, "#")
        grid.add_location(point_0_0)
        point_0_1 = Point(0, 1, "#")
        point_0_2 = Point(0, 2, "o")
        grid.add_location(point_0_2)
        grid.add_location(point_0_1)

        grid.add_location(Point(1, 0, "#"))
        point_2_0 = Point(2, 0, "*")
        grid.add_location(point_2_0)
        grid.grid_to_string()
        distance = grid.search_a_b(point_0_2, point_2_0, ["#", 'o', '*'])
        self.assertEqual(4, distance)


if __name__ == '__main__':
    unittest.main()
