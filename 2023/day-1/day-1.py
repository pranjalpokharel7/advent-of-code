file_name = "input.txt"
total = 0
with open(file_name, "r") as f:
    for line in f.readlines():
        s = ""
        r = ""
        for c in line:
            if c.isdigit():
                s += c
        if len(s):
            r += s[0] + s[-1]
        total += int(r)

print(total)
