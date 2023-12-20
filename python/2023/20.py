import sys
from math import prod, lcm
from collections import defaultdict, deque, Counter


def parse(data):
    conjs = set()
    srcs = defaultdict(dict)
    ff = {}
    dsts = {}
    for line in data.splitlines():
        lhs, rhs = line.split(' -> ')
        tdsts = rhs.split(', ')
        src = lhs if lhs.startswith('b') else lhs[1:]
        if lhs.startswith('&'):
            conjs.add(src)
        elif lhs.startswith('%'):
            ff[src] = 0
        for i in tdsts:
            srcs[i][src] = 0
        dsts[src] = tdsts
    return ff, conjs, srcs, dsts


def solve(ff, conjs, srcs, dsts):
    p1 = p2 = 0
    sigc = Counter()
    sink = next(iter(srcs.keys() - dsts.keys()))
    loop_ends = srcs[next(iter(srcs[sink]))].keys()
    loops = {}
    click = 0
    done = False
    while not done:
        click += 1
        dq = deque([(0, 'button', 'broadcaster')])
        while dq:
            signal, src, dst = dq.popleft()
            sigc[signal] += 1
            if dst == 'broadcaster':
                for o in dsts[dst]:
                    dq.append((signal, dst, o))
            elif dst in ff:
                if signal:
                    continue
                ff[dst] = 1 - ff[dst]
                for o in dsts[dst]:
                    dq.append((ff[dst], dst, o))
            elif dst in conjs:
                srcs[dst][src] = signal
                emit = 1 - all(srcs[dst].values())
                for o in dsts[dst]:
                    dq.append((emit, dst, o))
                if not signal and dst in loop_ends and dst not in loops:
                    loops[dst] = click
                    if len(loops) == len(loop_ends):
                        done = True
                        break
            else:
                assert dst == sink, (signal, src, dst)
        if click == 1000:
            p1 = prod(sigc.values())
    p2 = lcm(*loops.values())
    return p1, p2


data = open(sys.argv[-1]).read()
parsed = parse(data.strip())
p1, p2 = solve(*parsed)
print(p1)
print(p2)

assert p1 == 825896364, p1
assert p2 == 243566897206981, p2
