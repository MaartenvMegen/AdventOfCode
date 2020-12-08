import os
import unittest
from src.utility.InstructionRunner import InstructionRunner

THIS_DIR = os.path.dirname(os.path.abspath(__file__))


def run_modified_instructions(filename):
    program = InstructionRunner.compile_program(os.path.join(THIS_DIR, filename))

    for swap_index, instruction in enumerate(program):
        instruction, arguments = instruction
        if instruction == InstructionRunner._Instructions["nop"]:
            modified_instruction = InstructionRunner._Instructions["jmp"],arguments
        elif instruction == InstructionRunner._Instructions["jmp"]:
            modified_instruction = InstructionRunner._Instructions["nop"], arguments
        else:
            continue

        modified_instructions = [instruction if not index == swap_index else modified_instruction for
                                     index, instruction in enumerate(program)]

        machine = InstructionRunner(modified_instructions, 0)
        code, value = machine.run()
        if code == 0:
            print(f'program succesfully returned value {value}')
            return code, value
    return -1, 0


def run_machine(filename):
    program = InstructionRunner.compile_program(os.path.join(THIS_DIR, filename))
    machine = InstructionRunner(program, 0)
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
