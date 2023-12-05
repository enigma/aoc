import sys


def parse(lines):
    sections = lines.split("\n\n")
    seeds = [int(i) for i in sections[0][6:].split()]
    mappings = []
    for section in sections[1:]:
        mapping = []
        for line in section.splitlines()[1:]:
            mapping.append([int(i) for i in line.split()])
        mapping.sort(key=lambda v: (v[1]+v[2], v[1]))
        mappings.append(mapping)
    return seeds, mappings


def find(seed, mappings):
    current = seed
    for mapping in mappings:
        for (dst, src, step) in mapping:
            if src <= current < src + step:
                current = dst + current - src
                break
    return current


def find_range(rng, mappings):
    rng_start, total_seeds = rng
    ranges = [[rng_start, rng_start+total_seeds]]
    for mapping in mappings:
        mapped_seeds = sum(b-a for a, b in ranges)
        # Do not lose seeds!
        assert (mapped_seeds == total_seeds), (mapped_seeds, total_seeds)
        next_ranges = []
        mappings_left = mapping[::-1]
        to_map = ranges[::-1]
        while to_map:
            current = to_map.pop()
            cur_start, cur_end = current
            while mappings_left and cur_start >= sum(mappings_left[-1][1:]):
                mappings_left.pop()
            if not mappings_left:
                next_ranges.append(current)
                continue
            dst, src, step = mappings_left[-1]
            if cur_end < src:
                next_ranges.append(current)
                continue
            if cur_start < src:
                next_ranges.append((cur_start, src))
                cur_start = src
            if cur_end >= src + step:
                to_map.append((src + step, cur_end))
                cur_end = src + step
            # Now we have only stuff in within bounds
            assert src <= cur_start < src + step, (src, cur_start, src + step)
            assert src < cur_end <= src + step, (src, cur_end, src + step)
            nxt_start = dst + cur_start - src
            nxt_end = dst + cur_end - src
            next_ranges.append((nxt_start, nxt_end))
        next_ranges.sort(key=lambda v: v[::-1])
        ranges = next_ranges
    return min(i[0] for i in ranges)


def solve(parsed):
    seeds, mappings = parsed
    part1 = min(find(seed, mappings) for seed in seeds)

    pairs = (seeds[2*i:2*i+2] for i in range(len(seeds)//2))
    part2 = min(find_range(pair, mappings) for pair in pairs)
    return part1, part2


data = open(sys.argv[-1]).read()
lines = data.strip().strip()
parsed = parse(lines)
part1, part2 = solve(parsed)
print(part1)
print(part2)

assert part1 == 309796150
assert part2 == 50716416
