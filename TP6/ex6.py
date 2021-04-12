blink = True

def pressed ():
    global blink
    while True:
        if input.button_is_pressed(Button.A):
            blink = False
        basic.pause (500)
    
control.in_background(pressed)

while blink:
    led.toggle(0, 0)
    basic.pause(1000)