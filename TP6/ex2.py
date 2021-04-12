
def onIn_background1():
    while True:
        led.toggle(4, 4)
        basic.pause (1000)

def onIn_background2():
    while True:
        led.toggle(0, 0)
        basic.pause (1500)

control.in_background(onIn_background1)
control.in_background(onIn_background2)
