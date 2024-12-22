import sys
from collections import Counter


def parse(full_input):
    return [int(x) for x in full_input.splitlines()]


def next_secret(secret, mod=16777216):
    secret = (secret ^ (secret * 64)) % mod
    secret = (secret ^ (secret // 32)) % mod
    secret = (secret ^ (secret * 2048)) % mod
    return secret


def solve(data):
    p1, p2 = 0, Counter()
    for start in data:
        secret, deltas, seen = start, [], Counter()
        for i in range(2000):
            was, secret = secret, next_secret(secret)
            deltas.append((secret % 10) - (was % 10))
            if i >= 4:
                seen.setdefault(tuple(deltas[-4:]), secret % 10)
        p1 += secret
        p2 += seen
    return p1, p2.most_common(1)[0][1]


if __name__ == "__main__":
    full_input = open(sys.argv[-1], "r").read()
    data = parse(full_input)
    p1, p2 = solve(data)
    print(p1)
    print(p2)
