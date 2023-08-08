import sys
from collections import deque

def solve(text, size):
    cnt = set()
    prev = deque()
    for n, char in enumerate(text, 1):
        while char in cnt:
            cnt.remove(prev.popleft())
        cnt.add(char)
        prev.append(char)
        if len(prev) == size:
            return n

def solve(text, size):
    cnt = set()
    prev = 0
    for n, char in enumerate(text, 1):
        while char in cnt:
            cnt.remove(text[prev])
            prev += 1
        cnt.add(char)
        if n - prev == size:
            return n

text = open(sys.argv[-1], 'r').read().strip()
print(solve(text, 4))
print(solve(text, 14))
