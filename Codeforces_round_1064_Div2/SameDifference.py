def main(s):
    last_char = s[-1]
    moves = 0
    for char in s:
        if char != last_char:
            moves += 1
    return moves

t = int(input())
for i in range(t):
    a = int(input())
    b = input().strip()
    print(main(b))
