import math

t = int(input())
for _ in range(t):
    x, y, z = map(int, input().split())
    K = x // y
    if z <= K:
        print(1)
    else:
        g = math.gcd(x, y)
        a1 = x // g
        if a1 == 1:
            print(1)
        else:
            print(2)