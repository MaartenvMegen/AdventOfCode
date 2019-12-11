from enum import Enum, IntEnum
from operator import itemgetter
from queue import Queue


class Color(Enum):
    BLACK = 0
    WHITE = 1


class Orientation(IntEnum):
    UP = 0,
    RIGHT = 1,
    DOWN = 2,
    LEFT = 3


orientation_to_position_delta = {Orientation.UP: (0, -1), Orientation.RIGHT: (1, 0), Orientation.DOWN: (0, 1),
                                 Orientation.LEFT: (-1, 0)}

orientation_to_string = {Orientation.UP: "^", Orientation.RIGHT: ">", Orientation.DOWN: "v", Orientation.LEFT: "<"}


class Robot:
    def __init__(self):
        self.position = (0, 0)
        self.grid = dict()
        self.orientation = Orientation.UP
        self.running = False
        self.inputs = Queue()
        self.output_listeners = []
        self.paint_meter = set()
        self.pain_count = 0

    def send_data(self, value):
        self.inputs.put(value)

    def run(self):
        print("Painter started")
        self.running = True

        while self.running:
            # get input
            color_to_paint = Color(self.get_input())
            # paint according to input
            self.grid[self.position] = color_to_paint
            # register this square as painted
            self.paint_meter.add(self.position)
            self.pain_count += 1
            # get rotation instruction
            rotate_dir = self.get_input()
            # change orientation
            self.set_new_orientation(rotate_dir)
            # move
            self.move()
            # send color to program
            for listener in self.output_listeners:
                listener.send_data(self.grid[self.position].value)

        print("Finished painting")
        print("Amount of painted squares: {}".format(len(self.paint_meter)))
        self.render()

    def move(self):
        (x, y) = self.position
        dx, dy = orientation_to_position_delta[self.orientation]
        self.position = (x + dx, y + dy)
        if self.position not in self.grid:
            self.grid[self.position] = Color.BLACK

    def get_input(self):
        return self.inputs.get()

    def send_current_color(self, color):
        for listener in self.output_listeners:
            listener.send_data(color.value)

    def set_output_listener(self, listener):
        self.output_listeners.append(listener)

    def set_new_orientation(self, orientation_code):
        if orientation_code == 1:
            current_ordinal = self.orientation.value
            if current_ordinal < 3:
                self.orientation = Orientation(current_ordinal + 1)
            else:
                self.orientation = Orientation.UP

        elif orientation_code == 0:
            current_ordinal = self.orientation.value
            if current_ordinal > 0:
                self.orientation = Orientation(current_ordinal - 1)
            else:
                self.orientation = Orientation.LEFT

    def render(self):
        x_max = max(list(self.grid.keys()), key=itemgetter(0))[0]
        y_max = max(list(self.grid.keys()), key=itemgetter(1))[1]
        for y in range(0, y_max + 1):
            x_string = ""
            for x in range(0, x_max + 1):
                if (x, y) == self.position:
                    x_string += orientation_to_string[self.orientation]
                if (x, y) not in self.grid:
                    x_string += " "
                elif self.grid[(x, y)] == Color.WHITE:
                    x_string += "#"
                else:
                    x_string += " "
            print(x_string)

    def notify(self):
        self.running = False
