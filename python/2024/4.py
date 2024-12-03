import re
import sys


def part1(data):
    w = data.index("\n")
    patterns = [
        f"(?={X}.{{{i}}}{M}.{{{i}}}{A}.{{{i}}}{S})"
        for i in [0, w, w + 1, w - 1]
        for X, M, A, S in ["XMAS", "SAMX"]
    ]
    return sum(len(re.findall(pattern, data, re.DOTALL)) for pattern in patterns)


def part2(data):
    w = data.index("\n") - 1
    patterns = [
        f"(?={tl}.{tr}.{{{w}}}A.{{{w}}}{bl}.{br})"
        for tl, tr, bl, br in ["MMSS", "MSMS", "SSMM", "SMSM"]
    ]
    return sum(len(re.findall(pattern, data, re.DOTALL)) for pattern in patterns)


if __name__ == "__main__":
    full_input = open(sys.argv[-1], "r").read()
    print(f"part1 = {part1(full_input)}\npart2 = {part2(full_input)}")
