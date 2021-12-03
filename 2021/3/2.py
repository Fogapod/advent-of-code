with open("input") as f:
    data = [l[:-1] for l in f.readlines()]

bit_count = len(data[0])

data = [int(i, base=2) for i in data]

# 1st task
gamma = 0b0
epsilon = 0b0

for i in range(bit_count):
    ratio = 0

    for line in data:
        if line >> i & 1 == 1:
            ratio += 1
        else:
            ratio -= 1

    if ratio > 0:
        bit = 1
    elif ratio < 0:
        bit = 0
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
    ratio = 0

    for val in arr:
        if val >> position & 1 == 1:
            ratio += 1
        else:
            ratio -= 1

    return int(ratio >= 0)

for i in reversed(range(bit_count)):
    if len(oxy_candidates) > 1:
        bit = most_common_bit(oxy_candidates, i)
        oxy_candidates = list(filter(lambda x: x >> i & 1 == bit, oxy_candidates))

    if len(co2_candidates) > 1:
        bit = most_common_bit(co2_candidates, i)
        co2_candidates = list(filter(lambda x: x >> i & 1 != bit, co2_candidates))

print(oxy_candidates[0] * co2_candidates[0])
