def on_forever():
    if input.button_is_pressed(Button.A):
        basic.show_icon(IconNames.HAPPY)
    else:
        if input.button_is_pressed(Button.B):
            basic.show_icon(IconNames.SAD)
basic.forever(on_forever)
