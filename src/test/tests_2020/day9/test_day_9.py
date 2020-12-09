import itertools
import os
import unittest

from src.utility import lineyielder

THIS_DIR = os.path.dirname(os.path.abspath(__file__))


def get_failure(input, pre_amble):
    for index, number in enumerate(input):
        # start checking input
        if index >= pre_amble:
            found = False

            for x, y in itertools.combinations(input[index - pre_amble:index], 2):
                # print(f'checking sum {x} + {y} = {x + y} === {input[index]}')
                if x + y == input[index]:
                    found = True
                    print(f'match found for {x} + {y} = {x + y} === {input[index]}')

            if not found:
                print(f'no match found for {input[index]} at index {index}')
                return input[index]
    return None


def get_failure_efficient(input, pre_amble):
    for index, number in enumerate(input):
        # start checking input
        if index >= pre_amble:
            found = False
            check_range = input[index - pre_amble:index]
            for check_number in check_range:
                other_number = number - check_number
                if other_number in check_range and other_number != check_number:
                    found = True
            if not found:
                print(f'no match found for {input[index]} at index {index}')
                return input[index]
    return None


def get_cont_sum(input, desired_value):
    for index in range(0, len(input)):
        for search_index in range(index + 1, len(input)):
            print(f'checking {input[index]} to {input[search_index]}')
            contigious_sum = sum(input[index:search_index])
            if contigious_sum == desired_value:
                print(f'found! for {input[index]} to {input[search_index - 1]}')
                min_value = min(input[index:search_index])
                max_value = max(input[index:search_index])
                answer = min_value + max_value
                print(f'answer is {answer}')
                return answer
    return None


def get_cont_sum_efficient(input, desired_value):
    cumsum = calculate_cumsum(input)

    for index in range(0, len(input)):
        for search_index in range(index + 1, len(input)):

            # print(f'checking {input[index]} to {input[search_index]}')
            contigious_sum = cumsum[search_index - 1] - cumsum[index - 1]
            if contigious_sum > desired_value:
                # no need to expand range
                break

            if contigious_sum == desired_value:
                print(f'found! for {input[index]} to {input[search_index - 1]}')
                min_value = min(input[index:search_index])
                max_value = max(input[index:search_index])
                answer = min_value + max_value
                print(f'answer is {answer}')
                return answer
    return None


def calculate_cumsum(input):
    cumsum = []
    for index, value in enumerate(input):
        # print(index)
        if cumsum:
            cumsum.append(cumsum[index - 1] + value)
        else:
            cumsum = [value]
    return cumsum


class Day9Tester(unittest.TestCase):

    def test_input_100_invalid(self):
        pre_amble = 25
        input = [value for value in range(1, 26)]
        input.append(100)

        self.assertEqual(100, get_failure_efficient(input, pre_amble))

    def test_input_50_invalid(self):
        pre_amble = 25
        input = [value for value in range(1, 26)]
        input.append(50)

        self.assertEqual(50, get_failure_efficient(input, pre_amble))

    def test_input_26_valid(self):
        pre_amble = 25
        input = [value for value in range(1, 26)]
        input.append(26)

        self.assertIsNone(get_failure_efficient(input, pre_amble))

    def test_input_49_valid(self):
        pre_amble = 25
        input = [value for value in range(1, 26)]
        input.append(49)

        self.assertIsNone(get_failure_efficient(input, pre_amble))

    def test_example(self):
        pre_amble = 5
        input = [int(line) for line in lineyielder.yield_lines_fp("example.txt", THIS_DIR)]

        self.assertEqual(127, get_failure_efficient(input, pre_amble))

    def test_input(self):
        pre_amble = 25
        input = [int(line) for line in lineyielder.yield_lines_fp("input.txt", THIS_DIR)]

        self.assertEqual(258585477, get_failure_efficient(input, pre_amble))

    def test_part_b(self):
        desired_value = 258585477
        input = [int(line) for line in lineyielder.yield_lines_fp("input.txt", THIS_DIR)]
        self.assertEqual(36981213, get_cont_sum_efficient(input, desired_value))

    def test_part_b_example(self):
        desired_value = 127
        input = [int(line) for line in lineyielder.yield_lines_fp("example.txt", THIS_DIR)]
        self.assertEqual(62, get_cont_sum_efficient(input, desired_value))

    def test_cumsum(self):
        input = range(1, 10)
        cumsum = calculate_cumsum(input)
        print(list(input))
        print(cumsum)
        print(sum(input[3:5]))
        # with respect to original slicer - > upper and lower -1
        print(cumsum[4] - cumsum[2])
