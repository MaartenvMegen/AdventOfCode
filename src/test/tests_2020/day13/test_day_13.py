import math
import unittest

import numpy as np


def get_chinese_remainder(num, rem):
    outcomes = []
    prod = math.prod(num)

    print(num)
    print(rem)
    for i, number in enumerate(num):
        pp = prod // number
        inv = pow(pp, -1, number)
        print(f' pp {pp}, inv {inv} , rem {rem[i]}')
        outcomes.append(rem[i] * pp * inv)
    print(sum(outcomes) % prod)
    return sum(outcomes) % prod


def get_next_bus(busses, target):
    remaining_time = []
    for bus in busses:
        remaining_time.append(bus - target % bus)
    time_per_bus = zip(busses, remaining_time)
    bus, time = sorted(time_per_bus, key=lambda x: x[1])[0]
    return bus, time


class Day13Tester(unittest.TestCase):

    def test_example_a(self):
        target = 939
        busses = [7, 13, 59, 31, 19]
        bus, time = get_next_bus(busses, target)
        answer = bus * time
        self.assertEqual(295, answer)

    def test_a(self):
        with open("input.txt") as input:
            target = int(input.readline())
            busses = input.readline().strip().split(",")
        busses = [int(bus) for bus in busses if bus != "x"]

        bus, time = get_next_bus(busses, target)
        self.assertEqual(1915, bus * time)

    def test_b(self):
        with open("input.txt") as input:
            _ = input.readline()
            busses = input.readline().strip().split(",")
        print(busses)

        bus_order = [(int(nr), lane) for lane, nr in enumerate(busses) if nr != 'x']
        bus_order = [(int(nr), lane) for lane, nr in enumerate(busses) if nr != 'x']
        num, rem = zip(*bus_order)
        rem = [(num - rem) % num for rem, num in zip(rem, num)]

        self.assertEqual(294354277694107, get_chinese_remainder(num, rem))

    def test_b_chinese_example(self):
        num = [3, 4, 5]
        rem = [2, 3, 1]

        self.assertEqual(11, get_chinese_remainder(num, rem)
                         )

    def test_b_chinese_example_given(self):
        num = [17, 13, 19]
        rem = [0, 2, 3]
        rem = [(num - rem) % num for rem, num in zip(rem, num)]

        self.assertEqual(3417, get_chinese_remainder(num, rem)
                         )

    def test_example_b(self):
        with open("example.txt") as input:
            _ = input.readline()
            busses = input.readline().strip().split(",")
        print(busses)

        bus_order = [(int(nr), lane) for lane, nr in enumerate(busses) if nr != 'x']
        num, rem  = zip(*bus_order)
        rem = [(num - rem) % num for rem, num in zip(rem, num)]

        remainder = get_chinese_remainder(num, rem)
        self.assertEqual(1068781, remainder)

    def test_inv_mod(self):
        self.assertEqual(2, pow(20, -1, 3))
