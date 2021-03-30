from microbit import *
from time import sleep
import music

while True:
    if display.read_light_level() > 100:
        display.show(Image.HAPPY)
        music.play(music.JUMP_UP)
    else:
        display.show(Image.ASLEEP)
        music.play(music.JUMP_DOWN)
    sleep(100)