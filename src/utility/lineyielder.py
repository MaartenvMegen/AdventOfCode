import os


def yield_lines(filename: str):
    with open(filename, "r") as file:
        for line in file.readlines():
            # remove newline at end
            yield line.rstrip()


def yield_lines_fp(filename: str, current_dir: str) -> str:
    filename_full_path = os.path.join(current_dir, filename)
    with open(filename_full_path, "r") as file:
        for line in file.readlines():
            # remove newline at end
            yield line.rstrip()


def yield_chunks(filename: str, current_dir : str, delimiter: str):
    filename_full_path = os.path.join(current_dir, filename)
    with open(filename_full_path, "r") as file:
        return file.read().split(delimiter)
