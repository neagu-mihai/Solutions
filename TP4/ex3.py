from microbit import *
from time import sleep #import from time library sleep method 

ct = 0
while True:
    if ct < 9:
        if accelerometer.current_gesture() == "shake": #verify if the gesture is shake
            ct += 1
            display.show(str(ct)) #convert int to string
    else:
        display.show(Image.YES)
    sleep (100)
