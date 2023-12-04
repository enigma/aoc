import sys
from typing import NamedTuple
from itertools import zip_longest

class Card(NamedTuple):
    id: int
    winning: list
    drawn: list

    def matches(self):
        drawn = 0
        for i in self.drawn:
            drawn += 1 << i
        res = 0
        for w in self.winning:
            if (1 << w) & drawn > 0:
                res += 1
        return res

def solve(lines):
    cards = []
    for line in lines:
        parts = line.split(": ")
        cid = int(parts[0][5:])
        win, play = parts[1].split(' | ')
        cards.append(
            Card(cid,
                  [int(i) for i in win.split()],
                    [int(i) for i in play.split()]))
    p1 = 0
    p2 = 0
    l = len(cards)
    copies = [0 for _ in cards]
    for i, card in enumerate(cards):
        matches = card.matches()
        if matches:
            score = 1
            for j in range(1, matches):
                score += score
            p1 += score
        p2 += 1 + copies[i]
        for j in range(i+1, min(l, i+matches)+1):
            copies[j] += 1 + copies[i]
    return p1, p2
    

data = open(sys.argv[-1]).read()
lines = data.strip().split("\n")
part1, part2 = solve(lines)
print(part1)
print(part2)
assert (part1, part2) == (21558, 10425665)
