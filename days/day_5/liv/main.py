from typing import List

# from typing import Dict
# import numpy as np
import pandas as pd

# import re

INSTRUCTION_FILE_PATH = "days/day_5/liv/instruction_input.txt"
POSITION_FILE_PATH = "days/day_5/liv/position_input.txt"
TEST_POSITION_FILE_PATH = "days/day_5/liv/position_test_data.txt"
TEST_INSTRUCTION_FILE_PATH = "days/day_5/liv/instruction_test_data.txt"


def read_instruction_data(file_path: str) -> List[str]:
    lines = []
    with open(file_path) as fp:
        for line in fp:
            line = line.rstrip()
            lines.append(line)
    # print(lines)
    return lines


def read_position_data(file_path):
    row = []
    columns = []
    with open(file_path) as fp:
        for line in fp:
            # print(line)
            # line = line.strip()
            # print(len(line))
            for column_value in line:
                # column_value = re.sub("(\[|\])","",column_value)
                row.append(column_value)
            columns.append(row)
            row = []
    # print(columns)
    # array = np.array(columns)
    # array = array.transpose()
    data_frame = pd.DataFrame(columns).T
    # re.sub("\[|\]","",data_frame)
    return data_frame


def main():
    # position_input_data = read_position_data(POSITION_FILE_PATH)
    # print(position_input_data)
    position_input_data = read_position_data(TEST_POSITION_FILE_PATH)
    print(position_input_data)
    # instruction_input_data = read_instruction_data(INSTRUCTION_FILE_PATH)
    # print(instruction_input_data)
    instructions_input_data = read_instruction_data(TEST_INSTRUCTION_FILE_PATH)
    print(instructions_input_data)


if __name__ == "__main__":
    main()
