import sys
 
def solve(l, r, arr):
    if r - l == 1:
        return 0
    mid = (l + r) >> 1
 
    mal, mar = max(arr[l:mid]), max(arr[mid:r])
    
    ans = 0
    if mal > mar:
        ans += 1
        for i in range(mid - l):
            arr[l + i], arr[mid + i] = arr[mid + i], arr[l + i]
 
    return solve(l, mid, arr) + solve(mid, r, arr) + ans
 
def solve_main(m, arr):
    ans = solve(0, m, arr)
    return ans if arr == sorted(arr) else -1
 
def main():
    input = sys.stdin.read
    data = input().split()
    
    t = int(data[0])
    index = 1
    results = []
    
    for _ in range(t):
        m = int(data[index])
        arr = list(map(int, data[index + 1: index + 1 + m]))
        results.append(str(solve_main(m, arr)))
        index += 1 + m
    
    sys.stdout.write("\n".join(results) + "\n")
 
if __name__ == "__main__":
    main()
