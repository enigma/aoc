import sys


def parse(full_input):
    keys, schematics = [], []
    for block in full_input.split("\n\n"):
        pattern = [sum(k == "#" for k in col) for col in zip(*block.splitlines())]
        [keys, schematics][block[0] == "#"].append(pattern)
    return keys, schematics


def solve(keys, schematics):
    return sum(
        all(k + s <= 7 for k, s in zip(key, schematic))
        for key in keys
        for schematic in schematics
    )


if __name__ == "__main__":
    full_input = open(sys.argv[-1], "r").read()
    data = parse(full_input)
    print(p1 := solve(*data))
