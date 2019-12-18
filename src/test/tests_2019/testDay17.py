import unittest

from src.utility.OxygenRobot import OxygenRobot
from src.utility.VacuumRobot import VacuumRobot


class Day17Tester(unittest.TestCase):
    def test_robot_part_1(self):
        myrobot = VacuumRobot()
        myrobot.run()

        self.assertEqual(7780, myrobot.alignment_param_sum, "Failed to find proper locations for scaffold intersections")

    def test_robot_part_2(self):
        myrobot = VacuumRobot()

        # provide all inputs
        [myrobot.program.inputs.put(element) for element in myrobot.TOTAL_INPUT]
        # wake robot up from automatic mode
        myrobot.program.memory[0] = 2

        # start the robot
        myrobot.run()

        # how much dust is collected?
        self.assertEqual(1075882, myrobot.dust_collected, "Failed to collect all the dust")

if __name__ == '__main__':
    unittest.main()
