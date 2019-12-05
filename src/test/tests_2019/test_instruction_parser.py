import unittest

from src.utility.OpCodeRunner import Instruction, Opcode, ParameterMode


class MyTestCase(unittest.TestCase):
    def test_multiplication_direct_indirect(self):
        instruction_int = 1002
        instruction = Instruction.parse(instruction_int)
        self.assertEqual(Opcode.MULTIPLY,instruction.opcode)
        self.assertEqual(ParameterMode.POSITION,instruction.mode_1)
        self.assertEqual(ParameterMode.IMMEDIATE,instruction.mode_2)
        self.assertEqual(ParameterMode.POSITION,instruction.mode_3)

    def test_halt(self):
        instruction_int = 99
        instruction = Instruction.parse(instruction_int)
        self.assertEqual(Opcode.HALT,instruction.opcode)
        self.assertEqual(ParameterMode.POSITION,instruction.mode_1)
        self.assertEqual(ParameterMode.POSITION,instruction.mode_2)
        self.assertEqual(ParameterMode.POSITION,instruction.mode_3)

    def test_classic_instruction(self):
        instruction_int = 1
        instruction = Instruction.parse(instruction_int)
        self.assertEqual(Opcode.ADD, instruction.opcode)
        self.assertEqual(ParameterMode.POSITION, instruction.mode_1)
        self.assertEqual(ParameterMode.POSITION, instruction.mode_2)
        self.assertEqual(ParameterMode.POSITION, instruction.mode_3)

    # def test_input_handling(self):
    #     command_in = input()
    #     print(command_in)


if __name__ == '__main__':
    unittest.main()
