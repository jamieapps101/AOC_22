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
    sum_signal_strength = 0
    for command in cycle_commands:
        # print(f"Command is {command}")
        if command[0] == "noop":
            # print(f"Start of Cycle {cycle_count}")
            if (
                cycle_count == 20
                or cycle_count == 60
                or cycle_count == 100
                or cycle_count == 140
                or cycle_count == 180
                or cycle_count == 220
            ):
                # print(f"Required Cycle!")
                # print(f"X during Required Cycle is {X}")
                sum_signal_strength += cycle_count * X
            # print(f"End of Cycle {cycle_count}, noop started and ended")
            # print(f"X at cycle end is {X}")
            cycle_count += 1
        elif command[0] == "addx":
            # print(f"Start of Cycle {cycle_count}")
            stored_X_change = command[1]
            if (
                cycle_count == 20
                or cycle_count == 60
                or cycle_count == 100
                or cycle_count == 140
                or cycle_count == 180
                or cycle_count == 220
            ):
                # print(f"Required Cycle!")
                # print(f"X during Required Cycle is {X}")
                sum_signal_strength += cycle_count * X
            # print(f"End of Cycle {cycle_count}, addx started and running")
            # print(f"X at cycle end is {X}")
            cycle_count += 1
            # print(f"Start of Cycle {cycle_count}")
            if (
                cycle_count == 20
                or cycle_count == 60
                or cycle_count == 100
                or cycle_count == 140
                or cycle_count == 180
                or cycle_count == 220
            ):
                # print(f"Required Cycle!")
                # print(f"X during Required Cycle is {X}")
                sum_signal_strength += cycle_count * X
            if stored_X_change < 0:
                X += stored_X_change
            elif stored_X_change > 0:
                X += stored_X_change
            # print(f"End of Cycle {cycle_count}, addx finished")
            # print(f"X at cycle end is {X}")
            cycle_count += 1
            stored_X_change = 0
    print(f"Sum Signal Strength = {sum_signal_strength}")
    return X, cycle_count, sum_signal_strength


def main():
    input_data = read_file(FILE_PATH)
    # input_data = read_file(TEST_FILE_PATH)
    clock_cycle(input_data)


if __name__ == "__main__":
    main()

# CPU has one register: x
# Each tick of clock circuit = 1 cycle
# Starts with value 1
# Two possible instructions:
# addx V: after two cycles, X register increased by V
# V can be negative
# noop: takes one cycle to complete - has no effect
# consider instructions:
# noop
# addx 3
# addx -5
# start cycle 1: noop begins - X = 1
# end cycle 1: noop ends - X = 1
# start cycle 2: addx 3 begins - X = 1
# end cycle 2: addx 3 continues - X = 1
# start cycle 3: addx 3 continues - X = 1
# end cycle 3: addx 3 ends - X = 4
# start cycle 4: addx -5 begins - X = 4
# end cycle 4: addx -5 continues - X = 4
# start cycle 5: addx -5 continues - X = 4
# end cycle 5: addx -5 ends - X = -1

# Consider signal strength - cycle number*value of X during:
# 20th cycle and every 40th cycle after
# 60th
# 100th
# 140th
# 180th
# 220th
