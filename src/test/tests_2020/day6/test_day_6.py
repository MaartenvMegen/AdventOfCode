import os
import unittest
from src.utility import lineyielder

THIS_DIR = os.path.dirname(os.path.abspath(__file__))


def get_answered_by_anyone(group_questionair: str) -> set:
    answers = [set(individual) for individual in group_questionair.split('\n') if individual]
    return set.union(*answers) if answers else set()


def get_answered_by_everyone(group_questionair: str) -> set:
    answers = [set(individual) for individual in group_questionair.split('\n') if individual]
    return set.intersection(*answers) if answers else set()


def get_answers_from_file(filename, evaluation_method) -> list:
    return [evaluation_method(questionair) for questionair in lineyielder.yield_chunks(filename, THIS_DIR, "\n\n")]


class Day6Tester(unittest.TestCase):

    def test_part_a_example(self):
        group_answers = get_answers_from_file("example.txt", get_answered_by_anyone)
        answer = sum([len(answers) for answers in group_answers])
        self.assertEqual(11, answer)

    def test_part_a(self):
        group_answers = get_answers_from_file('input.txt', get_answered_by_anyone)
        answer = sum([len(answers) for answers in group_answers])
        self.assertEqual(6335, answer)

    def test_part_b_example(self):
        group_answers = get_answers_from_file('example.txt', get_answered_by_everyone)
        answer = sum([len(answers) for answers in group_answers])
        self.assertEqual(6, answer)

    def test_part_b(self):
        group_answers = get_answers_from_file('input.txt', get_answered_by_everyone)
        answer = sum([len(answers) for answers in group_answers])
        self.assertEqual(3392, answer)
