import unittest
from itertools import product

from src.utility.OpCodeRunner import OpcodeRunner

gravity_assist_opcode = [1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 9, 19, 1, 19, 5, 23, 2, 23, 13, 27, 1,
                         10, 27, 31, 2, 31, 6, 35, 1, 5, 35, 39, 1, 39, 10, 43, 2, 9, 43, 47, 1, 47, 5, 51, 2, 51, 9,
                         55, 1, 13, 55, 59, 1, 13, 59, 63, 1, 6, 63, 67, 2, 13, 67, 71, 1, 10, 71, 75, 2, 13, 75, 79, 1,
                         5, 79, 83, 2, 83, 9, 87, 2, 87, 13, 91, 1, 91, 5, 95, 2, 9, 95, 99, 1, 99, 5, 103, 1, 2, 103,
                         107, 1, 10, 107, 0, 99, 2, 14, 0, 0]


class Day2Tester(unittest.TestCase):

    def test_gravity_assist(self):
        program = OpcodeRunner(gravity_assist_opcode)
        program.run_program()

        print("position 0 contains: {}".format(program.get_result()))
        self.assertEqual(250703, program.get_result())

    def test_gravity_assist_outcome_with_runner(self):
        program = OpcodeRunner(gravity_assist_opcode)

        for noun, verb in product(range(100), range(100)):
            program.reset()
            program.set_value(1, noun)
            program.set_value(2, verb)

            program.run_program()
            if program.get_result() == 19690720:
                print("outcome is {}".format((100*noun)+verb))
                self.assertEqual(6417, (100*noun)+verb)
                break

    def test_short_opcode(self):
        opcode = [1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]
        program = OpcodeRunner(opcode)
        program.run_program()

        self.assertEqual([3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50], program.get_memory())

    def test_short_opcode2(self):
        opcode = [1, 0, 0, 0, 99]
        program = OpcodeRunner(opcode)
        program.run_program()

        self.assertEqual([2, 0, 0, 0, 99], program.get_memory())

    def test_short_opcode3(self):
        opcode = [2, 3, 0, 3, 99]
        program = OpcodeRunner(opcode)
        program.run_program()

        self.assertEqual([2, 3, 0, 6, 99], program.get_memory())

    def test_short_opcode4(self):
        opcode = [2, 4, 4, 5, 99, 0]
        program = OpcodeRunner(opcode)
        program.run_program()

        self.assertEqual([2, 4, 4, 5, 99, 9801], program.get_memory())

    def test_short_opcode6(self):
        program_memory = [1, 1, 1, 4, 99, 5, 6, 0, 99]
        program = OpcodeRunner(program_memory)
        program.run_program()

        self.assertEqual([30, 1, 1, 4, 2, 5, 6, 0, 99], program.get_memory())


if __name__ == '__main__':
    #unittest.main()
    suite = unittest.TestLoader().loadTestsFromTestCase(Day2Tester)
    unittest.TextTestRunner(verbosity=2).run(suite)
