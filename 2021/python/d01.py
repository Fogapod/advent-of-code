from pathlib import Path


file = Path(__file__)

with open(file.parent.parent / "input" / file.stem / "full") as f:
    data = [int(l[:-1]) for l in f.readlines()]

# task 1
counter = 0

for i in range(len(data) - 1):
    if data[i + 1] > data[i]:
        counter += 1

print(counter)

# task 2
counter = 0

WINDOW_SIZE = 3

current_sum = sum(data[i] for i in range(WINDOW_SIZE))

for i in range(len(data) - WINDOW_SIZE):
    new_sum = current_sum - data[i] + data[i + WINDOW_SIZE]

    if new_sum > current_sum:
        counter += 1

    current_sum = new_sum

print(counter)
