import sys


def ex(ns):
    if all(i == 0 for i in ns):
        while True:
            yield 0
    last = ns[-1]
    for d in ex([b-a for a, b in zip(ns, ns[1:])]):
        last += d
        yield last


def solve(data):
    numbers = []
    for line in data.splitlines():
        numbers.append(list(map(int, line.split())))
    p1 = sum(next(ex(ns)) for ns in numbers)
    p2 = sum(next(ex(ns[::-1])) for ns in numbers)
    return p1, p2


data = open(sys.argv[-1]).read()
part1, part2 = solve(data.strip())
print(part1)
print(part2)

assert part1 == 1641934234
assert part2 == 975
