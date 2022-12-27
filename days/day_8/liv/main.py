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
    # print(tree_grid)
    return tree_grid


def visibility_check(tree_grid: List[List[int]]):
    max_x_dimension = np.size(tree_grid, 0)
    # print(f"Max X = {max_x_dimension}")
    max_y_dimension = np.size(tree_grid, 1)
    # print(f"Max Y = {max_y_dimension}")
    visible_trees = 0
    for x_index in range(tree_grid.shape[0]):
        for y_index in range(tree_grid.shape[1]):
            tree = tree_grid[x_index, y_index]
            # print(x_index, y_index)
            # print(tree)
            row = tree_grid[x_index, :]
            column = tree_grid[:, y_index]
            # print(row)
            # print(column)
            left_view = row[0:y_index]
            right_view = row[y_index + 1 : max_y_dimension]  # noqa E203
            up_view = column[0:x_index]
            down_view = column[x_index + 1 : max_x_dimension]  # noqa E203
            # print(f"Left View = {left_view}")
            # print(f"Right View = {right_view}")
            # print(f"Up View = {up_view}")
            # print(f"Down View = {down_view}")
            blocking_tree_left_count = np.count_nonzero(left_view >= tree)
            blocking_tree_right_count = np.count_nonzero(right_view >= tree)
            blocking_tree_up_count = np.count_nonzero(up_view >= tree)
            blocking_tree_down_count = np.count_nonzero(down_view >= tree)
            # print(f"Blocking Left: {blocking_tree_left_count}")
            # print(f"Blocking Right: {blocking_tree_right_count}")
            # print(f"Blocking Up: {blocking_tree_up_count}")
            # print(f"Blocking Down: {blocking_tree_down_count}")
            if blocking_tree_left_count == 0:
                # print("Visible from LEFT")
                visible_trees += 1
            elif blocking_tree_right_count == 0:
                # print("Visible from RIGHT")
                visible_trees += 1
            elif blocking_tree_up_count == 0:
                # print("Visible from UP")
                visible_trees += 1
            elif blocking_tree_down_count == 0:
                # print("Visible from DOWN")
                visible_trees += 1
    print(visible_trees)
    return visible_trees


def main():
    input_data = read_file(FILE_PATH)
    # input_data = read_file(TEST_FILE_PATH)
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
