import unittest

from src.utility.OxygenRobot import OxygenRobot


class Day15Tester(unittest.TestCase):
    def test_robot_part_1(self):
        myrobot = OxygenRobot()
        myrobot.run()
        self.assertEqual(252, myrobot.distance_to_oxygen)
        self.assertEqual(350, myrobot.oxygenation_time)

if __name__ == '__main__':
    unittest.main()
