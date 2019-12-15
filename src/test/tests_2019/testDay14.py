import os
import re
import unittest
from collections import defaultdict

THIS_DIR = os.path.dirname(os.path.abspath(__file__))

resource_list = {}
shopping_list = defaultdict(int)
recipe_count_resource = defaultdict(int)


class Component:

    def __init__(self, name, min_amount, resources):
        self.name = name
        self.min_amount = min_amount
        self.resources = resources
        self.origin = set()
        self.desired_amount = 0
        self.expected_in_x_recipes = 0
        self.current_recipe_requests = 0

    def produce(self, amount_of_component, produced_items):
        self.desired_amount += amount_of_component
        self.current_recipe_requests += 1
        test = recipe_count_resource
        # print("Checking if {} {} can be produced, {}/{}. Current produced items {}".format(self.desired_amount,self.name, self.current_recipe_requests, recipe_count_resource[self.name],produced_items))
        if self.name == "ORE" or self.current_recipe_requests >= recipe_count_resource[self.name]:
            # did all recipes mentioning this component get processed? -> start fabricating
            produced_items.add(self.name)

            # how many do we need
            remainder = self.desired_amount % self.min_amount
            if self.min_amount == 1 or remainder == 0:
                production_requirement = self.desired_amount
            else:
                production_requirement = self.desired_amount + self.min_amount - remainder

            # print("Producing {} {}, Wasting {}".format(production_requirement, self.name, production_requirement-self.desired_amount))
            shopping_list[self.name] = production_requirement
            for amount, resource in self.resources:
                required_resource = amount * production_requirement // self.min_amount
                # print(
                #     "to make {} of {}, I would like {} of {}".format(production_requirement, self.name, required_resource, resource))

                resource_list[resource].produce(required_resource, produced_items)

    def set_origin(self, ancestors):
        self.origin.update(ancestors)
        new_ancestors = ancestors.copy()
        new_ancestors.add(self.name)
        for _, resource in self.resources:
            resource_list[resource].set_origin(new_ancestors)


class MyTestCase(unittest.TestCase):

    def setUp(self) -> None:
        resource_list.clear()
        shopping_list.clear()
        recipe_count_resource.clear()

    def test_something(self):
        with open(os.path.join(THIS_DIR, "testday14_example.txt"), "r") as file:
            [self.parse_recipe(re.split(", | => | ", line.strip())) for line in file.readlines()]

        resource_list["ORE"] = Component("ORE", 1, [])

        # this sets all of the origins so we can check if we are allowed to produce
        resource_list["FUEL"].set_origin(set())
        resource_list["FUEL"].produce(1, set())

        self.assertEqual(31, shopping_list['ORE'])

    def test_something_2(self):
        with open(os.path.join(THIS_DIR, "testday14_example_2.txt"), "r") as file:
            [self.parse_recipe(re.split(", | => | ", line.strip())) for line in file.readlines()]

        resource_list["ORE"] = Component("ORE", 1, [])

        # this sets all of the origins so we can check if we are allowed to produce
        resource_list["FUEL"].set_origin(set())
        resource_list["FUEL"].produce(1, set())

        self.assertEqual(165, shopping_list['ORE'])

    def test_something_3(self):
        with open(os.path.join(THIS_DIR, "testday14_example_3.txt"), "r") as file:
            [self.parse_recipe(re.split(", | => | ", line.strip())) for line in file.readlines()]

        resource_list["ORE"] = Component("ORE", 1, [])

        # this sets all of the origins so we can check if we are allowed to produce
        resource_list["FUEL"].set_origin(set())
        resource_list["FUEL"].produce(1, set())

        self.assertEqual(13312, shopping_list['ORE'])

    def test_something_actual(self):
        with open(os.path.join(THIS_DIR, "testday14_input.txt"), "r") as file:
            [self.parse_recipe(re.split(", | => | ", line.strip())) for line in file.readlines()]

        resource_list["ORE"] = Component("ORE", 1, [])

        # this sets all of the origins so we can check if we are allowed to produce
        resource_list["FUEL"].set_origin(set())
        resource_list["FUEL"].produce(1, set())

        self.assertEqual(168046, shopping_list['ORE'])

    def test_something_actual_part2(self):

        for fuel in range(6972986 - 1, 6972986 + 1, 1):
            resource_list.clear()
            shopping_list.clear()
            recipe_count_resource.clear()

            with open(os.path.join(THIS_DIR, "testday14_input.txt"), "r") as file:
                [self.parse_recipe(re.split(", | => | ", line.strip())) for line in file.readlines()]

            resource_list["ORE"] = Component("ORE", 1, [])

            # this sets all of the origins so we can check if we are allowed to produce
            resource_list["FUEL"].set_origin(set())
            resource_list["FUEL"].produce(fuel, set())

            if fuel > 6972986:
                self.assertLess(1000000000000, shopping_list["ORE"])
            else:
                self.assertGreater(1000000000000, shopping_list["ORE"])

    @staticmethod
    def parse_recipe(specs):
        component_name = specs[-1]
        min_producable_amount = int(specs[-2])
        resources = [(int(amount), component) for amount, component in zip(specs[0:-2:2], specs[1:-2:2])]
        for _, component in resources:
            recipe_count_resource[component] += 1
        # print(
        #     "Component {}, min amount: {}. created using: {}".format(component_name, min_producable_amount, resources))
        component = Component(component_name, min_producable_amount, resources)
        resource_list[component_name] = component


if __name__ == '__main__':
    unittest.main()
