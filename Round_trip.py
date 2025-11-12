def max_rating(R,X,D,rounds):
    rating = R
    count = 0
    for i in rounds:
        if i == "1":
            count += 1
            rating = max(0, rating - D)
        else:
            if rating < X:
                count += 1
    return count
    
t = int(input())
for _ in range(t):
    R,X,D,n = map(int,input().split())
    rounds = input()
    print(max_rating(R,X,D,rounds))
