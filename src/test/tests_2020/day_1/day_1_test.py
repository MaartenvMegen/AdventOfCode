import unittest

from src.utility import lineyielder


class Day1Tester(unittest.TestCase):

    def test_part_a_example(self):
        read_entries = set()
        answer = None
        for line in lineyielder.yield_lines("example1.txt"):
            read_entries.add(int(line))
            if (2020 - int(line)) in read_entries:
                answer = (2020 - int(line)) * int(line)
                break
        self.assertEqual(514579, answer)

    def test_part_a(self):
        read_entries = set()
        answer = None
        for line in lineyielder.yield_lines("parta.txt"):
            read_entries.add(int(line))
            if (2020 - int(line)) in read_entries:
                answer = (2020 - int(line)) * int(line)
                break
        self.assertEqual(870331, answer)

    def test_part_b_example(self):
        answer = None
        expenses = [int(line) for line in lineyielder.yield_lines("example1.txt")]

        for expense in expenses:
            remainder = 2020 - expense
            for inner_expense in expenses:
                leftover = remainder - inner_expense
                if leftover in expenses:
                    answer = expense, inner_expense, leftover
                    break
        self.assertEqual(241861950, answer[0] * answer[1] * answer[2])

    def test_part_b(self):
        answer = None
        expenses = [int(line) for line in lineyielder.yield_lines("parta.txt")]

        for expense in expenses:
            remainder = 2020 - expense
            for inner_expense in expenses:
                leftover = remainder - inner_expense
                if leftover in expenses:
                    answer = expense, inner_expense, leftover
                    break
        self.assertEqual(283025088, answer[0] * answer[1] * answer[2])
