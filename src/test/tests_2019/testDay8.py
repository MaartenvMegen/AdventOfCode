import unittest

from src.utility import Inputs
from src.utility.ImageDecoder import ImageDecoder


class Day8Tester(unittest.TestCase):
    def test_example_1(self):
        inputcode = 123456789012
        decoder = ImageDecoder(3, 2, inputcode)
        result = decoder.get_answer_to_part1()
        self.assertEqual(1, result)

    def test_actual_input(self):
        inputcode = Inputs.image_input
        decoder = ImageDecoder(25, 6, inputcode)
        result = decoder.get_answer_to_part1()
        self.assertEqual(2286, result)

    def test_example_render(self):
        inputcode = "0222112222120000"
        decoder = ImageDecoder(2, 2, inputcode)
        result = decoder.render()
        self.assertEqual([[' ', '#'], ['#', ' ']], result)

    def test_actual_render(self):
        inputcode = Inputs.image_input
        decoder = ImageDecoder(25, 6, inputcode)
        result = decoder.render()
        rendered_result = [[' ', '#', '#', ' ', ' ', ' ', ' ', '#', '#', ' ', '#', '#', '#', '#', ' ', '#', ' ', ' ', ' ', ' ', '#', '#', '#', ' ', ' '], ['#', ' ', ' ', '#', ' ', ' ', ' ', ' ', '#', ' ', ' ', ' ', ' ', '#', ' ', '#', ' ', ' ', ' ', ' ', '#', ' ', ' ', '#', ' '], ['#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#', ' ', ' ', ' ', '#', ' ', ' ', '#', ' ', ' ', ' ', ' ', '#', ' ', ' ', '#', ' '], ['#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#', ' ', ' ', '#', ' ', ' ', ' ', '#', ' ', ' ', ' ', ' ', '#', '#', '#', ' ', ' '], ['#', ' ', ' ', '#', ' ', '#', ' ', ' ', '#', ' ', '#', ' ', ' ', ' ', ' ', '#', ' ', ' ', ' ', ' ', '#', ' ', ' ', ' ', ' '], [' ', '#', '#', ' ', ' ', ' ', '#', '#', ' ', ' ', '#', '#', '#', '#', ' ', '#', '#', '#', '#', ' ', '#', ' ', ' ', ' ', ' ']]

        self.assertEqual(rendered_result, result)
        # no OJZLP, no DJZLP
        # Crap why cant i read properly
        # o = c



if __name__ == '__main__':
    unittest.main()
