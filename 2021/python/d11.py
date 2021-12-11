from copy import deepcopy
from pathlib import Path


file = Path(__file__)

# random big number that will never overflow
BORDER_VALUE = -1000000

with open(file.parent.parent / "input" / file.stem / "full") as f:
    grid = [[int(number) for number in line[:-1]] for line in f.readlines()]

REAL_GRID_SIDE = len(grid[0])

for i in range(len(grid)):
    grid[i] = [BORDER_VALUE] + grid[i] + [BORDER_VALUE]

grid = (
    [[BORDER_VALUE for _ in range(REAL_GRID_SIDE + 2)]]
    + grid
    + [[BORDER_VALUE for _ in range(REAL_GRID_SIDE + 2)]]
)

grid_backup = deepcopy(grid)


def print_grid():
    for i in range(REAL_GRID_SIDE):
        line = ""
        for j in range(REAL_GRID_SIDE):
            line += str(grid[i + 1][j + 1])

        print(line)

    print()


print_grid()

# part 1


def recursive_flash(x: int, y: int) -> int:
    flashed = 0

    grid[y][x] += 1

    if grid[y][x] == 10:
        flashed += 1

        for (offset_x, offset_y) in (
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (1, -1),
            (1, 0),
            (1, 1),
            (0, -1),
            (0, 1),
        ):
            flashed += recursive_flash(x + offset_x, y + offset_y)

    return flashed


def reset_flashed() -> int:
    flashed_count = 0

    for i in range(REAL_GRID_SIDE):
        for j in range(REAL_GRID_SIDE):
            if grid[i + 1][j + 1] > 9:
                grid[i + 1][j + 1] = 0

                flashed_count += 1

    return flashed_count


answer = 0

for _ in range(100):
    for i in range(REAL_GRID_SIDE):
        for j in range(REAL_GRID_SIDE):
            answer += recursive_flash(i + 1, j + 1)

    reset_flashed()


print("part 1:", answer)

# part 2
grid = grid_backup

GRID_AREA = REAL_GRID_SIDE * REAL_GRID_SIDE

day = 0

while True:
    day += 1

    for i in range(REAL_GRID_SIDE):
        for j in range(REAL_GRID_SIDE):
            recursive_flash(i + 1, j + 1)

    if reset_flashed() == GRID_AREA:
        break

print("part 2:", day)
