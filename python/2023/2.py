import sys
from collections import defaultdict
from typing import NamedTuple
import math

class Game(NamedTuple):
    game: int
    cubes: list

def parse(lines):
    for line in lines:
        game, scubes = line.split(': ')
        game = int(game[5:])
        cubes = []
        for ks in scubes.split('; '):
            subset = {}
            for el in ks.split(', '):
                ipart, name = el.split()
                subset[name] = int(ipart)
            cubes.append(subset)
        yield Game(game, cubes)

def possible(game, bag):
    for c in game.cubes:
        for k, v in c.items():
            if bag.get(k, 0) < v:
                return False
    return True

def part1(games, bag):
    return sum(g.game for g in games if possible(g, bag))

def power(game):
    n = {}
    for s in game.cubes:
        for k, v in s.items():
            n[k] = max(n.get(k, v), v)
    return math.prod(n.values())

def part2(games):
    return sum(power(c) for c in games)
    
data = open(sys.argv[-1]).read()
lines = data.strip().split("\n")
parsed = list(parse(lines))
part1 = part1(parsed, bag={'red': 12, 'green': 13, 'blue': 14})
part2 = part2(parsed)
print(part1)
print(part2)
assert (part1, part2) == (2162, 72513)