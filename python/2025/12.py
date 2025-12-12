import sys


def parse(data):
    *spieces, sregions = data.split("\n\n")
    sshape = [s[2:].strip() for s in spieces]
    regions = []
    for sregion in sregions.splitlines():
        size, *cnts = sregion.split()
        width, height = map(int, size[:-1].split("x"))
        regions.append(((width, height), list(map(int, cnts))))
    return sshape, regions


def can_fit(patterns, area):
    (width, height), cnts = area
    pieces = sum(cnts)
    board_pixels = width * height
    pieces_pixels = sum(p.count("#") * n for p, n in zip(patterns,cnts))

    if pieces <= (width // 3) * (height // 3):
        # Can be trivially solved by placing the pieces in a grid.
        return True
    if pieces_pixels > board_pixels:
        # Obviously not possible.
        return False
    assert False, "Dammit Eric."


def solve(patterns, areas):
    return sum(can_fit(patterns, area) for area in areas)


if __name__ == "__main__":
    data = parse(open(sys.argv[-1], "r").read())
    print(f"{solve(*data)}")
