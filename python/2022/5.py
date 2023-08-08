import sys
from copy import deepcopy

def parse(text):
    stacks = []
    moves = []
    mode = 0
    for line in text.split('\n'):
        if mode == 0:
            if not line:
                mode = 1             
                continue
            els = line[1::4]
            if not stacks:
                for _ in els: stacks.append([])
            for el, stack in zip(els, stacks):
                if el != ' ':
                    stack.append(el)
        if mode == 1:
            s = line.replace('move ', '').replace('from ', '').replace('to ', '')
            c, f, t = s.split()
            moves.append((int(c), f, t))
    res = {}
    for stack in stacks:
        key = stack.pop()
        res[key] = []
        while stack: res[key].append(stack.pop())
    return res, moves

def part1(lines):
    stacks, moves = deepcopy(lines)
    for cnt, frm, to in moves:
        for _ in range(cnt):
            stacks[to].append(stacks[frm].pop())
    return ''.join(stacks[k][-1] for k in sorted(stacks.keys(), key=int))

def part2(lines):
    stacks, moves = deepcopy(lines)
    for cnt, frm, to in moves:
        stacks[to].extend(stacks[frm][-cnt:])
        del stacks[frm][-cnt:]
    return ''.join(stacks[k][-1] for k in sorted(stacks.keys(), key=int))

text = open(sys.argv[-1], 'r').read().strip()
input_lines = parse(text)
print(part1(input_lines))
print(part2(input_lines))
