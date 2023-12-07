import sys
from typing import NamedTuple
from collections import Counter


VALS = {c: v for v, c in enumerate('AKQJT98765432')}
VALS2 = {c: v for v, c in enumerate('AKQT98765432J')}


class CamelHand(NamedTuple):
    hand: str
    bid: int

    def type(self):
        t = []
        for _, v in Counter(self.hand).most_common():
            if v > 1:
                t.append(v)
        while len(t) < 3:
            t.append(0)
        return tuple(t)

    def vals(self, vals=VALS):
        return tuple(-vals[i] for i in self.hand)

    def best_type(self):
        if 'J' not in self.hand:
            return self.type()
        c = Counter(self.hand.replace('J', ''))
        if not c:
            return (5, 0, 0)
        mc = c.most_common()
        v = mc[0][0]
        s = self.hand.replace('J', v)
        return CamelHand(s, self.bid).type()


def solve(lines):
    camel_hand = [CamelHand((p := l.split())[0], int(p[1])) for l in lines]
    camel_hand.sort(key=lambda c: (c.type(), c.vals()))
    p1 = sum(n * c.bid for n, c in enumerate(camel_hand, 1))

    camel_hand.sort(key=lambda c: (c.best_type(), c.vals(VALS2)))
    p2 = sum(n * c.bid for n, c in enumerate(camel_hand, 1))
    return p1, p2


data = open(sys.argv[-1]).read()
part1, part2 = solve(data.strip().splitlines())
print(part1)
print(part2)

assert part1 == 251806792
assert part2 == 252113488
