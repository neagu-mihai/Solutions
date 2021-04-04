from microbit import *
import os

ct = 0 
while True:
    if ct == 0:                              #check if the file 'light_file' exists
        fileValue = display.read_light_level()
        f = open ("light_file", 'w')
        f.write(str(light))                  #write the value from the sensor in file
        f.close()
        print('The file was created. The light value is: ' +  str(light))
        ct = 1
    elif ct == 1:
        newValue = display.read_light_level()  #read the current value from the sensor
        if abs(fileValue - newValue) >= 20:
            f = open ("light_file", 'w')
            f.write (str(newValue))
            f.close()
            fileValue = newValue
            print('The value was changed: ' + str(newValue))
        else:
            pass
    sleep(10000)