import sys

def parse(text):
    res = []
    for line in text.strip().split('\n'):
        res.append(list(map(int,line)))
    return res

def visibility(H, W):
    return { (1, 0): [(0, y) for y in range(H)],
             (-1, 0): [(W-1, y) for y in range(H)],
             (0, 1): [(x, 0) for x in range(W)],
             (0, -1): [(x, H-1) for x in range(W)], }
        
def walk(pos, direction, H, W):
    x, y = pos
    dx, dy = direction
    while 0 <= x < W and 0 <= y < H:
        yield (x, y)
        x += dx
        y += dy

def part1(forest):
    H, W = len(forest), len(forest[0])
    seen = set()
    for direction, positions in visibility(H, W).items():
        for starting in positions:
            cur_max = -1
            for (x, y) in walk(starting, direction, H, W):
                tree = forest[y][x]
                if tree > cur_max:
                    cur_max = tree
                    seen.add((x, y))
    return len(seen)

def part2(forest):
    H, W = len(forest), len(forest[0])
    res = 0
    for sy, row in enumerate(forest[1:-1], 1):
        for sx, el in enumerate(row[1:-1], 1):
            score = 1
            for (dx, dy) in visibility(H, W).keys():
                steps = 0
                for (x, y) in walk((sx + dx, sy + dy), (dx, dy), H, W):
                    steps += 1
                    if forest[y][x] >= el:
                        break
                score *= steps
            res = max(score, res)
    return res

text = open(sys.argv[-1], 'r').read().strip()
input_lines = parse(text)
print(part1(input_lines))
print(part2(input_lines))
