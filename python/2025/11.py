import sys

from collections import defaultdict
from functools import cache, reduce
from itertools import pairwise
from operator import mul


def parse(data):
    dag = defaultdict(set)
    for line in data.splitlines():
        src, *dsts = line.split()
        dag[src[:-1]].update(dsts)
    return dag


def solve(dag):
    @cache
    def ways(src, dst):
        if src == dst:
            return 1
        return sum(ways(d, dst) for d in dag[src])
    
    part1 = ways("you", "out")

    part2 = sum(reduce(mul, [ways(src, dst) for src, dst in pairwise(waypoints.split())])
                for waypoints in ["svr fft dac out", "svr dac fft out"])

    return part1, part2

if __name__ == "__main__":
    sdata = open(sys.argv[-1], "r").read()
    data = parse(sdata)
    p1, p2 = solve(data)
    print(f"{p1}")
    print(f"{p2}")