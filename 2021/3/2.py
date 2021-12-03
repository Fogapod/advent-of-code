with open("input") as f:
    data = [l[:-1] for l in f.readlines()]

bit_count = len(data[0])

data = [int(i, base=2) for i in data]

# 1st task
gamma = 0b0
epsilon = 0b0

for i in range(bit_count):
    count_0 = 0
    count_1 = 0

    for line in data:
        if line >> i & 1 == 1:
            count_1 += 1
        else:
            count_0 += 1

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

# 2nd task
oxy_candidates = data.copy()
co2_candidates = data.copy()

def most_common_bit(arr, position: int) -> int:
    count_0 = 0
    count_1 = 0

    for val in arr:
        if val >> position & 1 == 1:
            count_1 += 1
        else:
            count_0 += 1

    return int(count_1 >= count_0)

for i in reversed(range(bit_count)):
    if len(oxy_candidates) > 1:
        bit = most_common_bit(oxy_candidates, i)
        oxy_candidates = list(filter(lambda x: x >> i & 1 == bit, oxy_candidates))

    if len(co2_candidates) > 1:
        bit = most_common_bit(co2_candidates, i)
        co2_candidates = list(filter(lambda x: x >> i & 1 != bit, co2_candidates))

print(oxy_candidates[0] * co2_candidates[0])
