import itertools
import threading
import unittest
from threading import Lock

from src.utility import Inputs
from src.utility.OpCodeRunner import OpcodeRunner

event = threading.Event()


class Day7Tester(unittest.TestCase):

    def test_amplifiers(self):
        # Try setting 0-5 as input
        # provide inputs
        phase_settings = [4, 3, 2, 1, 0]
        all_outputs = []
        output = [0]

        for phase in phase_settings:
            input_data = [phase]
            input_data.extend(output)
            program = OpcodeRunner([3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0], input_data)
            program.run_program()
            output = program.get_outputs()
        print("output: {}".format(output))
        all_outputs.append(output[0])

        self.assertEqual(43210, output[0])

    def test_amplifiers_2(self):
        # Try setting 0-5 as input
        # provide inputs
        phase_settings = [0, 1, 2, 3, 4]
        all_outputs = []
        output = [0]

        for phase in phase_settings:
            input_data = [phase]
            input_data.extend(output)
            program = OpcodeRunner([3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23,
                                    101, 5, 23, 23, 1, 24, 23, 23, 4, 23, 99, 0, 0], input_data)
            program.run_program()
            output = program.get_outputs()
        print("output: {}".format(output))
        all_outputs.append(output[0])

        self.assertEqual(54321, output[0])

    def test_amplifier_actual(self):
        # Try setting 0-5 as input
        # provide inputs
        # phase_settings = [4,3,2,1,0]
        possible_setting = list(itertools.permutations(range(0, 5)))
        all_outputs = []
        for setting in possible_setting:
            output = [0]
            print("trying: {}".format(setting))
            for phase in setting:
                input_data = [phase]
                input_data.extend(output)
                program = OpcodeRunner(Inputs.amplifier_code, input_data)
                program.run_program()
                output = program.get_outputs()
            print("output: {}".format(output))
            all_outputs.append(output[0])

        print(sorted(all_outputs))

        self.assertEqual(67023, sorted(all_outputs)[-1])

    def test_amplifier_actual_feedback(self):
        possible_setting = list(itertools.permutations(range(5, 10)))
        all_outputs = []
        for setting in possible_setting:
            result = self.run_amplifiers_and_get_result(Inputs.amplifier_code,setting)
            all_outputs.append(result)

        max_thrust = sorted(all_outputs)[-1]
        print("maximum attainable thrust: {}".format(max_thrust))
        self.assertEqual(7818398, max_thrust)

    def test_feedback_example_1(self):
        memory = [3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26,
                  27, 4, 27, 1001, 28, -1, 28, 1005, 28, 6, 99, 0, 0, 5]

        # create output listeneners

        print("Hooking up amplifiers")
        phase_setting = [9, 8, 7, 6, 5]
        result = self.run_amplifiers_and_get_result(memory, phase_setting)
        self.assertEqual(139629729, result)

    def test_feedback_example_2(self):
        memory = [3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
                  -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
                  53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10]

        # create amplifiers and hookup inputs/outputs
        print("Hooking up amplifiers")
        phase_setting = [9, 7, 8, 5, 6]
        result = self.run_amplifiers_and_get_result(memory, phase_setting)
        self.assertEqual(18216, result)

    def run_amplifiers_and_get_result(self, memory, phase_setting):
        names = ["A", "B", "C", "D", "E"]
        amplifiers = []
        amplifier = None
        # create amplifiers and initialize with the correct settings
        for index, phase in enumerate(phase_setting):
            amplifier = OpcodeRunner(memory, [phase], names[index])
            amplifiers.append(amplifier)
            if len(amplifiers) > 1:
                amplifiers[-2].set_output_listener(amplifier)
        # connect the last back to the first
        amplifier.set_output_listener(amplifiers[0])
        amplifier.set_complete_listeners(self)
        # run the amplifiers
        print("Starting amplifiers")
        for amplifier in amplifiers:
            threading.Thread(target=amplifier.run_program).start()
        # provide start input to amplifier A
        amplifiers[0].send_data(0)
        print("Waiting for amplifiers to complete")
        event.wait()
        event.clear()
        print("Collecting result")
        return amplifiers[-1].get_outputs()[-1]

    def notify(self):
        event.set()
        print("amplifiers completed")


if __name__ == '__main__':
    unittest.main()
