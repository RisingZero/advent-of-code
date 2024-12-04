#!/usr/bin/env python3

import sys

test_input = """
    MMMSXXMASM
    MSAMXMSMSA
    AMXSXMAAMM
    MSAMASMSMX
    XMASAMXAMM
    XXAMMXXAMA
    SMSMSASXSS
    SAXAMASAAA
    MAMMMXMMMM
    MXMXAXMASX
"""

if len(sys.argv) == 2:
    with open(sys.argv[1]) as f:
        INPUT = f.read().strip()
else:
    INPUT = test_input.strip()

haystack = [[c for c in l.strip()] for l in INPUT.split("\n")]
N = len(haystack)
M = len(haystack[0])
MATCH = "XMAS"


# Part 1
def check(table, r, c, dir):
    for i in range(len(MATCH)):
        if (
            r + dir[0] * i < 0
            or r + dir[0] * i >= N
            or c + dir[1] * i < 0
            or c + dir[1] * i >= M
        ):
            return False
        if table[r + dir[0] * i][c + dir[1] * i] != MATCH[i]:
            return False
    return table[r + dir[0] * i][c + dir[1] * i] == MATCH[i]


count = 0
for dir in ((1, 1), (1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0), (-1, 1), (0, 1)):
    for r in range(N):
        for c in range(M):
            if haystack[r][c] == "X":
                count += check(haystack, r, c, dir)
print(f"Part 1:\n{count = }")

# Part 2
count = 0
for i in range(1, N - 1):
    for j in range(1, M - 1):
        if haystack[i][j] == "A":
            if (
                haystack[i - 1][j - 1] == "M"
                and haystack[i + 1][j + 1] == "S"
                or haystack[i - 1][j - 1] == "S"
                and haystack[i + 1][j + 1] == "M"
            ) and (
                haystack[i - 1][j + 1] == "M"
                and haystack[i + 1][j - 1] == "S"
                or haystack[i - 1][j + 1] == "S"
                and haystack[i + 1][j - 1] == "M"
            ):
                count += 1
print(f"Part 2:\n{count = }")