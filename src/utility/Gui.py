import threading
import time
import tkinter as tk

from src.test.tests_2020.day5.test_day_5 import parse_boarding_pass
from src.utility import lineyielder
from src.utility.PositionUtils import Grid, Point


def update_text(stringvar):
    grid = Grid()

    all_possible_ids = set(range(0, 127 * 8 + 7))
    for id in all_possible_ids:
        grid.add_location(Point(id // 8, id % 8, "_"))
    stringvar.set(grid.grid_to_string())

    update_cnt = 0
    for line in lineyielder.yield_lines("../\/test\/tests_2020\day5\input.txt"):
        id = parse_boarding_pass(line)
        grid.add_location(Point(id // 8, id % 8, '#'))
        if update_cnt % 20 == 0:
            stringvar.set(grid.grid_to_string())
        update_cnt += 1
        time.sleep(0.005)

    final_result = grid.grid_to_string() + "finished\n"
    stringvar.set(final_result)


class Gui:

    def run(self, text_update_task):
        # must be run in main thread!
        root = tk.Tk()

        stringvar = tk.StringVar()
        a = tk.Label(root, textvariable=stringvar, font="TkFixedFont", background='white', foreground='black')
        a.pack()

        # GUI updating stuff must run in another thread
        x = threading.Thread(target=text_update_task, args=[stringvar])
        x.start()
        root.mainloop()


if __name__ == '__main__':
    gui = Gui()
    gui.run(update_text)