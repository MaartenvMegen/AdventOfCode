import os
import threading
import time

from src.utility import Inputs
from src.utility.OpCodeRunner import OpcodeRunner
from queue import Queue


class Position:
    def __init__(self, position, reached_from, type):
        self.position = position
        self.reached_from = reached_from
        self.type = type


NORTH = 1
SOUTH = 2
EAST = 3
WEST = 4

WALL = 0
EMPTY = 1
OXYGEN = 2


class OxygenRobot:
    def __init__(self):
        self.program = OpcodeRunner(Inputs.maintenance_robot_code, name="OxygenRobot")
        self.program_thread = threading.Thread(target=self.program.run_program)
        self.program.set_complete_listeners(self)
        self.program.set_output_listener(self)
        self.running = True
        self.move_result = Queue()
        self.known_locations = {}
        self.current_location = (0, 0)
        self.current_command = NORTH
        self.oxygen_location = None
        self.distance_to_oxygen = 0
        self.oxygenation_time = 0

    def run(self):
        print("OxygenRobot getting to work")
        self.known_locations[(0, 0)] = Position((0, 0), None, EMPTY)
        self.program_thread.start()
        while self.running:
            # print("I am now going into direction: {}".format(self.current_command))
            # send move north (1), south (2), west (3), and east (4)
            self.program.send_data(self.current_command)
            # get result: 0 - hit_wall, 1 - move ok, 2= found oxygen
            move_result = self.move_result.get()
            if move_result == WALL:
                wall_pos = self.command_position_offset(self.current_command, self.current_location)
                self.known_locations[wall_pos] = Position(wall_pos, self.current_location, WALL)
            if move_result == EMPTY:
                empty_pos = self.command_position_offset(self.current_command, self.current_location)
                if empty_pos not in self.known_locations.keys():
                    self.known_locations[empty_pos] = Position(empty_pos, self.current_location, EMPTY)
                self.current_location = empty_pos
            if move_result == OXYGEN:
                oxygen_pos = self.command_position_offset(self.current_command, self.current_location)
                self.oxygen_location = oxygen_pos
                self.known_locations[oxygen_pos] = Position(oxygen_pos, self.current_location, OXYGEN)
                self.current_location = oxygen_pos
                # break
            try:
                self.current_command = self.calculate_next_move()
            except:
                print("No more options!")
                break
            # self.render()
            # ime.sleep(0.25)

        self.render()
        print('Oxygen found at location : {}'.format(self.oxygen_location))
        self.get_minimum_path()

    def calculate_next_move(self):
        # determine possible orientations
        possible_locs = self.get_edge_locs(self.current_location)

        unknowns = [(orientation, location) for orientation, location in possible_locs if
                    location not in self.known_locations.keys()]

        # TODO MAKE THIS RETURN ORIENTATION INSTEAD OF POSITION
        # is there something to explore
        if unknowns:
            return unknowns[0][0]
        # go back to origin
        else:
            if not self.known_locations[self.current_location].reached_from:
                raise Exception("No more options!")
            return [orientation for orientation, location in possible_locs if
                    self.known_locations[self.current_location].reached_from == location][0]

    def get_edge_locs(self, position):
        possible_locs = []
        for i in range(1, 5):
            possible_locs.append((i, self.command_position_offset(i, position)))
        return possible_locs

    def command_position_offset(self, command, position):
        (x, y) = position
        if command == NORTH:
            y = y + 1
        elif command == SOUTH:
            y = y - 1
        elif command == EAST:
            x = x + 1
        elif command == WEST:
            x = x - 1
        else:
            print("Unexpected command")

        return x, y

    def notify(self):
        print("Program has halted")
        self.running = False

    def send_data(self, value):
        self.move_result.put(value)

    def render(self):
        all_locs = list(self.known_locations.keys())
        x_locs = list(zip(*all_locs))[0]
        y_locs = list(zip(*all_locs))[1]
        x_min, x_max = min(x_locs), max(x_locs)
        y_min, y_max = min(y_locs), max(y_locs)
        print("current board size {} {} to {} {}".format(x_min, y_min, x_max, y_max))
        print("---------------------------")
        for y in range(y_min, y_max + 1):
            x_string = ""
            for x in range(x_min, x_max + 1):
                x_string += self.symbol_for_loc((x, y))
            print(x_string)
        print("---------------------------")

    def symbol_for_loc(self, param):
        if param == self.current_location:
            return "R"
        type_to_symbol = {WALL: ".", EMPTY: "#", OXYGEN: "O"}
        if param in self.known_locations.keys():
            type = self.known_locations[param].type
            return type_to_symbol[type]
        else:
            return " "

    def get_minimum_path(self):
        allowed_positions = [key for key, value in self.known_locations.items() if value.type == EMPTY]
        evaluated_postitions = set()
        starting_pos = self.oxygen_location
        evaluated_postitions.add(starting_pos)

        counter = 0
        back_to_origin = 0
        while True:
            # determine all_edge locs
            edge_locs = []
            for position in evaluated_postitions:
                [edge_locs.append(loc) for _, loc in self.get_edge_locs(position) if
                 loc not in evaluated_postitions and loc in allowed_positions]

            if not edge_locs:
                print('Everything oxygenated again after {} minutes.'.format(counter))
                self.oxygenation_time = counter
                break
            # add edge locs to evaluated positions
            [evaluated_postitions.add(location) for location in edge_locs]

            counter += 1

            if (0, 0) in edge_locs:
                print('Its {} steps back to origin.'.format(counter))
                self.distance_to_oxygen = counter
