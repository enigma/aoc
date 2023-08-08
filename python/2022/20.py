import sys
from collections import deque

def parse(text):
    return list(map(int, text.split()))

def solve(data, factor = 1, repetitions = 1):
    order = tuple((i, d*factor) for i, d in enumerate(data))
    mixing = deque(order)
    for _ in range(repetitions):
        for (cur, rot) in order:
            idx = mixing.index(order[cur])
            del mixing[idx]
            mixing.rotate(-rot-idx)
            mixing.appendleft((cur, rot))
    zero = order[data.index(0)]
    mixing.rotate(-mixing.index(zero))
    return sum(mixing[i % len(data)][1] for i in [1000, 2000, 3000])

def part1(data):
    return solve(data)

def part2(data):
    return solve(data, factor=811589153, repetitions=10)

text = open(sys.argv[-1], 'r').read()
parsed = parse(text)
print(part1(parsed))
print(part2(parsed))
