from src.utility import lineyielder


class InstructionRunner:

    def __init__(self, program, pointer_start):
        self._program = program
        self._pc = pointer_start
        self._accumulator = 0
        self._executed_instructions = set()

    def _nop(self, arg):
        self._pc += 1

    def _jump(self, arg):
        self._pc += int(arg)

    def _acc(self, arg):
        self._accumulator += int(arg)
        self._pc += 1

    _Instructions = {
        "nop": _nop,
        "acc": _acc,
        "jmp": _jump
    }

    def run(self):
        while self._pc not in self._executed_instructions:
            if self._pc >= len(self._program) or self._pc < 0:
                break
            instruction, arguments = self._program[self._pc]
            # print(f"Executing {instruction} {argument} current acc {self.accumulator}")
            self._executed_instructions.add(self._pc)
            instruction(self, *arguments)

        if self._pc in self._executed_instructions:
            # print("halted")
            return -1, self._accumulator
        else:
            print('finished')
            return 0, self._accumulator

    @staticmethod
    def _parse_line(line):
        parts = line.split(' ')
        if len(parts) < 2:
            raise InstructionParseException(f"Incomplete input: {line}")
        instruction = parts[0]
        if instruction not in InstructionRunner._Instructions.keys():
            raise InstructionParseException(f"Invalid instruction: {instruction}")
        arguments = parts[1:]
        return InstructionRunner._Instructions[instruction], tuple(arguments)

    @staticmethod
    def compile_program(filename):
        return [InstructionRunner._parse_line(line) for line in lineyielder.yield_lines(filename)]


class InstructionParseException(Exception):
    pass
