import os
import unittest

from src.utility import lineyielder

THIS_DIR = os.path.dirname(os.path.abspath(__file__))


def parse_boarding_pass(boarding_pass):
    row_spec = boarding_pass[:7]
    column_spec = boarding_pass[7:]

    rows = search(row_spec, range(0, 128), "F", "B")
    columns = search(column_spec, range(0, 8), "L", "R")

    id = rows[0] * 8 + columns[0]
    return id


def search(spec, searchlist, lower_char, upper_char):
    index = 0
    while not len(searchlist) == 1:
        direction = spec[index]
        index += 1

        middle_index = (len(searchlist)) // 2
        if direction == lower_char:
            searchlist = searchlist[:middle_index]
        if direction == upper_char:
            searchlist = searchlist[middle_index:]
    return searchlist


class Day5Tester(unittest.TestCase):

    def test_id(self):
        boarding_pass = "FBFBBFFRLR"
        id = parse_boarding_pass(boarding_pass)
        self.assertEqual(357, id)

        boarding_pass = "FFFBBBFRRR"
        id = parse_boarding_pass(boarding_pass)
        self.assertEqual(119, id)

        boarding_pass = "BBFFBBFRLL"
        id = parse_boarding_pass(boarding_pass)
        self.assertEqual(820, id)

    def test_a(self):
        ids = []
        for line in lineyielder.yield_lines_fp("input.txt", THIS_DIR):
            ids.append(parse_boarding_pass(line))
        self.assertEqual(826, max(ids))

    def test_b(self):
        all_possible_ids = set(range(0, 127 * 8 + 7))

        found_ids = set()
        for line in lineyielder.yield_lines_fp("input.txt", THIS_DIR):
            found_ids.add(parse_boarding_pass(line))

        missing = [id for id in all_possible_ids if id not in found_ids]

        myseat = [id for id in missing if id + 1 in found_ids and id - 1 in found_ids]
        self.assertEqual(678, myseat[0])
