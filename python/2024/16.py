from enum import Enum
import sys
from heapq import heappush, heappop
from pyrsistent import pvector


class Dir(Enum):
    EAST = (0, 1)
    SOUTH = (1, 0)
    WEST = (0, -1)
    NORTH = (-1, 0)

    def counterclockwise(self):
        dy, dx = self.value
        return Dir((-dx, dy))

    def clockwise(self):
        dy, dx = self.value
        return Dir((dx, -dy))

    def __lt__(self, other):
        return 0


def parse(full_input):
    res = []
    start = end = None
    for y, line in enumerate(full_input.strip().splitlines()):
        row = []
        for x, c in enumerate(line):
            match c:
                case "S":
                    s = "."
                    start = (y, x)
                case "E":
                    s = "."
                    end = (y, x)
                case _:
                    s = c
            row.append(s)
        res.append(row)
    return res, start, end


def part1(data, sdir=Dir.EAST):
    grid, start, end = data
    fringe = [(0, (start, 0, sdir))]
    visited = set()
    H = len(grid)
    W = len(grid[0])
    while fringe:
        _heu, (pos, cost, dir) = heappop(fringe)
        if pos == end:
            return cost
        if (vk := (pos, dir)) in visited:
            continue
        visited.add(vk)
        # visited.add((pos, dir.clockwise().clockwise()))
        for ndir, cst in zip(
            [dir, dir.counterclockwise(), dir.clockwise()], [1, 1000, 1000]
        ):
            dy, dx = ndir.value
            if (
                cst == 1
                and 0 <= (ny := pos[0] + dy) < H
                and 0 <= (nx := pos[1] + dx) < W
                and grid[ny][nx] == "."
            ):
                spent = cost + cst
                heu = spent + abs(ny - end[0]) + abs(nx - end[1])
                heappush(fringe, (heu, ((ny, nx), spent, dir)))
            else:
                spent = cost + cst
                heu = spent + abs(ny - end[0]) + abs(nx - end[1])
                heappush(fringe, (heu, (pos, spent, ndir)))
    return None


def part1path(grid, start, end, cache, sdir=Dir.EAST):
    fringe = [(0, (start, 0, sdir, pvector([(start, sdir, 0)])))]
    visited = set()
    H = len(grid)
    W = len(grid[0])
    while fringe:
        _heu, (pos, cost, dir, path) = heappop(fringe)
        if pos == end:
            for s, d, c in path:
                cache[s, d] = cost - c
            return cost, path
        if (vk := (pos, dir)) in visited:
            continue
        if sv := cache.get((vk := (pos, dir)), None):
            heappush(fringe, (cost + sv, (end, cost + sv, dir, path)))
            continue
        visited.add(vk)
        for ndir, cst in zip(
            [dir, dir.counterclockwise(), dir.clockwise()], [1, 1000, 1000]
        ):
            dy, dx = ndir.value
            if (
                cst == 1
                and 0 <= (ny := pos[0] + dy) < H
                and 0 <= (nx := pos[1] + dx) < W
                and grid[ny][nx] == "."
            ):
                spent = cost + cst
                heu = spent + abs(ny - end[0]) + abs(nx - end[1])
                heappush(
                    fringe,
                    (heu, ((ny, nx), spent, dir, path + [((ny, nx), dir, spent)])),
                )
            else:
                spent = cost + cst
                heu = spent + abs(ny - end[0]) + abs(nx - end[1])
                heappush(fringe, (heu, (pos, spent, ndir, path + [(pos, ndir, spent)])))
    return None


def part2(data, sdir=Dir.EAST):
    grid, start, end = data
    H = len(grid)
    W = len(grid[0])
    seen = {}
    cap, path = part1path(grid, start, end, seen, sdir)
    res = set(p for p, _, _ in path)
    fringe = [(start, sdir, 0)]
    visited = set()
    while fringe:
        nxt_fringe = []
        for pos, dir, spent in fringe:
            if (pos, dir) in visited:
                continue
            visited.add((pos, dir))
            for ndir, cost_first in zip(
                [dir, dir.counterclockwise(), dir.clockwise()], [1, 1000, 1000]
            ):
                dy, dx = ndir.value
                if (
                    cost_first == 1
                    and 0 <= (ny := pos[0] + dy) < H
                    and 0 <= (nx := pos[1] + dx) < W
                    and grid[ny][nx] == "."
                ):
                    cost_rest, _ = part1path(grid, (ny, nx), end, seen, dir)
                    if spent + cost_rest + cost_first == cap:
                        res.add(pos)
                        nxt_fringe.append(((ny, nx), dir, spent + cost_first))
                else:
                    cost_rest, _ = part1path(grid, pos, end, seen, ndir)
                    if spent + cost_rest + cost_first == cap:
                        res.add(pos)
                        nxt_fringe.append((pos, ndir, spent + cost_first))
        fringe = nxt_fringe
    return len(res)


if __name__ == "__main__":
    full_input = open(sys.argv[-1], "r").read()
    data = parse(full_input)
    print(p1 := part1(data))
    print(p2 := part2(data))
