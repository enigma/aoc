import sys
from math import lcm


def count(start, instr, map, p2=False):
    i, cur = 0, start
    while cur != 'ZZZ' if not p2 else cur[-1] != 'Z':
        cur = map[cur]['R' == instr[i % len(instr)]]
        i += 1
    return i


def solve(data):
    instr, smap = data.split('\n\n')
    map = {i[:3]: (i[7:10], i[12:15]) for i in smap.split('\n')}
    p1 = count('AAA', instr, map)
    cur = [i for i in map.keys() if i[-1] == 'A']
    p2 = lcm(*[count(c, instr, map, True) for c in cur])

    return p1, p2


data = open(sys.argv[-1]).read()
part1, part2 = solve(data.strip())
print(part1)
print(part2)

assert part1 == 18727
assert part2 == 18024643846273
