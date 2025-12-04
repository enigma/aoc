import sys


def npos(y, x):
    DD = [-1, 0, 1]
    return [(y + dy, x + dx) for dy in DD for dx in DD if dy or dx]


def parse(data):
    return {
        (y, x)
        for y, row in enumerate(data.splitlines())
        for x, e in enumerate(row)
        if e == "@"
    }


def removable(paper):
    return {k for k in paper if sum(1 for n in npos(k[0], k[1]) if n in paper) < 4}


def part1(paper):
    return len(removable(paper))


def part2(paper):
    return len(gone := removable(paper)) + (part2(paper - gone) if gone else 0)


if __name__ == "__main__":
    sdata = open(sys.argv[-1], "r").read()
    data = parse(sdata)
    p1 = part1(data)
    print(f"{p1}")

    p2 = part2(data)
    print(f"{p2}")
