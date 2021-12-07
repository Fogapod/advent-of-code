import re
from pathlib import Path


VALUES_RE = re.compile(r"(\d+),(\d+) -> (\d+),(\d+)")
GRID_SIDE = 1000


file = Path(__file__)

with open(file.parent.parent / "input" / file.stem / "full") as f:
    data = [VALUES_RE.match(i) for i in f.readlines()]

vectors = [(int(i[1]), int(i[2]), int(i[3]), int(i[4])) for i in data]

grid = [[0 for _ in range(GRID_SIDE)] for _ in range(GRID_SIDE)]


def print_grid():
    value_map = {
        0: ".",
        1: "1",
        2: "2",
        3: "3",
        4: "4",
        5: "5",
        6: "6",
        7: "7",
        8: "8",
        9: "9",
    }

    for row in grid:
        for val in row:
            print(value_map.get(val, "!"), end="")

        print()


# task 1
for v in vectors:
    if (x_len := v[2] - v[0]) != 0 and v[1] == v[3]:
        start_x = min(v[0], v[2])

        for i in range(start_x, start_x + abs(x_len) + 1):
            grid[v[1]][i] += 1

    elif (y_len := v[3] - v[1]) != 0 and v[0] == v[2]:
        start_y = min(v[1], v[3])

        for i in range(start_y, start_y + abs(y_len) + 1):
            grid[i][v[0]] += 1

# print_grid()


def count_dangerous() -> int:
    return sum(sum(1 for i in row if i > 1) for row in grid)


print(count_dangerous())  # 4993

grid = [[0 for _ in range(GRID_SIDE)] for _ in range(GRID_SIDE)]

# task 2
for v in vectors:
    x1, y1, x2, y2 = v

    if y1 == y2:
        dist_x = abs(x1 - x2)
        start_x = min(x1, x2)

        for i in range(start_x, start_x + dist_x + 1):
            grid[y1][i] += 1

    elif x1 == x2:
        dist_y = abs(y1 - y2)
        start_y = min(y1, y2)

        for i in range(start_y, start_y + dist_y + 1):
            grid[i][x1] += 1
    else:
        dist_x = x2 - x1
        dist_y = y2 - y1

        start_x = min(x1, x2)

        if dist_x - dist_y == 0:
            start_y = min(y1, y2)
            for i in range(abs(dist_x) + 1):
                grid[start_y + i][start_x + i] += 1
        else:
            start_y = max(y1, y2)
            for i in range(abs(dist_x) + 1):
                grid[start_y - i][start_x + i] += 1


# print_grid()

print(count_dangerous())  # 21101
