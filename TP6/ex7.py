def btn_a():
    while True:
        if input.button_is_pressed(Button.A):
            control.raise_event(1,1)
        basic.pause(500)
    
def btn_b():
    while True:
        if input.button_is_pressed(Button.B):
            control.raise_event(2,1)
        basic.pause(500)

control.in_background(btn_a)
control.in_background(btn_b)

def show_temp():
    print (input.temperature())

def show_light():
    print (input.light_level())

control.on_event(1, 1, show_temp)
control.on_event(2, 1, show_light)