import sys
from math import log10, floor


def parse(s):
    return [
        (int((parts := line.split())[0][:-1]), list(map(int, parts[1:])))
        for line in s.splitlines()
    ]


def _can_do(exp, terms, ti=None, part2=False):
    if ti is None:
        return _can_do(exp, terms, len(terms) - 1, part2)
    if ti < 0:
        return exp == 0
    term = terms[ti]
    ti -= 1
    return (
        (exp % term == 0 and _can_do(exp // term, terms, ti, part2))
        or (exp >= term and _can_do(exp - term, terms, ti, part2))
        or (
            part2
            and exp % (f := 10 ** (1 + floor(log10(term)))) == term
            and _can_do((exp - term) // f, terms, ti, part2)
        )
    )


def solve(data):
    part1 = part2 = 0
    for goal, terms in data:
        if _can_do(goal, terms):
            part1 += goal
        elif _can_do(goal, terms, part2=True):
            part2 += goal
    return part1, part1 + part2


if __name__ == "__main__":
    full_input = open(sys.argv[-1], "r").read()
    data = parse(full_input)
    p1, p2 = solve(data)
    print(f"part1 = {p1}\npart2 = {p2}")
