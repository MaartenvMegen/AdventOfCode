import os
import re
import unittest
from collections import defaultdict
from math import prod

from src.utility import lineyielder

THIS_DIR = os.path.dirname(os.path.abspath(__file__))


def parse_rule(rule_string):
    result = re.split(": | or |-", rule_string)
    return result[0], tuple([int(i) for i in result[1:]])


def parse_ticket(ticket_string):
    return [(column, int(number)) for column, number in enumerate(ticket_string.split(','))]


def valid_number_for_rule(number, range):
    return range[0] <= number <= range[1] or range[2] <= number <= range[3]


def check_rules_for_number(number, rules):
    for rule, range in rules.items():
        if valid_number_for_rule(number, range):
            # yield rule name if valid
            yield rule


def get_values_for_departure_columns(myticket, possible_column_names):
    departure_columns = []
    for column, name in possible_column_names.items():
        if "departure" in name:
            departure_columns.append(myticket[column])
    return departure_columns


def get_ticket_column_names(rule_file, ticket_file):
    rules = parse_rules(rule_file)
    column_values = parse_column_values(ticket_file)

    possible_ticket_columns = defaultdict(set)
    ticket_errors = []
    for column, number in column_values:
        valid_rules_for_number = set(check_rules_for_number(number, rules))
        valid_rules_for_column = possible_ticket_columns[column]

        if valid_rules_for_number & valid_rules_for_column:
            possible_ticket_columns[column] = valid_rules_for_column.intersection(valid_rules_for_number)
        elif valid_rules_for_number:
            possible_ticket_columns[column] = valid_rules_for_number
        else:
            ticket_errors.append(number)

    uniques = determine_rule_per_column(possible_ticket_columns)

    return ticket_errors, uniques


def parse_column_values(ticket_file):
    column_values = []
    for line in lineyielder.yield_lines_fp(ticket_file, THIS_DIR):
        column_values.extend(parse_ticket(line))
    return column_values


def determine_rule_per_column(possible_ticket_columns):
    uniques = dict()
    unique = False
    while not unique:
        unique = True
        for column, rules in possible_ticket_columns.items():
            # get rid of values which are unique to another column
            for rule_column, rule in uniques.items():
                # dont throw away your own uniques
                if rule_column != column:
                    rules.discard(rule)
            # add own uniques
            if rules and len(rules) == 1:
                uniques[column] = list(rules)[0]
            # if column is not yet unique, keep processing
            if rules and len(rules) > 1:
                unique = False

    return uniques


def parse_rules(rule_file):
    rules = {}
    for line in lineyielder.yield_lines_fp(rule_file, THIS_DIR):
        rule, values = parse_rule(line)
        rules[rule] = values
    return rules


class Day16Tester(unittest.TestCase):
    def test_parse_rule(self):
        rule_str = "class: 1-3 or 5-7"
        rule, range = parse_rule(rule_str)
        self.assertTrue(valid_number_for_rule(3, range))

    def test_get_rules_for_number(self):
        rules = dict()
        rules["class"] = (1, 3, 5, 7)
        rules["test"] = (7, 10, 13, 15)

        self.assertEqual(["class"], list(check_rules_for_number(5, rules)))
        self.assertEqual([], list(check_rules_for_number(11, rules)))
        self.assertEqual(["test"], list(check_rules_for_number(14, rules)))
        self.assertCountEqual(["test", "class"], list(check_rules_for_number(7, rules)))

    def test_get_uniques(self):
        possible_values = dict()
        possible_values[0] = {'test1'}
        possible_values[1] = {'test1', 'test2'}
        possible_values[2] = {'test1', 'test2', 'test3'}
        uniques = determine_rule_per_column(possible_values)
        self.assertEqual('test1', uniques[0])
        self.assertEqual('test2', uniques[1])
        self.assertEqual('test3', uniques[2])

    def test_parse_ticket(self):
        ticket = "341,509,751,132,486,645,898,425,899,794,699,609,636,807,906,514,63,748,568,535"
        result = [341, 509, 751, 132, 486, 645, 898, 425, 899, 794, 699, 609, 636, 807, 906, 514, 63, 748, 568, 535]
        self.assertEqual(list(zip(range(0, len(result)), result)), parse_ticket(ticket))

    def test_a(self):
        ticket_errors, column_names = get_ticket_column_names("rules.txt", "tickets.txt")
        self.assertEqual(25984, sum(ticket_errors))

    def test_b_example(self):
        ticket_errors, column_names = get_ticket_column_names("rules_example.txt", "tickets_example.txt")
        self.assertEqual("row", column_names[0])
        self.assertEqual("class", column_names[1])
        self.assertEqual("seat", column_names[2])

    def test_b_actual(self):
        myticket = [113, 197, 59, 167, 151, 107, 79, 73, 109, 157, 199, 193, 83, 53, 89, 71, 149, 61, 67, 163]
        ticket_errors, column_names = get_ticket_column_names("rules.txt", "tickets.txt")
        departure_columns = get_values_for_departure_columns(myticket, column_names)
        self.assertEqual(1265347500049, prod(departure_columns))
