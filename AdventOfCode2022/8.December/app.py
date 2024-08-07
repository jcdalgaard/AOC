"""Solution for AoC 2022 Day 08 - Treetop Tree House"""

from collections import defaultdict


def input_per_line(file: str):
    """This is for when each line is an input to the puzzle. The newline character is stripped."""
    with open(file, 'r') as input_file:
        return [line.rstrip() for line in input_file.readlines()]


def text_to_grid(text_map: list) -> (dict, int, int):
    """Take in a list of items that need to be parsed into a dictionary where each set of keys is a coordinate.
    Return a dictionary representation of the map.
    """
    grid_map = defaultdict(int)
    max_y = len(text_map)
    max_x = len(text_map[0])
    for y, line in enumerate(text_map):
        for x, number in enumerate(line):
            grid_map[(x, y)] = int(number)
    return grid_map, max_x, max_y


def determine_if_tree_is_visible(a_tree_map: dict, coordinates: tuple, map_height: int, map_width: int) -> bool:
    """A tree is visible if:
    a) it's on the edge of the map
    b) All trees in any of the cardinal directions are shorter than the tree.
    """
    this_tree_height = a_tree_map[coordinates]
    test_values_north = []
    test_values_south = []
    test_values_east = []
    test_values_west = []
    # First deal with the edges
    # coordinates = (x, y)
    if coordinates[0] == 0 or coordinates[0] == (map_width - 1) or coordinates[1] == 0 or coordinates[1] == (
            map_height - 1):
        return True
    # Look left - or a decreasing value for x
    for x in reversed(range(-1, coordinates[0])):
        if a_tree_map[(x, coordinates[1])] >= this_tree_height:
            test_values_west.append(False)
        else:
            test_values_west.append(True)
    # Look right - or increasing value for x
    for x in range(coordinates[0] + 1, map_width):
        if a_tree_map[(x, coordinates[1])] >= this_tree_height:
            test_values_east.append(False)
        else:
            test_values_east.append(True)
    # Look up - or decreasing value for y
    for y in reversed(range(-1, coordinates[1])):
        if a_tree_map[(coordinates[0], y)] >= this_tree_height:
            test_values_north.append(False)
        else:
            test_values_north.append(True)
    # Look down - increasing value for y
    for y in range(coordinates[1] + 1, map_height):
        if a_tree_map[(coordinates[0], y)] >= this_tree_height:
            test_values_south.append(False)
        else:
            test_values_south.append(True)
    return all(test_values_north) or all(test_values_south) or all(test_values_east) or all(test_values_west)


def compute_scenic_score(a_tree_map: dict, coordinates: tuple, map_height: int, map_width: int) -> int:
    """
    A tree's scenic score computed as follows:
    1. In each cardinal direction, add 1 until you cannot continue (because a tree is bigger or same size)
    2. Multiply these 4 numbers.
    """
    this_tree_height = a_tree_map[coordinates]
    scenic_value_north = 0
    scenic_value_south = 0
    scenic_value_east = 0
    scenic_value_west = 0
    # First deal with the edges
    # coordinates = (x, y)
    if coordinates[0] == 0 or coordinates[0] == (map_width - 1) or coordinates[1] == 0 or coordinates[1] == (
            map_height - 1):
        return 0  # If a tree is right on the edge, at least one of its viewing distances will be zero and something
                  # times 0 is 0.
    # Look left - or a decreasing value for x
    for x in reversed(range(coordinates[0])):
        scenic_value_west += 1
        if a_tree_map[(x, coordinates[1])] >= this_tree_height:
            break
    # Look right - or increasing value for x
    for x in range(coordinates[0] + 1, map_width):
        scenic_value_east += 1
        if a_tree_map[(x, coordinates[1])] >= this_tree_height:
            break
    # Look up - or decreasing value for y
    for y in reversed(range(coordinates[1])):
        scenic_value_north += 1
        if a_tree_map[(coordinates[0], y)] >= this_tree_height:
            break
    # Look down - increasing value for y
    for y in range(coordinates[1] + 1, map_height):
        scenic_value_south += 1
        if a_tree_map[(coordinates[0], y)] >= this_tree_height:
            break
    return scenic_value_north * scenic_value_south * scenic_value_east * scenic_value_west


if __name__ == "__main__":
    # Assume positive y is downwards and positive x is to the right.
    tree_map_text = input_per_line(r"C:\source\AdventOfCode2022\8.December\t.txt")
    tree_map, maximum_width, maximum_height = text_to_grid(tree_map_text)
    visible_trees = []
    scenic_values = []
    for this_x in range(maximum_width):
        for this_y in range(maximum_height):
            visible_trees.append(
                determine_if_tree_is_visible(tree_map, (this_x, this_y), maximum_height, maximum_width))
            scenic_values.append(compute_scenic_score(tree_map, (this_x, this_y), maximum_height, maximum_width))

    visible_tree_count = sum(tree for tree in visible_trees if tree)
    print(f"There are {visible_tree_count} trees visible from outside the grid.")
    print("Time to assess which tree has the highest scenic score.")
    print(f"The highest scenic score is: {max(scenic_values)}")