from pathlib import Path

from functools import cache
from collections import Counter


file = Path(__file__)


with open(file.parent.parent / "input" / file.stem / "full") as f:
    initial_string = f.readline()[:-1]
    f.readline()
    rules = [line[:-1].split(" -> ") for line in f.readlines()]


rules = {rule[0]: rule[1] for rule in rules}


def _final_length(string: str, steps: int) -> int:
    return len(string) * 2 ** 10 - (2 ** 10 - 1)


@cache
def solve_pair(a: str, b: str, steps: int) -> Counter:
    if steps == 0:
        return Counter(a)

    steps -= 1

    k = rules[f"{a}{b}"]

    return solve_pair(a, k, steps) + solve_pair(k, b, steps)


# part 1
steps = 10

counter = Counter()

for i in range(0, len(initial_string) - 1):
    counter += solve_pair(initial_string[i], initial_string[i + 1], steps)

counter.update(initial_string[-1])

most_common = counter.most_common()

print("part 1:", most_common[0][1] - most_common[-1][1])

# part 2
steps = 40

counter = Counter()

for i in range(0, len(initial_string) - 1):
    counter += solve_pair(initial_string[i], initial_string[i + 1], steps)

counter.update(initial_string[-1])

most_common = counter.most_common()

print("part 2:", most_common[0][1] - most_common[-1][1])
