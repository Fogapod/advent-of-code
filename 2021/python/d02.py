from pathlib import Path


file = Path(__file__)

with open(file.parent.parent / "input" / file.stem / "full") as f:
    data = [(l.split()[0], int(l.split()[1])) for l in f.readlines()]

# task 1
coords = {"horizontal": 0, "vertical": 0}

for (command, dist) in data:
    if command == "forward":
        coords["horizontal"] += dist
    elif command == "up":
        coords["vertical"] -= dist
    elif command == "down":
        coords["vertical"] += dist
    else:
        print(f"Unknown command {command} {dist}")
        break

print(coords["horizontal"] * coords["vertical"])

# task 2
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
