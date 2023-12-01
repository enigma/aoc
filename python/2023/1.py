import sys
from collections import defaultdict

DIGITS = list(map(str, range(1, 10)))
WORDS = 'one two three four five six seven eight nine'.split()


def solve(line, include_words=False):
    firsts = defaultdict(lambda: len(line))
    lasts = defaultdict(lambda: 0)
    digits = enumerate(DIGITS, 1)
    words = enumerate(WORDS, 1)
    for val, d in (*digits, *words) if include_words else digits:
        if (first_pos := line.find(d)) >= 0:
            firsts[val] = min(firsts[val], first_pos)
            lasts[val] = max(lasts[val], line.rindex(d))
    first = min(firsts.keys(), key=firsts.__getitem__)
    last = max(lasts.keys(), key=lasts.__getitem__)
    return 10*first + last


lines = open(sys.argv[-1]).read().strip().split()
part1 = sum(solve(i) for i in lines)
part2 = sum(solve(i, True) for i in lines)
print(part1)
print(part2)
assert (part1, part2) == (55538, 54875)
