import sys


def parse(data):
    return [
        (-1 if line.startswith("L") else 1) * int(line[1:])
        for line in data.splitlines()
    ]


def part1(data):
    cur, zeros = 50, 0
    for d in data:
        cur = (cur + d) % 100
        if cur == 0:
            zeros += 1
    return zeros


def part2(data):
    cur, zeros = 50, 0
    for d in data:
        if d >= 0:
            crossed = (cur + d) // 100
        elif 0 < cur <= -d:
            crossed = (-d - cur) // 100 + 1
        else:
            crossed = (-d) // 100
        cur = (cur + d) % 100
        zeros += crossed
    return zeros


if __name__ == "__main__":
    data = parse(open(sys.argv[-1], "r").read())
    p1 = part1(data)
    p2 = part2(data)
    print(f"{p1=}")
    print(f"{p2=}")
    assert (p1, p2) == (962, 5782), (p1, p2)
