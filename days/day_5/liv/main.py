from typing import List, Dict
import re

INSTRUCTION_FILE_PATH = "days/day_5/liv/instruction_input.txt"
POSITION_FILE_PATH = "days/day_5/liv/position_input.txt"
TEST_POSITION_FILE_PATH = "days/day_5/liv/position_test_data.txt"
TEST_INSTRUCTION_FILE_PATH = "days/day_5/liv/instruction_test_data.txt"


def read_instruction_data(file_path: str) -> List[str]:
    instruction_data = []
    with open(file_path) as fp:
        for line in fp:
            line = line.rstrip()
            instruction_data.append(line)
    # print(lines)
    return instruction_data


def read_position_data(file_path: str) -> Dict[str, List]:
    # crate_layout = {}
    crate_pile = []
    biggest_crate_pile = 0
    with open(file_path) as fp:
        for line in fp:
            line = line.rstrip()
            crate_pile.append(line)
            if len(line) > biggest_crate_pile:
                biggest_crate_pile = len(line)
    row_length = 0
    for row_index in range(len(crate_pile)):
        # print(row_index)
        current_row = crate_pile[row_index]
        row_length = len(current_row)
        if row_length < biggest_crate_pile:
            buffer = biggest_crate_pile - row_length
            # print(crate_pile)
            crate_pile[row_index] = crate_pile[row_index] + buffer * " "
            # print(crate_pile)
    # print(crate_pile_length)
    return crate_pile


def extract_columns(crate_pile: List[str]) -> Dict[str, List[str]]:
    column_references = []
    last_string = crate_pile[-1]
    pile_of_crates = {}
    # -1 takes you to the last value... very stupid
    for index, value in enumerate(last_string):
        if value != " ":
            column_references.append(index)
            pile_of_crates[value] = []
    # print(column_references)
    for row in reversed(crate_pile[0:-1]):
        # print(row)
        for reference in column_references:
            key = crate_pile[-1][reference]
            crate_value = row[reference]
            if crate_value != " ":
                pile_of_crates[key].append(crate_value)
    print(pile_of_crates)
    return pile_of_crates


def apply_instructions(
    pile_of_crates: Dict[str, List[str]], instruction_data: List[str]
) -> Dict[str, List[str]]:
    INSTRUCTION_RE = re.compile(r"^move ([0-9]+) from ([0-9]+) to ([0-9]+)$")
    for instruction in instruction_data:
        match = INSTRUCTION_RE.match(instruction)
        if match is not None:
            moving_crate_count = int(match.group(1))
            source_pile = match.group(2)
            target_pile = match.group(3)
            buffer_pile = []
            for index in range(moving_crate_count):
                crate_in_transit = pile_of_crates[source_pile].pop()
                buffer_pile.append(crate_in_transit)
            for index in range(moving_crate_count):
                crate_in_transit = buffer_pile.pop()
                pile_of_crates[target_pile].append(crate_in_transit)
    return pile_of_crates


def top_crates(pile_of_crates: Dict[str, List[str]]):
    crate_list = []
    for pile in pile_of_crates:
        top_crate = pile_of_crates[pile][-1]
        crate_list.append(top_crate)
    return crate_list


def main():
    position_input_data = read_position_data(POSITION_FILE_PATH)
    # print(position_input_data)
    # position_input_data = read_position_data(TEST_POSITION_FILE_PATH)
    # print(position_input_data)
    instructions_input_data = read_instruction_data(INSTRUCTION_FILE_PATH)
    # print(instruction_input_data)
    # instructions_input_data = \
    #   read_instruction_data(TEST_INSTRUCTION_FILE_PATH)
    # print(instructions_input_data)
    extracted_columns = extract_columns(position_input_data)
    rearranged_piles = apply_instructions(
        extracted_columns, instructions_input_data
    )  # noqa E501
    # print(rearranged_piles)
    uppermost_crates = top_crates(rearranged_piles)
    print("".join(uppermost_crates))


if __name__ == "__main__":
    main()
