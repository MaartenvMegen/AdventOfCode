from enum import Enum


# TODO make instruction respect direct/indirect mode


class Opcode(Enum):
    ADD = 1
    MULTIPLY = 2
    INPUT = 3
    OUTPUT = 4
    JUMP_IF_TRUE = 5
    JUMP_IF_FALSE = 6
    LESS_THAN = 7
    EQUALS = 8
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

    def __init__(self, program, inputs=None):
        self.program = program
        self.runnable = program.copy()
        self.pointer = 0
        self.opcode_to_method = {Opcode.ADD: self.add, Opcode.MULTIPLY: self.multiply, Opcode.INPUT: self.get_input,
                                 Opcode.OUTPUT: self.output, Opcode.JUMP_IF_TRUE: self.jump_if_true,
                                 Opcode.JUMP_IF_FALSE: self.jump_if_false, Opcode.LESS_THAN: self.less_than,
                                 Opcode.EQUALS: self.equals}
        if inputs:
            self.inputs = inputs.copy()
        else:
            self.inputs = None
        self.outputs = []

    def reset(self):
        self.runnable = self.program.copy()
        self.pointer = 0

    def run_program(self):
        # run while instruction pointer does not contain halt
        while Instruction.parse(self.runnable[self.pointer]).opcode != Opcode.HALT:
            # do action
            instruction = Instruction.parse(self.runnable[self.pointer])
            operation = self.opcode_to_method.get(instruction.opcode,
                                                  lambda: print("Unexpected opcode: {}".format(instruction.opcode)))
            operation(instruction)

    def set_value(self, address_2, value):
        self.runnable[address_2] = value

    def get_input(self, instruction):
        input_value = self.inputs.pop(0)
        address = self.runnable[self.pointer + 1]
        self.set_value(address, input_value)
        self.pointer += 2

    def output(self, instruction):
        print("Program result: {}".format(self.load_from_addr_pointer(self.pointer + 1)))
        self.outputs.append(self.load_from_addr_pointer(self.pointer + 1))
        self.pointer += 2

    def get_value(self, pointer, mode):
        if mode == ParameterMode.POSITION:
            return self.load_from_addr_pointer(pointer)
        elif mode == ParameterMode.IMMEDIATE:
            return self.runnable[pointer]
        else:
            print("ERROR: unexpected mode")
            return None

    def add(self, instruction):
        value_1 = self.get_value(self.pointer + 1, instruction.mode_1)
        value_2 = self.get_value(self.pointer + 2, instruction.mode_2)
        address = self.runnable[self.pointer + 3]

        self.set_value(address, value_1 + value_2)
        self.pointer += 4

    def multiply(self, instruction):
        value_1 = self.get_value(self.pointer + 1, instruction.mode_1)
        value_2 = self.get_value(self.pointer + 2, instruction.mode_2)
        address = self.runnable[self.pointer + 3]

        self.set_value(address, value_1 * value_2)
        self.pointer += 4

    def jump_if_true(self, instruction):
        value_1 = self.get_value(self.pointer + 1, instruction.mode_1)
        if value_1 != 0:
            self.pointer = self.get_value(self.pointer + 2, instruction.mode_2)
        else:
            self.pointer += 3

    def jump_if_false(self, instruction):
        value_1 = self.get_value(self.pointer + 1, instruction.mode_1)
        if value_1 == 0:
            self.pointer = self.get_value(self.pointer + 2, instruction.mode_2)
        else:
            self.pointer += 3

    def less_than(self, instruction):
        value_1 = self.get_value((self.pointer + 1), instruction.mode_1)
        value_2 = self.get_value(self.pointer + 2, instruction.mode_2)
        address = self.runnable[self.pointer + 3]
        if value_1 < value_2:
            self.set_value(address, 1)
        else:
            self.set_value(address, 0)
        self.pointer += 4

    def equals(self, instruction):
        value_1 = self.get_value((self.pointer + 1), instruction.mode_1)
        value_2 = self.get_value(self.pointer + 2, instruction.mode_2)
        address = self.runnable[self.pointer + 3]
        if value_1 == value_2:
            self.set_value(address, 1)
        else:
            self.set_value(address, 0)
        self.pointer += 4

    def load_from_addr_pointer(self, pointer):
        return self.runnable[self.runnable[pointer]]

    def set_pointer(self, address):
        self.pointer = address

    def get_result(self):
        return self.runnable[0]

    def get_memory(self):
        return self.runnable

    def get_outputs(self):
        return self.outputs
