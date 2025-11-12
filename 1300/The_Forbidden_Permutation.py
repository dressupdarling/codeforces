import sys
input = sys.stdin.read

def main():
    data = input().split()
    idx = 0
    t = int(data[idx])
    idx += 1

    for _ in range(t):
        n = int(data[idx])
        m = int(data[idx + 1])
        d = int(data[idx + 2])
        idx += 3

        p = list(map(int, data[idx:idx + n]))
        idx += n

        a = list(map(int, data[idx:idx + m]))
        idx += m

        pos = [0] * (n + 1)
        for i in range(n):
            pos[p[i]] = i + 1  # 1-indexed like in the C++ code

        ans = int(1e9)

        for i in range(m - 1):
            u = a[i]
            v = a[i + 1]

            if pos[v] <= pos[u] or pos[v] - pos[u] > d:
                ans = 0
                break

            ans = min(ans, pos[v] - pos[u])

            dist = pos[v] - pos[u]
            swap_need = d - dist + 1
            swap_possible = (pos[u] - 1) + (n - pos[v])

            if swap_possible >= swap_need:
                ans = min(ans, swap_need)

        print(ans)

if __name__ == "__main__":
    main()
