from typing import List

FILE_PATH = "days/day_4/liv/input.txt"


def read_file(file_path: str) -> List[str]:
    lines = []
    with open(file_path) as fp:
        for line in fp:
            line = line.rstrip()
            lines.append(line)
    # print(lines)
    return lines


def section_coverage(input_data: List[str]):
    elf_one = ""
    elf_two = ""
    elf_one_minimum = ""
    elf_one_maximum = ""
    elf_two_minimum = ""
    elf_two_maximum = ""
    elf_one_range = []
    elf_two_range = []
    assignment_covered_count = 0
    for pair_assignment in input_data:
        elf_one, elf_two = pair_assignment.split(",")
        # print(elf_one)
        # print(elf_two)
        elf_one_minimum, elf_one_maximum = elf_one.split("-")
        elf_two_minimum, elf_two_maximum = elf_two.split("-")
        # print(elf_one_minimum)
        # print(elf_one_maximum)
        # print(elf_two_minimum)
        # print(elf_two_maximum)
        elf_one_minimum, elf_one_maximum, elf_two_minimum, elf_two_maximum = (
            int(elf_one_minimum),
            int(elf_one_maximum),
            int(elf_two_minimum),
            int(elf_two_maximum),
        )
        # print(elf_one_minimum)
        # print(elf_one_maximum)
        # print(elf_two_minimum)
        # print(elf_two_maximum)
        elf_one_range.extend(range(elf_one_minimum, elf_one_maximum + 1))
        elf_two_range.extend(range(elf_two_minimum, elf_two_maximum + 1))
        # print(elf_one_range)
        # print(elf_two_range)
        if (
            elf_two_range.count(elf_one_minimum) == 1
            and elf_two_range.count(elf_one_maximum) == 1
            and elf_one_range.count(elf_two_minimum) == 1
            and elf_one_range.count(elf_two_maximum) == 1
        ):
            assignment_covered_count += 1
        elif (
            elf_two_range.count(elf_one_minimum) == 1
            and elf_two_range.count(elf_one_maximum) == 1
        ):
            assignment_covered_count += 1
        elif (
            elf_one_range.count(elf_two_minimum) == 1
            and elf_one_range.count(elf_two_maximum) == 1
        ):
            assignment_covered_count += 1
        # print(assignment_covered_count)
        elf_one = ""
        elf_two = ""
        elf_one_minimum = ""
        elf_one_maximum = ""
        elf_two_minimum = ""
        elf_two_maximum = ""
        elf_one_range = []
        elf_two_range = []
    return assignment_covered_count


# store values either side of the dash as
# pair 1 min, pair 1 max range and pair 2 min, pair 2 max range
# expand the ranges
# store in a pair 1 full range and pair 2 full range variable
# if first and last value in pair 1 present in pair 2
# add 1 to fully covered count
# if first and last value in pair 2 present in pair 1
# add 1 to fully covered count
# reset variables!


def main():
    input_data = read_file(FILE_PATH)
    # input_data = ['2-4,6-8','2-3,4-5','5-7,7-9','2-8,3-7',
    # '6-6,4-6','2-6,4-8']
    # print(input_data)
    covered_sections = section_coverage(input_data)
    print(covered_sections)


if __name__ == "__main__":
    main()
