import unittest

from src.utility import Inputs
from src.utility.OpCodeRunner import OpcodeRunner


class Day5Tests(unittest.TestCase):
    def test_in_out(self):
        # should give input back as output
        memory = [3, 0, 4, 0, 99]
        input_data = [1]
        program = OpcodeRunner(memory, input_data)
        program.run_program()
        self.assertEqual(input_data, program.get_outputs(), "Output should match input")

    def test_in_out_immediate_mode(self):
        # only halts if code succesfully sets a halt code
        memory = [1002, 4, 3, 4, 33]
        program = OpcodeRunner(memory)
        program.run_program()

    def test_actual_input_id1(self):
        memory = Inputs.airco_codes
        program = OpcodeRunner(memory, [1])
        program.run_program()
        self.assertEqual(12428642, program.get_outputs()[-1])

    def test_actual_input_id5(self):
        memory = Inputs.airco_codes
        program = OpcodeRunner(memory, [5])
        program.run_program()
        self.assertEqual(918655, program.get_outputs()[-1])

    def test_equal_to_8_position_mode_unequal(self):
        memory = [3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]
        program = OpcodeRunner(memory, [7])
        program.run_program()
        self.assertEqual(0, program.get_outputs()[-1])

    def test_equal_to_8_position_mode_equal(self):
        memory = [3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]
        program = OpcodeRunner(memory, [8])
        program.run_program()
        self.assertEqual(1, program.get_outputs()[-1])

    def test_less_then_8_position_mode_less_than(self):
        memory = [3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]
        program = OpcodeRunner(memory, [7])
        program.run_program()
        self.assertEqual(1, program.get_outputs()[-1])

    def test_equal_to_8_immediate_mode_equal(self):
        memory = [3, 3, 1108, -1, 8, 3, 4, 3, 99]
        program = OpcodeRunner(memory, [8])
        program.run_program()
        self.assertEqual(1, program.get_outputs()[-1])

    def test_equal_to_8_immediate_mode_unequal(self):
        memory = [3, 3, 1108, -1, 8, 3, 4, 3, 99]
        program = OpcodeRunner(memory, [7])
        program.run_program()
        self.assertEqual(0, program.get_outputs()[-1])

    def test_jump_position_mode_input_non_zero(self):
        memory = [3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9]
        program = OpcodeRunner(memory, [7])
        program.run_program()
        self.assertEqual(1, program.get_outputs()[-1])

    def test_jump_position_mode_input_zero(self):
        memory = [3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9]
        program = OpcodeRunner(memory, [0])
        program.run_program()
        self.assertEqual(0, program.get_outputs()[-1])

    def test_jump_immediate_mode_input_non_zero(self):
        memory = [3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]
        program = OpcodeRunner(memory, [7])
        program.run_program()
        self.assertEqual(1, program.get_outputs()[-1])

    def test_jump_immediate_mode_input_zero(self):
        memory = [3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]
        program = OpcodeRunner(memory, [0])
        program.run_program()
        self.assertEqual(0, program.get_outputs()[-1])


if __name__ == '__main__':
    unittest.main()
