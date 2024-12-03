import sys
import pyrsistent as p


def parse(full_input):
    grid = [list(line.strip()) for line in full_input.strip().splitlines()]
    pos = None
    for y, row in enumerate(grid):
        for x, cell in enumerate(row):
            if cell in '<>v^':
                pos = (y, x)
                break
    y, x = pos
    match grid[y][x]:
        case "<":
            dir = (0, -1)
        case ">":
            dir = (0, 1)
        case "v":
            dir = (1, 0)
        case "^":
            dir = (-1, 0)
        case _:
            raise ValueError(f"Unknown direction {grid[y][x]}")
    grid[y][x] = "."
    return grid, pos, dir


def solve(data, part2=False):
    grid, pos, dd = data
    H, W = len(grid), len(grid[0])
    fringe = [(pos, dd, None, p.s(), p.s())]
    obss = set()
    while fringe:
        pos, dd, obs, seen_so_far, seen_poss = fringe.pop()
        (y, x), (dy, dx) = pos, dd
        if not (0 <= y + dy < H and 0 <= x + dx < W):
            if not part2 and obs is None:
                return len(seen_poss) + 1
            continue
        if obs in obss:
            continue
        if (pos, dd) in seen_so_far:
            obss.add(obs)
            print(pos, dd, obs, len(seen_so_far), len(obss))
            continue
        seen_so_far = seen_so_far.add((pos, dd))
        seen_poss = seen_poss.add((y, x))

        if grid[ny := y + dy][nx := x + dx] == "#" or (ny, nx) == obs:
            dy, dx = dx, -dy
            fringe.append(((y, x), (dy, dx), obs, seen_so_far, seen_poss))
        else:
            fringe.append(((ny, nx), (dy, dx), obs, seen_so_far, seen_poss))
            if part2 and not obs and (ny, nx) not in seen_poss:
                fringe.append(((y, x), (dx, -dy), (ny, nx), seen_so_far, seen_poss))
    return len(obss)

def solve(data):
    grid, pos, dd = data
    H, W = len(grid), len(grid[0])
    fringe = [(pos, dd, None, p.s(), p.s())]
    obss = set()
    part1 = None
    while fringe:
        pos, dd, obs, seen_so_far, seen_poss = fringe.pop()
        (y, x), (dy, dx) = pos, dd
        if not (0 <= y + dy < H and 0 <= x + dx < W):
            if obs is None:
                part1 = len(seen_poss) + 1
            continue
        if obs in obss:
            continue
        if (pos, dd) in seen_so_far:
            obss.add(obs)
            print(pos, dd, obs, len(seen_so_far), len(obss))
            continue
        seen_so_far = seen_so_far.add((pos, dd))
        seen_poss = seen_poss.add((y, x))

        if grid[ny := y + dy][nx := x + dx] == "#" or (ny, nx) == obs:
            dy, dx = dx, -dy
            fringe.append(((y, x), (dy, dx), obs, seen_so_far, seen_poss))
        else:
            fringe.append(((ny, nx), (dy, dx), obs, seen_so_far, seen_poss))
            if not obs and (ny, nx) not in seen_poss:
                fringe.append(((y, x), (dx, -dy), (ny, nx), seen_so_far, seen_poss))
    return part1, len(obss)
if __name__ == "__main__":
    full_input = open(sys.argv[-1], "r").read()
    data = parse(full_input)
    p1, p2 = solve(data)
    print(f"part1 = {p1}\npart2 = {p2}")
