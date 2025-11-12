MOD = 10**9 + 7
max_n = 200005

dp = [0] * max_n
for i in range(9):
    dp[i] = 2
dp[9] = 3
for i in range(10, max_n):
    dp[i] = (dp[i-9] + dp[i-10]) % MOD

def solve():
    import sys
    input = sys.stdin.read
    data = input().split()
    
    t = int(data[0])
    index = 1
    for _ in range(t):
        n = data[index]
        m = int(data[index+1])
        index += 2
        
        ans = 0
        for ch in n:
            x = int(ch)
            if m + x < 10:
                ans += 1
            else:
                ans += dp[m + x - 10]
            ans %= MOD
        print(ans)

solve()
