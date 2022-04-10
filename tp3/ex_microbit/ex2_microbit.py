def on_button_pressed_a():
    for i in range(5):
        for j in range(5):
            if i == j:
                led.plot(i, j)
input.on_button_pressed(Button.A, on_button_pressed_a)

def on_button_pressed_b():
    for k in range(5):
        for l in range(5):
            if k + l == 4:
                led.plot(k, l)
input.on_button_pressed(Button.B, on_button_pressed_b)

def on_logo_pressed():
    for m in range(5):
        for n in range(5):
            if m == n or m + n == 4:
                led.plot(m, n)
input.on_logo_event(TouchButtonEvent.PRESSED, on_logo_pressed)