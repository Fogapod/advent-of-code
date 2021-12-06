import functools


DAYS_1 = 80
DAYS_2 = 256

with open("input") as f:
    top_level_fish = [int(f) for f in f.read().split(",")]

@functools.cache
def solve(days_until_duplication: int, days: int) -> int:
    days -= days_until_duplication

    if days > 0:
        return solve(7, days) + solve(9, days)

    return 1

print(sum([solve(f, DAYS_1) for f in top_level_fish]))
print(sum([solve(f, DAYS_2) for f in top_level_fish]))
