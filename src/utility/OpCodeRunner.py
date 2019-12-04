from enum import Enum


class Opcode(Enum):
    # arithmetic
    ADD = 1
    MULTIPLY = 2
    INCREMENT = 3
    DECREMENT = 4
    SUBTRACT = 5
    DIVIDE = 6
    # Jumps
    JUMP = 7
    # Data transfer
    MOVE = 8
    # Control
    HALT = 99


class OpcodeRunner:

    def __init__(self, program):
        self.program = program
        self.runnable = program.copy()
        self.pointer = 0
        self.opcode_to_method = {Opcode.DIVIDE: self.divide, Opcode.ADD: self.add, Opcode.MULTIPLY: self.multiply,
                                 Opcode.JUMP: self.jump, Opcode.MOVE: self.move, Opcode.SUBTRACT: self.subtract,
                                 Opcode.INCREMENT: self.increment, Opcode.DECREMENT: self.decrement}

    def reset(self):
        self.runnable = self.program.copy()
        self.pointer = 0

    def run_program(self):
        # run while instruction pointer does not contain halt
        while Opcode(self.runnable[self.pointer]) != Opcode.HALT:
            # do action
            opcode = Opcode(self.runnable[self.pointer])
            operation = self.opcode_to_method.get(opcode, lambda: print("Unexpected opcode: {}".format(opcode)))
            operation()

    def move(self):
        # move data from location a, to location b as specified in pointers 1 and 2
        address_1 = self.pointer + 1
        address_2 = self.pointer + 2

        value = self.load_from_addr_pointer(address_1)
        self.set_value(address_2, value)
        self.pointer += 3

    def set_value(self, address_2, value):
        self.runnable[address_2] = value

    def jump(self):
        self.pointer = self.load_from_addr_pointer(self.pointer + 1)

    def add(self):
        addres, value1, value2 = self.get_arithmetic_data()
        self.set_value(addres, value1 + value2)

    def increment(self):
        address = self.pointer + 1
        value = self.load_from_addr_pointer(address)
        self.set_value(address, value + 1)
        self.pointer += 2

    def decrement(self):
        address = self.pointer + 1
        value = self.load_from_addr_pointer(address)
        self.set_value(address, value - 1)
        self.pointer += 2

    def subtract(self):
        addres, value1, value2 = self.get_arithmetic_data()
        self.set_value(addres, value1 - value2)

    def multiply(self):
        addres, value1, value2 = self.get_arithmetic_data()
        self.set_value(addres, value1 * value2)

    def divide(self):
        addres, value1, value2 = self.get_arithmetic_data()
        self.set_value(addres, value1 / value2)

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
