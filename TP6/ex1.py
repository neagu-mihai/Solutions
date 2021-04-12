def onIn_background():
    while True:
        led.toggle(2, 2)
        basic.pause(2000)
control.in_background(onIn_background)

def on_forever():
    level = input.light_level()
    print(level)
    basic.pause(5000)
basic.forever(on_forever)