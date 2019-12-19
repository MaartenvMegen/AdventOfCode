import threading
import time
from itertools import product
from operator import itemgetter
from queue import Queue

from src.utility import Inputs
from src.utility.OpCodeRunner import OpcodeRunner


class TractorBeamTester:

    def __init__(self):
        self.program = OpcodeRunner(Inputs.tractorBeamTestCode, name="DroneDeployer")
        self.program_thread = threading.Thread(target=self.program.run_program)
        self.program.set_complete_listeners(self)
        self.program.set_output_listener(self)
        self.grid_data = dict()
        self.program_output = Queue()

    def run(self):

        for x in range(1450, 1550):
            for y in range(750, 800, 1):
                value = self.get_value_for_x_y(x, y)
                value_100y = self.get_value_for_x_y(x, y + 99)
                value_100x = self.get_value_for_x_y(x + 99, y)
                if value_100x and value_100y and value:
                    print("point {},{} fits".format(x, y))
                    return x * 1000 + y
        return None

    def get_value_for_x_y(self, x, y):
        self.program = OpcodeRunner(Inputs.tractorBeamTestCode, name="DroneDeployer")
        self.program_thread = threading.Thread(target=self.program.run_program)
        self.program.set_complete_listeners(self)
        self.program.set_output_listener(self)
        self.program_thread.start()
        self.program.send_data(x)
        self.program.send_data(y)
        return self.program_output.get()

    def send_data(self, value):
        self.program_output.put(value)
        # print("incoming data: {}".format(value))

    def notify(self):
        # print("program halted")
        pass

    def render(self):
        x_min = min(list(self.grid_data.keys()), key=itemgetter(0))[0]
        y_min = min(list(self.grid_data.keys()), key=itemgetter(1))[1]
        x_max = max(list(self.grid_data.keys()), key=itemgetter(0))[0]
        y_max = max(list(self.grid_data.keys()), key=itemgetter(1))[1]
        for y in range(y_min, y_max + 1):
            x_string = ""
            for x in range(x_min, x_max + 1):
                if (x, y) in self.grid_data.keys():
                    value = self.grid_data[x, y]
                    if value == 1:
                        x_string += "#"
                    else:
                        x_string += "."
            if x_string:
                print(x_string)
