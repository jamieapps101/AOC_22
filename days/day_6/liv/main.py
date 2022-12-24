FILE_PATH = "days/day_6/liv/input.txt"


def read_file(file_path: str) -> str:
    with open(file_path) as fp:
        input_data = ""
        line = fp.read()
        input_data = line.rstrip()
        # print(input_data)
    return input_data


def identify_marker(input_data: str):
    marker_buffer = ""
    position_count = 0
    for char in input_data:
        if len(marker_buffer) < 14:
            marker_buffer += char
            # print(marker_buffer)
            position_count += 1
            # print(position_count)
        elif len(marker_buffer) == 14:
            # print(position_count)
            potential_marker = set(marker_buffer)
            # print(potential_marker)
            if len(potential_marker) == 14:
                print(position_count)
                break
            else:
                marker_buffer = marker_buffer[1:]
                # print(marker_buffer)
                marker_buffer += char
                # print(marker_buffer)
                position_count += 1

            # check if the characters are unique (DONE)
            # if yes, print the position count and exit the for loop
            # if not, remove the first character and append the new one


def main():
    input_data = read_file(FILE_PATH)
    # print(input_data)
    # input_data = "mjqjpqmgbljsphdztnvjfqwrcgsmlb"
    identify_marker(input_data)


if __name__ == "__main__":
    main()
