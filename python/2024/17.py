import sys
from enum import Enum
from z3 import Optimize, BitVec, sat


class Op(Enum):
    ADV = 0
    BXL = 1
    BST = 2
    JNZ = 3
    BXC = 4
    OUT = 5
    BDV = 6
    CDV = 7


def parse(full_input):
    instructions = []
    stats, ops = full_input.strip().split("\n\n")
    a, b, c = (int(s[12:]) for s in stats.splitlines())
    instructions = [Op(i) for i in (int(i) for i in ops[8:].split(","))]
    return (a, b, c, 0), instructions


def combov(state, n):
    if n < 4:
        return n
    return state[n - 4]


def run(state, instrs: list[Op], res: list[int]):
    a, b, c, pc = state
    match instrs[pc]:
        case Op.ADV:
            # ADV: Divide A by 2^n (n is combo operand), store in A
            combo = combov(state, instrs[pc + 1].value)
            return (a >> combo, b, c, pc + 2)
        case Op.BXL:
            # BXL: XOR B with literal n, store in B
            return (a, b ^ instrs[pc + 1].value, c, pc + 2)
        case Op.BST:
            # BST: Store n mod 8 in B
            combo = combov(state, instrs[pc + 1].value)
            return (a, combo % 8, c, pc + 2)
        case Op.JNZ:
            # JNZ: If A != 0, jump to n
            if a != 0:
                return (a, b, c, instrs[pc + 1].value)
            return (a, b, c, pc + 2)
        case Op.BXC:
            # BXC: XOR B with C, store in B (ignore operand)
            return (a, b ^ c, c, pc + 2)
        case Op.OUT:
            # OUT: Output n mod 8
            combo = combov(state, instrs[pc + 1].value)
            res.append(combo % 8)
            return (a, b, c, pc + 2)
        case Op.BDV:
            # BDV: Divide A by 2^n, store in B
            combo = combov(state, instrs[pc + 1].value)
            return (a, a >> combo, c, pc + 2)
        case Op.CDV:
            # CDV: Divide A by 2^n, store in C
            combo = combov(state, instrs[pc + 1].value)
            return (a, b, a >> combo, pc + 2)
        case xxx:
            raise ValueError(f"Unknown instruction: {xxx}")


def part1(data):
    state, instructions = data
    res = []
    while state[-1] < len(instructions):
        state = run(state, instructions, res)
    return ",".join(str(i) for i in res)


def _pretty_print(data):
    _, instructions = data
    for n, inst in enumerate(instructions):
        combov = None
        if n < len(instructions) - 1:
            vv = literal = instructions[n + 1].value
            combov = vv if vv < 4 else "ABCX"[vv - 4]
        if n % 2 == 1:
            continue
        match inst:
            case Op.ADV:
                print(f"{n:2} {inst.name}: A = A >> {combov}")
            case Op.BXL:
                print(f"{n:2} {inst.name}: B = B ^ {literal}")
            case Op.BST:
                print(f"{n:2} {inst.name}: B = {combov} % 8")
            case Op.JNZ:
                print(f"{n:2} {inst.name}: if A != 0, jump to {literal}")
            case Op.BXC:
                print(f"{n:2} {inst.name}: B = B ^ C")
            case Op.OUT:
                print(f"{n:2} {inst.name}:             output {combov} % 8")
            case Op.BDV:
                print(f"{n:2} {inst.name}: A = A >> {combov}")
            case Op.CDV:
                print(f"{n:2} {inst.name}: C = A >> {combov}")
            case _:
                print(f"{n:2} {inst.name} ???")


def part2(data):
    _, instructions = data
    opt = Optimize()
    A = BitVec("A", 64)
    opt.add(A >= 0)
    curr_A = A
    for expected in [i.value for i in instructions]:
        B = curr_A & 7  # A % 8
        B = B ^ 1
        C = curr_A >> B
        B = B ^ C
        B = B ^ 6
        B = B & 7  # B % 8

        opt.add(B == expected)

        curr_A = curr_A >> 3

    opt.minimize(A)

    if opt.check() == sat:
        m = opt.model()
        return m[A].as_long()
    return None


if __name__ == "__main__":
    full_input = open(sys.argv[-1], "r").read()

    data = parse(full_input)
    print(p1 := part1(data))
    #_pretty_print(data)
    print(p2 := part2(data))
