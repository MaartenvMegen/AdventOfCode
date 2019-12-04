import unittest

import numpy


class Day4Tester(unittest.TestCase):
    def test_count_valid_passwords(self):
        range_min = 307237
        range_max = 769058

        total_valids = [number for number in range(range_min,range_max)  if self.is_valid(str(number))]
        self.assertEqual(889, len(total_valids))

        total_valids = [number for number in range(range_min, range_max) if self.is_valid2(str(number))]
        self.assertEqual(589, len(total_valids))

    def is_valid(self, number):
        unique_digits = set([digit for digit in number])

        valid = False

        for digit in unique_digits:
            if number.count(digit) > 1:
                valid = True

        for index, digit in enumerate(number):
            if index > 0 and number[index -1] > number[index]:
                valid = False

        return valid

    def is_valid2(self, number):
        unique_digits = set([digit for digit in number])

        valid = False

        for digit in unique_digits:
            if number.count(digit) == 2:
                valid = True

        for index, digit in enumerate(number):
            if index > 0 and number[index -1] > number[index]:
                valid = False

        return valid

if __name__ == '__main__':
    unittest.main()
