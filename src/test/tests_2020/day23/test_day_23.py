import unittest


class Node():
    def __init__(self, value):
        self.value = value
        self.next = None

    def set_next(self, node):
        self.next = node

    def get_next(self):
        return self.next


class Cups():
    def __init__(self):
        self.head = Node(None)
        self.nodes_map = {}

    def add_num(self, value):
        prev_next = self.head.next
        self.head.next = Node(value)
        new_node = self.head.next
        if prev_next:
            new_node.next = prev_next
        else:
            new_node.next = new_node
        self.head = new_node
        self.nodes_map[value] = new_node

    def cut_next_three(self):
        after_three_ref = self.head.next.next.next.next
        next_three = self.head.next
        self.head.next = after_three_ref
        return next_three

    def insert_three_at_number(self, number, next_tree):
        target_node = self.nodes_map[number]
        target_next = target_node.next
        target_node.next = next_tree
        next_tree.next.next.next = target_next

    def advance_head(self):
        self.head = self.head.next


def print_cups(cups):
    node = cups.head.next
    nr_string = ""
    while node != cups.head:
        nr_string += str(node.value) + " "
        node = node.next
    nr_string += "(" + str(node.value) + ")"
    print(nr_string)


def get_answer_string(cups):
    cups.head = cups.nodes_map[1]
    node = cups.head.next
    nr_string = ""
    while node.next != cups.head:
        nr_string += str(node.value)
        node = node.next
    nr_string += str(node.value)
    return nr_string


class Day23Tester(unittest.TestCase):

    def test_part_a(self):
        input = "389125467"
        cups = self.run_part_1(input)
        self.assertEqual("67384529", get_answer_string(cups))
        # 98573642 is too high

    def test_a(self):
        input = "685974213"
        cups = self.run_part_1(input)
        self.assertEqual("82635947", get_answer_string(cups))

    def run_part_1(self, input):
        nr_list = [int(number) for number in input]
        max_nr = max(nr_list) + 1
        cups = Cups()
        for nr in nr_list:
            cups.add_num(nr)
        cups.head = cups.nodes_map[nr_list[0]]
        for i in range(1, 101):
            print(f'-- move {i} --')
            self.play_round(cups, max_nr)
        print('final')
        print_cups(cups)
        return cups

    def test_b_example(self):
        input = "389125467"
        value_1, value_2 = self.run_part_2(input)
        self.assertEqual(149245887792, value_1 * value_2)

    def test_b(self):
        input = "685974213"
        value_1, value_2 = self.run_part_2(input)
        self.assertEqual(157047826689, value_1 * value_2)

    def run_part_2(self, input):
        nr_list = [int(number) for number in input]
        max_nr = max(nr_list) + 1
        cups = Cups()
        for nr in nr_list:
            cups.add_num(nr)
        print("append large amount of numbers")
        for nr in range(max_nr, 1_000_000 + 1):
            cups.add_num(nr)
        max_nr = 1_000_000 + 1
        print('start playing rouns')
        cups.head = cups.nodes_map[nr_list[0]]
        for i in range(1, 10000000 + 1):
            # print(f'-- move {i} --')
            if i % 100000 == 0:
                print(f'round {i} completed')
            self.play_round(cups, max_nr)
        print('final')
        value_1 = cups.nodes_map[1].next.value
        value_2 = cups.nodes_map[1].next.next.value
        return value_1, value_2

    def play_round(self, cups, max_nr):
        next_three = cups.cut_next_three()
        next_three_values = [next_three.value, next_three.next.value, next_three.next.next.value]
        target = (cups.head.value - 1) % max_nr
        while target in next_three_values or target == 0:
            target = (target - 1) % max_nr

        cups.insert_three_at_number(target, next_three)
        cups.advance_head()
