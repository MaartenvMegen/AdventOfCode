import unittest

from src.utility.TractorBeamTester import TractorBeamTester


class Day18Tester(unittest.TestCase):
    def test_if_ship_can_fit(self):
        beamtester = TractorBeamTester()
        result = beamtester.run()
        self.assertEqual(1509773, result)

if __name__ == '__main__':
    unittest.main()
