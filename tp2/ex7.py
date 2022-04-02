def on_forever():
    for index in range(5):
        led.plot(index, 1)
        basic.pause(500)
        led.unplot(index, 1)
basic.forever(on_forever)
