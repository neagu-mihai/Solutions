from microbit import *

def setBrightness (value):
    for i in range(5):
        for j in range(value):
            display.set_pixel(j, i, 9)

while True:
    level = display.read_light_level ()
    display.clear()
    if level < 50:
        setBrightness(5)
    elif level < 100:
        setBrightness(4)
    elif level < 150:
        setBrightness(3)
    elif level < 200:
        setBrightness(2)
    else:
        setBrightness(1)
    sleep (1000)
