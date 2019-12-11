import threading
import unittest
from itertools import product

from src.utility import Inputs
from src.utility.OpCodeRunner import OpcodeRunner
from src.utility.robot import Color, Robot


class Day11Tester(unittest.TestCase):
    def test_example_1(self):
        robot = Robot()
        program = OpcodeRunner(Inputs.painter_program, name="painter program")
        program.set_output_listener(robot)
        robot.set_output_listener(program)
        program.set_complete_listeners(robot)

        threading.Thread(target=program.run_program).start()
        robothread = threading.Thread(target=robot.run)
        robothread.start()

        print("sending start input to program")
        # part 1: send 0
        # part 2: send 1
        program.send_data(1)

        robothread.join()


if __name__ == '__main__':
    unittest.main()
