import os
import unittest

from src.utility import lineyielder

THIS_DIR = os.path.dirname(os.path.abspath(__file__))
import re


def get_answers_from_file(filename):
    answers = []
    group_answers = set()
    evaluation_counter = 0

    for line in lineyielder.yield_lines(os.path.join(THIS_DIR, filename)):
        if not line:
            # store passport if valid and create a new one
            evaluation_counter += 1
            answers.append(group_answers)
            group_answers = set()
        else:
            for char in line:
                group_answers.add(char)

    print(f'evaluated {evaluation_counter} group answers')
    return answers


def get_answers_from_file_part_2(filename):
    answers = []
    group_answers = set()
    evaluation_counter = 0

    start = True
    for line in lineyielder.yield_lines(os.path.join(THIS_DIR, filename)):
        if not line:
            # store passport if valid and create a new one
            evaluation_counter += 1
            answers.append(group_answers)
            print(group_answers)
            group_answers = set()
            start = True
        else:
            if start:
                group_answers.update(set(line))
                start = False
            else:
                group_answers = group_answers.intersection(set(line))

    print(f'evaluated {evaluation_counter} group answers')
    return answers


class Day6Tester(unittest.TestCase):

    def test_part_a_example(self):
        group_answers = get_answers_from_file('example.txt', )
        answer = sum([len(answers) for answers in group_answers])
        self.assertEqual(11, answer)


    def test_part_a(self):
        group_answers = get_answers_from_file('input.txt', )
        answer = sum([len(answers) for answers in group_answers])
        self.assertEqual(6335, answer)


    def test_part_b_example(self):
        group_answers = get_answers_from_file_part_2('example.txt', )
        answer = sum([len(answers) for answers in group_answers])
        self.assertEqual(6, answer)

    def test_part_b(self):
        group_answers = get_answers_from_file_part_2('input.txt', )
        answer = sum([len(answers) for answers in group_answers])
        self.assertEqual(3392, answer)
