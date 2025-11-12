def way_to_long():
    n = int(input())  
 
    for _ in range(n):
        word = input()
        length_of_word = len(word)
 
        if length_of_word > 10:
 
            list_word = list(word)
            letter_first = list_word[0]
            letter_last = list_word[-1]
 
            list_word.pop(0)
            list_word.pop(-1)
 
            new_word = "".join(list_word)
            length_new_word = len(new_word)
            abbr = letter_first + str(length_new_word) + letter_last
            print(abbr)
 
        else:
            print(word)
 
way_to_long()
