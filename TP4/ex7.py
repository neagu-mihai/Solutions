from microbit import *

ct = 0
while True:
    if pin_logo.is_touched():
        ct += 1
        if ct == 1:
            display.scroll(display.read_light_level())
        elif ct == 2:
            display.scroll(temperature())
        elif ct == 3:
            display.scroll(microphone.sound_level())
        else:
            ct = 0
    sleep(100)