t = int(input())
for _ in range(t):
    z = int(input())
    x = list(map(int, input().split()))

    mx = max(x)
    total = 0
    for i in range(z):
        j = (i + 1) % z
        total += max(x[i], x[j])

    result = total - mx
    print(result)