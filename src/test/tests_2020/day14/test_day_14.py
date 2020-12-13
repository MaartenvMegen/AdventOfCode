import os
import unittest

from src.utility import lineyielder

THIS_DIR = os.path.dirname(os.path.abspath(__file__))


class Day14Tester(unittest.TestCase):

    def test_example_a(self):
        for line in lineyielder.yield_lines_fp("example.txt", THIS_DIR):
            print(line)