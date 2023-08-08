import sys

def parse(i):
    elves = lines.split('\n\n')
    for elf in elves:
        yield list(map(int, elf.split()))

def part1(meals):
    return max(map(sum, meals))

from heapq import heappushpop, heappush
def part2(meals):
    heap = []
    for meal in map(sum, meals):
        f = heappush if len(heap) < 3 else heappushpop
        f(heap, meal)
    return sum(heap)

lines = open(sys.argv[-1]).read().strip()
meals = list(parse(lines))
print(part1(meals))
print(part2(meals))
