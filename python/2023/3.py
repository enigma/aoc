import sys

def solve(lines):
    H, W = len(lines), len(lines[0])
    p1 = 0
    p2 = {}
    for y, line in enumerate(lines):
        x = 0
        while x < W:
            digits = 0
            while x + digits < W and line[x + digits].isdigit():
                digits += 1
            if digits:
                parsed = int(line[x:x+digits])
                near_symbols = False
                for ny in range(max(0, y-1), min(H, y+2)):
                    for nx in range(max(0, x-1), min(W, x+digits+1)):
                        c = lines[ny][nx]
                        if c == '.' or c.isdigit():
                            continue
                        near_symbols = True
                        if c == '*':
                            k = (ny, nx)
                            if k not in p2:
                                p2[k] = [0, 1]
                            p2[k][0] += 1
                            p2[k][1] *= parsed
                if near_symbols:
                    p1 += parsed
            x += digits + 1
    return p1, sum(v[1] for _, v in p2.items() if v[0] > 1)

data = open(sys.argv[-1]).read()
lines = data.strip().split("\n")
part1, part2 = solve(lines)
print(part1)
print(part2)
assert (part1, part2) == (521515, 69527306)
