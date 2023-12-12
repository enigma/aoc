import sys
from functools import cache
import re


def parse(data):
    for line in data.splitlines():
        o, b = line.split()
        yield o, tuple(int(i) for i in b.split(','))


@cache
def arrangements(record: str, conf: list[int]):
    if not conf:
        return 1 if all(i != '#' for i in record) else 0
    b = conf[0]
    result = 0
    if re.match('[?#]{%d}(\\.|\\?|$)' % b, record):
        result += arrangements(record[b+1:], conf[1:])
    if record and record[0] != '#':
        result += arrangements(record[1:], conf)
    return result


def solve(records):
    p1 = p2 = 0
    for a, b in records:
        p1 += arrangements(a, b)
        p2 += arrangements('?'.join(a for _ in range(5)), b * 5)
    return p1, p2


data = open(sys.argv[-1]).read()
parsed = list(parse(data.strip()))
p1, p2 = solve(parsed)
print(p1)
print(p2)

assert p1 == 6852, p1
assert p2 == 8475948826693, p2
