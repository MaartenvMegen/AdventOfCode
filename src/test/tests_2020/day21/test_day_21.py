import itertools
import math
import os
import unittest
import re
from collections import defaultdict

from src.utility import lineyielder
from parsy import regex, decimal_digit, string, whitespace, seq

THIS_DIR = os.path.dirname(os.path.abspath(__file__))

ingredients = regex(r'[a-z]+').sep_by(whitespace).tag('ingredients').desc('ingredient')
allergens = string(" (contains ") >> regex(r'[a-z]+').sep_by(string(", ")).tag('allergens').desc('allergens') << string(
    ")\n")
toc_element = seq(ingredients, allergens).map(dict)
toc = toc_element.at_least(1)


class Day21Tester(unittest.TestCase):

    def test_set_removal(self):
        set_a = {'mxmxvkd', 'nhms', 'sqjhc', 'kfcds'}
        set_b = {'mxmxvkd'}
        print(set_a - set_b)

    def test_example_a(self):
        with open(os.path.join(THIS_DIR, "example.txt")) as f:
            table_of_contents = toc.parse(f.read())

        all_ingredients, known_ingredient_for_allergen = self.reduce_table_of_contents(table_of_contents)

        # unused ingredients =
        unused = set(all_ingredients.keys()) - set(known_ingredient_for_allergen.values())
        totals = sum([all_ingredients[ingredient] for ingredient in unused])
        print(f'great succes, {totals}')
        self.assertEqual(5, totals)

    def test_example_b(self):
        with open(os.path.join(THIS_DIR, "example.txt")) as f:
            table_of_contents = toc.parse(f.read())

        all_ingredients, known_ingredient_for_allergen = self.reduce_table_of_contents(table_of_contents)

        # unused ingredients =
        unused = set(all_ingredients.keys()) - set(known_ingredient_for_allergen.values())
        totals = sum([all_ingredients[ingredient] for ingredient in unused])

        # sort and print
        answer_b = ",".join([known_ingredient_for_allergen[value] for value in sorted(known_ingredient_for_allergen)])
        self.assertEqual('mxmxvkd,sqjhc,fvjkl', answer_b)
        print(f'great succes, {totals}')

    def test_a_and_b(self):
        with open(os.path.join(THIS_DIR, "input.txt")) as f:
            table_of_contents = toc.parse(f.read())

        all_ingredients, known_ingredient_for_allergen = self.reduce_table_of_contents(table_of_contents)

        # unused ingredients =
        unused = set(all_ingredients.keys()) - set(known_ingredient_for_allergen.values())
        totals = sum([all_ingredients[ingredient] for ingredient in unused])

        print(f'great succes, {totals}')
        self.assertEqual(2436, totals)
        answer_b = ",".join([known_ingredient_for_allergen[value] for value in sorted(known_ingredient_for_allergen)])
        self.assertEqual("dhfng,pgblcd,xhkdc,ghlzj,dstct,nqbnmzx,ntggc,znrzgs", answer_b)

    def reduce_table_of_contents(self, table_of_contents):
        # for each allergen, mark the ingredient as possible
        allergen_info = defaultdict(list)
        all_ingredients = defaultdict(int)
        for content in table_of_contents:
            for allergen in content['allergens']:
                allergen_info[allergen].append(set(content["ingredients"]))
            for ingredient in content['ingredients']:
                all_ingredients[ingredient] += 1

        known_ingredient_for_allergen = dict()
        while len(allergen_info) != len(known_ingredient_for_allergen):
            # reduce the sets if possible
            for allergen, possible_ingredient_sets in allergen_info.items():
                # remove already known elements
                for index, ingredient_set in enumerate(possible_ingredient_sets):
                    ingredient_set = ingredient_set - set(known_ingredient_for_allergen.values())
                    allergen_info[allergen][index] = ingredient_set
                # see if there is common element to multiple sets
                if allergen not in known_ingredient_for_allergen.keys():
                    temp_set = possible_ingredient_sets[0]
                    for i in range(len(possible_ingredient_sets) - 1):
                        temp_set = temp_set.intersection(possible_ingredient_sets[i + 1])
                    if len(temp_set) == 1:
                        print(f'found a known ingredient {temp_set} for {allergen}')
                        known_ingredient_for_allergen[allergen] = temp_set.pop()

        return all_ingredients, known_ingredient_for_allergen
