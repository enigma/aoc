import sys
from functools import cache


def to_trie(patterns):
    root = {}
    for pattern in patterns:
        node = root
        for c in pattern:
            node = node.setdefault(c, {})
        node["$"] = True
    return root


def parse(full_input):
    patterns, designs = full_input.strip().split("\n\n")
    return to_trie(patterns.split(", ")), designs.splitlines()


def solve(data):
    patterns, designs = data

    @cache
    def can_do(design):
        if not design:
            return True
        res, node = 0, patterns
        for i, c in enumerate(design):
            if not (node := node.get(c)):
                break
            if "$" in node:
                res += can_do(design[i + 1 :])
        return res

    solutions = [can_do(design) for design in designs]
    return sum(1 for s in solutions if s > 0), sum(solutions)


if __name__ == "__main__":
    full_input = open(sys.argv[-1], "r").read()
    data = parse(full_input)
    p1, p2 = solve(data)
    print(p1)
    print(p2)
