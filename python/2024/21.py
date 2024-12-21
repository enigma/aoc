import sys
from functools import cache
from itertools import pairwise, chain


def make_map(pad):
    res = {}
    for n, i in enumerate(pad):
        if i == " ":
            continue
        cur = res[i] = {}
        for d, j in zip("<>^v", [n - 1, n + 1, n - 4, n + 4]):
            if 0 <= j < len(pad) and pad[j] != " ":
                cur[d] = pad[j]
    return res


NUMPAD_MAP = make_map("789 456 123  0A")
DIRPAD_MAP = make_map(" ^A <v>")


def parse(full_input):
    return full_input.strip().splitlines()


@cache
def minimal_paths(src, dst, is_dirpad=False):
    neighs = DIRPAD_MAP if is_dirpad else NUMPAD_MAP
    res = []
    fringe = [(src, tuple())]
    while fringe:
        layer, fringe = fringe, []
        for cur, walked in layer:
            if cur == dst:
                res.append(walked)
                continue
            for nxt, d in neighs[cur].items():
                fringe.append((d, walked + (nxt,)))
        if res:
            break
    return res


@cache
def solve(pattern, robots, is_dirpad=0):
    if not robots or not pattern:
        return len(pattern)
    res = 0
    for src, dst in chain([("A", pattern[0])], pairwise(pattern)):
        min_cost = None
        for path in minimal_paths(src, dst, is_dirpad):
            if not path:
                min_cost = 1
                break
            cost = solve("".join(chain(path, "A")), robots - 1, is_dirpad=1)
            if min_cost is None or cost < min_cost:
                min_cost = cost
        res += min_cost
    return res


def part1(data, n=2):
    return sum(solve(pattern, n + 1) * int(pattern[:-1]) for pattern in data)


def part2(data):
    return part1(data, 25)


if __name__ == "__main__":
    full_input = open(sys.argv[-1], "r").read()
    data = parse(full_input)
    print(p1 := part1(data))
    print(p2 := part2(data))
