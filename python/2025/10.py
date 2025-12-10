#!/usr/bin/env -S uv run --script
#
# /// script
# requires-python = ">=3.14"
# dependencies = [
#     "z3-solver",
# ]
# ///

import sys
from functools import reduce
from operator import or_
from heapq import heappush, heappop
from z3 import Sum, Int, Optimize, sat


def parse(data):
    machines = []
    for line in data.splitlines():
        diagram, *schematics, requirements = line.split(" ")
        diagram = int(diagram[1:-1][::-1].replace(".", "0").replace("#", "1"), 2)
        schematics = [
            tuple(1 << int(i) for i in s[1:-1].split(",")) for s in schematics
        ]
        requirements = [int(i) for i in requirements[1:-1].split(",")]
        machines.append((diagram, schematics, requirements))
    return machines


def presses2diagram(diagram, schematics):
    ss = [reduce(or_, schematic) for schematic in schematics]
    seen = {}
    queue = [(0, 0, 0)]
    while queue:
        pressed, mask, cur = heappop(queue)
        if mask == diagram:
            return pressed
        if seen.get(k := (mask, cur), pressed + 1) <= pressed:
            continue
        seen[k] = pressed
        if cur == len(schematics):
            continue
        heappush(queue, (pressed, mask, cur + 1))
        heappush(queue, (pressed + 1, mask ^ ss[cur], cur + 1))
    raise ValueError("No solution found?")


def part1(machines):
    return sum(presses2diagram(d, s) for d, s, _ in machines)


def presses2joltage(schematics, requirements):
    opt = Optimize()

    presses = [Int(f"press_{i}") for i in range(len(schematics))]
    for press in presses:
        opt.add(press >= 0)

    for j in range(len(requirements)):
        contrib = [presses[i] for i, s in enumerate(schematics) if (1 << j) in s]
        opt.add(Sum(contrib) == requirements[j])

    opt.minimize(Sum(presses))

    if opt.check() == sat:
        model = opt.model()
        return sum(model[i].as_long() for i in presses)
    raise ValueError("No solution found?")


def part2(machines):
    return sum(presses2joltage(s, r) for _, s, r in machines)


if __name__ == "__main__":
    sdata = open(sys.argv[-1], "r").read()
    data = parse(sdata)
    print(f"{part1(data)}")
    print(f"{part2(data)}")
