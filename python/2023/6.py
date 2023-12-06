import sys
from math import ceil, floor, prod


def parse(data):
    lines = data.splitlines()
    time = [int(i) for i in lines[0].split()[1:]]
    dist = [int(i) for i in lines[1].split()[1:]]
    return list(zip(time, dist))


def wins(time, dist):
    sq = (time**2 - 4*dist)**.5
    t1 = ceil((time - sq) / 2)
    t2 = floor((time + sq) / 2)
    return t2 - t1 + (1 if sq != int(sq) else -1)


def solve(lines):
    times = lines[0].split()[1:]
    dists = lines[1].split()[1:]
    return (
            prod(wins(int(t), int(d)) for (t, d) in zip(times, dists)),
            wins(*(int(''.join(i)) for i in [times, dists]))
    )


data = open(sys.argv[-1]).read()
part1, part2 = solve(data.strip().splitlines())
print(part1)
print(part2)

assert part1 == 1084752
assert part2 == 28228952
