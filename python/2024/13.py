import sys


def parse(full_input):
    res = []
    for puzzle in full_input.split("\n\n"):
        ast, bst, pst = puzzle.splitlines()
        ap, bp = [
            tuple(int(s[2:].replace(",", "")) for s in ss[-2:])
            for ss in [ast.split(), bst.split()]
        ]
        pp = tuple(int(s.split("=")[-1].replace(",", "")) for s in pst.split()[-2:])
        res.append((ap, bp, pp))
    return res


def cost(xa, ya, xb, yb, xp, yp):
    """
    Solve the equation for integer a and b:
    a * xa + b * xb = xp
    a * ya + b * yb = yp
    """
    b_num = xa * yp - ya * xp
    b_den = xa * yb - ya * xb
    if b_num % b_den != 0 or (b := b_num // b_den) < 0:
        return 0
    a_num = xp - b * xb
    if a_num % xa != 0 or (a := a_num // xa) < 0:
        return 0
    return a * 3 + b


def solve(data, extra=0):
    return sum(
        cost(xa, ya, xb, yb, xp + extra, yp + extra)
        for (xa, ya), (xb, yb), (xp, yp) in data
    )


def part1(data):
    return solve(data)


def part2(data):
    return solve(data, 10**13)


if __name__ == "__main__":
    full_input = open(sys.argv[-1], "r").read()
    data = parse(full_input)
    print(p1 := part1(data))
    print(p2 := part2(data))
