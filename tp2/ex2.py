def on_forever():
    led.toggle(2, 3)
    basic.pause(100)
basic.forever(on_forever)
