from typing import List

# from typing import Dict

FILE_PATH = "days/day_3/liv/input.txt"


def read_file(file_path: str) -> List[str]:
    lines = []
    with open(file_path) as fp:
        for line in fp:
            line = line.rstrip()
            lines.append(line)
    # print(lines)
    return lines


# def set_priority_values(items: str):
#     priority_values = {}
#     for item in items:
#         if item.islower():
#             if item not in priority_values:
#                 priority_values[item] = ord(item) - 96
#         elif item not in priority_values:
#             priority_values[item] = ord(item) - 65 + 27
#     # print(priority_values)
#     return priority_values


# dict = {}
# for n in n1:
# if # condition #
# if key not in dict:
# dict[key] = []
# dict[key].append(value)
# print dict
# example of appending to a dictionary from stackoverflow


# def rucksack_split(
#     input_data: List[str],
#     item_priorities: Dict[str, int],
# ):
#     priority_sum = 0
#     for bag_contents in input_data:
#         items_per_bag = len(bag_contents) // 2
#         # print(items_per_bag)
#         rucksack_one = bag_contents[items_per_bag:]
#         # print(rucksack_one)
#         rucksack_two = bag_contents[:items_per_bag]
#         # print(rucksack_two)
#         shared_item = "".join(set(rucksack_one).intersection(rucksack_two))
#         # print(shared_item)
#         item_priority = item_priorities[shared_item]
#         # print(item_priority)
#         priority_sum += item_priority
#         # print(priority_sum)
#     print(priority_sum)
#     return priority_sum


def group_elves(input_data: List[str]):
    elf_count = 0
    elf_trio = []
    all_elf_groups = []
    for elf in input_data:
        if elf_count < 2:
            elf_trio.append(elf)
            elf_count += 1
            # print(elf_trio)
            # print(elf_count)
        elif elf_count == 2:
            elf_trio.append(elf)
            all_elf_groups.append(elf_trio)
            # print(elf_count)
            # print(elf_trio)
            # print(all_elf_groups)
            elf_trio = []
            elf_count = 0
            # print(elf_trio)
            # print(elf_count)
    # print(elf_count)
    # print(elf_trio)
    # print(all_elf_groups)
    return all_elf_groups


def main():
    input_data = [
        "vJrwpWtwJgWrhcsFMMfFFhFp",
        "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
        "PmmdzqPrVvPwwTWBwg",
        "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
        "ttgJtRGJQctTZtZT",
        "CrZsJsPPZsGzwwsLwLmpwMDw",
    ]
    # input_data = read_file(FILE_PATH)
    # item_priorities = set_priority_values(
    # "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
    # )
    # rucksack_split(input_data, item_priorities)
    elf_groups = group_elves(input_data)
    print(elf_groups)


if __name__ == "__main__":
    main()

# logic (challenge 1):
# half the length of the strings
# identify the common character that appears in each string
# assign priorities to lowercase and uppercase alphabet
# sum the priorities of all items appearing twice

# logic (challenge 2):
# group whole strings into threes
# identify common character between the three
# calculate priority value and add to total
