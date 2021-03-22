from microbit import display

var1 = 80
var2 = 20.2
var3 = "microbit"

print(type(var1))
print(type(var2))
print(type(var3))

#bonus
display.scroll(type(var1))
sleep(0.25)
display.scroll(type(var2))
sleep(0.25)
display.scroll(type(var3))