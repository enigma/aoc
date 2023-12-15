import sys


def parse(data):
    return data.split(',')


def ahash(s):
    cv = 0
    for c in s:
        cv += ord(c)
        cv *= 17
        cv %= 256
    return cv


def solve(records):
    p1 = sum(map(ahash, records))
    p2 = 0
    hashmap = [[] for _ in range(256)]
    for i in records:
        if i.endswith('-'):
            lab = i[:-1]
            box = ahash(lab)
            hashmap[box] = [j for j in hashmap[box] if not j.startswith(lab)]
        if i.find('=') > 0:
            lab = i[:i.find('=')]
            box = ahash(lab)
            newbox = []
            found = False
            for j in hashmap[box]:
                if j.startswith(lab):
                    newbox.append(i)
                    found = True
                else:
                    newbox.append(j)
            if not found:
                newbox.append(i)
            hashmap[box] = newbox
    for box in range(256):
        for slot, lens in enumerate(hashmap[box]):
            p2 += (box + 1) * (slot + 1) * int(lens[-1])
    return p1, p2


data = open(sys.argv[-1]).read()
parsed = parse(data.strip())
p1, p2 = solve(parsed)
print(p1)
print(p2)

assert p1 == 504036, p1
assert p2 == 295719, p2
