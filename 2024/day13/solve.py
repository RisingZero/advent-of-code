import re
from z3 import *
import sys
from typing import Tuple, Optional

test_input = """
    Button A: X+94, Y+34
    Button B: X+22, Y+67
    Prize: X=8400, Y=5400

    Button A: X+26, Y+66
    Button B: X+67, Y+21
    Prize: X=12748, Y=12176

    Button A: X+17, Y+86
    Button B: X+84, Y+37
    Prize: X=7870, Y=6450

    Button A: X+69, Y+23
    Button B: X+27, Y+71
    Prize: X=18641, Y=10279
"""

if len(sys.argv) == 2:
    with open(sys.argv[1]) as f:
        INPUT = f.read().strip()
else:
    INPUT = test_input.strip()

REGEX = re.compile(r"\s*Button [A|B]: X\+([0-9]+), Y\+([0-9]+)\n\s*Button [A|B]: X\+([0-9]+), Y\+([0-9]+)\n\s*Prize: X=([0-9]+), Y=([0-9]+)")

class ClawProblem:
    def __init__(self, a: Tuple[int,int], b: Tuple[int,int], prize: Tuple[int,int]):
        self.a = a
        self.b = b
        self.prize = prize
    
    def solve(self, bound: Optional[int]) -> Tuple[int,int]:
        s = Solver()
        A, B = Ints("A B")
        s.add(self.a[0] * A + self.b[0] * B == self.prize[0])
        s.add(self.a[1] * A + self.b[1] * B == self.prize[1])
        if bound is not None:
            s.add(A >= 0, A <= bound)
            s.add(B >= 0, B <= bound)
        if s.check() == sat:
            m = s.model()
            return m[A].as_long(), m[B].as_long()
        else:
            return None

    def solution_cost(self, bound=None) -> int:
        if s := self.solve(bound):
            return s[0] * 3 + s[1]
        else:
            return 0


    @staticmethod
    def from_tuple(t: Tuple[int,int,int,int,int,int]) -> "ClawProblem":
        return ClawProblem(
            (int(t[0]), int(t[1])),
            (int(t[2]), int(t[3])),
            (int(t[4]), int(t[5]))
        )
    
    @staticmethod
    def from_tuple_adjusted(t: Tuple[int,int,int,int,int,int]) -> "ClawProblem":
        return ClawProblem(
            (int(t[0]), int(t[1])),
            (int(t[2]), int(t[3])),
            (int(t[4]) + 10000000000000, int(t[5]) + 10000000000000)
        )


# Part 1
m = REGEX.findall(INPUT)
problems = map(ClawProblem.from_tuple, m)
result = sum(map(lambda x: x.solution_cost(100), problems))
print(f"Part 1:\n{result = }")

# Part 2
problems = map(ClawProblem.from_tuple_adjusted, m)
result = sum(map(ClawProblem.solution_cost, problems))
print(f"Part 2:\n{result = }")