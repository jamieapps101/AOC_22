from typing import List, Tuple
import re

FILE_PATH = "days/day_10/liv/input.txt"
TEST_FILE_PATH = "days/day_10/liv/test_data.txt"
COMMAND_RE = re.compile(r"^([addx]+|[noop]+)")
X_VALUE_RE = re.compile(r"(\-?[0-9]*)$")


def read_file(file_path: str) -> List[Tuple[str, int]]:
    with open(file_path) as fp:
        cycle_commands = []
        instruction = ()
        for line in fp:
            line = line.rstrip()
            command = COMMAND_RE.match(line)
            # print(command)
            x_value = X_VALUE_RE.search(line)
            # print(x_value)
            parsed_x_value = None
            if x_value.group(1) == "":
                parsed_x_value = None
            else:
                parsed_x_value = int(x_value.group(1))
                # print(f"Parsed X Value = {parsed_x_value}")
            instruction = (command.group(1), parsed_x_value)
            # print(f"Instruction: {instruction}")
            cycle_commands.append(instruction)
    # print(f"Cycle Commands are: {cycle_commands}")
    return cycle_commands


def clock_cycle(cycle_commands: List[Tuple[str, int]]):
    X = 1
    cycle_count = 1
    stored_X_change = 0
    crt_grid = []
    crt_row = []
    pixel_count = 0
    for command in cycle_commands:
        # print(f"Command is {command}")
        if command[0] == "noop":
            # print(f"Start of Cycle {cycle_count}")
            pixel_count, crt_row, crt_grid = crt_operations(
                X, pixel_count, crt_row, crt_grid
            )
            # print(f"Pixel Count is {pixel_count}")
            # print(f"CRT Row is {crt_row}")
            # print(f"CRT Grid is {crt_grid}")
            # print(f"X during Cycle {cycle_count} is {X}")
            # print(f"End of Cycle {cycle_count}, noop started and ended")
            # print(f"X at cycle end is {X}")
            cycle_count += 1
        elif command[0] == "addx":
            # print(f"Start of Cycle {cycle_count}")
            stored_X_change = command[1]
            # print(f"X during Cycle {cycle_count} is {X}")
            pixel_count, crt_row, crt_grid = crt_operations(
                X, pixel_count, crt_row, crt_grid
            )
            # print(f"Pixel Count is {pixel_count}")
            # print(f"CRT Row is {crt_row}")
            # print(f"CRT Grid is {crt_grid}")
            # print(f"End of Cycle {cycle_count}, addx started and running")
            # print(f"X at cycle end is {X}")
            cycle_count += 1
            # print(f"Start of Cycle {cycle_count}")
            # print(f"X during Cycle {cycle_count} is {X}")
            pixel_count, crt_row, crt_grid = crt_operations(
                X, pixel_count, crt_row, crt_grid
            )
            # print(f"Pixel Count is {pixel_count}")
            # print(f"CRT Row is {crt_row}")
            # print(f"CRT Grid is {crt_grid}")
            if stored_X_change < 0:
                X += stored_X_change
            elif stored_X_change > 0:
                X += stored_X_change
            # print(f"End of Cycle {cycle_count}, addx finished")
            # print(f"X at cycle end is {X}")
            cycle_count += 1
            stored_X_change = 0
    # print(crt_grid)
    # print(len(crt_grid))
    return crt_grid


def crt_operations(
    sprite_position: int, pixel_count: int, crt_row: List[str], crt_grid: List
) -> List[List[str]]:
    if (
        pixel_count == sprite_position
        or pixel_count == sprite_position - 1
        or pixel_count == sprite_position + 1
    ):
        crt_action = "#"
        crt_row.append(crt_action)
        pixel_count += 1
    else:
        crt_action = " "
        crt_row.append(crt_action)
        pixel_count += 1
    if pixel_count == 40:
        crt_grid.append(crt_row)
        # print(f"CRT Grid: {crt_grid}")
        crt_row = []
        pixel_count = 0
    return pixel_count, crt_row, crt_grid


def render_grid(crt_grid: List[List[str]]):
    for row in crt_grid:
        print("".join(row))


def main():
    input_data = read_file(FILE_PATH)
    # input_data = read_file(TEST_FILE_PATH)
    crt_grid = clock_cycle(input_data)
    render_grid(crt_grid)


if __name__ == "__main__":
    main()

# Part Two
# X = horizontal position of sprite
# sets horizontal position of middle of 3 pixel wide sprite.
# CRT has a screen 40 pixels wide and 6 pixels high
# CRT draws from the top row L-R onwards
# Leftmost pixel is 0, rightmost pixel is 39
# CRT draws one pixel per clock cycle
# CRT will complete the 6 'rows' and then go back to the start of row 1
# Can determine if the sprite is visible once CRT draws a pixel:
# X in a given cycle determines where the sprite is
# If pixel in sprite is currently being drawn, pixel will be lit (#)
# If pixel not in sprite is currently being drawn, pixel will be dark (.)
