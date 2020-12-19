import itertools
import math
import os
import unittest
import re
from src.utility import lineyielder
from parsy import regex, decimal_digit, string, whitespace, seq

THIS_DIR = os.path.dirname(os.path.abspath(__file__))

rule_nr = decimal_digit.at_least(1).concat() << string(":") << whitespace
combos = decimal_digit.at_least(1).concat().sep_by(string(" "))
or_of_combos = seq(combos.sep_by(string(" | ")).tag("combos")).map(dict)
str_value_rule = string('"') >> seq(regex(r'[a-z]').tag("value")).map(dict) << string('"')
option_rule = str_value_rule | or_of_combos
final_parser = seq(number=rule_nr, specification=option_rule)


class Day19Tester(unittest.TestCase):

    def test_parse_rules(self):
        rules = dict()
        for line in lineyielder.yield_lines_fp("example.txt", THIS_DIR):
            rule = final_parser.parse(line)
            rules[rule['number']] = rule['specification']
        parser = self.get_parser_for_rule("0", rules)
        parser.parse("ababbb")

    def test_example_a(self):
        rules = dict()
        for line in lineyielder.yield_lines_fp("example.txt", THIS_DIR):
            rule = final_parser.parse(line)
            rules[rule['number']] = rule['specification']

        valids = 0
        parser = self.get_parser_for_rule("0", rules)
        for line in lineyielder.yield_lines_fp("example_values.txt", THIS_DIR):
            try:
                parser.parse(line)
                valids += 1
            except:
                pass
        print(valids)

    def test_a(self):
        rules = dict()
        for line in lineyielder.yield_lines_fp("rules.txt", THIS_DIR):
            rule = final_parser.parse(line)
            rules[rule['number']] = rule['specification']

        valids = 0
        parser = self.get_parser_for_rule("0", rules)
        for line in lineyielder.yield_lines_fp("input.txt", THIS_DIR):
            try:
                parser.parse(line)
                valids += 1
            except:
                pass
        print(valids)

    def test_example_b(self):
        rules = dict()
        for line in lineyielder.yield_lines_fp("example_b_rules.txt", THIS_DIR):
            rule = final_parser.parse(line)
            rules[rule['number']] = rule['specification']

        parser_42 = self.get_parser_for_rule("42", rules)
        parser_31 = self.get_parser_for_rule("31", rules)

        valids = 0
        for n, m in itertools.product(range(1, 8), repeat=2):
            if n > m:
                parser = seq(parser_42.times(n), parser_31.times(m))
                for line in lineyielder.yield_lines_fp("example_b_values.txt", THIS_DIR):
                    try:
                        parser.parse(line)
                        valids += 1
                    except:
                        pass
        self.assertEqual(12, valids)

    def test_b(self):
        rules = dict()
        for line in lineyielder.yield_lines_fp("rules.txt", THIS_DIR):
            # print(f'parsing {line}')
            rule = final_parser.parse(line)
            rules[rule['number']] = rule['specification']

        parser_42 = self.get_parser_for_rule("42", rules)
        parser_31 = self.get_parser_for_rule("31", rules)
        parser = seq(parser_42.at_least(2), parser_31.times(1))
        for n in range(2, 9):
            # at least 3 at most 2 etc
            parser = parser | seq(parser_42.at_least(n + 1), parser_31.at_most(n))
        # above does not rule out situations where 42 > 2 but 31 still much higher
        valids = 0
        for n, m in itertools.product(range(1, 8), repeat=2):
            # at least 3 at most 2 etc
            if n > m:
                parser = seq(parser_42.times(n), parser_31.times(m))
                for line in lineyielder.yield_lines_fp("input.txt", THIS_DIR):
                    try:
                        parser.parse(line)
                        valids += 1
                    except:
                        pass
        self.assertEqual(321, valids)

    def get_parser_for_rule(self, number, rules):
        # print(f"current rules spec {rules[number]}")
        current_rule = rules[number]
        if "value" in current_rule.keys():
            # print("rules have a value returning validator")
            return string(current_rule["value"])

        outer_parsers = []
        if "combos" in current_rule.keys():
            for combo in current_rule['combos']:
                parsers = []
                for rule_number in combo:
                    parsers.append(self.get_parser_for_rule(rule_number, rules))
                outer_parsers.append(seq(*parsers))

        if len(outer_parsers) > 1:
            parser = outer_parsers[0]
            for i in range(1, len(outer_parsers)):
                # print(f'merging outer parser {i}')
                parser = parser | outer_parsers[i]
        else:
            parser = outer_parsers[0]

        return parser
