def candies_add_x():
    n_candies = int(input())
    add_x = (n_candies%3)
    if add_x == 1:
        print(2)
    elif add_x == 2:
        print(1)
    else:
        print(0)
 
t = int(input())
for _ in range(t):
    candies_add_x()
