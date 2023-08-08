import sys

DIR = {
    'R': (1, 0),
    'U': (0, 1),
    'L': (-1, 0),
    'D': (0, -1),
}

def parse(text):
    res = []
    for line in text.strip().split('\n'):
        k, amount = line.split()
        res.append((DIR[k], int(amount)))
    return res

def next_tail(tail, head):
    nx, ny = head[0] - tail[0], head[1] - tail[1]
    if abs(nx) < 2 and abs(ny) < 2:
        return tail
    tail = list(tail)
    if abs(nx) > 0:
        tail[0] += nx // abs(nx)
    if abs(ny) > 0:
        tail[1] += ny // abs(ny)
    return tail

def part1(intructions):
    seen = set()
    T,  H = [0, 0], [0, 0]
    seen.add(tuple(T))
    for (dx, dy), steps in intructions:
        for _ in range(steps):
            H = H[0] + dx, H[1] + dy
            T = next_tail(T, H)
            seen.add(tuple(T))
    return len(seen)

def part2(intructions):
    seen = set()
    tails = [[0, 0] for _ in range(10)]
    seen.add(tuple(tails[0]))
    for (dx, dy), steps in intructions:
        for _ in range(steps):
            head = tails[0]
            next_tails = [(head[0] + dx, head[1] + dy)]
            for knot in tails[1:]:
                next_tails.append(next_tail(knot, next_tails[-1]))
            tails = next_tails
            seen.add(tuple(tails[-1]))
    return len(seen)

text = open(sys.argv[-1], 'r').read().strip()
input_lines = parse(text)
print(part1(input_lines))
print(part2(input_lines))
