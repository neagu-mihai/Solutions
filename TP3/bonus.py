from microbit import display, sleep, temperature

#2
for i in range(5):
    sleep(200)
    for j in range(5):
        sleep(200)
        display.set_pixel(i, j, 9)
    sleep(200)
    
display.clear()

#3
display.scroll(temperature()) #display the temperature