import collections
import operator
import os
import unittest
from math import prod

from src.utility import lineyielder

THIS_DIR = os.path.dirname(os.path.abspath(__file__))


class Day10Tester(unittest.TestCase):

    def test_example_a(self):
        adapters = [int(line) for line in lineyielder.yield_lines_fp("example.txt", THIS_DIR)]
        adapters.append(0)
        adapters = sorted(adapters)
        adapters.append(adapters[-1]+3)
        #print(adapters)

        diff = list(map(operator.sub, adapters[1:], adapters[:-1]))
        #print(diff)
        results = collections.Counter(diff)
        self.assertEqual(7,results[1])
        self.assertEqual(5,results[3])

    def test_example2_a(self):
        adapters = [int(line) for line in lineyielder.yield_lines_fp("example2.txt", THIS_DIR)]
        adapters.append(0)
        adapters = sorted(adapters)
        adapters.append(adapters[-1]+3)
        #print(adapters)

        diff = list(map(operator.sub, adapters[1:], adapters[:-1]))
        #print(diff)
        results = collections.Counter(diff)
        self.assertEqual(22,results[1])
        self.assertEqual(10,results[3])

    def test_a(self):
        adapters = [int(line) for line in lineyielder.yield_lines_fp("input.txt", THIS_DIR)]
        adapters.append(0)
        adapters = sorted(adapters)
        adapters.append(adapters[-1] + 3)
        #print(adapters)

        diff = list(map(operator.sub, adapters[1:], adapters[:-1]))
        #print(diff)
        results = collections.Counter(diff)
        #rint(results)
        #print(f'answer is {results[1]*results[3]}')
        self.assertEqual(2450, results[1]*results[3])

    def get_adapter_config(self, input, index, target ):
        current_value = input[index]
        #print(f'target is {target} current value {current_value}')
        if current_value == target:
            return 1

        # maxdiff is 3 sorted so at most 3 offset evaluations
        # only need to count how many times we reached the end
        result = 0
        for offset in range(1,4):
            if index+offset < len(input):
                if input[index+offset]-current_value <= 3:
                    #print(f'checking adapter {input[index+offset]} from {current_value}')
                    prev_result = self.get_adapter_config(input, index+offset, target)
                    result += prev_result
                else:
                    break
        #print(f'returning {result}')
        return result

    def test_example_b(self):
        adapters = [int(line) for line in lineyielder.yield_lines_fp("example.txt", THIS_DIR)]
        adapters.append(0)
        adapters = sorted(adapters)
        adapters.append(adapters[-1] + 3)

        max_variations = self.get_adapter_config(adapters, 0, adapters[-1] )
        self.assertEqual(8, max_variations)

    def test_example_b2(self):
        adapters = [int(line) for line in lineyielder.yield_lines_fp("example2.txt", THIS_DIR)]
        adapters.append(0)
        adapters = sorted(adapters)
        adapters.append(adapters[-1] + 3)

        max_variations = self.get_adapter_config(adapters, 0, adapters[-1] )
        self.assertEqual(19208, max_variations)

    def test_b(self):
        adapters = [int(line) for line in lineyielder.yield_lines_fp("input.txt", THIS_DIR)]
        adapters.append(0)
        adapters = sorted(adapters)
        adapters.append(adapters[-1] + 3)
        #print(adapters)
        diff = list(map(operator.sub, adapters[1:], adapters[:-1]))

        # get chunks with diff value 1, ignore diff value 3 as it does not offer any possible variations
        chunks = []
        prev = 0
        chunk = []
        for diff_value in diff:
            if prev != 1 and diff_value == 1:
                chunks.append(chunk)
                chunk = [1]
            elif diff_value == 1:
                chunk.append(1)
            prev = diff_value
        #print(chunks)

        # calculate possible variations for each chunk with diffs of 1
        chunk_vars = []
        for chunk in chunks:
            if chunk:
                chunk_range = range(0,len(chunk)+1)
                #print(list(chunk_range))
                variations = self.get_adapter_config(chunk_range, 0, chunk_range[-1])
                #print(f'max variations for chunk {chunk} = {variations}')
                chunk_vars.append(variations)
        #print(chunk_vars)
        answer = prod(chunk_vars)
        #print(answer)

        self.assertEqual(32396521357312, answer)