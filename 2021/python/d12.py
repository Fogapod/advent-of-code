from pathlib import Path


file = Path(__file__)


with open(file.parent.parent / "input" / file.stem / "full") as f:
    data = [line[:-1].split("-") for line in f.readlines()]


starting_points = []

connections = {}

for (start, end) in data:
    if start == "start":
        starting_points.append(end)
    elif end == "start":
        starting_points.append(start)
    else:
        if existing := connections.get(start):
            existing.append(end)
        else:
            connections[start] = [end]

        if existing := connections.get(end):
            existing.append(start)
        else:
            connections[end] = [start]


# part 1
def check_all_paths(path: str, point: str, visited) -> int:
    path += f",{point}"

    if point == "end":
        return 1

    if point.islower():
        if point in visited:
            return 0

        # prevent list mutation during recursion
        new_visited = visited + [point]
    else:
        new_visited = visited

    result = 0

    for next_point in connections[point]:
        result += check_all_paths(path, next_point, new_visited)

    return result


paths = 0

for start in starting_points:
    paths += check_all_paths("start", start, [])


print("part 1:", paths)

#     start
#     /   \
# c--A-----b--d
#     \   /
#      end

# part 2
def check_all_paths(
    path: str, point: str, visited, second_visit_happenned: bool
) -> int:
    path += f",{point}"

    if point == "end":
        return 1

    if point.islower():
        if point in visited:
            if second_visit_happenned:
                return 0
            else:
                second_visit_happenned = True

        # prevent list mutation during recursion
        new_visited = visited + [point]
    else:
        new_visited = visited

    result = 0

    for next_point in connections[point]:
        result += check_all_paths(path, next_point, new_visited, second_visit_happenned)

    return result


paths = 0

for start in starting_points:
    paths += check_all_paths("start", start, [], False)


print("part 2:", paths)
