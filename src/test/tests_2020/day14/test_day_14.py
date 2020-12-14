import itertools
import os
import re
import unittest

from src.utility import lineyielder

THIS_DIR = os.path.dirname(os.path.abspath(__file__))


def get_regs_from_mask_v2(mask, register):
    reversed_mask = list(mask).copy()
    reversed_mask.reverse()
    floating_bits = [pow(2, i) for (i, maskbit) in enumerate(reversed_mask) if maskbit == "X"]

    # get all possible combinations of floating bits
    values = set()
    for L in range(0, len(floating_bits) + 1):
        for bits in itertools.combinations(floating_bits, L):
            values.add(sum(bits))

    # use float bit values as masks for the register addresses
    results = []
    value_mask_nr = int("".join(["1" if maskbit == "1" else "0" for maskbit in mask]), 2)
    not_float_mask_nr = int("".join(["1" if maskbit != "X" else "0" for maskbit in mask]), 2)
    for float_values in values:
        # first remove all floating bits from the register value so its easier to OR later
        floats_removed = register & not_float_mask_nr
        results.append(floats_removed | float_values | value_mask_nr)
    return results


def apply_mask_v1(mask, value):
    value_mask_nr = int("".join(["1" if maskbit == "X" else "0" for maskbit in mask]), 2)
    mask_mask_nr = int("".join(["1" if maskbit == "1" else "0" for maskbit in mask]), 2)
    new_value = (value_mask_nr & value) + mask_mask_nr
    return new_value


def parse_line_v1(line, mask, memory):
    split = re.split(" = |\[|\]", line)
    if split[0] == "mask":
        mask = split[1]
    if split[0] == "mem":
        register = split[1]
        value = int(split[3])
        memory[register] = apply_mask_v1(mask, value)
    return mask, memory


def parse_line_v2(line, mask, memory):
    split = re.split(" = |\[|\]", line)
    if split[0] == "mask":
        # print(f"update mask with mask value: {split[1]}")
        mask = split[1]
    if split[0] == "mem":
        register = int(split[1])
        value = int(split[3])
        regs = get_regs_from_mask_v2(mask, register)
        for reg in regs:
            memory[reg] = value
    return mask, memory


class Day14Tester(unittest.TestCase):

    def test_example_a(self):
        mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"
        memory = {}
        for line in lineyielder.yield_lines_fp("example.txt", THIS_DIR):
            mask, memory = parse_line_v1(line, mask, memory)
        print(memory)
        self.assertEqual(165, sum(memory.values()))

    def test_a(self):
        mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"
        memory = {}
        for line in lineyielder.yield_lines_fp("input.txt", THIS_DIR):
            mask, memory = parse_line_v1(line, mask, memory)
        print(memory)
        self.assertEqual(9967721333886, sum(memory.values()))

    def test_create_value(self):
        self.assertEqual("00000000000000000000000000001011", '{:032b}'.format(11))

    def test_apply_mask(self):
        value = 11
        mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"
        new_value = apply_mask_v1(mask, value)
        self.assertEqual(new_value, 73)

    def test_apply_mask_2(self):
        value = 101
        mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"
        new_value = apply_mask_v1(mask, value)
        self.assertEqual(new_value, 101)

    def test_apply_mask_3(self):
        value = 0
        mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"
        new_value = apply_mask_v1(mask, value)
        self.assertEqual(new_value, 64)

    def test_get_regs_from_mask(self):
        register = 42
        mask = "000000000000000000000000000000X1001X"
        self.assertEqual([26, 27, 58, 59], sorted(get_regs_from_mask_v2(mask, register)))

    def test_get_regs_from_mask_2(self):
        register = 26
        mask = "00000000000000000000000000000000X0XX"
        self.assertEqual([16, 17, 18, 19, 24, 25, 26, 27], sorted(get_regs_from_mask_v2(mask, register)))

    def test_example_b2(self):
        mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"
        memory = {}
        for line in lineyielder.yield_lines_fp("example2.txt", THIS_DIR):
            print('parse line')
            mask, memory = parse_line_v2(line, mask, memory)
        self.assertEqual(88, sum(memory.values()))

    def test_example_b2(self):
        mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"
        memory = {}
        for line in lineyielder.yield_lines_fp("example3.txt", THIS_DIR):
            print('parse line')
            mask, memory = parse_line_v2(line, mask, memory)
        self.assertEqual(208, sum(memory.values()))

    def test_b(self):
        mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"
        memory = {}
        for line in lineyielder.yield_lines_fp("input.txt", THIS_DIR):
            print('parse line')
            mask, memory = parse_line_v2(line, mask, memory)
        self.assertEqual(4355897790573, sum(memory.values()))
