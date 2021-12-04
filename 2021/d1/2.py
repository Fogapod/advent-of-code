with open("input") as f:
    data = [int(l[:-1]) for l in f.readlines()]

counter = 0

WINDOW_SIZE = 3

current_sum = sum(data[i] for i in range(WINDOW_SIZE))

for i in range(len(data) - WINDOW_SIZE):
    new_sum = current_sum - data[i] + data[i + WINDOW_SIZE]

    if new_sum > current_sum:
        counter += 1

    current_sum = new_sum

print(counter)
