from microbit import *

#store the status of touch
#if it was pressed to set pixel or to add new pixel
NEW_PIXEL = 0
SET_PIXEL = 1
touch = SET_PIXEL

#store coordinates 
x = 0
y = 0

#change touch status
def toggle_touch():
    global touch
    touch = 1 - touch

#add a new pixel where empty place is available
#returns the position of the new pixel as (x,y) tuple
def newPixel():
    clear = False
    temp_x = 0
    temp_y = 0

    while not clear:
        if display.get_pixel (temp_x, temp_y) == 0:
            clear = True
        elif temp_x < 5:
            temp_x = temp_x+1
        elif temp_y < 5:
            temp_y = temp_y + 1
    display.set_pixel (temp_x, temp_y, 9)
    return (temp_x, temp_y)
    
def moveX():
    global x
    if x < 4:
        x = x+1
    else:
        x = 0
    checkOverlaps()
    
def moveY():
    global y
    if y < 4:
        y = y+1
    else:
        y = 0
    checkOverlaps()

#handle the case when the pixel overlaps an existing one
def checkOverlaps ():
    global x
    global y
    #spot is empty
    #set pixel
    if display.get_pixel (x, y) == 0:
        display.set_pixel (x, y, 9)
    #spot is not empty
    else :
        #blink LED
        display.set_pixel (x, y, 9)
        sleep (200)
        display.set_pixel (x, y, 0)
        sleep (200)
        #bring LED back to original state
        display.set_pixel (x, y, 9)
        #if logo is touched clear pixel and change touch state
        if pin_logo.is_touched():
            display.set_pixel (x, y, 0)
            toggle_touch()
        #if button was pressed change coordinate
        elif button_a.was_pressed ():
            moveY ()
        elif button_b.was_pressed ():
            moveX ()
        #nothing happended, call function again
        else:
            checkOverlaps ()
while True:
    if pin_logo.is_touched():
        toggle_touch()
        if touch == NEW_PIXEL:
            (x, y) = newPixel ()
    elif button_a.was_pressed():
        #clear current position and move
        display.set_pixel (x, y, 0)
        moveY()
    elif button_b.was_pressed():
        #clear current position and move
        display.set_pixel (x, y, 0)
        moveX()
    sleep (100)