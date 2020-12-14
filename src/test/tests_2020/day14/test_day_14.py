import itertools
import os
import re
import unittest

from src.utility import lineyielder

THIS_DIR = os.path.dirname(os.path.abspath(__file__))


class SeaportComputer:
    def __init__(self):
        self.memory = {}
        self.mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"

    def set_mask(self, mask):
        self.mask = mask

    def write_v1(self, register_value):
        register, value = register_value
        value = apply_mask_to_value(self.mask, value)
        self.memory[register] = value

    def write_v2(self, register_value):
        register, value = register_value
        regs = get_regs_from_mask(self.mask, register)
        for reg in regs:
            self.memory[reg] = value


def get_regs_from_mask(mask, register):
    values = get_floating_bit_values(mask)

    no_float_mask, mask_value = get_submasks(mask)
    return [get_masked_register_value(floating_bits, no_float_mask, register, mask_value) for floating_bits in values]


def get_masked_register_value(floating_bits, no_float_mask, register, mask_value):
    no_float_reg_bits = register & no_float_mask
    return no_float_reg_bits | floating_bits | mask_value


def get_submasks(mask):
    # returns a mask for ALL non floating bits and a mask for the value of the non floating mask bits
    mask_value = int("".join(["1" if maskbit == "1" else "0" for maskbit in mask]), 2)
    no_float_mask = int("".join(["1" if maskbit != "X" else "0" for maskbit in mask]), 2)
    return no_float_mask, mask_value


def get_floating_bit_values(mask):
    reversed_mask = list(mask).copy()
    reversed_mask.reverse()
    floating_bits = [pow(2, i) for (i, maskbit) in enumerate(reversed_mask) if maskbit == "X"]

    # get all possible combinations of floating bit values
    values = set()
    for L in range(0, len(floating_bits) + 1):
        for bits in itertools.combinations(floating_bits, L):
            values.add(sum(bits))
    return values


def apply_mask_to_value(mask, value):
    value_mask_nr = int("".join(["1" if maskbit == "X" else "0" for maskbit in mask]), 2)
    mask_mask_nr = int("".join(["1" if maskbit == "1" else "0" for maskbit in mask]), 2)
    new_value = (value_mask_nr & value) + mask_mask_nr
    return new_value


def parse_line_to_instruction(line, version=1):
    split = re.split(" = |\[|\]", line)
    if split[0] == "mask":
        mask = split[1]
        return SeaportComputer.set_mask, mask
    if split[0] == "mem":
        register = int(split[1])
        value = int(split[3])
        if version == 1:
            return SeaportComputer.write_v1, (register, value)
        if version == 2:
            return SeaportComputer.write_v2, (register, value)


def run_computer_and_get_memory(filename, version=1):
    computer = SeaportComputer()
    for line in lineyielder.yield_lines_fp(filename, THIS_DIR):
        instruction, data = parse_line_to_instruction(line, version)
        instruction(computer, data)
    return sum(computer.memory.values())


class Day14Tester(unittest.TestCase):

    def test_example_a(self):
        answer = run_computer_and_get_memory("example.txt")
        self.assertEqual(165, answer)

    def test_a(self):
        answer = run_computer_and_get_memory("input.txt")
        self.assertEqual(9967721333886, answer)

    def test_create_value(self):
        self.assertEqual("00000000000000000000000000001011", '{:032b}'.format(11))

    def test_apply_mask(self):
        value = 11
        mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"
        new_value = apply_mask_to_value(mask, value)
        self.assertEqual(new_value, 73)

    def test_apply_mask_2(self):
        value = 101
        mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"
        new_value = apply_mask_to_value(mask, value)
        self.assertEqual(new_value, 101)

    def test_apply_mask_3(self):
        value = 0
        mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"
        new_value = apply_mask_to_value(mask, value)
        self.assertEqual(new_value, 64)

    def test_get_regs_from_mask(self):
        register = 42
        mask = "000000000000000000000000000000X1001X"
        self.assertEqual([26, 27, 58, 59], sorted(get_regs_from_mask(mask, register)))

    def test_get_regs_from_mask_2(self):
        register = 26
        mask = "00000000000000000000000000000000X0XX"
        self.assertEqual([16, 17, 18, 19, 24, 25, 26, 27], sorted(get_regs_from_mask(mask, register)))

    def test_example_b2(self):
        answer = run_computer_and_get_memory("example2.txt", version=2)
        self.assertEqual(88, answer)

    def test_example_b3(self):
        answer = run_computer_and_get_memory("example3.txt", version=2)
        self.assertEqual(208, answer)

    def test_b(self):
        answer = run_computer_and_get_memory("input.txt", version=2)
        self.assertEqual(4355897790573, answer)
