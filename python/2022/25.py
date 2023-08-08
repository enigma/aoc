import sys

DIGITS = '=-012'
def dec_from_snafu(s):
    return 5 * dec_from_snafu(s[:-1]) + DIGITS.index(s[-1]) - 2 if s else 0

def snafu_from_dec(n):
    res = []
    while n:
        cur = (n+2) % 5
        res.append(DIGITS[cur])
        n = (n - (cur - 2)) // 5
    return ''.join(res[::-1])

def parse(text):
    return [dec_from_snafu(s) for s in text.strip().split('\n')]

def part1(data):
    return snafu_from_dec(sum(data))

lines = open(sys.argv[-1]).read().strip()
parsed = list(parse(lines))
print(p1:=part1(parsed))

assert p1 == '2-1=10=1=1==2-1=-221', p1