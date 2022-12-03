from typing import List

FILE_PATH = "days/day_2/liv/input.txt"
# work out how to provide file path as an argument


def read_file(file_path: str) -> List[str]:
    lines = []
    with open(file_path) as fp:
        for line in fp:
            line = line.rstrip()
            lines.append(line)
    # print(lines)
    return lines


def shape_selection_evaluation(input_data: List[str]):
    shape_selection_total = 0

    # A/X Rock B/Y Paper C/Z Scissors
    # shape selection score: A/X = 1, B/Y = 2, C/Z = 3
    # round outcome score: lose = 0 draw = 3 win = 6
    # A beats Z
    # B beats X
    # C beats Y
    # A/X draw, B/Y draw, C/Z draw
    # A loses Y
    # B loses Z
    # C loses X

    for move_pair in input_data:
        if move_pair[2] == "X" and move_pair[0] == "A":
            shape_selection_total += 3
        elif move_pair[2] == "X" and move_pair[0] == "B":
            shape_selection_total += 1
        elif move_pair[2] == "X" and move_pair[0] == "C":
            shape_selection_total += 2
        elif move_pair[2] == "Y" and move_pair[0] == "A":
            shape_selection_total += 1
        elif move_pair[2] == "Y" and move_pair[0] == "B":
            shape_selection_total += 2
        elif move_pair[2] == "Y" and move_pair[0] == "C":
            shape_selection_total += 3
        elif move_pair[2] == "Z" and move_pair[0] == "A":
            shape_selection_total += 2
        elif move_pair[2] == "Z" and move_pair[0] == "B":
            shape_selection_total += 3
        elif move_pair[2] == "Z" and move_pair[0] == "C":
            shape_selection_total += 1
        # print(shape_selection_total)
    # print(shape_selection_total)
    return shape_selection_total


# shape depends on the desired outcome and what opponent plays
# where [2] = x:
# if [0] is A, + 3 (Z)
# if [0] is B, + 1 (X)
# if [0] is C, + 2 (Y)
# where [2] = y:
# if [0] is A, + 1 (X)
# if [0] is B, + 2 (Y)
# if [0] is C, + 3 (Z)
# where [2] = z:
# if [0] is A, + 2 (Y)
# if [0] is B, + 3 (Z)
# if [0] is C, + 1 (X)


def round_outcome_evaluation(input_data: List[str]):
    round_outcome_total = 0
    for move_pair in input_data:
        if move_pair[2] == "Y":
            round_outcome_total += 3
        elif move_pair[2] == "Z":
            round_outcome_total += 6
        # print(round_outcome_total)
    # print(round_outcome_total)
    return round_outcome_total


# change this to x = 0, y = 3, z = 6ß

# def set_move_values(input_data: List[str]) -> List[List[str]]:
# opponent_moves = []
# your_moves = []
# for move_pair in input_data:
# split_moves = move_pair.split()
# opponent_moves.append(split_moves[0])
# your_moves.append(split_moves[1])
# print(opponent_moves)
# print(your_moves)
# return opponent_moves, your_moves
# opponent_moves,your_moves = set_move_values(input_data)
# using a tuple here to allow the two output variables
# opponent_moves and your_moves to be stored in separate variables


def main():
    input_data = read_file(FILE_PATH)
    # input_data = ['A Y', 'B X', 'C Z']
    shape_selection_total = shape_selection_evaluation(input_data)
    round_selection_total = round_outcome_evaluation(input_data)
    strategy_guide_total = shape_selection_total + round_selection_total
    print(strategy_guide_total)


if __name__ == "__main__":
    main()

# get input data
# define opponent move and your move variables
# for each pairing, calculate if win, loss or draw
# store running total score in an int variable
