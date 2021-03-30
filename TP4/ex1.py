from microbit import *

while True:
    if button_a.is_pressed() and button_b.is_pressed(): #check if the both buttons are pressed
        display.scroll("OUI")
    else:
        display.scroll("NO")
    sleep (100)