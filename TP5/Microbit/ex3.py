import os

for file in os.listdir():
    if not file.startswith('.'):
        print(os.size(file), file)