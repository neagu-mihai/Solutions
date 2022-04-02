list2: List[Image] = []
k = 0

def on_forever():
    global list2, k
    list2 = [images.icon_image(IconNames.HEART),
        images.icon_image(IconNames.SMALL_HEART),
        images.icon_image(IconNames.HAPPY),
        images.icon_image(IconNames.SAD)]
    k = 0
    while k < 4:
        if input.button_is_pressed(Button.A):
            k += -1
            if k < 0:
                k = 0
            print(k)
            list2[k].show_image(0)
        else:
            if input.button_is_pressed(Button.B):
                list2[k].show_image(0)
                k += 1
                if k > 3:
                    k = 3
basic.forever(on_forever)
