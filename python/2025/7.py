import sys

from collections import Counter


def parse(data):
    rows = data.splitlines()
    start = rows[0].index("S")
    splits = [[i for i, c in enumerate(row) if c == "^"] for row in rows[1:]]
    return start, splits


def part1(data):
    result = 0
    start, splitters = data
    beams = set([start])
    for layer in map(set, splitters):
        hits = layer & beams
        beams = (beams - hits) | {s+d for s in hits for d in [-1, 1]}
        result += len(hits)
    return result


def part2(data):
    start, splitters = data
    ways = Counter({start: 1})
    for layer in map(set, splitters):
        splits = {h: ways[h] for h in layer & ways.keys()}
        ways -= splits
        for pos, count in splits.items():
            ways[pos+1] += count
            ways[pos-1] += count
    return sum(ways.values())


if __name__ == "__main__":
    sdata = open(sys.argv[-1], "r").read()
    data = parse(sdata)
    print(f"{part1(data)}")
    print(f"{part2(data)}")
