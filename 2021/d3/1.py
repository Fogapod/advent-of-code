with open("input") as f:
    data = [l[:-1] for l in f.readlines()]

gamma = 0b0
epsilon = 0b0

for i in range(len(data[0])):
    count_0 = 0
    count_1 = 0

    for line in data:
        if line[i] == "0":
            count_0 += 1
        else:
            count_1 += 1

    if count_0 > count_1:
        bit = 0
    elif count_1 > count_0:
        bit = 1
    else:
        print(f"whoops, equal number of bits on row {i}")
        break

    gamma |= bit << i

    # python has signed numbers, so mask bit back from negative
    epsilon |= (~bit + 2) << i

print(gamma * epsilon)
