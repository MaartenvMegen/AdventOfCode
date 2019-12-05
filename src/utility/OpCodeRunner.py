from enum import Enum


# TODO make instruction respect direct/indirect mode


class Opcode(Enum):
    ADD = 1
    MULTIPLY = 2
    INPUT = 3
    OUTPUT = 4
    HALT = 99


class ParameterMode(Enum):
    POSITION = 0
    IMMEDIATE = 1


class Instruction:

    def __init__(self, opcode, mode_1, mode_2, mode_3):
        self.opcode = opcode
        self.mode_1 = mode_1
        self.mode_2 = mode_2
        self.mode_3 = mode_3

    @staticmethod
    def parse(int_code):
        string_code = str(int_code)
        length = len(string_code)
        if length < 5:
            # add zero padding
            padding = "".join(["0" for _ in range(5 - length)])
            string_code = padding + string_code
        return Instruction(Opcode(int(string_code[3:5])), ParameterMode(int(string_code[2])),
                           ParameterMode(int(string_code[1])), ParameterMode(int(string_code[0])))


class OpcodeRunner:

    def __init__(self, program):
        self.program = program
        self.runnable = program.copy()
        self.pointer = 0
        self.opcode_to_method = {Opcode.ADD: self.add, Opcode.MULTIPLY: self.multiply, Opcode.INPUT: self.get_input,
                                 Opcode.OUTPUT: self.output}

    def reset(self):
        self.runnable = self.program.copy()
        self.pointer = 0

    def run_program(self):
        # run while instruction pointer does not contain halt
        while Instruction.parse(self.runnable[self.pointer]).opcode != Opcode.HALT:
            # do action
            opcode = Instruction.parse(self.runnable[self.pointer]).opcode
            operation = self.opcode_to_method.get(opcode, lambda: print("Unexpected opcode: {}".format(opcode)))
            operation()

    def get_input(self, address):
        print("Please provide program input:")
        input_value = input()
        self.set_value(address, input_value)
        self.pointer += 2

    def output(self, address):
        print("Program result: {}".format(self.load_from_addr_pointer(address)))
        self.pointer += 2

    def set_value(self, address_2, value):
        self.runnable[address_2] = value

    def add(self):
        addres, value1, value2 = self.get_arithmetic_data()
        self.set_value(addres, value1 + value2)

    def multiply(self):
        addres, value1, value2 = self.get_arithmetic_data()
        self.set_value(addres, value1 * value2)

    def get_arithmetic_data(self):
        value1 = self.load_from_addr_pointer(self.pointer + 1)
        value2 = self.load_from_addr_pointer(self.pointer + 2)
        addres = self.runnable[self.pointer + 3]
        self.pointer += 4
        return addres, value1, value2

    def load_from_addr_pointer(self, pointer):
        return self.runnable[self.runnable[pointer]]

    def set_pointer(self, address):
        self.pointer = address

    def get_result(self):
        return self.runnable[0]

    def get_memory(self):
        return self.runnable
