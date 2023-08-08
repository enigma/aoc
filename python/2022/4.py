import sys

def parse(text):
    lines = []
    for line in text.split('\n'):
        r1, r2 = line.split(',')
        r1, r2 = [tuple(map(int, x.split('-'))) for x in [r1, r2]]
        lines.append(tuple(sorted([r1, r2])))
    return lines

def part1(lines):
    res = 0
    for r1, r2 in lines:
        inner = (min(r1[0], r2[0]), max(r1[1], r2[1]))
        if inner in [r1, r2]:
            res += 1
    return res

def part2(lines):
    res = 0
    for r1, r2 in lines:
        a, b = r1
        if a <= r2[0] <= b:
            res += 1
    return res

text = open(sys.argv[-1], 'r').read().strip()
input_lines = parse(text)
print(part1(input_lines))
print(part2(input_lines))
