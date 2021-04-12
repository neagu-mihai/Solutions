def check_temp():
    while True:
        temp = input.temperature()
        if temp > 15:
            control.raise_event (1, 1)
        else:
            control.raise_event (1, 0)
        basic.pause (500)

def check_light():
    while True:
        level = input.light_level()
        if level > 100:
            control.raise_event (2, 1)
        else:
            control.raise_event (2, 0)
        basic.pause (500)

def turn_on_temp_light():
    led.plot (0,0)

def turn_off_temp_light():
    led.unplot (0,0)

def turn_on_light_light():
    led.plot (4,4)

def turn_off_light_light():
    led.unplot (4,4)

control.in_background (check_temp)
control.in_background (check_light)
control.on_event(1, 1, turn_on_temp_light)
control.on_event(1, 0, turn_off_temp_light)
control.on_event(2, 1, turn_on_light_light)
control.on_event(2, 0, turn_off_light_light)