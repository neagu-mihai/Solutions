class Capteurs:
    temperature = 0
    lumiere = 0
    humidite = 0

    #constructor
    def __init__(self, temperature, lumiere, humidite):
        self.temperature = temperature
        self.lumiere = lumiere
        self.humidite = humidite

    #getter
    def getTemperature(self):
        return self.temperature

    def getLumiere(self):
        return self.lumiere

    def getHumidite(self):
        return self.humidite

    #setter
    def setTemperature(self, valeur):
        self.temperature = valeur 

    def setLumiere(self, valeur):
        self.lumiere = valeur 

    def setHumidite(self, valeur):
        self.humidite = valeur    

    def printData(self):
        print('La temperature est: '  + str(self.temperature) + '\n' +
              'La lumiere est: '  + str(self.lumiere) + '\n' +
              'La humidite est: '  + str(self.humidite) + '\n')

data = Capteurs(26, 25, 40)
data.printData()

data.setTemperature(20)
data.setLumiere(10)
data.setHumidite(50)

print(data.getHumidite())
