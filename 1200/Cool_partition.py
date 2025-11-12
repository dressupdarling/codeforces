def solve():
    n = int(input())
    v = list(map(int, input().split()))
    distinct = [0] * n
    freq = [0] * (n + 1)
    
    total = 0
    for i in range(n):
        freq[v[i]] += 1
        if freq[v[i]] == 1:
            distinct[i] = 1
        distinct[i] += distinct[i - 1] if i > 0 else 0
 
    freq = [0] * (n + 1)
 
    ans = 0
    end = n - 1
    total = 0
    for i in range(n - 1, -1, -1):
        freq[v[i]] += 1
        if freq[v[i]] == 1:
            total += 1
        
        if total == distinct[end]:
            ans += 1
            for j in range(i, end + 1):
                freq[v[j]] = 0
            end = i - 1
            total = 0
 
    print(ans)
 
t = int(input())
for _ in range(t):
    solve()
