import sys
from collections import Counter
import re


def solve(data, part2=False):
    do = True
    res = 0
    for m in re.finditer(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)", data, re.MULTILINE):
        g0 = m.group(0)
        match g0:
            case "do()":
                do = True
            case "don't()":
                do = False
            case _:
                if not part2 or do:
                    a, b = m.groups()
                    res += int(a) * int(b)
    return res


def part1(data):
    return solve(data)


def part2(data):
    return solve(data, True)


if __name__ == "__main__":
    data = open(sys.argv[-1], "r").read()
    print(f"part1 {part1(data)}\tpart2 {part2(data)}")
