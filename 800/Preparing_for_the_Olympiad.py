t = int(input())
for _ in range(t):
    n = int(input())
    a = list(map(int, input().split()))
    b = list(map(int, input().split()))
    m = 0
    s = 0
    for i in range(n):
        if i == n - 1:
            m += a[i]
        else:
            if a[i] - b[i + 1] > 0:
                m += a[i]
                s += b[i + 1]
    print(m - s)
