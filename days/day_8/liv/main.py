from typing import List
import numpy as np

FILE_PATH = "days/day_8/liv/input.txt"
TEST_FILE_PATH = "days/day_8/liv/test_data.txt"


def read_file(file_path: str) -> List[int]:
    tree_row = []
    all_trees = []
    with open(file_path) as fp:
        for line in fp:
            line = line.rstrip()
            for char in line:
                char = int(char)
                tree_row.append(char)
                # print(tree_row)
            all_trees.append(tree_row)
            tree_row = []
        # print(all_trees)
    # print(all_trees)
    tree_grid = np.array(all_trees, np.int32)
    print(tree_grid)
    return tree_grid


def visibility_check(tree_grid: List[List[int]]):
    max_x_dimension = np.size(tree_grid, 0)
    # print(f"Max X = {max_x_dimension}")
    max_y_dimension = np.size(tree_grid, 1)
    # print(f"Max Y = {max_y_dimension}")
    highest_scenic_score = 0
    for x_index in range(tree_grid.shape[0]):
        for y_index in range(tree_grid.shape[1]):
            tree = tree_grid[x_index, y_index]
            print(x_index, y_index)
            print(tree)
            row = tree_grid[x_index, :]
            column = tree_grid[:, y_index]
            print(row)
            print(column)
            left_view = row[0:y_index]
            right_view = row[y_index + 1 : max_y_dimension]  # noqa E203
            up_view = column[0:x_index]
            down_view = column[x_index + 1 : max_x_dimension]  # noqa E203
            print(f"Left View = {left_view}")
            print(f"Right View = {right_view}")
            print(f"Up View = {up_view}")
            print(f"Down View = {down_view}")
            blocking_tree_left_count = np.count_nonzero(left_view >= tree)
            blocking_tree_right_count = np.count_nonzero(right_view >= tree)
            blocking_tree_up_count = np.count_nonzero(up_view >= tree)
            blocking_tree_down_count = np.count_nonzero(down_view >= tree)
            # print(f"Blocking Left: {blocking_tree_left_count}")
            # print(f"Blocking Right: {blocking_tree_right_count}")
            # print(f"Blocking Up: {blocking_tree_up_count}")
            # print(f"Blocking Down: {blocking_tree_down_count}")
            left_scenic_score = 0
            if blocking_tree_left_count == 0:
                # print(f"Left View = {left_view}")
                # print(f"Size of Left View: {len(left_view)}")
                left_scenic_score += len(left_view)
                # print(f"No Blocking Trees - Left Scenic Score:
                # {left_scenic_score}")
            elif blocking_tree_left_count != 0:
                reversed_left_view = np.array(list(reversed(left_view)))
                # print(f"Reversed Left View = {reversed_left_view}")
                for pine in reversed_left_view:
                    # print(f"Pine = {pine}")
                    if pine < tree:
                        left_scenic_score += 1
                    else:
                        left_scenic_score += 1
                        # print(f"Left Scenic Score (end for loop):
                        # {left_scenic_score}")
                        break
            right_scenic_score = 0
            if blocking_tree_right_count == 0:
                right_scenic_score += len(right_view)
                # print(f"No Blocking Trees - Right Scenic Score:
                # {right_scenic_score}")
            elif blocking_tree_right_count != 0:
                for pine in right_view:
                    if pine < tree:
                        right_scenic_score += 1
                    else:
                        right_scenic_score += 1
                        # print(f"Right Scenic Score (end for loop):
                        # {right_scenic_score}")
                        break
            up_scenic_score = 0
            if blocking_tree_up_count == 0:
                up_scenic_score += len(up_view)
                # print(f"No Blocking Trees - Up Scenic Score:
                # {up_scenic_score}")
            elif blocking_tree_up_count != 0:
                reversed_up_view = np.array(list(reversed(up_view)))
                for pine in reversed_up_view:
                    if pine < tree:
                        up_scenic_score += 1
                    else:
                        up_scenic_score += 1
                        # print(f"Up Scenic Score (end for loop):
                        # {up_scenic_score}")
                        break
            down_scenic_score = 0
            if blocking_tree_down_count == 0:
                down_scenic_score += len(down_view)
                # print(f"No Blocking Trees - Down Scenic Score:
                # {down_scenic_score}")
            elif blocking_tree_down_count != 0:
                for pine in down_view:
                    if pine < tree:
                        down_scenic_score += 1
                    else:
                        down_scenic_score += 1
                        # print(f"Down Scenic Score (end for loop):
                        # {down_scenic_score}")
                        break
            # print(f"Left Scenic Score: {left_scenic_score}")
            # print(f"Right Scenic Score: {right_scenic_score}")
            # print(f"Up Scenic Score: {up_scenic_score}")
            # print(f"Down Scenic Score: {down_scenic_score}")
            tree_scenic_score = (
                left_scenic_score
                * right_scenic_score
                * up_scenic_score
                * down_scenic_score
            )
            # print(f"Tree Scenic Score: {tree_scenic_score}")
            if tree_scenic_score > highest_scenic_score:
                highest_scenic_score = tree_scenic_score
                # print(f"Highest Scenic Score (for loop):
                # {highest_scenic_score}")
    print(f"Highest Scenic Score: {highest_scenic_score}")
    return highest_scenic_score


def main():
    # input_data = read_file(FILE_PATH)
    input_data = read_file(TEST_FILE_PATH)
    visibility_check(input_data)


if __name__ == "__main__":
    main()

# 0 is shortest, 9 is tallest
# Tree is visible if all other trees between it
# and the edge of the grid are shorter than it
# Up, down, left, right directions only
# All trees on the edge of the grid are visible -
# only interior trees need checking
# If a tree is visible from only one direction, it is still visible
# How many trees are visible from the outside of the grid?

#         if tree < tree_grid
#         # up and down is the column check - everything with the same y coord
#         # left and right is the row check - everything with the same x coord
#         # tree_index = tree.index()
#         if tree is tree[0,:] or tree[max_x_dimension,:]:
#             visible_trees += 1
#         elif tree is tree[:,0] or tree[:,max_y_dimension]:
#             visible_trees += 1
#         print(visible_trees)

# if tree has an x coord of 0 or max - on the outside, so visible
# if tree has a y coord of 0 or max - on the outside, so visible
# else look up, right, down and left to see if any equal to
# or larger numbers are present
# if yes, tree is not visible in that direction
# if no, tree is visible - increment visible_direction_count by 1
# if visible_direction_count = 4, tree is visible
# increment visible_trees by 1

# part two notes
# stop at edge or first same or taller tree
# count number of trees and multiply together
