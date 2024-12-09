import sys
from dataclasses import dataclass
from collections import deque


@dataclass
class Data:
    size: int
    value: int
    start: int


@dataclass
class Gap:
    size: int
    start: int | None = None


def parse(full_input):
    idx = 0
    filled = 0
    res = []
    for i, d in enumerate(full_input.strip()):
        size = int(d)
        if i % 2 == 0:
            res.append(Data(size, idx, filled))
            idx += 1
        else:
            res.append(Gap(size, start=filled))
        filled += size
    return res

def score(files):
    return sum(d.value * i for d in files for i in range(d.start, d.start + d.size))

def solve_part1(data):
    files = deque()
    gaps = deque()
    for d in data:
        if isinstance(d, Data):
            files.append(d)
        elif isinstance(d, Gap):
            gaps.append(d)
    while gaps:
        gap = gaps.popleft()
        if not gap.size:
            continue
        if not files:
            gaps.appendleft(gap)
            break
        file = files.pop()
        if file.start + file.size < gap.start:
            files.append(file)
            continue
        if file.size >= gap.size:
            files.appendleft(Data(gap.size, file.value, gap.start))
            if leftover := file.size - gap.size:
                files.append(Data(leftover, file.value, file.start))
        else:
            files.appendleft(Data(file.size, file.value, gap.start))
            gaps.appendleft(Gap(gap.size - file.size, start=gap.start + file.size))
    return score(files)


def solve_part2(data):
    files = deque()
    gaps = [deque() for _ in range(10)]
    for d in data:
        if isinstance(d, Data):
            files.append(d)
        elif isinstance(d, Gap):
            if d.size:
                gaps[d.size].append(d)
    moved = []
    while files:
        file = files.pop()
        earliest = None
        for gapd in gaps[file.size :]:
            if gapd and (earliest is None or gapd[0].start < earliest.start):
                earliest = gapd[0]
        if earliest and earliest.start < file.start + file.size:
            moved.append(Data(file.size, file.value, earliest.start))
            gaps[earliest.size].popleft()
            if leftover := earliest.size - file.size:
                gapd = gaps[leftover]
                gapd.appendleft(Gap(leftover, start=earliest.start + file.size))
                for i in range(1, len(gapd)):
                    if gapd[i].start < gapd[i - 1].start:
                        gapd[i], gapd[i - 1] = gapd[i - 1], gapd[i]
        else:
            moved.append(file)
    return score(moved)


def solve(data):
    part1 = solve_part1(data)
    part2 = solve_part2(data)
    return part1, part2


if __name__ == "__main__":
    full_input = open(sys.argv[-1], "r").read()
    data = parse(full_input)
    p1, p2 = solve(data)
    print(f"part1 = {p1}\npart2 = {p2}")
    assert p1 == 6398252054886, p1
    assert p2 == 6415666220005, p2
