from typing import List

FILE_PATH = "days/day_2/liv/input.txt"
# work out how to provide file path as an argument


def read_file(file_path: str) -> List[str]:
    lines = []
    with open(file_path) as fp:
        for line in fp:
            line = line.rstrip()
            lines.append(line)
    print(lines)
    return lines


def set_move_values(input_data: List[str]) -> List[List[str]]:
    # opponent_moves = []
    # your_moves = []
    for set_of_moves in input_data:
        pass


def main():
    pass


if __name__ == "__main__":
    main()

# get input data
# define opponent move and your move variables
# for each pairing, calculate if win, loss or draw
# store running total score in an int variable
