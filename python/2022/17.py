import sys
from itertools import cycle

def parse(text):
    table = {'>': 1, '<': -1}
    return [table[i] for i in text.strip()]

LEFT = sum((0b1000000 << 7 * i) for i in range(4))
RIGHT = LEFT >> 6
def left(n):
    if LEFT & n:
        return None
    return n << 1

def right(n):
    if RIGHT & n:
        return None
    return n >> 1

PIECES = [[0b0011110,0,0,0],
          [0b0001000,
           0b0011100,
           0b0001000, 0],
          [0b0011100,
           0b0000100,
           0b0000100, 0],
          [0b0010000]*4,
          [0b0011000,
           0b0011000, 0, 0]]

assemble = lambda p: sum(p[i] << 7*(3-i) for i in range(4))
PIECES = [assemble(x) for x in PIECES]

def solve(instrs, goal):
    stack = cycle(enumerate(instrs))
    placed = [0] * 5_000
    highest = 0
    cache = {}
    for n, (piece_id, piece) in enumerate(cycle(enumerate(PIECES))):
        y = highest + 3
        next_piece = piece
        board = 0
        while True:
            instr_id, idx = next(stack)
            plausible = [0, right, left][idx](next_piece)
            if plausible and not (plausible & board):
                next_piece = plausible
            board = (board >> 7) + (placed[y-1] << (7*3))
            if y and not (next_piece & board):
                y -= 1
            else:
                mask = (1 << 7) - 1
                placed[y+0] |= (next_piece >> (7*3)) & mask
                placed[y+1] |= (next_piece >> (7*2)) & mask
                placed[y+2] |= (next_piece >> (7*1)) & mask
                placed[y+3] |= (next_piece) & mask
                highest = max(max([y+i+1 for i in range(4) if placed[y+i]]), highest)
                break
        key = instr_id, piece_id
        if prev := cache.get(key):
            prev_n, prev_highest = prev
            height_cycle = highest - prev_highest
            piece_cycle = n - prev_n
            skipped_cycles, remainder = divmod(goal - n - 1, piece_cycle)
            if not remainder:
                return skipped_cycles * height_cycle + highest
        else:
            cache[key] = n, highest

        if n + 1 == goal:
            return highest


lines, p1e, p2e = '''>>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>''', 3068, 1514285714288
instrs = parse(lines)
instrs = parse(lines)
p1=solve(instrs, 2022)
assert p1 == p1e, (p1, p1e)
p2=solve(instrs, 1000000000000)
assert p2 == p2e, p2

lines, p1e, p2e = open(sys.argv[-1]).read().strip(), 3193, 1577650429835
instrs = parse(lines)
print(p1:=solve(instrs, 2022))
assert p1 == p1e, p1
print(p2:=solve(instrs, 1000000000000))
assert p2 == p2e, p2
