import threading
import time
from _queue import Empty
from enum import Enum
from queue import Queue

from src.utility.Screen import ObjectDetails, Screen


class State(Enum):
    EXPECTING_X = 0
    EXPECTING_Y = 1
    EXPECTING_OBJECT_INFO = 2


tilts = {"left": -1, "right": 1, "neutral": 0}


class Joystick:
    def __init__(self):
        self.tilt = 0

    def get_value(self):
        return self.tilt

    def set_tilt(self, tilt_spec):
        self.tilt = tilts[tilt_spec]


class SegmentsDisplay:
    def __init__(self):
        self.score = 0

    def set_score(self, score):
        self.score = score


class Arcade:
    def __init__(self, program):
        # state is y,x, details
        self.inputs = Queue()
        self.pixel_builder = {}
        self.state = State.EXPECTING_X
        self.running = False
        self.screen = Screen()
        self.score_display = SegmentsDisplay()
        self.program = program
        self.joystick = Joystick()
        self.current_ball_pos = (0, 0)
        self.current_paddle_pos = (0, 0)
        self.desired_joystick_orientation = Queue()
        self.one_object_received = False
        self.initial_run = True

    def run(self):
        self.running = True
        joystick_thread = threading.Thread(target=self.provide_joystick_orientation)
        joystick_thread.start()
        print('Started arcade')
        while self.running:
            self.get_input_data()
        # give other thread time to die?
        time.sleep(1)

    def send_data(self, value):
        self.inputs.put(value)

    def get_input_data(self):
        try:
            value = self.inputs.get(timeout=1)
        except Empty:
            return

        if self.state == State.EXPECTING_X:
            self.pixel_builder["x"] = value
            self.state = State.EXPECTING_Y
        elif self.state == State.EXPECTING_Y:
            self.pixel_builder["y"] = value
            self.state = State.EXPECTING_OBJECT_INFO
        else:
            if self.pixel_builder["x"] == -1:
                self.score_display.set_score(value)
                self.state = State.EXPECTING_X
            else:
                details = ObjectDetails(value)
                self.pixel_builder["details"] = details
                self.state = State.EXPECTING_X
                self.update_positions(details)
                self.screen.display(self.pixel_builder["y"], self.pixel_builder["x"], self.pixel_builder["details"])

    def update_positions(self, details):
        if self.joystick.tilt == 0 and not self.initial_run:
            # cannot expect a change in position of joystick
            if details == ObjectDetails.BALL:
                self.current_ball_pos = (self.pixel_builder["x"], self.pixel_builder["y"])
                self.determine_joystick_orientation()
            if details == ObjectDetails.HORIZONTAL_PADDLE:
                self.current_paddle_pos = (self.pixel_builder["x"], self.pixel_builder["y"])

        else:
            if details == ObjectDetails.HORIZONTAL_PADDLE:
                self.current_paddle_pos = (self.pixel_builder["x"], self.pixel_builder["y"])
                if not self.one_object_received:
                    self.one_object_received = True
                else:
                    self.determine_joystick_orientation()
                    self.one_object_received = False
                    self.initial_run = False
            if details == ObjectDetails.BALL:
                self.current_ball_pos = (self.pixel_builder["x"], self.pixel_builder["y"])
                if not self.one_object_received:
                    self.one_object_received = True
                else:
                    self.determine_joystick_orientation()
                    self.one_object_received = False
                    self.initial_run = False




    def notify(self):
        #parse final results
        time.sleep(1)
        self.running = False
        self.screen.get_blocks()
        print("final score: {}".format(self.score_display.score))

    def provide_joystick_orientation(self):
        print("Starting joystick thread")
        # wait for first data to arrive to be able to provide proper input

        while self.running:
            #time.sleep(0.1)
            # allow program to update the screen before we render
            #self.render_info()
            #print('please provide desired joystick orientation: left/right/neutral')
            #value = input()
            #self.joystick.set_tilt(value)

            #print("sending: {} to program".format(self.joystick.get_value()))
            try:
                self.program.send_data(self.desired_joystick_orientation.get(timeout=2))
            except Empty:
                pass

    def determine_joystick_orientation(self):
        paddle_pos = self.current_paddle_pos[0]
        bal_pos = self.current_ball_pos[0]
        #print("paddle at {}. ball at {}".format(paddle_pos, bal_pos))

        if paddle_pos > bal_pos:
            self.joystick.set_tilt("left")
        elif paddle_pos < bal_pos:
            self.joystick.set_tilt("right")
        else:
            self.joystick.set_tilt("neutral")

        self.desired_joystick_orientation.put(self.joystick.get_value())

    def render_info(self):
        self.screen.render()
        print("current score: {}".format(self.score_display.score))
