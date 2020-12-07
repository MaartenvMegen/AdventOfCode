import os
import unittest
from collections import defaultdict

from src.utility import lineyielder

THIS_DIR = os.path.dirname(os.path.abspath(__file__))


class Bag():
    def __init__(self, color, bag_spec : dict):
        self.color = color
        self.bag_spec = bag_spec
        self.origins = set()

    def add_origin(self, bag):
        self.origins.add(bag)

    def get_origins(self):
        bags = set()
        for origin in self.origins:
            bags.add(origin)
            bags.update(origin.get_origins())
        return bags

    def get_contents(self, bags):
        contents = defaultdict(int)
        for color, amount in self.bag_spec.items():
            contents[color] += int(amount)
            sub_contents = bags[color].get_contents(bags)
            for color, spec_amount in sub_contents.items():
                contents[color] += int(amount) * spec_amount
        return contents


def parse_line(line):
    bag_color, content = line.split(' bags contain ')
    contains_bags = content.split(", ")
    bag_spec = dict()
    for bag in contains_bags:
        if bag != "no other bags":
            amount, color_str = bag.split(' ',1)
            color = color_str.split(" bag", 1)
            bag_spec[color[0]] = amount

    return Bag(bag_color, bag_spec)


def get_bags_from_file(filename):
    bags = dict()
    for line in lineyielder.yield_lines_fp(filename, THIS_DIR):
        bag = parse_line(line[:-1])
        bags[bag.color] = bag
    return bags


def set_origins(bags):
    [bags[color].add_origin(bag) for bag in bags.values() for color in bag.bag_spec.keys()]


class Day7Tester(unittest.TestCase):

    def test_part_a_example(self):
        bags = get_bags_from_file('example.txt')
        set_origins(bags)
        anwers = len(bags['shiny gold'].get_origins())
        self.assertEqual(4, anwers)

    def test_part_a_input(self):
        bags = get_bags_from_file('input.txt')
        set_origins(bags)
        anwers = len(bags['shiny gold'].get_origins())
        self.assertEqual(213, anwers)

    def test_part_b_example(self):
        bags = get_bags_from_file('example.txt')
        answer = sum(bags['shiny gold'].get_contents(bags).values())
        self.assertEqual(32, answer)

    def test_part_b_example_2(self):
        bags = get_bags_from_file('example2.txt')
        answer = sum(bags['shiny gold'].get_contents(bags).values())
        self.assertEqual(126, answer)

    def test_part_b(self):
        bags = get_bags_from_file('input.txt')
        answer = sum(bags['shiny gold'].get_contents(bags).values())
        self.assertEqual(38426, answer)