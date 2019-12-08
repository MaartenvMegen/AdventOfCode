import numpy as np


class ImageDecoder:
    def __init__(self, x, y, inputcode):
        self.x = x
        self.y= y
        self.inputcode = [int(x) for x in str(inputcode)]

    def get_answer_to_part1(self):
        layer_length = self.x * self.y
        layers = self.split_into_chunks(layer_length)
        layer_info = []
        for chunk in layers:
            print(chunk)
            zero_count = chunk.count(0)
            one_count = chunk.count(1)
            two_count = chunk.count(2)
            layer_info.append((zero_count, one_count, two_count))

        print(sorted(layer_info))
        zero_count, one_count, two_count = sorted(layer_info)[0]
        print(one_count*two_count)
        return one_count*two_count

    def render(self):
        layer_length = self.x * self.y
        layers = self.split_into_chunks(layer_length)
        layers.reverse()
        decoded_image = [" " for _ in range(layer_length)]
        number_to_char = {1: "#", 0: " "}
        for layer in layers:
            for digit_index, digit in enumerate(layer):
                if digit < 2:
                    decoded_image[digit_index] = number_to_char[digit]

        rows = [decoded_image[i:i + self.x] for i in range(0, len(decoded_image), self.x)]
        for row in rows:
            print("".join(row))
        return rows

    def split_into_chunks(self, layer_length):
        return [self.inputcode[i:i + layer_length] for i in range(0, len(self.inputcode), layer_length)]
