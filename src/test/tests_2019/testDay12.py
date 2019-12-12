import unittest
from itertools import product


class Moon:
    def __init__(self, position):
        self.position = position
        self.velocity = (0, 0, 0)

    def set_velocity(self, velocity):
        self.velocity = velocity

    def move(self):
        self.position = tuple(item1 + item2 for item1, item2 in zip(self.position, self.velocity))

    def get_relative_pos(self, other):
        return tuple(item1 - item2 for item1, item2 in zip(self.position, other.position))

    def update_velocity(self, other):
        dx, dy, dz = self.get_relative_pos(other)
        inc_x = self.offset(dx)
        inc_y = self.offset(dy)
        inc_z = self.offset(dz)
        velocity_incs = (inc_x, inc_y, inc_z)
        self.velocity = tuple(item1 + item2 for item1, item2 in zip(self.velocity, velocity_incs))

    def get_potential_energy(self):
        return sum([abs(value) for value in self.position])

    def get_kinetic_energy(self):
        return sum([abs(value) for value in self.velocity])

    def get_total_energy(self):
        return self.get_kinetic_energy() * self.get_potential_energy()

    def offset(self, delta):
        if delta < 0:
            return 1
        if delta == 0:
            return 0
        else:
            return -1

    @staticmethod
    def parse_moon(specification):
        xyz = specification.split(",")
        x_spec = int(xyz[0].split("=")[1])
        y_spec = int(xyz[1].split("=")[1])
        z_spec = int(xyz[2].split("=")[1][:-1])

        return Moon((x_spec, y_spec, z_spec))

    def to_string(self):
        x, y, z = self.position
        xv, yv, zv = self.velocity
        return "pos=<x= {}, y= {}, z= {}, vel=<x = {}, y= {}, z= {}>".format(x, y, z, xv, yv, zv)

    def to_string_x(self):
        x, y, z = self.position
        xv, yv, zv = self.velocity
        return "<pos={}, vel={}>".format(x, xv)

    def to_string_y(self):
        x, y, z = self.position
        xv, yv, zv = self.velocity
        return "<pos={}, vel={}>".format(y, yv)

    def to_string_z(self):
        x, y, z = self.position
        xv, yv, zv = self.velocity
        return "<pos={}, vel={}>".format(z, zv)


class Day12Tester(unittest.TestCase):

    def test_actual(self):
        with open("testday12-actual.txt", "r") as file:
            moons = [Moon.parse_moon(line.strip()) for line in file.readlines()]

        for iteration in range(1001):
            print("after {} steps".format(iteration))
            current_energy = sum([moon.get_total_energy() for moon in moons])
            print("current energy: {}".format(current_energy))

            for moon in moons:
                print(moon.to_string())

            for moon, other_moon in product(moons, moons):
                if not moon == other_moon:
                    moon.update_velocity(other_moon)

            for moon in moons:
                moon.move()

        self.assertEqual(9876, current_energy)

    def test_actual_part2(self):
        hist_x = set()
        hist_y = set()
        hist_z = set()

        rep_x_found = False
        rep_y_found = False
        rep_z_found = False

        rep_x = 0
        rep_y = 0
        rep_z = 0

        with open("testday12-actual.txt", "r") as file:
            moons = [Moon.parse_moon(line.strip()) for line in file.readlines()]

        iteration = 0
        while not (rep_x_found and rep_y_found and rep_z_found):

            history_description = self.get_hist_description_x(moons)
            if history_description not in hist_x:
                hist_x.add(history_description)
            elif not rep_x_found:
                print('Found x rep at {}'.format(iteration))
                rep_x_found = True
                rep_x = iteration

            history_description = self.get_hist_description_y(moons)
            if history_description not in hist_y:
                hist_y.add(history_description)
            elif not rep_y_found:
                print('Found y rep at {}'.format(iteration))
                rep_y_found = True
                rep_y = iteration

            history_description = self.get_hist_description_z(moons)
            if history_description not in hist_z:
                hist_z.add(history_description)
            elif not rep_z_found:
                print('Found z rep at {}'.format(iteration))
                rep_z_found = True
                rep_z = iteration

            for moon, other_moon in product(moons, moons):
                if not moon == other_moon:
                    moon.update_velocity(other_moon)

            for moon in moons:
                moon.move()

            iteration += 1

        answer = self.get_lcm(rep_x, rep_y, rep_z)
        self.assertEqual(307043147758488, answer)

    @staticmethod
    def get_lcm(x, y, z):
        print("find common divisors")
        divisors = []
        for index in range(1, 286332 + 1):
            if x % index == 0 and y % index == 0 and z % index == 0:
                print("{} is shared by all numbers as divisor".format(index))
                divisors.append(index)
        total = x * y * z
        greatest_common_divisor = max(divisors)
        answer = total / (greatest_common_divisor * greatest_common_divisor)
        return answer

    def get_hist_description_x(self, moons):
        history_description = ""
        for moon in moons:
            history_description += moon.to_string_x()
        return history_description

    def get_hist_description_y(self, moons):
        history_description = ""
        for moon in moons:
            history_description += moon.to_string_y()
        return history_description

    def get_hist_description_z(self, moons):
        history_description = ""
        for moon in moons:
            history_description += moon.to_string_z()
        return history_description


if __name__ == '__main__':
    unittest.main()
