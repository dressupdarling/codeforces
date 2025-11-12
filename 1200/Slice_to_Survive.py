import sys
 
 
def steps(d, memo=None):
    if memo is None:
        memo = {}
    if d in memo:
        return memo[d]
    if d == 1:
        return 0
    res = 1 + steps((d + 1) // 2, memo)
    memo[d] = res
    return res
 
 
def main():
    input = sys.stdin.read().split()
    t = int(input[0])
    idx = 1
    for _ in range(t):
        n = int(input[idx])
        m = int(input[idx + 1])
        a = int(input[idx + 2])
        b = int(input[idx + 3])
        idx += 4
 
        row_part = min(a, n - a + 1)
        col_part = min(b, m - b + 1)
 
        option1 = steps(row_part) + steps(m)
        option2 = steps(n) + steps(col_part)
 
        total = min(option1, option2) + 1
        print(total)
 
 
if __name__ == "__main__":
    main()
