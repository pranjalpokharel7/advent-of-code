file_name = "input.txt"
red = 12
green = 13
blue = 14
total = 0

def compare_count(c: str, n: int):
    if c == "r":
        return n <= red;
    elif c == "g":
        return n <= green;
    else:
        return n <= blue;

with open(file_name, "r") as f:
    for line in f.readlines():
        desc, game = line.split(':')
        game_number = desc.split(" ")[1]

        # each line is a game which is either possible or not
        possible = True 

        # evaluate each game
        game = game.strip()
        color = ""
        num = 0
        
        # iterate by character
        for i in range(len(game) - 1):
            cd = game[i] # prev digit
            nd = game[i+1] # next digit

            if cd.isdigit():
                num = num * 10 + int(cd)

            if cd == " " and not nd.isdigit():
                p = compare_count(nd, num) 
                if not p:
                    possible = False
                    break;

            if cd == "," or cd == ";":
                num = 0
        
        if possible:
            total += int(game_number)

print(total)
