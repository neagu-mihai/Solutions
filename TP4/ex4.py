from microbit import *
from time import sleep

left = 0
right = 0

while True:
    if accelerometer.current_gesture() == "right":
        right += 1
        display.show(right)
    elif accelerometer.current_gesture() =="left":
        left += 1
        display.show(left)
    else:
        display.clear()
    sleep (100)