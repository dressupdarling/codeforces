def max_turns(n, x_k, y_k, x_d, y_d):
    ans = 0
    if x_d > x_k:
        ans = max(ans, x_d)
    elif x_d < x_k:
        ans = max(ans, n - x_d)
    if y_d > y_k:
        ans = max(ans, y_d)
    elif y_d < y_k:
        ans = max(ans, n - y_d)
    return ans
 
t = int(input())
for _ in range(t):
    n, x_k, y_k, x_d, y_d = map(int, input().split())
    print(max_turns(n, x_k, y_k, x_d, y_d))
