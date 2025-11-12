def solve():
    t = int(input())
    for _ in range(t):
        n = int(input())
        G = [list(map(int, input().split())) for _ in range(n)]
        
        p = [0] * (2 * n + 2) 
        seen = set()
        
        for i in range(n):
            for j in range(n):
                k = i + j + 2  
                if p[k] == 0:
                    p[k] = G[i][j]
                    seen.add(G[i][j])
        
        all_numbers = set(range(1, 2 * n + 1))
        missing = (all_numbers - seen).pop()
        p[1] = missing
        
        print(*p[1:2*n+1])
 
solve()
