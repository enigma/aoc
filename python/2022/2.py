import sys

def parse(i):
    for line in i.split('\n'):
        yield line.split(' ')

def score(them, me):
    points = me + 1
    if them == me:
        points += 3
    if me == (them + 1) % 3:
        points += 6
    return points

def part1(rounds):
    points = 0
    for them, me in rounds:
        points += score('ABC'.index(them), 'XYZ'.index(me))
    return points

def part2(rounds):
    points = 0
    for them, outcome in rounds:
        them = 'ABC'.index(them)
        me = (them + 'XYZ'.index(outcome) - 1) % 3
        points += score(them, me)
    return points
    
lines = open(sys.argv[-1]).read().strip()
stuff = list(parse(lines))
print(part1(stuff))
print(part2(stuff))

assert part1(stuff) == 13675, part1(stuff)
assert part2(stuff) == 14184, part2(stuff)
