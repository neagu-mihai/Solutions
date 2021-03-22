from microbit import display

nameList = ["Maria", "Ioana", "Maria", "Andrei", "Cristian", "Mihaela", "Maria"]
nameList.append("Cristian")
nameList.append("Mihaela")

#a
copyList = nameList.copy()
copyList.sort()
print(nameList)
print(copyList)

#b
auxList = []
for name in nameList:
    if name not in auxList:
        auxList.append(name)

for name in auxList:
    print("Le nom " + name + " a le numéro d'occurences: " + str(nameList.count(name)))

#c
occurenceList = []
for name in auxList:
    occurenceList.append(nameList.count(name))

minCount = min(occurenceList)

maxCount = max(occurenceList)

for name in auxList:
    if nameList.count(name) == minCount:
        print("Le nom avec le numero minimal d'occurences " + str(minCount) + " est " + name)
    elif nameList.count(name) == maxCount:
        print("Le nom avec le numero maximal d'occurences " + str(maxCount) + " est " + name)

#d
copyList.reverse()
display.scroll(copyList)

nameList.pop()
display.scroll(nameList)

#bonus        
display.scroll(nameList)
display.scroll(copyList)

copyList.reverse()
display.scroll(copyList)

nameList.pop()
display.scroll(nameList)
