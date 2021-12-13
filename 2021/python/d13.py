from pathlib import Path


file = Path(__file__)


with open(file.parent.parent / "input" / file.stem / "full") as f:
    coords, folds = f.read()[:-1].split("\n\n")

coords = [
    (
        int(line.split(",")[0]),
        int(line.split(",")[1]),
    )
    for line in coords.split("\n")
]


folds = [(line[11], int(line[13:])) for line in folds.split("\n")]


def print_grid():
    for y in range(size_y):
        line = ""

        for x in range(size_x):
            if grid[y][x] > 0:
                line += "#"
            else:
                line += "."

        print(line)
    print()


def count_points() -> int:
    result = 0

    for y in range(size_y):
        for x in range(size_x):
            if grid[y][x] > 0:
                result += 1

    return result


# part 1
size_x = max(coords, key=lambda i: i[0])[0] + 1
size_y = max(coords, key=lambda i: i[1])[1] + 1

grid = [[0 for _ in range(size_x)] for _ in range(size_y)]

for (x, y) in coords:
    grid[y][x] = 1


first_fold = folds[0]

if first_fold[0] == "x":
    for y in range(size_y):
        for x in range(first_fold[1]):
            grid[y][x] += grid[y][size_x - x - 1]

    size_x -= first_fold[1] + 1
else:
    for y in range(first_fold[1]):
        for x in range(size_x):
            grid[y][x] += grid[size_y - y - 1][x]

    size_y -= first_fold[1] + 1


print("part 1:", count_points())


# part 2
size_x = max(coords, key=lambda i: i[0])[0] + 1
size_y = max(coords, key=lambda i: i[1])[1] + 1

grid = [[0 for _ in range(size_x)] for _ in range(size_y)]

for (x, y) in coords:
    grid[y][x] = 1


for (axis, offset) in folds:
    if axis == "x":
        for y in range(size_y):
            for x in range(offset):
                grid[y][x] += grid[y][size_x - x - 1]

        size_x -= offset + 1
    else:
        for y in range(offset):
            for x in range(size_x):
                grid[y][x] += grid[size_y - y - 1][x]

        size_y -= offset + 1

print("part 2:")
print_grid()
