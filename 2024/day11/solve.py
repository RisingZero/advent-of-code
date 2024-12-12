#!/usr/bin/env python3

from functools import cache
import sys

TEST_INPUT = "125 17"

if len(sys.argv) == 3:
    with open(sys.argv[1]) as f:
        INPUT = f.read().strip()
    N = int(sys.argv[2])
else:
    INPUT = TEST_INPUT.strip()
    N = 6

# Part 1 (N = 25)
stones = [int(s) for s in INPUT.split()]
for i in range(N):
    next_stones = []
    for stone in stones:
        if stone == 0:
            next_stones.append(1)
        elif len(str(stone)) % 2 == 0:
            next_stones.append(int(str(stone)[: len(str(stone)) // 2]))
            next_stones.append(int(str(stone)[len(str(stone)) // 2 :]))
        else:
            next_stones.append(stone * 2024)
    stones = next_stones
print(f"Part 1:\n{len(stones) = }")

# Part 2 (N = 75)
N = 75


@cache
def f(n):
    if n == 0:
        return (1,)
    elif len(str(n)) % 2 == 0:
        return (int(str(n)[: len(str(n)) // 2]), int(str(n)[len(str(n)) // 2 :]))
    else:
        return (n * 2024,)


@cache
def recursive_apply(n, depth):
    if depth == 0:
        return 1
    blinked = f(n)
    return sum(recursive_apply(b, depth - 1) for b in blinked)


stones = [int(s) for s in INPUT.split()]
num_stones = sum(recursive_apply(s, N) for s in stones)
print(f"Part 2:\n{num_stones = }")
