import sys
from collections import Counter

ASCII_TO_DIR = {
    ">": (0, 1),
    "<": (0, -1),
    "^": (-1, 0),
    "v": (1, 0),
}

DIR_TO_ASCII = {v: k for k, v in ASCII_TO_DIR.items()}


def parse(full_input):
    res = []
    me = None
    grids, dirs = full_input.strip().split("\n\n")
    for y, line in enumerate(grids.splitlines()):
        cur = []
        for x, c in enumerate(line):
            match c:
                case "@":
                    me = (y, x)
                    cur.append(".")
                case "#":
                    cur.append(c)
                case _:
                    cur.append(c)
        res.append(cur)

    return me, res, [ASCII_TO_DIR[c] for c in dirs.strip() if c != "\n"]


def pprint(me, grid):
    for y, line in enumerate(grid):
        for x, c in enumerate(line):
            if (y, x) == me:
                print("@", end="")
            else:
                print(c, end="")
        print()
    print()


STONE = "O"


def part1(data):
    me, grid, dirs = data
    H = len(grid)
    W = len(grid[0])
    my, mx = me
    del me
    for dy, dx in dirs:
        cy, cx = my, mx
        while (
            0 <= (ny := cy + dy) < H
            and 0 <= (nx := cx + dx) < W
            and grid[ny][nx] == STONE
        ):
            cy, cx = ny, nx
        if 0 <= ny < H and 0 <= nx < W and grid[ny][nx] == ".":
            if grid[my + dy][mx + dx] == STONE:
                grid[cy + dy][cx + dx] = STONE
            grid[my := my + dy][mx := mx + dx] = "."
    score = 0
    for y, line in enumerate(grid):
        for x, c in enumerate(line):
            if c == STONE:
                score += y * 100 + x
    return score


def expand(grid):
    res = []
    for line in grid:
        cur = []
        for c in line:
            if c == STONE:
                cur.extend(["[", "]"])
            else:
                cur.extend([c, c])
        res.append(cur)
    return res


STONES = "[]"


def part2(data):
    me, grid, dirs = data
    H = len(grid)
    W = len(grid[0]) * 2
    my, mx = me
    mx *= 2
    grid = expand(grid)
    del me

    def _can_move(cy, cx, dy, dx):
        if 0 <= (ny := cy + dy) < H and 0 <= (nx := cx + dx) < W:
            if (nxt := grid[ny][nx]) == ".":
                return True
            match nxt:
                case "[":
                    return _can_move(ny, nx, dy, dx) and (
                        not dy or _can_move(ny, nx + 1, dy, dx)
                    )
                case "]":
                    return _can_move(ny, nx, dy, dx) and (
                        not dy or _can_move(ny, nx - 1, dy, dx)
                    )
            return False
        return False

    def _move(cy, cx, dy, dx):
        nxt = grid[ny := cy + dy][nx := cx + dx]
        if dy == 0:
            if nxt in STONES:
                _move(ny, nx, dy, dx)
            grid[ny][nx] = grid[cy][cx]
            grid[cy][cx] = "."
            return
        match nxt:
            case "[":
                _move(ny, nx, dy, dx)
                _move(ny, nx + 1, dy, dx)
                grid[ny + dy][nx : nx + 2] = "[]"
                grid[ny][nx : nx + 2] = ".."
                grid[cy][cx] = "."

            case "]":
                _move(ny, nx, dy, dx)
                _move(ny, nx - 1, dy, dx)
                grid[ny + dy][nx - 1 : nx + 1] = "[]"
                grid[ny][nx - 1 : nx + 1] = ".."
                grid[cy][cx] = "."

    for dy, dx in dirs:
        cy, cx = my, mx
        if _can_move(cy, cx, dy, dx):
            _move(cy, cx, dy, dx)
            my, mx = cy + dy, cx + dx
    score = 0
    for y, line in enumerate(grid):
        for x, c in enumerate(line):
            if c == "[":
                score += y * 100 + x
    return score


if __name__ == "__main__":
    full_input = open(sys.argv[-1], "r").read()
    data = parse(full_input)
    print(p1 := part1(data))
    data = parse(full_input)
    print(p2 := part2(data))
