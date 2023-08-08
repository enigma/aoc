import sys
from collections import Counter

def dist(a, b):
    return abs(a[0] - b[0]) + abs(a[1] - b[1])

def parse(text):
    res = dict()
    for row in text.strip().split('\n'):
        p = row.split()
        sensor = int(p[2][2:-1]), int(p[3][2:-1])
        beacon = int(p[-2][2:-1]), int(p[-1][2:])
        res[sensor] = dist(sensor, beacon)
    return res

def part1(by_radius, max_side=4_000_000):
    intervals = []
    for (x, y), radius in by_radius.items():
        arm = radius - abs(max_side // 2 - y) 
        if arm < 0:
            continue
        intervals.append((x - arm, x + arm))
    intervals.sort()
    return max(right for _, right in intervals) - intervals[0][0]

def part2(by_radius, max_side=4_000_000):
    positive, negative = Counter(), Counter()
    for ((x, y), r) in by_radius.items():
        positive[y - x + r + 1] += 1
        positive[y - x - r - 1] += 1
        negative[y + x + r + 1] += 1
        negative[y + x - r - 1] += 1
    positive, negative = [set(k for k, v in d.items() if v > 1) for d in [positive, negative]]
    for a in positive:
        for b in negative:
            pos = (b - a) // 2, (a + b) // 2
            if all(0 < c < max_side for c in pos):
                if all(dist(pos, scanner) > radius for scanner, radius in by_radius.items()):
                    return 4_000_000 * pos[0] + pos[1]

text = open(sys.argv[-1], 'r').read()
parsed = parse(text)
print(part1(parsed))
print(part2(parsed))
