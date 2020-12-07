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

    def get_origins(self, bags):
        for origin in self.origins:
            bags.add(origin)
            bags.update(origin.get_origins(bags))
        return bags

    def get_contents(self, bags):
        #print(f'Getting contents for bag {self.color}')

        contents = defaultdict(int)
        for color, amount in self.bag_spec.items():
            #print(f'Getting contents for sub bag {color} of amount {amount}')
            contents[color] += int(amount)
            sub_contents = bags[color].get_contents(bags)
            #print(f'{color} bag contains {sub_contents}')
            for color, spec_amount in sub_contents.items():
                contents[color] += int(amount) * spec_amount
        #print(f'{self.color} bag contains {contents}')

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


class Day7Tester(unittest.TestCase):

    def test_part_a_example(self):
        bags = dict()

        for line in lineyielder.yield_lines_fp("example.txt", THIS_DIR):
            bag = parse_line(line[:-1])
            bags[bag.color] = bag

        # set origins
        for bag in bags.values():
            for color in bag.bag_spec.keys():
                if color in bags.keys():
                    #print(f'adding color {color} to origin {bag.color}')
                    bags[color].add_origin(bag)

        anwers = len(bags['shiny gold'].get_origins(set()))
        self.assertEqual(4, anwers)

    def test_part_a_input(self):
        bags = dict()

        for line in lineyielder.yield_lines_fp("input.txt", THIS_DIR):
            bag = parse_line(line[:-1])
            bags[bag.color] = bag

        # set origins
        for bag in bags.values():
            for color in bag.bag_spec.keys():
                if color in bags.keys():
                    bags[color].add_origin(bag)

        anwers = len(bags['shiny gold'].get_origins(set()))
        self.assertEqual(213, anwers)

    def test_part_b_example(self):
        bags = dict()

        for line in lineyielder.yield_lines_fp("example.txt", THIS_DIR):
            bag = parse_line(line[:-1])
            bags[bag.color] = bag
        answer = sum(bags['shiny gold'].get_contents(bags).values())
        self.assertEqual(32, answer)

    def test_part_b_example_2(self):
        bags = dict()

        for line in lineyielder.yield_lines_fp("example2.txt", THIS_DIR):
            bag = parse_line(line[:-1])
            bags[bag.color] = bag

        answer = sum(bags['shiny gold'].get_contents(bags).values())
        self.assertEqual(126, answer)

    def test_part_b(self):
        bags = dict()

        for line in lineyielder.yield_lines_fp("input.txt", THIS_DIR):
            bag = parse_line(line[:-1])
            bags[bag.color] = bag

        answer = sum(bags['shiny gold'].get_contents(bags).values())
        self.assertEqual(38426, answer)