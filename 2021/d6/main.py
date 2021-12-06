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

# for rust solution
def gen_solution_maps():
    print(
        f"""
const SOLUTION_MAP_1: [i64; 10] = [
    {", ".join(str(solve(i, 80)) for i in range(10))}
];
        """
    )
    print(
        f"""
const SOLUTION_MAP_2: [i64; 10] = [
    {", ".join(str(solve(i, 256)) for i in range(10))}
];
        """
    )


# this does not work
def gen_ugly_code():
    print(
        " +\n".join(
            f"SOLUTION_MAP_2.get_unchecked((input.get_unchecked({i}) - b'0') as usize)"
            for i in range(0, 600, 2)
        )
    )


# gen_solution_maps()
# gen_ugly_code()
