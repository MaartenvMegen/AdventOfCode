from enum import Enum

from queue import Queue

from src.utility.GrowingList import GrowingList


class Opcode(Enum):
    ADD = 1
    MULTIPLY = 2
    INPUT = 3
    OUTPUT = 4
    JUMP_IF_TRUE = 5
    JUMP_IF_FALSE = 6
    LESS_THAN = 7
    EQUALS = 8
    ADJUST_RELATIVE_BASE = 9
    HALT = 99


class ParameterMode(Enum):
    POSITION = 0
    IMMEDIATE = 1
    RELATIVE = 2


class Instruction:

    def __init__(self, opcode, mode_1, mode_2, mode_3):
        self.opcode = opcode
        self.mode_1 = mode_1
        self.mode_2 = mode_2
        self.mode_3 = mode_3

    @staticmethod
    def parse(int_code):
        string_code = str(int_code).zfill(5)
        return Instruction(Opcode(int(string_code[-2:])), ParameterMode(int(string_code[2])),
                           ParameterMode(int(string_code[1])), ParameterMode(int(string_code[0])))


class OpcodeRunner:

    def __init__(self, program, inputs=None, name="generic"):
        self.program = program
        self.memory = GrowingList(program.copy())
        self.pointer = 0
        self.relative_base = 0
        self.opcode_to_method = {Opcode.ADD: self.add, Opcode.MULTIPLY: self.multiply, Opcode.INPUT: self.get_input,
                                 Opcode.OUTPUT: self.output, Opcode.JUMP_IF_TRUE: self.jump_if_true,
                                 Opcode.JUMP_IF_FALSE: self.jump_if_false, Opcode.LESS_THAN: self.less_than,
                                 Opcode.EQUALS: self.equals, Opcode.ADJUST_RELATIVE_BASE: self.adjust_relative_base}
        self.output_listeners = []
        self.completion_listeners = []
        self.name = name

        if inputs:
            self.inputs = inputs.copy()
            self.inputs = Queue()
            for input_data in inputs:
                self.inputs.put(input_data)
        else:
            self.inputs = Queue()
        self.outputs = []

    def reset(self):
        self.memory = self.program.copy()
        self.pointer = 0

    def run_program(self):
        print("Booting opcode runner: {}".format(self.name))
        while Instruction.parse(self.memory[self.pointer]).opcode != Opcode.HALT:
            instruction = Instruction.parse(self.memory[self.pointer])
            operation = self.opcode_to_method.get(instruction.opcode,
                                                  lambda: print("Unexpected opcode: {}".format(instruction.opcode)))
            # print("performing: {} - {} - {} - {}".format(instruction.opcode, instruction.mode_1, instruction.mode_2,
            #                                              instruction.mode_3))
            operation(instruction)

        for listener in self.completion_listeners:
            listener.notify()

    def set_value(self, address, value, mode):
        if mode == ParameterMode.POSITION:
            self.memory[address] = value
        elif mode == ParameterMode.RELATIVE:
            self.memory[address + self.relative_base] = value
        else:
            print("ERROR: unexpected mode: {}".format(mode))

    def get_input(self, instruction):
        try:
            input_value = self.inputs.get(timeout=5)
        except:
            print("Timeout waiting for input. Shutting down: ".format(self.name))
            exit(-1)
        address = self.memory[self.pointer + 1]
        self.set_value(address, input_value, instruction.mode_1)
        self.pointer += 2

    def output(self, instruction):
        value = self.get_value(self.pointer + 1, instruction.mode_1)
        #print("Program result: {}".format(value))
        for listener in self.output_listeners:
            listener.send_data(value)
        self.outputs.append(value)
        self.pointer += 2

    def get_value(self, pointer, mode):
        if mode == ParameterMode.POSITION:
            return self.memory[self.memory[pointer]]
        elif mode == ParameterMode.IMMEDIATE:
            return self.memory[pointer]
        elif mode == ParameterMode.RELATIVE:
            return self.memory[self.relative_base + self.memory[pointer]]
        else:
            print("ERROR: unexpected mode: {}".format(mode))
            return None

    def add(self, instruction):
        address, value_1, value_2 = self.get_params(instruction)
        self.set_value(address, value_1 + value_2, instruction.mode_3)

    def multiply(self, instruction):
        address, value_1, value_2 = self.get_params(instruction)
        self.set_value(address, value_1 * value_2, instruction.mode_3)

    def get_params(self, instruction):
        value_1 = self.get_value(self.pointer + 1, instruction.mode_1)
        value_2 = self.get_value(self.pointer + 2, instruction.mode_2)
        address = self.memory[self.pointer + 3]
        self.pointer += 4
        return address, value_1, value_2

    def jump_if_true(self, instruction):
        self.jump_if(instruction, lambda x: x != 0)

    def jump_if_false(self, instruction):
        self.jump_if(instruction, lambda x: x == 0)

    def jump_if(self, instruction, comparison):
        value_1 = self.get_value(self.pointer + 1, instruction.mode_1)
        if comparison(value_1):
            self.pointer = self.get_value(self.pointer + 2, instruction.mode_2)
        else:
            self.pointer += 3

    def less_than(self, instruction):
        self.compare(instruction, lambda x, y: x < y)

    def equals(self, instruction):
        self.compare(instruction, lambda x, y: x == y)

    def compare(self, instruction, comparison):
        address, value_1, value_2 = self.get_params(instruction)
        if comparison(value_1, value_2):
            self.set_value(address, 1, instruction.mode_3)
        else:
            self.set_value(address, 0, instruction.mode_3)

    def adjust_relative_base(self, instruction):
        value = self.get_value(self.pointer + 1, instruction.mode_1)
        self.relative_base += value
        self.pointer += 2

    def set_pointer(self, address):
        self.pointer = address

    def get_result(self):
        return self.memory[0]

    def get_memory(self):
        return self.memory

    def get_outputs(self):
        return self.outputs

    def send_data(self, value):
        self.inputs.put(int(value))

    def set_output_listener(self, listener):
        self.output_listeners.append(listener)

    def set_complete_listeners(self, listener):
        self.completion_listeners.append(listener)
