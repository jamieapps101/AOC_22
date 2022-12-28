from typing import List, Tuple
import re

FILE_PATH = "days/day_9/liv/input.txt"
TEST_FILE_PATH = "days/day_9/liv/test_data.txt"
DIRECTION_RE = re.compile(r"^([A-Z]+)")
DISTANCE_RE = re.compile(r"([0-9]+)$")


def read_file(file_path: str) -> List[Tuple[str, int]]:
    with open(file_path) as fp:
        movement_data = []
        instruction = ()
        for line in fp:
            line = line.rstrip()
            direction = DIRECTION_RE.match(line)
            # print(direction)
            distance = DISTANCE_RE.search(line)
            # print(distance)
            instruction = (direction.group(1), int(distance.group(1)))
            movement_data.append(instruction)
    # print(movement_data)
    return movement_data


def tail_movements(head_coordinates: List[int], tail_coordinates: List[int]):
    if (
        abs(head_coordinates[0] - tail_coordinates[0]) >= 2
        or abs(head_coordinates[1] - tail_coordinates[1]) >= 2
    ):
        if (
            head_coordinates[0] == tail_coordinates[0]
            and head_coordinates[1] > tail_coordinates[1]
        ):
            tail_coordinates[1] += 1
            # Head Up movement
        elif (
            head_coordinates[0] == tail_coordinates[0]
            and head_coordinates[1] < tail_coordinates[1]
        ):
            tail_coordinates[1] -= 1
            # Head Down movement
        elif (
            head_coordinates[1] == tail_coordinates[1]
            and head_coordinates[0] > tail_coordinates[0]
        ):
            tail_coordinates[0] += 1
            # Head Right movement
        elif (
            head_coordinates[1] == tail_coordinates[1]
            and head_coordinates[0] < tail_coordinates[0]
        ):
            tail_coordinates[0] -= 1
            # Head Left movement
        elif (
            head_coordinates[0] > tail_coordinates[0]
            and head_coordinates[1] > tail_coordinates[1]
        ):
            tail_coordinates[0] += 1
            tail_coordinates[1] += 1
            # Head Right Up diagonal
        elif (
            head_coordinates[0] < tail_coordinates[0]
            and head_coordinates[1] < tail_coordinates[1]
        ):
            tail_coordinates[0] -= 1
            tail_coordinates[1] -= 1
            # Head Left Down diagonal
        elif (
            head_coordinates[0] > tail_coordinates[0]
            and head_coordinates[1] < tail_coordinates[1]
        ):
            tail_coordinates[0] += 1
            tail_coordinates[1] -= 1
            # Head Right Down diagonal
        elif (
            head_coordinates[0] < tail_coordinates[0]
            and head_coordinates[1] > tail_coordinates[1]
        ):
            tail_coordinates[0] -= 1
            tail_coordinates[1] += 1
            # Head Left Up diagonal
    return tail_coordinates


