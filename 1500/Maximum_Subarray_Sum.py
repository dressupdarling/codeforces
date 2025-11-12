def main():
    t = int(input())
    for _ in range(t):
        n, k = map(int, input().split())
        s = input()
        a = list(map(int, input().split()))
        
        pos = -1
        for i in range(n):
            if s[i] == '0':
                pos = i
                a[i] = int(-1e13)
 
        mx = 0
        curr = 0
        for i in range(n):
            curr = max(curr + a[i], a[i])
            mx = max(mx, curr)
 
        if mx > k or (mx != k and pos == -1):
            print("No")
            continue
 
        if pos != -1:
            mx = 0
            curr = 0
            L = 0
            R = 0
            
            for i in range(pos + 1, n):
                curr += a[i]
                mx = max(mx, curr)
            L = mx
 
            mx = 0
            curr = 0
            for i in range(pos - 1, -1, -1):
                curr += a[i]
                mx = max(mx, curr)
            R = mx
            
            a[pos] = k - L - R
 
        print("Yes")
        print(" ".join(map(str, a)))
 
if __name__ == "__main__":
    main()
