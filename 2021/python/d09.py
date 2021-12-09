from pathlib import Path


file = Path(__file__)

with open(file.parent.parent / "input" / file.stem / "full") as f:
    grid = [
        [9] + [int(point) for point in line.rstrip()] + [9] for line in f.readlines()
    ]

# these are different in example
grid_width = len(grid[0])
grid_height = len(grid) + 2

grid = [[9 for i in range(grid_width)]] + grid + [[9 for i in range(grid_width)]]

# part 1
answer = 0
for y in range(1, grid_height - 1):
    for x in range(1, grid_width - 1):
        if (
            grid[y][x] < grid[y - 1][x]
            and grid[y][x] < grid[y][x - 1]
            and grid[y][x] < grid[y + 1][x]
            and grid[y][x] < grid[y][x + 1]
        ):
            answer += grid[y][x] + 1

print("part 1:", answer)

# part 2
def flood(x: int, y: int) -> int:
    if x == 0 or x == grid_width or y == 0 or y == grid_height or grid[y][x] == 9:
        return 0

    grid[y][x] = 9

    return 1 + flood(x - 1, y) + flood(x, y - 1) + flood(x + 1, y) + flood(x, y + 1)


floods = []
for y in range(1, grid_height - 1):
    for x in range(1, grid_width - 1):
        if (
            grid[y][x] < grid[y - 1][x]
            and grid[y][x] < grid[y][x - 1]
            and grid[y][x] < grid[y + 1][x]
            and grid[y][x] < grid[y][x + 1]
        ):
            answer += grid[y][x] + 1

            floods.append(flood(x, y))

floods = sorted(floods)

print("part 2:", floods[-3] * floods[-2] * floods[-1])
