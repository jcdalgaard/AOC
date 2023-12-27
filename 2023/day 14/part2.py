y = 1000000000
seq2 = [96141,96141,96124,96105,96094,96097,96095,96093,96096,96112,96132]
rev =  seq2.copy()
inp = [0 for x in (range(20445))]
rev.reverse()
for x in range(len(inp)-1,-1,-1):
    inp[x] = rev[x%len(rev)]

print(inp[(y-1)%len(inp)])

