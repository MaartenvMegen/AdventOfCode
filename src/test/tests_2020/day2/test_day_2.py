import os
import unittest

from src.utility import lineyielder

THIS_DIR = os.path.dirname(os.path.abspath(__file__))
import re


def get_valid_passwords_from_file(filename, validator):
    count = 0
    for line in lineyielder.yield_lines(os.path.join(THIS_DIR, filename)):
        (minrep, maxrep, char, _, passwd) = re.split(' |:|-', line)
        if validator(minrep, maxrep, char, passwd):
            count += 1
    return count


def validate_toboggen(pos_a, pos_b, char, passwd):
    return (passwd[int(pos_a) - 1] == char) != (passwd[int(pos_b) - 1] == char)


def validate_slide(minrep, maxrep, char, passwd):
    occurence = passwd.count(char)
    return int(minrep) <= occurence <= int(maxrep)


class Day2Tester(unittest.TestCase):

    def test_part_a_example(self):
        count = get_valid_passwords_from_file('example1.txt', validate_slide)
        self.assertEqual(2, count)

    def test_part_a(self):
        count = get_valid_passwords_from_file('part_a.txt', validate_slide)
        self.assertEqual(493, count)

    def test_part_b_example(self):
        count = get_valid_passwords_from_file("example1.txt", validate_toboggen)
        self.assertEqual(1, count)

    def test_part_b(self):
        count = get_valid_passwords_from_file("part_a.txt", validate_toboggen)
        self.assertEqual(593, count)
