with open("input") as f:
    data = [(l.split()[0], int(l.split()[1])) for l in f.readlines()]

coords = {"horizontal": 0, "vertical": 0}
aim = 0

for (command, dist) in data:
    if command == "forward":
        coords["horizontal"] += dist
        coords["vertical"] += aim * dist
    elif command == "up":
        aim -= dist
    elif command == "down":
        aim += dist
    else:
        print(f"Unknown command {command} {dist}")
        break

print(coords["horizontal"] * coords["vertical"])
