import itertools
import math
import os
import unittest
import re
from src.utility import lineyielder

THIS_DIR = os.path.dirname(os.path.abspath(__file__))


def parse_line(line):
    return re.findall("\d|\+|\*|\(|\)", line)


def calculate(instructions):
    prev_value = 0
    operator = "+"
    for instruction in instructions:
        if instruction == "+" or instruction == "*":
            operator = instruction
        elif instruction == ")":
            return prev_value
        elif instruction == "(":
            current_value = calculate(instructions)
            prev_value = add_or_mult(prev_value, current_value, operator)
        else:
            current_value = int(instruction)
            prev_value = add_or_mult(prev_value, current_value, operator)
    return prev_value


def calculate_adv(instructions):
    prev_operator = "*"
    mult_list = []
    mult_candidate = None
    sum_list = []

    for instruction in instructions:

        if instruction == "*" and prev_operator == "*":
            # print(f"adding {mult_candidate} to mult list")
            if mult_candidate:
                mult_list.append(mult_candidate)
            mult_candidate = None
        elif instruction == "+" and prev_operator == "*":
            # print(f'adding {mult_candidate} to sumlist')
            sum_list.append(mult_candidate)
            mult_candidate = None
            prev_operator = instruction
        elif instruction == "+" and prev_operator == "+":
            pass
        elif instruction == "*" and prev_operator == "+":
            # print(f'found * current sum = {sum(sum_list)}')
            mult_list.append(sum(sum_list))
            sum_list = []
            prev_operator = instruction
        elif instruction == "(":
            current_value = calculate_adv(instructions)
            if prev_operator == "*":
                # print(f'make {current_value} a mult candidate')
                mult_candidate = current_value
            if prev_operator == "+":
                # print(f'adding {current_value} to sumlist')
                sum_list.append(current_value)
        elif instruction == ")":
            break
        # now instruction must be a value
        elif prev_operator == "*":
            current_value = int(instruction)
            # print(f'make {current_value} a mult candidate')
            mult_candidate = current_value
        elif prev_operator == "+":
            current_value = int(instruction)
            # print(f'adding  {current_value} to sum list')
            sum_list.append(current_value)

    # cleanup
    if mult_candidate:
        mult_list.append(mult_candidate)
    if sum_list:
        mult_list.append(sum(sum_list))
    value = math.prod(mult_list)
    print(f"returning {value}")
    return value


def add_or_mult(a, b, operator):
    if operator == "*":
        return a * b
    else:
        return a + b


class Day18Tester(unittest.TestCase):

    def test_a(self):
        results = []
        for line in lineyielder.yield_lines_fp("input.txt", THIS_DIR):
            result = calculate(iter(parse_line(line)))
            results.append(result)
        self.assertEqual(53660285675207, sum(results))

    def test_line(self):
        line = "1 + 2 * 3 + 4 * 5 + 6"
        self.assertEqual(71, calculate(iter(parse_line(line))))

    def test_advanced(self):
        line = "2 * 3 + (4 * 5)"
        self.assertEqual(46, calculate_adv(iter(parse_line(line))))

    def test_b(self):
        results = []
        for line in lineyielder.yield_lines_fp("input.txt", THIS_DIR):
            result = calculate_adv(iter(parse_line(line)))
            results.append(result)
        self.assertEqual(141993988282687, sum(results))
