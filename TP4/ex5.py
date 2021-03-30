from microbit import *
from time import sleep

ct = True
while True:
    if pin_logo.is_touched():
        ct = not ct
    if ct:
        display.show(Image.DIAMOND)
    else:
        display.show(Image.DIAMOND_SMALL)
    sleep (100)