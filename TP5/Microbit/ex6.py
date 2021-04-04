from microbit import *
import os

ct = 0
while True:
    if button_a.was_pressed():
        name = "file_" + str(ct)
        open (name, 'w')
        print("New file was created: " , os.listdir())
        ct += 1
    elif button_b.was_pressed():
        try:
            os.remove(os.listdir()[-1])
            print("Last file was deleted: ", os.listdir())
        except:
            print("No files!")
    else:
        pass