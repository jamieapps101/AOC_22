from typing import Dict, List, Tuple
import re

# import json

FILE_PATH = "days/day_7/liv/input.txt"
TEST_FILE_PATH = "days/day_7/liv/test_data.txt"


def read_file(file_path: str) -> List[str]:
    lines = []
    with open(file_path) as fp:
        for line in fp:
            line = line.rstrip()
            lines.append(line)
    # print(lines)
    return lines


def create_directories(input_data: List[str]) -> Dict:
    directories = {}
    directory_tracker = []
    CHANGE_DIRECTORY_OUTWARDS = re.compile(r"^\$ cd \.\.")
    CHANGE_DIRECTORY_INWARDS = re.compile(r"^\$ cd ([a-z]+)")
    RESET_OUTERMOST_DIRECTORY = re.compile(r"^\$ cd /")
    CHECK_FILES = re.compile(r"^\$ ls")
    NEW_DIRECTORY = re.compile(r"^dir ([a-z]+)")
    NEW_FILE = re.compile(r"^([0-9]+) ([a-z\.]+)")
    for line in input_data:
        change_directory_outwards = CHANGE_DIRECTORY_OUTWARDS.match(line)
        change_directory_inwards = CHANGE_DIRECTORY_INWARDS.match(line)
        reset_outermost_directory = RESET_OUTERMOST_DIRECTORY.match(line)
        check_files = CHECK_FILES.match(line)
        new_directory = NEW_DIRECTORY.match(line)
        new_file = NEW_FILE.match(line)
        if change_directory_outwards is not None:
            directory_tracker.pop()
        if change_directory_inwards is not None:
            # directories[change_directory_inwards.group(1)] = {}
            directory_tracker.append(change_directory_inwards.group(1))
        if reset_outermost_directory is not None:
            directory_tracker = []
        if check_files is not None:
            continue
        if new_directory is not None:
            # print("Making new directory now!")
            buffer = directories
            for key in directory_tracker:
                buffer = buffer[key]
            buffer[new_directory.group(1)] = {}
        if new_file is not None:
            buffer = directories
            # print(f"directory_tracker = {directory_tracker}")
            for key in directory_tracker:
                # print(f"   key = {key}")
                buffer = buffer[key]
            buffer[new_file.group(2)] = int(new_file.group(1))
        # print(json.dumps(directories,indent = 4))
    # print(json.dumps(directories,indent = 4))
    return directories


# if contains cd + str of letters:
#   create new empty dictionary with key value str of letters
# if contains cd .. reset active directory
# if contains cd / reset active directory to / (outermost)
# if ls check active directory and loop to next line (files)
# if starts with dir + str: create new empty dictionary with key value str
# if starts with numbers:
#   create new entry in active dictionary with value numbers.


def sum_directory_sizes(entries: dict, directory_name: str) -> Tuple[int, int]:
    total_sizes = {}
    total_size = 0
    for entry_name in entries:
        entry = entries[entry_name]
        if isinstance(entry, int):
            total_size += entry
            # total_sizes[entry_name] = entry
        elif isinstance(entry, dict):
            directory_total_size, directory_total_sizes = sum_directory_sizes(
                entry, entry_name
            )
            total_sizes[entry_name] = directory_total_size
            total_size += directory_total_size
            print(directory_total_sizes)
            for entry_name, entry in directory_total_sizes.items():
                total_sizes[directory_name + "/" + entry_name] = entry
    return total_size, total_sizes


def get_required_disk_space(total_size: int):
    free_disk_space = 70000000 - total_size
    disk_space_required = 30000000 - free_disk_space
    print(disk_space_required)
    return disk_space_required


def find_ideal_directory(total_sizes: Dict, disk_space_required: int):
    sized_directories = list(total_sizes.items())
    print(f"sized_directories = {sized_directories}")
    sorted_sized_directories = sorted(
        sized_directories, key=lambda item: item[1], reverse=True
    )
    file_to_delete = None
    previous_directory = None
    print(f"sorted_size_directories = {sorted_sized_directories}")
    for directory in sorted_sized_directories:
        item_size = directory[1]
        print(f"item_size = {item_size}")
        if item_size < disk_space_required:
            if previous_directory is not None:
                previous_directory_name = previous_directory
                file_to_delete = previous_directory_name
            break
        previous_directory = directory
    print(file_to_delete)


def main():
    # input_data = read_file(TEST_FILE_PATH)
    input_data = read_file(FILE_PATH)
    directories = create_directories(input_data)
    total_size, total_sizes = sum_directory_sizes(directories, "/")
    print(f"Total Size = {total_size}")
    print(f"Total Sizes = {total_sizes}")
    disk_space_required = get_required_disk_space(total_size)
    find_ideal_directory(total_sizes, disk_space_required)


if __name__ == "__main__":
    main()
