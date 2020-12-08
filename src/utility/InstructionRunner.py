class InstructionRunner:

    def __init__(self, program, pointer_start):
        self.program = program
        self.pointer = pointer_start
        self.accumulator = 0
        self.execute = {
            "nop": lambda arg: self.nop(arg),
            "acc": lambda arg: self.increment(arg),
            "jmp": lambda arg: self.jump(arg)
        }
        self.executed_instruction = set()

    def nop(self, arg):
        self.pointer += 1

    def jump(self, arg):
        self.pointer += int(arg)

    def increment(self, arg):
        self.accumulator += int(arg)
        self.pointer += 1

    def run(self):
        while self.pointer not in self.executed_instruction:
            if self.pointer > len(self.program)-1:
                break
            instruction = self.program[self.pointer][0]
            argument = self.program[self.pointer][1]
            #print(f"Executing {instruction} {argument} current acc {self.accumulator}")
            self.executed_instruction.add(self.pointer)
            self.execute[instruction](argument)

        if self.pointer in self.executed_instruction:
            #print("halted")
            return -1, self.accumulator
        else:
            print('finished')
            return 0, self.accumulator