def head_movements(movement_data: List[Tuple[str, int]]):
    head_coordinates = [0, 0]
    tail_coordinates = [0, 0]
    tail_location_log = {}
    for instruction in movement_data:
        if instruction[0] == "R":
            for step in range(instruction[1]):
                # print(f"Right Step = {step}")
                head_coordinates[0] += 1
                # print(f"Head Coordinates = {head_coordinates}")
                tail_coordinates = tail_movements(head_coordinates, tail_coordinates)
                # print(f"Tail Coordinates = {tail_coordinates}")
                candidate_tail_location = tuple(tail_coordinates)
                # print(f"Log Tail Location = {candidate_tail_location}")
                if not candidate_tail_location in tail_location_log:
                    tail_location_log[candidate_tail_location] = 1
                # print(f"Tail Location Log: {tail_location_log}")
        elif instruction[0] == "L":
            for step in range(instruction[1]):
                # print(f"Left Step = {step}")
                head_coordinates[0] -= 1
                # print(f"Head Coordinates = {head_coordinates}")
                tail_coordinates = tail_movements(head_coordinates, tail_coordinates)
                # print(f"Tail Coordinates = {tail_coordinates}")
                candidate_tail_location = tuple(tail_coordinates)
                # print(f"Log Tail Location = {candidate_tail_location}")
                if not candidate_tail_location in tail_location_log:
                    tail_location_log[candidate_tail_location] = 1
                # print(f"Tail Location Log: {tail_location_log}")
        elif instruction[0] == "U":
            for step in range(instruction[1]):
                # print(f"Up Step = {step}")
                head_coordinates[1] += 1
                # print(f"Head Coordinates = {head_coordinates}")
                tail_coordinates = tail_movements(head_coordinates, tail_coordinates)
                # print(f"Tail Coordinates = {tail_coordinates}")
                candidate_tail_location = tuple(tail_coordinates)
                # print(f"Log Tail Location = {candidate_tail_location}")
                if not candidate_tail_location in tail_location_log:
                    tail_location_log[candidate_tail_location] = 1
                # print(f"Tail Location Log: {tail_location_log}")
        elif instruction[0] == "D":
            for step in range(instruction[1]):
                # print(f"Down Step = {step}")
                head_coordinates[1] -= 1
                # print(f"Head Coordinates = {head_coordinates}")
                tail_coordinates = tail_movements(head_coordinates, tail_coordinates)
                # print(f"Tail Coordinates = {tail_coordinates}")
                candidate_tail_location = tuple(tail_coordinates)
                # print(f"Log Tail Location = {candidate_tail_location}")
                if not candidate_tail_location in tail_location_log:
                    tail_location_log[candidate_tail_location] = 1
                # print(f"Tail Location Log: {tail_location_log}")
        # print(f"Head Coordinates: {head_coordinates}")
        # if head_coordinates[0] - tail_coordinates[0] >= 2 or head_coordinates[1] - tail_coordinates[1] >= 2:
        #     if head_coordinates[0] == tail_coordinates[0] and head_coordinates[1] > tail_coordinates[1]:
        #         tail_coordinates[1] += 1
        #     elif head_coordinates[0] == tail_coordinates[0] and head_coordinates[1] < tail_coordinates[1]:
        #         tail_coordinates[1] -= 1
        #     elif head_coordinates[1] == tail_coordinates[1] and head_coordinates[0] > tail_coordinates[0]:
        #         tail_coordinates[0] += 1
        #     elif head_coordinates[1] == tail_coordinates[1] and head_coordinates[0] < tail_coordinates[0]:
        #         tail_coordinates[0] -= 1
        #     elif head_coordinates[0] > tail_coordinates[0] and head_coordinates[1] > tail_coordinates[1]:
        #         tail_coordinates[0] += 1
        #         tail_coordinates[1] += 1
        #     elif head_coordinates[0] < tail_coordinates[0] and head_coordinates[1] < tail_coordinates[1]:
        #         tail_coordinates[0] -= 1
        #         tail_coordinates[1] -= 1
        #     elif head_coordinates[0] > tail_coordinates[0] and head_coordinates[1] < tail_coordinates[1]:
        #         tail_coordinates[0] += 1
        #         tail_coordinates[1] -= 1
        #     elif head_coordinates[0] < tail_coordinates[0] and head_coordinates[1] > tail_coordinates[1]:
        #         tail_coordinates[0] -= 1
        #         tail_coordinates[1] += 1
        #   # print(f"Tail Coordinates: {tail_coordinates}")
    tail_locations = sum(tail_location_log.values())
    print(f"Number of Unique Tail Locations: {tail_locations}")
    return head_coordinates, tail_locations


# positive/negative coordinates - [x,y] (list for head coordinates, list for tail coordinates) - DONE
# read in the head movements - DONE
# apply movements to the head - DONE
# apply consequent movements to the tail if needed
# if head x coordinate or head y coordinate is more than 2+ tail coordinates:
# if head is aligned with tail in x and head y is > tail y: HEAD UP (DONE)
# tail moves +1 in y
# if head is aligned with tail in x and head y is < tail y: HEAD DOWN (DONE)
# tail moves -1 in y
# if head is aligned with tail in y and head x is > tail x: HEAD RIGHT (DONE)
# tail moves +1 in x
# if head is aligned with tail in y and head x is < tail x: HEAD LEFT (DONE)
# tail moves -1 in x
# if head is not aligned with tail in x or y and head x and y are both > tail x and y: (RU diagonal)
# tail moves +1 in x and y (DONE)
#  if head is not aligned with tail in x or y and head x and y are both < tail x and y: (LD diagonal)
# tail moves -1 in x and y (DONE)
# if head is not aligned with tail in x or y, head x is > tail x and head y is < tail y: (RD diagonal)
# tail moves +1 in x and -1 in y (DONE)
# if head is not aligned with tail in x or y, head x is < tail x and head y is > tail y: (LU diagonal)
# tail moves -1 in x and +1 in y
# use a dictionary to log the coordinates the tail has visited

#           |
#           |
#           |
#           |
#           |
# - - - - - | - - - - -
#           |
#           |
#           |
#           |


def main():
    input_data = read_file(FILE_PATH)
    # input_data = read_file(TEST_FILE_PATH)
    head_movements(input_data)


if __name__ == "__main__":
    main()
