from typing import List

FILE_PATH = "days/day_1/input_data/input.txt"


def read_file(file_path: str) -> List[str]:
    lines = []
    with open(file_path) as fp:
        for line in fp:
            line = line.rstrip()
            lines.append(line)
    return lines


def sum_calories(input_data: List[str]) -> List[int]:
    """input_data is the list of strings from FILE_PATH.
    which will end up as a list of ints."""
    numbers = []
    sum_totals = []
    for number in input_data:
        if number != "":
            number = int(number)
            numbers.append(number)
        else:
            group_total = sum(numbers)
            sum_totals.append(group_total)
            numbers = []
    return sum_totals


def main():
    calorie_counts = read_file(FILE_PATH)
    total_group_calories = sum_calories(calorie_counts)
    # most_calorific_elf = max(total_group_calories)
    # print(most_calorific_elf)
    sorted_calories_list = sorted(total_group_calories, reverse=True)
    top_three_calorific_elves = sum(sorted_calories_list[0:3])
    # slicer for integers - the last index is non-inclusive
    # so the slicer above with return indexed values 0, 1 and 2.
    print(top_three_calorific_elves)


if __name__ == "__main__":
    main()
