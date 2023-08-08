import sys

def parse(text):
    result = []
    for line in text.split('\n'):
        if ' ' in line:
            _, arg = line.split()
            result.append(int(arg))
        else:
            result.append(None)
    return result

def runner(instructions):
    cycle = 0
    value = 1
    for arg in instructions:
        is_add = arg is not None
        for _ in range(1 + is_add):
            cycle += 1
            yield (cycle, value)
        value += arg if is_add else 0

def part1(instructions):
    strength = 0
    run = iter(runner(instructions))
    for goal in [20, 60, 100, 140, 180, 220]:
        cycle, value = 0, None
        while cycle < goal:
            cycle, value = next(run)
        strength += cycle * value
    return strength

def part2(instructions):
    crt = [list('.'*40) for _ in range(6)]
    for i, v in runner(instructions):
        pixel = (i-1) % 40
        crt[(i % 240) // 40][pixel] = '#' if v - 1 <= pixel <= v + 1 else '.'
    return '\n'.join(''.join(p) for p in crt)

text = open(sys.argv[-1], 'r').read().strip()
input_lines = parse(text)
print(part1(input_lines))
print(part2(input_lines))
