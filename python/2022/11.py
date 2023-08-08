import sys
from collections import namedtuple, Counter
from copy import deepcopy

Monkey = namedtuple('Monkey', 'monkey_no, items, op, divisor, if_true, if_false'.split(', '))

def parse_op(ops):
    match ops.split():
        case '*', 'old': return lambda old: old * old
        case '*', n: return lambda old: old * int(n) 
        case '+', n: return lambda old: old + int(n) 
        case _: assert False, ('wtf', ops)

def parse(text):
    result = []
    for monkey_no, line in enumerate(text.split('\n\n')):
        rows = line.split("\n")
        items = [int(i) for i in rows[1].split(': ')[-1].split(',')]
        op = parse_op(rows[2].split('old ')[-1])
        divisor = int(rows[3].split()[-1])
        if_true = int(rows[4].split()[-1])
        if_false = int(rows[5].split()[-1])
        result.append(Monkey(monkey_no, items, op, divisor, if_true, if_false))
    return result

def solve(monkeys, rounds, limit):
    monkeys = deepcopy(monkeys)
    c = Counter()
    for _ in range(rounds):
        for n, m in enumerate(monkeys):
            items = m.items[:]
            del m.items[:]
            for item in items:
                c[n] += 1
                new_item= limit(m.op(item))
                next_monkey = m.if_true if new_item % m.divisor == 0 else m.if_false
                monkeys[next_monkey].items.append(new_item)
    result = 1
    for _, v in c.most_common(2):
        result *= v
    return result

def part1(monkeys):
    return solve(monkeys, 20, lambda n: n // 3)

def part2(monkeys):
    ceiling = 1
    for n in monkeys: ceiling *= n.divisor
    return solve(monkeys, 10_000, lambda n: n % ceiling)

text = open(sys.argv[-1], 'r').read().strip()
input_lines = parse(text)
print(part1(input_lines))
print(part2(input_lines))
