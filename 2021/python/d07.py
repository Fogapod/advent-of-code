import statistics
import math

from pathlib import Path


file = Path(__file__)

with open(file.parent.parent / "input" / file.stem / "full") as f:
    data = [int(i) for i in f.read().split(",")]


# task 1, manual
sorted_numbers = sorted(data)
lowest, highest = sorted_numbers[0], sorted_numbers[-1]

lowest_dist = sum((n - lowest) for n in sorted_numbers)

for i in range(lowest + 1, highest + 1):
    dist = sum(abs(n - i) for n in sorted_numbers)

    if dist < lowest_dist:
        lowest_dist = dist

# print(lowest_dist)

# task 1, median
median = int(statistics.median(data))

print(median, sum(abs(n - median) for n in sorted_numbers))

# task 2
# FIXME: produces wrong result for example if int() is used instead of round()
mean = int(sum(data) / len(data))

def triangular_number(n: int) -> float:
    return n * (n + 1) / 2


print(mean, int(sum(triangular_number(abs(n - mean)) for n in sorted_numbers)))
