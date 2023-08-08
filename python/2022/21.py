import sys
from collections import deque
from operator import add, sub, mul, floordiv
from functools import cache

def parse(text):
    ops = {'+': add, '*': mul, '-': sub, '/': floordiv}
    result = dict()
    for i in text.strip().split('\n'):
        alias, definition = i.split(': ')
        if len(definition) > 4:
            lhs, op, rhs = definition.split()
            result[alias] = (lhs, ops[op], rhs)
        else:
            result[alias] = int(definition)
    return result

def part1(data):
    @cache
    def evaluate(s):
        match data[s]:
            case int(): return data[s]
            case (lhs, op, rhs): return op(evaluate(lhs), evaluate(rhs))
    return evaluate('root')

from fractions import Fraction
def part2(data):
    def inverse_lhs(other, n, op, hs):
        if op is add: return other - n, hs
        if op is sub: return n - other, hs
        if op is mul: return Fraction(other, n), hs
        return Fraction(n, other), hs
    def inverse_rhs(other, hs, op, n):
        if op is add: return other - n, hs
        if op is sub: return other + n, hs
        if op is mul: return Fraction(other, n), hs
        return other * n, hs

    @cache
    def evaluate(s):
        if s == 'humn': return (True, None)
        match data[s]:
            case int(): return (False, data[s])
            case (lhs, op, rhs):
                l, r = evaluate(lhs), evaluate(rhs)
                if l[0] or r[0]: return True, (l[1], op, r[1])
                else: return False, op(l[1], r[1])
    lhs, rhs = map(evaluate, data['root'][::2])
    if lhs[0]: mine, other = lhs[1], rhs[1]
    else: mine, other = rhs[1], lhs[1]

    while mine != None:
        match mine:
            case (int() as n, op, hs):
                other, mine = inverse_lhs(other, n, op, hs)
            case (hs, op, int() as n):
                other, mine = inverse_rhs(other, hs, op, n)
    return other

text = open(sys.argv[-1], 'r').read()
parsed = parse(text)
print(part1(parsed))
print(part2(parsed))
