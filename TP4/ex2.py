from microbit import *

while True:
    if button_a.is_pressed() and button_b.is_pressed(): #check if the both buttons are pressed
        display.show(Image.YES)
    else:
        display.show(Image.NO)
    sleep(100)