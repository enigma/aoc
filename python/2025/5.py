import sys


def parse(data):
    sr, si = data.strip().split("\n\n")
    ranges = [[int(i) for i in line.split("-")] for line in sr.splitlines()]
    ingredients = [int(i) for i in si.splitlines()]
    return ranges, ingredients


def part1(ranges, ingredients):
    return sum(1 for i in ingredients if any(r[0] <= i <= r[1] for r in ranges))


def part2(ranges, _ingredients):
    merged = []
    for a, b in sorted(ranges):
        if merged and a <= merged[-1][1]:
            merged[-1][1] = max(merged[-1][1], b)
        else:
            merged.append([a, b])
    return sum(1 + b - a for a, b in merged)


if __name__ == "__main__":
    sdata = open(sys.argv[-1], "r").read()
    data = parse(sdata)
    print(f"{part1(*data)}")
    print(f"{part2(*data)}")
