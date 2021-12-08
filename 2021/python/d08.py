from pathlib import Path


file = Path(__file__)

with open(file.parent.parent / "input" / file.stem / "full") as f:
    data = [line.rsplit("|", 1) for line in f.readlines()]

# clean up data a little, removing newlines and spaces
for i in range(len(data)):
    (patterns, output) = data[i]

    data[i] = (patterns[:-1].split(" "), output[1:-1].split(" "))

# part 1
count = 0

#                            1, 4, 7, 8
unique_segment_counts = set((2, 4, 3, 7))

for (_, outputs) in data:
    for value in outputs:
        if len(value) in unique_segment_counts:
            count += 1

print("part 1:", count)

# part 2
SEGMENT_COUNT = 7

SEGMENTS_TO_VALUE_MAP = [0 for _ in range(2 ** SEGMENT_COUNT)]

for (i, combination) in enumerate(
    (
        "abcefg",
        "cf",
        "acdeg",
        "acdfg",
        "bcdf",
        "abdfg",
        "abdefg",
        "acf",
        "abcdefg",
        "abcdfg",
    )
):
    SEGMENTS_TO_VALUE_MAP[sum(1 << (ord(c) - ord("a")) for c in combination)] = i

num_sum = 0

for (inputs, outputs) in data:
    # sort by length
    inputs = sorted(inputs, key=lambda i: len(i))

    segment_map = {}

    # unique lengths
    one = inputs[0]
    seven = inputs[1]
    four = inputs[2]
    eight = inputs[9]

    # difference between 1 and 7 us segment "a"
    segment_map["a"] = [i for i in seven if i not in one][0]

    # segment 4 has all letters of 1 + "b" and "d"
    b_or_d = [i for i in four if i not in one]

    # 0, 6 and 9 have 6 segments turned on
    zero_or_six_or_nine = (inputs[6], inputs[7], inputs[8])

    # 0, 6 and 9 are missing segments "c", "d" and "e"
    c_or_d_or_e = [
        i
        for i in eight
        if i not in zero_or_six_or_nine[0]
        or i not in zero_or_six_or_nine[1]
        or i not in zero_or_six_or_nine[2]
    ]

    # d is in both places
    if b_or_d[0] in c_or_d_or_e:
        segment_map["d"], segment_map["b"] = b_or_d[0], b_or_d[1]
    else:
        segment_map["b"], segment_map["d"] = b_or_d[0], b_or_d[1]

    # remove decoded d
    c_or_e = [c for c in c_or_d_or_e if c != segment_map["d"]]

    # 1 has "c" segment
    if c_or_e[0] in one:
        segment_map["c"], segment_map["e"] = c_or_e[0], c_or_e[1]
    else:
        segment_map["e"], segment_map["c"] = c_or_e[0], c_or_e[1]

    # 1 has "c" and "f"
    if one[0] == segment_map["c"]:
        segment_map["f"] = one[1]
    else:
        segment_map["f"] = one[0]

    segment_map["g"] = [c for c in eight if c not in segment_map.values()][0]

    segment_map = {v: k for (k, v) in segment_map.items()}

    # for (k, v) in segment_map.items():
    #     print(f'{1 << (ord(k) - ord("a"))} -> {1 << (ord(v) - ord("a"))}')

    num = 0
    for i, value in enumerate(reversed(outputs)):
        v_len = len(value)

        if v_len == 2:
            digit = 1
        elif v_len == 3:
            digit = 7
        elif v_len == 4:
            digit = 4
        elif v_len == 7:
            digit = 8
        else:
            digit = SEGMENTS_TO_VALUE_MAP[
                sum(1 << (ord(segment_map[c]) - ord("a")) for c in value)
            ]

        num += digit * (10 ** i)

    num_sum += num

print("part 2:", num_sum)
