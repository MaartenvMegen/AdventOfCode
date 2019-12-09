import unittest

from src.utility.GrowingList import GrowingList


class GrowingListTester(unittest.TestCase):
    def test_lookup_grow(self):
        testlist = GrowingList([0, 2, 3])
        self.assertEqual(0, testlist[5])

    def test_put_grow(self):
        testlist = GrowingList([0, 2, 3])
        testlist[5] = 4
        self.assertEqual(4, testlist[5])

    if __name__ == '__main__':
        unittest.main()
