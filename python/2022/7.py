import sys

def parse(text):
    fs = {'/': {}}
    pwd = []
    last_cmd = None
    for line in text.strip().split('\n'):
        if line[0] == '$':
            cmd = line.split()
            if cmd[1] == 'cd':
                if (path:=cmd[2]) == '/':
                    pwd = [fs[path]]
                elif path == '..':
                    pwd.pop()
                else:
                    pwd.append(pwd[-1][path])
            if cmd[1] == 'ls':
                continue
        else:
            size, name = line.split()
            pwd[-1][name] = {} if size == 'dir' else int(size)
    return fs

def total_sizes(fs):
    cur_size = 0
    for n, v in fs.items():
        if type(v) == dict:
            for latest in total_sizes(v):
                yield latest
            cur_size += latest
        else:
            cur_size += v
    yield cur_size

def part1(fs):
    return sum(size for size in total_sizes(fs) if size <= 100000)

def part2(fs):
    sizes = list(total_sizes(fs))
    need_gone = sizes[-1] - (70000000 - 30000000)
    return min(i for i in sizes if i >= need_gone)

text = open(sys.argv[-1], 'r').read().strip()
input_lines = parse(text)
print(part1(input_lines))
print(part2(input_lines))
