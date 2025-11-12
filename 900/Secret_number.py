def main():
    import sys
    input = sys.stdin.read
    data = input().split()
    
    t = int(data[0])
    index = 1
    
    for _ in range(t):
        r = int(data[index])
        index += 1
        
        d = 11
        ans = []
        while r >= d:
            if r % d == 0:
                ans.append(r // d)
            d = (d - 1) * 10 + 1
        
        print(len(ans))
        print(' '.join(map(str, ans[::-1])))
 
if __name__ == "__main__":
    main()
