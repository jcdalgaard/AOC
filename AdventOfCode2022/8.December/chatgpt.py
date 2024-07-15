with open('t.txt') as f:
        grid = f.read().strip().split('\n')

for x in range(len(grid)):
    grid[x] = grid[x].split()

        

 
num_visible_trees = 0

# Iterate through the rows
for row in grid:
    # Find the tallest tree in the row
    tallest_tree = max(row)
    # Iterate through the trees in the row
    for tree in row:
        # If the tree is taller than all the other trees in the row, it is visible
        if tree == tallest_tree:
            num_visible_trees += 1

# Iterate through the columns
for col in range(len(grid[0])):
    # Find the tallest tree in the column
    tallest_tree = max(grid[row][col] for row in range(len(grid)))
    # Iterate through the trees in the column
    for row in range(len(grid)):
        tree = grid[row][col]
        # If the tree is taller than all the other trees in the column, it is visible
        if tree == tallest_tree:
            num_visible_trees += 1

# Subtract the trees on the edge, which are already counted twice
num_visible_trees -= 2 * len(grid[0])

print(num_visible_trees)