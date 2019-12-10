import unittest

from src.utility import Inputs
from src.utility.AsteroidDestroyer import rotate_and_kill, get_best_asteroid, get_asteroids


class Day10Tester(unittest.TestCase):
    def test_example_1(self):
        input_map = [".#..#",
                     ".....",
                     "#####",
                     "....#",
                     "...##"]
        asteroids = get_asteroids(input_map)
        asteroid_count, base_location = get_best_asteroid(asteroids)
        self.assertEqual((3, 4), base_location)
        self.assertEqual(8, asteroid_count)

    def test_example_2(self):
        input_map = ["......#.#.",
                     "#..#.#....",
                     "..#######.",
                     ".#.#.###..",
                     ".#..#.....",
                     "..#....#.#",
                     "#..#....#.",
                     ".##.#..###",
                     "##...#..#.",
                     ".#....####"]
        asteroids = get_asteroids(input_map)
        asteroid_count, base_location = get_best_asteroid(asteroids)
        self.assertEqual((5, 8), base_location)
        self.assertEqual(33, asteroid_count)

    def test_actual(self):
        input_map = Inputs.asteroid_map
        asteroids = get_asteroids(input_map)

        asteroid_count, base_location = get_best_asteroid(asteroids)
        print("found {} asteroids on point {}".format(asteroid_count, base_location))

        kill_list = rotate_and_kill(base_location, asteroids)

        print("The 200th asteroid destroyed is: {}".format(kill_list[199]))
        print("answer: {}".format(kill_list[199][0] * 100 + kill_list[199][1]))
        self.assertEqual(512, kill_list[199][0] * 100 + kill_list[199][1])

    def test_sample_destroy(self):
        input_map = [".#..##.###...#######",
                     "##.############..##.",
                     ".#.######.########.#",
                     ".###.#######.####.#.",
                     "#####.##.#.##.###.##",
                     "..#####..#.#########",
                     "####################",
                     "#.####....###.#.#.##",
                     "##.#################",
                     "#####.##.###..####..",
                     "..######..##.#######",
                     "####.##.####...##..#",
                     ".#####..#.######.###",
                     "##...#.##########...",
                     "#.##########.#######",
                     ".####.#.###.###.#.##",
                     "....##.##.###..#####",
                     ".#.#.###########.###",
                     "#.#.#.#####.####.###",
                     "###.##.####.##.#..##"]
        asteroids = get_asteroids(input_map)

        asteroid_count, base_location = get_best_asteroid(asteroids)
        print("found {} asteroids on point {}".format(asteroid_count, base_location))
        kill_list = rotate_and_kill(base_location, asteroids)
        self.assertEqual((8, 2), kill_list[199], "The 200th destroyed is asteroid is not at this location")


if __name__ == '__main__':
    unittest.main()
