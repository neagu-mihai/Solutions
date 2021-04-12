level = 0

def on_in_background():
    global level
    while True:
        level = input.light_level()
        basic.pause(2000)
control.in_background(on_in_background)

def on_microbit_evt():
    print(level)
control.on_event(0, EventBusValue.MICROBIT_EVT_ANY, on_microbit_evt)