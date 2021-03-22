from microbit import display

a = input("La valeur pour a est: ")
b = input("La valeur pour b est: ")
c = input("La valeur pour c est: ")
d = input("La valeur pour d est: ")

result = int(a)+int(b)*int(c)+int(d)

print(result)

#bonus
display.scroll(result)