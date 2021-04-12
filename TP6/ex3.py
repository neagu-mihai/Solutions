def background_task():
    while True:
        level = input.light_level()
        control.raise_event(1,level)
        pause(2000)
control.in_background(background_task)
pause (1000)

while True:
    print (control.event_value())
    basic.pause (2000)