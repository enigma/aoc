import sys
from collections import namedtuple, Counter
from itertools import chain
from copy import deepcopy

def parse(text):
    result = []
    for pair in text.strip().split('\n\n'):
        result.append(list(map(eval, pair.split('\n'))))
    return result

def compare(p1, p2):
    tp1 = type(p1)
    tp2 = type(p2)
    if tp1 is list and tp1 != tp2:
        return compare(p1, [p2])
    elif tp1 is not list and tp1 != tp2:
        return compare([p1], p2)
    elif tp1 is not list and tp2 is not list:
        return (p1 > p2) - (p1 < p2)
    stack1 = p1[::-1]
    stack2 = p2[::-1]
    while stack1 and stack2:
        head = compare(stack1.pop(), stack2.pop())
        if head != 0:
            return head
    if not stack1 and not stack2: return 0
    if not stack1: return -1        
    if not stack2: return 1
    assert False, (stack1, stack2)

def part1(pairs):
    counter = 0
    for n, (p1, p2) in enumerate(pairs, 1):
        if compare(p1, p2) != 1:
            counter += n
    return counter

def part2(pairs):
    div = [[[2]], [[6]]]
    all_pairs = [p for pair in pairs for p in pair] + div
    modified = True
    while modified:
        modified = False
        for i in range(len(all_pairs) - 1):
            if compare(all_pairs[i], all_pairs[i+1]) != -1:
                all_pairs[i], all_pairs[i+1] = all_pairs[i+1], all_pairs[i]
                modified = True
    prod = 1
    for n, i in enumerate(all_pairs, 1):
        if i in div:
            prod *= n
    return prod

from functools import cmp_to_key
def part2(pairs):
    div = [[[2]], [[6]]]
    all_pairs = [p for pair in pairs for p in pair] + div
    all_pairs.sort(key=cmp_to_key(compare))
    prod = 1
    for n, i in enumerate(all_pairs, 1):
        if i in div:
            prod *= n
    return prod

text = open(sys.argv[-1], 'r').read().strip()
input_lines = parse(text)
print(part1(input_lines))
print(part2(input_lines))
