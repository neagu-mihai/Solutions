from microbit import *
import os

while True:
    f = open ("file", 'w')
    f.write(str(temperature()))
    f.close()
    
    sleep(10000)