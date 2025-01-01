from dataclasses import dataclass
from enum import Enum
import sys
from itertools import combinations


class Op(Enum):
    AND = 1
    OR = 2
    XOR = 3


def parse(full_input):
    top, bottom = full_input.split("\n\n")
    vals = {line[:3]: [0, 1][line.endswith("1")] for line in top.splitlines()}
    ops = {}
    for line in bottom.splitlines():
        l, sop, r, _, res = line.split()
        ops[res] = (Op[sop], l, r)
    return vals, ops


def evalk(values, ops, key, deps=frozenset()):
    if key in values:
        return values[key]
    if key in deps:
        raise ValueError(f"cycle detected: {key} in {deps}")
    match ops[key]:
        case (Op.AND, l, r):
            return evalk(values, ops, l, deps | {key}) & evalk(values, ops, r, deps | {key})
        case (Op.OR, l, r):
            return evalk(values, ops, l, deps | {key}) | evalk(values, ops, r, deps | {key})
        case (Op.XOR, l, r):
            return evalk(values, ops, l, deps | {key}) ^ evalk(values, ops, r, deps | {key})


def evalv(values, ops, v):
    vs = sorted(k for k in ops.keys() if k.startswith(v))
    return sum(evalk(values, ops, z) * (2**i) for i, z in enumerate(vs))


def part1(values, ops):
    return evalv(values, ops, "z")


@dataclass
class Invokation:
    opv: Op
    a: str
    b: str

    def __init__(self, opv, a, b):
        if a > b:
            a, b = b, a
        self.opv = opv
        self.a = a
        self.b = b

    def __eq__(self, other):
        return self.opv == other.opv and self.a == other.a and self.b == other.b

    def __hash__(self):
        return hash((self.opv, self.a, self.b))


def part2(_values, ops):
    ops2wire = {Invokation(opv, a, b): k for k, (opv, a, b) in ops.items()}

    def highwatermark(ops2wire: dict[Invokation, str]) -> tuple[int, set[str]]:
        carries = {}
        correct = set()
        itermediates = set()
        op = lambda opv, a, b: ops2wire.get(Invokation(opv, a, b), None)
        for i in range(45):
            """
            https://en.wikipedia.org/wiki/Adder_(electronics)
            https://en.wikipedia.org/wiki/Adder_(electronics)#/media/File:Fulladder.gif

            xor1AA = xAA xor yAA
            s = xor2AA = xor1AA xor carry_in_AA

            and1AA = xor1AA and carry_in_AA
            and2AA = xAA and yAA
            c_out = or1AA = and1AA or and2AA
            """
            idx = f"{i:02}"
            xor1 = op(Op.XOR, f"x{idx}", f"y{idx}")
            and2 = op(Op.AND, f"x{idx}", f"y{idx}")
            if i == 0:
                carries[i] = and2
                continue
            if (
                not xor1
                or not carries[i - 1]
                or op(Op.XOR, xor1, carries[i - 1]) != f"z{idx}"
            ):
                return i - 1, correct

            correct |= itermediates | {xor1, carries[i - 1]}

            and1 = op(Op.AND, xor1, carries[i - 1])
            carries[i] = op(Op.OR, and1, and2)
            itermediates = {and1, and2}
        return 45, correct

    swaps = set()
    watermark, used = highwatermark(ops2wire)
    for _ in range(4):
        for i, j in combinations(ops2wire.keys(), 2):
            if (
                (opi := ops2wire[i]) == "z00"
                or (opj := ops2wire[j]) == "z00"
                or opi in used
                or opj in used
            ):
                continue
            ops2wire |= {i: opj, j: opi}
            new_watermark, new_used = highwatermark(ops2wire)
            if new_watermark > watermark:
                swaps.add((opi, opj))
                watermark, used = new_watermark, new_used
                break
            ops2wire |= {i: opi, j: opj}
    return ",".join(sorted(k for ks in swaps for k in ks))


if __name__ == "__main__":
    full_input = open(sys.argv[-1], "r").read()
    data = parse(full_input)
    print(p1 := part1(*data))
    print(p2 := part2(*data))
