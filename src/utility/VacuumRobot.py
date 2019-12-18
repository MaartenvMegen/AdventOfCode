import os
import threading
import time

from src.utility import Inputs
from src.utility.OpCodeRunner import OpcodeRunner
from queue import Queue

ROBOT_FACE_NORTH = 94

NEWLINE = 10
SCAFFOLD = 35



class Position:
    def __init__(self, position, reached_from, type):
        self.position = position
        self.reached_from = reached_from
        self.type = type


NORTH = 1
SOUTH = 2
EAST = 3
WEST = 4


class VacuumRobot:
    INPUT_A = [ord(char) for char in "L,6,R,8,R,12,L,6,L,8\n"]
    INPUT_B = [ord(char) for char in "L,10,L,8,R,12\n"]
    INPUT_C = [ord(char) for char in "L,8,L,10,L,6,L,6\n"]
    COMMANDS = [ord(char) for char in "A,B,A,C,B,C,B,A,C,B\n"]
    QUESTION = [ord(char) for char in "n\n"]

    TOTAL_INPUT = COMMANDS + INPUT_A + INPUT_B + INPUT_C + QUESTION

    def __init__(self):
        self.program = OpcodeRunner(Inputs.vacuum_robot_code, name="VacuumRobot")
        self.program_thread = threading.Thread(target=self.program.run_program)
        self.program.set_complete_listeners(self)
        self.program.set_output_listener(self)
        self.running = True
        self.imaging_data = Queue()
        self.image = []
        self.known_scaffolds = set()
        self.robot_loc = (0, 0)
        self.robot_orientation = NORTH
        self.dust_collected = 0
        self.alignment_param_sum = 0

    def run(self):
        self.program_thread.start()
        time.sleep(2)
        self.get_data()
        self.render()
        self.get_intersections()
        self.program.inputs = VacuumRobot.TOTAL_INPUT
        self.program.memory[0] = 2
        # provide all inputs
        #self.find_path()

    def notify(self):
        print("Program has halted")
        self.running = False

    def send_data(self, value):
        if value > 127:
            print('incoming data {}'.format(value))
            self.dust_collected = value
        self.imaging_data.put(value)

    def get_data(self):
        loc_x = 0
        loc_y = 0
        other_symbols = set()
        try:
            while True:
                data = self.imaging_data.get(timeout=1)
                if data == SCAFFOLD:
                    self.known_scaffolds.add((loc_x, loc_y))
                    loc_x += 1
                elif data == NEWLINE:
                    loc_x = 0
                    loc_y += 1
                elif data == ROBOT_FACE_NORTH:
                    self.robot_orientation = NORTH
                    self.robot_loc = (loc_x, loc_y)
                    loc_x += 1
                else:
                    other_symbols.add(data)
                    loc_x += 1
                self.image.append(data)
        except:
            print("no more data")
            print(other_symbols)

    def render(self):
        current_line = ""
        for ascii_char in self.image:
            if ascii_char == 10:
                print(current_line)
                current_line = ""
            else:
                current_line += chr(ascii_char)

    def get_intersections(self):
        # iterate through locs in set, check if neighbours are in set
        intersections = set()
        alignment_params = []
        for loc in self.known_scaffolds:
            loc_x, loc_y = loc
            # if has 4 neighbours, add to intersectinos
            east = (loc_x + 1, loc_y)
            south = (loc_x, loc_y + 1)
            west = (loc_x - 1, loc_y)
            north = (loc_x, loc_y - 1)
            neighbours = [north, east, south, west]
            if all([(neighbour in self.known_scaffolds) for neighbour in neighbours]):
                print('Intersection at: {}'.format(loc))
                alignment_param = loc_x * loc_y
                alignment_params.append(alignment_param)
                intersections.add(loc)
        print("sum alignments is: {}".format(sum(alignment_params)))
        self.alignment_param_sum = sum(alignment_params)

        for y in range(37):
            x_line = ""
            for x in range(60):
                if (x, y) in intersections:
                    x_line += "O"
                elif (x, y) in self.known_scaffolds:
                    x_line += "#"
                else:
                    x_line += "."
            print(x_line)

    def find_path(self):
        # go straight unless not possible
        # then check  there should only be on left/right
        print("found following path:")
        parsed_locations = set()
        current_location = self.robot_loc
        orientation = self.robot_orientation
        step_counter = 0
        result = ""
        while not parsed_locations.issuperset(self.known_scaffolds):
            straight_on = VacuumRobot.loc_for_orientation(current_location, orientation)
            if straight_on in self.known_scaffolds:
                current_location = straight_on
                step_counter += 1
            else:
                result += str(step_counter)
                step_counter = 1
                loc, rotation, orientation = self.get_new_loc_and_rotation_and_orientation(orientation, current_location)
                result += rotation
                current_location = loc
            parsed_locations.add(current_location)

        print(result)
        self.findLongestRepeatingSubSeq(result)

    def get_new_loc_and_rotation_and_orientation(self, orientation, current_location):
        if orientation == NORTH or orientation == SOUTH:
            east_loc = VacuumRobot.loc_for_orientation(current_location, EAST)
            west_loc = VacuumRobot.loc_for_orientation(current_location, WEST)
            if east_loc in self.known_scaffolds:
                rotation = VacuumRobot.get_rotation(orientation, EAST)
                return east_loc, rotation, EAST
            if west_loc in self.known_scaffolds:
                rotation = VacuumRobot.get_rotation(orientation, WEST)
                return west_loc, rotation, WEST
        if orientation == EAST or orientation == WEST:
            north_loc = VacuumRobot.loc_for_orientation(current_location, NORTH)
            south_loc = VacuumRobot.loc_for_orientation(current_location, SOUTH)
            if north_loc in self.known_scaffolds:
                rotation = VacuumRobot.get_rotation(orientation, NORTH)
                return north_loc, rotation, NORTH
            if south_loc in self.known_scaffolds:
                rotation = VacuumRobot.get_rotation(orientation, SOUTH)
                return south_loc, rotation, SOUTH

    @staticmethod
    def get_rotation(orientation, desired_direction):
        if orientation == NORTH and desired_direction == WEST:
            return "L"
        if orientation == NORTH and desired_direction == EAST:
            return "R"
        if orientation == SOUTH and desired_direction == WEST:
            return "R"
        if orientation == SOUTH and desired_direction == EAST:
            return "L"

        if orientation == EAST and desired_direction == NORTH:
            return "L"
        if orientation == EAST and desired_direction == SOUTH:
            return "R"
        if orientation == WEST and desired_direction == NORTH:
            return "R"
        if orientation == WEST and desired_direction == SOUTH:
            return "L"

    @staticmethod
    def loc_for_orientation(loc, orientation):
        loc_x, loc_y = loc
        if orientation == NORTH:
            return loc_x, loc_y - 1
        if orientation == EAST:
            return loc_x + 1, loc_y
        if orientation == SOUTH:
            return loc_x, loc_y + 1
        if orientation == WEST:
            return loc_x - 1, loc_y

    @staticmethod
    def findLongestRepeatingSubSeq(result):
        from collections import Counter
        # a = "R8R8R4R4R8L6L2R4R4R8R8R8L6L2"
        times = 3
        for n in range(1, len(result) // times + 1)[::-1]:
            substrings = [result[i:i + n] for i in range(len(result) - n + 1)]
            freqs = Counter(substrings)
            for freq, occurence in freqs.items():
                if occurence >= 2:
                    print("sequence: {} appears {} times".format(freq, occurence))
                # break
        # A            B        A           C         B        C          B       A            C        B
        # L6R8R12L6L8 L10L8R12 L6R8R12L6L8 L8L10L6L6 L10L8R12 L8L10L6L6 L10L8R12 L6R8R12L6L8 L8L10L6L6 L10L8
        # A B A C B C B A C B
        # A: L6R8R12L6l8
        # B: L10L8R12
        # C: L8L10L6L6