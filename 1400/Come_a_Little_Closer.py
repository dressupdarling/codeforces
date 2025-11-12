import sys
import random
 
class MinMax:
    def __init__(self, a, b):
        self.mx1 = max(a, b)
        self.mx2 = min(a, b)
        self.mn1 = min(a, b)
        self.mn2 = max(a, b)
        self.fix_mx()
        self.fix_mn()
    
    def fix_mx(self):
        if self.mx1 < self.mx2:
            self.mx1, self.mx2 = self.mx2, self.mx1
    
    def fix_mn(self):
        if self.mn1 > self.mn2:
            self.mn1, self.mn2 = self.mn2, self.mn1
    
    def add(self, x):
        if x > self.mx1:
            self.mx2 = self.mx1
            self.mx1 = x
        elif x > self.mx2:
            self.mx2 = x
        
        if x < self.mn1:
            self.mn2 = self.mn1
            self.mn1 = x
        elif x < self.mn2:
            self.mn2 = x
    
    def get_seg(self, x):
        res_min = self.mn1
        res_max = self.mx1
        if x == res_min:
            res_min = self.mn2
        if x == res_max:
            res_max = self.mx2
        return res_max - res_min + 1
 
def solve(tc):
    n = int(sys.stdin.readline())
    coord = []
    for _ in range(n):
        x, y = map(int, sys.stdin.readline().split())
        coord.append((x, y))
    
    if n <= 2:
        print(n)
        return
    
    xc = MinMax(coord[0][0], coord[1][0])
    yc = MinMax(coord[0][1], coord[1][1])
    
    for i in range(2, n):
        xc.add(coord[i][0])
        yc.add(coord[i][1])
    
    ans = xc.get_seg(-1) * yc.get_seg(-1)
    
    for i in range(n):
        x = xc.get_seg(coord[i][0])
        y = yc.get_seg(coord[i][1])
        if x * y == n - 1:
            ans = min(ans, min((x + 1) * y, x * (y + 1)))
        else:
            ans = min(ans, x * y)
    
    print(ans)
 
multi = True
 
def main():
    t = 1
    if multi:
        t = int(sys.stdin.readline())
    for i in range(1, t + 1):
        solve(i)
        print()
 
if __name__ == "__main__":
    main()
