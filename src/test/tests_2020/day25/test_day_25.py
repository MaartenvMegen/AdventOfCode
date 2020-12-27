import unittest
from numba import jit


@jit()
def decode(pubkey1, pubkey2):
    value = 1
    subject = 7
    div_value = 20201227
    loops = 0
    while value != pubkey1:
        value = value * subject
        value = value % div_value
        loops += 1
    value = 1
    for I in range(loops):
        value = value * pubkey2
        value = value % div_value
    return value


class Day25Tester(unittest.TestCase):

    def test_example(self):
        pubkey = 5764801
        pubkey2 = 17807724
        value = decode(pubkey, pubkey2)
        self.assertEqual(14897079, value)

    def test_pt_1(self):
        pubkey = 14222596
        pubkey2 = 4057428
        value = decode(pubkey, pubkey2)
        self.assertEqual(3286137, value)
