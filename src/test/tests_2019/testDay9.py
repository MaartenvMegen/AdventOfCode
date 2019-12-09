import unittest

from src.utility import Inputs
from src.utility.OpCodeRunner import OpcodeRunner


class Day9Tester(unittest.TestCase):
    def test_example_1(self):
        input_code = [109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99]
        program = OpcodeRunner(input_code)
        program.run_program()
        self.assertEqual(input_code, program.get_outputs())

    def test_example_2(self):
        input_code = [1102,34915192,34915192,7,4,7,99,0]
        program = OpcodeRunner(input_code)
        program.run_program()
        self.assertEqual(16, len(str(program.get_outputs()[0])))
        self.assertEqual(1219070632396864, program.get_outputs()[0])

    def test_example_3(self):
        input_code = [104,1125899906842624,99]
        program = OpcodeRunner(input_code)
        program.run_program()
        self.assertEqual(16, len(str(program.get_outputs()[0])))
        self.assertEqual(1125899906842624, program.get_outputs()[0])

    def test_part_1(self):
        program = OpcodeRunner(Inputs.boost_code,[1])
        program.run_program()
        self.assertEqual(2870072642, program.get_outputs()[0])

    def test_part_2(self):
        program = OpcodeRunner(Inputs.boost_code,[2])
        program.run_program()
        self.assertEqual(58534, program.get_outputs()[0])

if __name__ == '__main__':
    unittest.main()
