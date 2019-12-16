import itertools
import unittest


def perform_fft(string_number):
    pattern = [0, 1, 0, -1]
    totals = []
    for shift in range(0, len(string_number)):
        current_pattern = [ele for ele in pattern for i in range(shift + 1)]
        list_number = []
        for index in range(0, len(string_number)):
            pattern_to_apply = current_pattern[(index + 1) % len(current_pattern)]
            modified_num = int(string_number[index]) * pattern_to_apply
            list_number.append(modified_num)
        totals.append(abs(sum(list_number)) % 10)
    output_string = ""
    current_number = output_string.join([str(total) for total in totals])
    return current_number


class Day16Tester(unittest.TestCase):
    def test_exmple_1(self):
        numbers = ['12345678', '48226158', '34040438', '03415518']
        for phase in range(3):
            current_number = perform_fft(numbers[phase])
            print("after phase {} number is {}:".format(phase + 1, current_number))
            self.assertEqual(numbers[phase + 1], current_number)

    def test_example_2(self):
        current_number = '80871224585914546619083218645595'
        for i in range(100):
            current_number = perform_fft(current_number)
        print("current number 1-8: {}".format(current_number[0:7]))
        self.assertEqual('24176176', current_number[0:8])

    def test_example_3(self):
        current_number = '19617804207202209144916044189917'
        for i in range(100):
            current_number = perform_fft(current_number)
        print("current number 1-8: {}".format(current_number[0:7]))
        self.assertEqual('73745418', current_number[0:8])

    def test_example_4(self):
        current_number = '69317163492948606335995924319873'
        for i in range(100):
            current_number = perform_fft(current_number)
        print("current number 1-8: {}".format(current_number[0:7]))
        self.assertEqual('52432133', current_number[0:8])

    def test_actual_1(self):
        current_number = '59712692690937920492680390886862131901538154314496197364022235676243731306353384700179627460533651346711155314756853419495734284609894966089975988246871687322567664499495407183657735571812115059436153203283165299263503632551949744441033411147947509168375383038493461562836199103303184064429083384309509676574941283043596285161244885454471652448757914444304449337194545948341288172476145567753415508006250059581738670546703862905469451368454757707996318377494042589908611965335468490525108524655606907405249860972187568380476703577532080056382150009356406585677577958020969940093556279280232948278128818920216728406595068868046480073694516140765535007'

        for i in range(100):
            current_number = perform_fft(current_number)
        print("current number 1-8: {}".format(current_number[0:7]))
        self.assertEqual('67481260', current_number[0:8])

    def test_example_1_part_2(self):
        result = '98765432109876543210'
        # test finding message using offset
        offset = 7
        final_message = result[offset: offset + 8]
        self.assertEqual('21098765', final_message)

    def test_example_2_part_2(self):
        # take info from original message

        result = '03036732577212944063491565474664'
        # test finding offset
        offset = int(result[0:7])

        self.assertEqual(303673, offset)

    @unittest.skip
    def test_example_3_part_2(self):
        input_message = '03036732577212944063491565474664'
        offset = int(input_message[0:7])

        current_number = input_message * 10000
        for i in range(100):
            current_number = perform_fft(current_number)

        self.assertEqual('84462026', current_number[offset: offset + 8])


if __name__ == '__main__':
    unittest.main()
