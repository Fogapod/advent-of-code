with open("input") as f:
    data = [int(l[:-1]) for l in f.readlines()]

counter = 0

for i in range(len(data) - 1):
    if data[i + 1] > data[i]:
        counter += 1

print(counter)
