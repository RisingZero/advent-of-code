#!/usr/bin/env python3

import sys

test_input = """
3   4
4   3
2   5
1   3
3   9
3   3
"""

if len(sys.argv) == 2:
    with open(sys.argv[1]) as f:
        INPUT = f.read().strip()
else:
    INPUT = test_input.strip()

# read lines and split into two lists
left = [int(l.split()[0]) for l in INPUT.split("\n")]
right = [int(l.split()[1]) for l in INPUT.split("\n")]

# Part 1
left.sort()
right.sort()
distance = sum([abs(l - r) for l, r in zip(left, right)])
print(f"Part 1:\n{distance = }")

# Part 2
right_occurrences = [right.count(l) for l in left]
similarity = sum([l * r for l, r in zip(left, right_occurrences)])
print(f"Part 2:\n{similarity = }")
