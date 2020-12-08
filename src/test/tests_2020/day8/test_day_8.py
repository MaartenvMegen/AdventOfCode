import os
import unittest
import copy
from src.utility import lineyielder
from src.utility.InstructionRunner import InstructionRunner

THIS_DIR = os.path.dirname(os.path.abspath(__file__))


def run_modified_instructions(filename):
    instructions = [line.split(' ') for line in lineyielder.yield_lines_fp(filename, THIS_DIR)]
    for index, instruction in enumerate(instructions):
        modified_instruction = copy.deepcopy(instructions)
        if instruction[0] == "nop":
            modified_instruction[index][0] = "jmp"
        elif instruction[0] == "jmp":
            modified_instruction[index][0] = "nop"
        machine = InstructionRunner(modified_instruction, 0)
        code, value = machine.run()
        if code == 0:
            print(f'program succesfully returned value {value}')
            return code, value
    return -1, 0


def run_machine(filename):
    instructions = [line.split(' ') for line in lineyielder.yield_lines_fp(filename, THIS_DIR)]
    machine = InstructionRunner(instructions, 0)
    return machine.run()


class Day8Tester(unittest.TestCase):

    def test_example(self):
        code, value = run_machine("example.txt")
        self.assertEqual(-1, code)
        self.assertEqual(5, value)

    def test_input(self):
        code, value = run_machine("input.txt")
        self.assertEqual(-1, code)
        self.assertEqual(1723, value)

    def test_find_nop_or_jump_(self):
        code, value = run_modified_instructions("input.txt")
        self.assertEqual(846, value)
        self.assertEqual(0, code)

    def test_find_nop_or_jump_example(self):
        code, value = run_modified_instructions("example.txt")
        self.assertEqual(8, value)
        self.assertEqual(0, code)
