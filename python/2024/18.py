import sys
from heapq import heappush, heappop


def parse(full_input):
    return {
        p: k
        for k, p in enumerate(
            [tuple(int(i) for i in line.split(",")) for line in full_input.splitlines()]
        )
    }


def part1(data, lim=1024, W=71, H=71):
    fringe = [(0, (0, (0, 0)))]
    goal = (W - 1, H - 1)
    seen = set()
    while fringe:
        _heu, (cost, pos) = heappop(fringe)
        if pos == goal:
            return cost
        if pos in seen:
            continue
        if (bad := data.get(pos)) and bad <= lim:
            continue
        seen.add(pos)
        x, y = pos
        for dx, dy in [(1, 0), (0, -1), (-1, 0), (0, 1)]:
            if 0 <= (nx := x + dx) < W and 0 <= (ny := y + dy) < H:
                heu = cost + abs(nx - goal[0]) + abs(ny - goal[1])
                heappush(fringe, (heu, (cost + 1, (nx, ny))))
    return None


def part2(data, W=71, H=71):
    low = 1024
    high = len(data) + 1
    while low < high:
        mid = (low + high) // 2
        r = part1(data, W=W, H=H, lim=mid)
        if r is None:
            high = mid
        else:
            low = mid + 1
    r = [k for k, v in data.items() if v == low]
    return ",".join(str(i) for i in r[0])


if __name__ == "__main__":
    full_input = open(sys.argv[-1], "r").read()
    data = parse(full_input)
    print(p1 := part1(data))
    print(p2 := part2(data))
