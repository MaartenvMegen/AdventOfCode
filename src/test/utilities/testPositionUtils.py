import unittest

from src.utility.PositionUtils import angle_between


class PositionUtilTester(unittest.TestCase):

    def test_angle(self):
        self.assertEqual(0, angle_between((15, 3), (15, 1)), "up is not 0 degrees")
        self.assertEqual(90, angle_between((15, 3), (17, 3)), 'right is not 90 degrees')
        self.assertEqual(180, angle_between((15, 3), (15, 5)), "down is not 180 degrees")
        self.assertEqual(270, angle_between((15, 3), (13, 3)), "left is not 270 degrees")



if __name__ == '__main__':
    unittest.main()
