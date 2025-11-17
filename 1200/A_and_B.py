def min_block_swaps(positions):
    if not positions:
        return 0
    c = [positions[i] - i for i in range(len(positions))]
    m = c[len(c) // 2]
    return sum(abs(x - m) for x in c)


import sys

data = sys.stdin.read().strip().split()
t = int(data[0])
idx = 1

for _ in range(t):
    n = int(data[idx]);
    idx += 1
    s = data[idx];
    idx += 1

    posA, posB = [], []
    for i, ch in enumerate(s):
        if ch == 'a':
            posA.append(i)
        else:
            posB.append(i)

    swapsA = min_block_swaps(posA)
    swapsB = min_block_swaps(posB)

    print(min(swapsA, swapsB))