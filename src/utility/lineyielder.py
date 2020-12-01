import os


def yield_lines(filename: str):
    with open(filename, "r") as file:
        for line in file.readlines():
            # remove newline at end
            yield line.rstrip()


def yield_chunks(filename: str, delimiter: str):
    for line in yield_lines(filename):
        yield line.split(delimiter)