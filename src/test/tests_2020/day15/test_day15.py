import os
import unittest
from collections import defaultdict
from numba import njit, numba
from numba.typed import List
THIS_DIR = os.path.dirname(os.path.abspath(__file__))


@njit()
def take_turn(spoken_last, turn, memory):
    if spoken_last != 0:
        word = turn - 1 - spoken_last
    else:
        word = 0

    if memory[word] != -1:
        last_spoken = memory[word]
    else:
        last_spoken = 0

    memory[word] = turn

    return word, last_spoken

@njit()
def play_game(starting_words, until=2020):
    # Init memory
    memory = [-1] * until

    # Starting phase: Initialize with starting words
    for turn, word in enumerate(starting_words, 1):
        memory[word] = turn

    # Play game until desired end time
    prev_word = starting_words[-1]
    spoken_last = 0


    for turn in range(len(starting_words) + 1, until + 1):
        prev_word, spoken_last = take_turn(spoken_last, turn, memory)

    # Once done return the last word that was spoken
    return prev_word


class Day15Tester(unittest.TestCase):
    def test_example_a1(self):
        prev_word = play_game([0, 3, 6])
        self.assertEqual(436, prev_word)

    def test_example_a2(self):
        prev_word = play_game([1, 3, 2])
        self.assertEqual(1, prev_word)

    def test_example_a3(self):
        prev_word = play_game([2, 1, 3])
        self.assertEqual(10, prev_word)

    def test_example_a4(self):
        prev_word = play_game([1, 2, 3])
        self.assertEqual(27, prev_word)

    def test_a(self):
        prev_word = play_game([17, 1, 3, 16, 19, 0])
        self.assertEqual(694, prev_word)

    def test_example_b1(self):
        prev_word = play_game(List([0,3,6]), until=30000000)
        self.assertEqual(175594, prev_word)

    def test_b(self):
        numba_list = List()
        [numba_list.append(x) for x in [17, 1, 3, 16, 19, 0]]
        prev_word = play_game(numba_list, until=30000000)
        self.assertEqual(21768614, prev_word)

