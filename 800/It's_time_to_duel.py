t = int(input())
for _ in range(t):
    n = int(input())
    a = list(map(int, input().split()))
 
   
    dp = [set() for _ in range(n)]  
 
    dp[1].add(a[0])
 
    possible = True
    for i in range(2, n):  
        ai = a[i - 1] 
        current_ds = set()
        prev_ds = dp[i - 1]
        if not prev_ds:
            possible = False
            break
        for d_prev in prev_ds:
            if ai == 0:
 
                if d_prev == 1:
                    current_ds.add(0)
            else:
 
                if d_prev == 0:
                    current_ds.add(0)
                    current_ds.add(1)
                elif d_prev == 1:
                    current_ds.add(1)
        dp[i] = current_ds
        if not dp[i]:
            possible = False
            break
 
    if not possible:
        print("YES")
        continue
    last_a = a[-1]
    valid = False
    for d in dp[n - 1]:
        if (1 - d) == last_a:
            valid = True
            break
    if valid:
        print("NO")
    else:
        print("YES")
