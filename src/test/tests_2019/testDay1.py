import math
import unittest

module_weight = [122281,
                 124795,
                 58593,
                 133744,
                 67625,
                 109032,
                 50156,
                 80746,
                 130872,
                 79490,
                 126283,
                 146564,
                 73075,
                 130170,
                 139853,
                 92599,
                 96965,
                 58149,
                 94254,
                 89074,
                 52977,
                 148092,
                 92073,
                 136765,
                 144755,
                 142487,
                 54827,
                 135588,
                 91411,
                 51597,
                 70040,
                 68880,
                 117120,
                 137115,
                 72829,
                 100048,
                 65187,
                 131464,
                 95813,
                 146891,
                 128799,
                 94568,
                 67178,
                 94903,
                 67193,
                 127613,
                 115782,
                 85360,
                 129820,
                 50989,
                 63471,
                 106724,
                 145768,
                 55169,
                 77555,
                 82978,
                 87728,
                 69141,
                 95518,
                 82985,
                 83387,
                 83089,
                 64372,
                 127931,
                 99277,
                 58930,
                 99098,
                 95621,
                 147797,
                 64102,
                 118857,
                 71014,
                 84881,
                 147294,
                 72166,
                 71348,
                 149240,
                 117963,
                 89181,
                 144770,
                 102444,
                 99103,
                 72341,
                 56076,
                 128515,
                 51319,
                 147595,
                 98431,
                 141102,
                 148617,
                 84685,
                 111427,
                 82351,
                 57021,
                 63834,
                 113059,
                 119970,
                 87078,
                 120631,
                 124942
                 ]


class Day1Tester(unittest.TestCase):
    total_fuel = 0

    def test_total_module(self):
        total_fuel = self.get_total_fuel_req(module_weight)
        print("Total fuel  {}".format(total_fuel))

    def get_total_fuel_req(self, modules):
        total_fuel = 0
        for weight in modules:
            module_fuel = self.calculate_fuel_req(weight, 0)
            total_fuel += module_fuel

        return total_fuel

    def test_module_weight_100756(self):
        total_fuel = self.get_total_fuel_req([100756])
        self.assertEqual(50346, total_fuel)

    def test_module_weight_1969(self):
        total_fuel = self.get_total_fuel_req([1969])
        self.assertEqual(966, total_fuel)

    def test_module_weight_14(self):
        fuel_reqs = sum([math.floor(weight / 3) - 2 for weight in [14]])
        self.total_fuel += fuel_reqs
        self.calculate_fuel_req(fuel_reqs, 0)
        print("Total fuel for module {}".format(fuel_reqs))
        print("total fuel  {}".format(self.total_fuel))
        self.assertEqual(2, self.total_fuel)

    def calculate_fuel_req(self, weight, prev_total):
        fuel_required_for_weight = math.floor(weight / 3) - 2

        if fuel_required_for_weight > 0:
            fuel_required = self.calculate_fuel_req(fuel_required_for_weight, prev_total)
            return fuel_required_for_weight + fuel_required
        else:
            return 0


if __name__ == '__main__':
    unittest.main()
