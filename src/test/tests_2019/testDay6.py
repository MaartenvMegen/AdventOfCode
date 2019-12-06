import unittest

from src.utility import Inputs
from src.utility.OrbitAnalyzer import OrbitAnalyzer


class Day6Tester(unittest.TestCase):
    def test_example_orbit_input(self):
        orbits = ["COM)B",
                  "B)C",
                  "C)D",
                  "D)E",
                  "E)F",
                  "B)G",
                  "G)H",
                  "D)I",
                  "E)J",
                  "J)K",
                  "K)L"]
        analyzer = OrbitAnalyzer(orbits)
        analyzer.analyse()
        self.assertEqual(42, analyzer.get_orbit_count())

    def test_example_orbital_transfers(self):
        orbits = ["COM)B",
                  "B)C",
                  "C)D",
                  "D)E",
                  "E)F",
                  "B)G",
                  "G)H",
                  "D)I",
                  "E)J",
                  "J)K",
                  "K)L",
                  "K)YOU",
                  "I)SAN"]
        analyzer = OrbitAnalyzer(orbits)
        analyzer.analyse()
        self.assertEqual(4, analyzer.get_transfers("YOU", "SAN"))

    def test_actual_travel(self):
        analyzer = OrbitAnalyzer(Inputs.orbit_input_spec)
        analyzer.analyse()
        self.assertEqual(457, analyzer.get_transfers("YOU", "SAN"))

    def test_actual_input(self):
        analyzer = OrbitAnalyzer(Inputs.orbit_input_spec)
        analyzer.analyse()
        self.assertEqual(295936, analyzer.get_orbit_count())


if __name__ == '__main__':
    unittest.main()
