import threading
import unittest

from src.utility import Inputs
from src.utility.Arcade import Arcade
from src.utility.OpCodeRunner import OpcodeRunner, ParameterMode
from src.utility.Screen import Screen


class Day13Tester(unittest.TestCase):

    def test_something(self):
        arcade_program = OpcodeRunner(Inputs.arcade_program, name="arcade")
        # play for free!
        arcade_program.memory[0] = 2
        arcade = Arcade(arcade_program)

        arcade_program_thread = threading.Thread(target=arcade_program.run_program)
        arcade_thread = threading.Thread(target=arcade.run)

        arcade_program.set_output_listener(arcade)
        arcade_program.set_complete_listeners(arcade)

        arcade_thread.start()
        arcade_program_thread.start()

        arcade_program_thread.join()
        arcade_thread.join()

        self.assertEqual(17138, arcade.score_display.score)
        # amount of points = 17138


if __name__ == '__main__':
    unittest.main()
