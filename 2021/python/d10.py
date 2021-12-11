from pathlib import Path


file = Path(__file__)

with open(file.parent.parent / "input" / file.stem / "full") as f:
    lines = [line.rstrip() for line in f.readlines()]

CHARACTER_COSTS = {
    ")": 3,
    "]": 57,
    "}": 1197,
    ">": 25137,
}

OPENING_TO_CLOSING_MAP = {
    "(": ")",
    "[": "]",
    "{": "}",
    "<": ">",
}

OPENING = set("([{<")

# part 1
answer = 0

for line in lines:
    stack = []

    for c in line:
        if c in OPENING:
            stack.append(c)

            continue

        # assuming input does not start with closing bracket
        popped = stack.pop()

        if popped in OPENING and OPENING_TO_CLOSING_MAP[popped] != c:
            answer += CHARACTER_COSTS[c]
            break


print("part 1:", answer)

# part 2
CHARACTER_COSTS = {
    "(": 1,
    "[": 2,
    "{": 3,
    "<": 4,
}

scores = []

for line in lines:
    stack = []

    valid = True

    for c in line:
        if c in OPENING:
            stack.append(c)

            continue

        # assuming input does not start with closing bracket
        popped = stack.pop()

        if popped in OPENING and OPENING_TO_CLOSING_MAP[popped] != c:
            valid = False

            break

    if not valid:
        continue

    score = 0
    for unclosed in reversed(stack):
        score = score * 5 + CHARACTER_COSTS[unclosed]

    scores.append(score)

scores.sort()

print("part 2:", scores[int(len(scores) / 2)])
