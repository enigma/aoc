import sys
from itertools import pairwise


class Node:
    def __init__(self, value):
        self.value = value
        self.before = set()
        self.after = set()

    def __hash__(self):
        return hash(self.value)

    def __eq__(self, other):
        return self.value == other.value

    def __lt__(self, other):
        if other in self.after:
            return True
        if other in self.before:
            return False
        for a in self.after:
            if a < other:
                self.after.add(other)
                other.before.add(self)
                return True
        for b in self.before:
            if other < b:
                self.before.add(other)
                other.after.add(self)
                return False
        raise ValueError(
            f"No order between {self} ({self.after}) and {other} ({other.before})"
        )

    def __str__(self):
        return f"Node({self.value})"

    def __repr__(self):
        return f"Node({self.value})"


def parse(full_input):
    p1, p2 = full_input.split("\n\n")
    nodes = {}
    for line in p1.splitlines():
        l, r = line.strip().split("|")
        l, r = int(l), int(r)
        if not (ln := nodes.get(l)):
            ln = nodes[l] = Node(l)
        if not (rn := nodes.get(r)):
            rn = nodes[r] = Node(r)
        ln.after.add(rn)
        rn.before.add(ln)
    updates = []
    for line in p2.splitlines():
        update = []
        for el in map(int, line.strip().split(",")):
            if not (n := nodes.get(el)):
                n = nodes[el] = Node(el)
            update.append(n)
        updates.append(update)

    return nodes, updates


def solve(data):
    _, updates = data
    part1 = part2 = 0
    for update in updates:
        mid = len(update) // 2
        if all(a < b for a, b in pairwise(update)):
            part1 += update[mid].value
        else:
            part2 += sorted(update)[mid].value
    return part1, part2


if __name__ == "__main__":
    full_input = open(sys.argv[-1], "r").read()
    data = parse(full_input)
    p1, p2 = solve(data)
    print(f"part1 = {p1}\npart2 = {p2}")
