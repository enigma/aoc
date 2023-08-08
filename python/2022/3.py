import sys

def priority(c):
    if 'a' <= c <= 'z':
        return ord(c) - ord('a') + 1
    elif 'A' <= c <= 'Z':
        return ord(c) - ord('A') + 27
    return 0

def badge_priority(badge):
    l = len(badge)
    a, b = badge[:l//2], badge[l//2:]
    c = set(a) & set(b)
    return priority(c.pop())

def group_badge_priority(lines):
    a, b, c = map(set, lines)
    common = a & b & c
    return priority(common.pop())

def part1(input_lines):
    return sum(map(badge_priority, input_lines))

def part2(input_lines):
    result = 0
    for i in range(0, len(input_lines), 3):
        result += group_badge_priority(input_lines[i:i+3])
    return result

input_lines = open(sys.argv[-1], 'r').read().strip().split('\n')
print(part1(input_lines))
print(part2(input_lines))

