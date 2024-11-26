with open('t.txt') as f:
    inp = f.read().strip().split('\n')

locs = set()
rope = [[0,0] for _ in range(10)]
locs.add(tuple(rope[-1]))

for row in inp:
    d, s = row.split()
    s = int(s)
    for _ in range(s):
        dy, dx = {'U': (1, 0), 'D': (-1, 0), 
                  'L': (0, -1), 'R': (0, 1)}[d]
        rope[0][0] += dy
        rope[0][1] += dx
        for tail in range(1,10):
            dy = rope[tail-1][0] - rope[tail][0]
            dx = rope[tail-1][1] - rope[tail][1]
            if max(abs(dy), abs(dx)) > 1:
                rope[tail][0] += dy//abs(dy) if dy else 0
                rope[tail][1] += dx//abs(dx) if dx else 0
            locs.add(tuple(rope[-1]))
print(len(locs))