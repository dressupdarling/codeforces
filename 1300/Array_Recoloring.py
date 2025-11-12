import sys
 
def main():
    t = int(sys.stdin.readline().strip())
    for _ in range(t):
        n, k = map(int, sys.stdin.readline().split())
        a = list(map(int, sys.stdin.readline().split()))
        ans = 0
 
        if k > 1:
            a.sort(reverse=True)
            ans = sum(a[:k+1])
        else:
            l = max(a[:-1])
            r = max(a[1:])
            ans = max(l + a[-1], r + a[0])
 
        print(ans)
 
if __name__ == "__main__":
    main()
