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

cuisine =  Capteurs(0, 0, 0)
hall = Capteurs(0, 0, 0)
chambreCouche = Capteurs(0, 0, 0)
salleMange = Capteurs(0, 0, 0)

chambres = {
    "cuisine" : cuisine,
    "hall" : hall,
    "chambreCouche" : chambreCouche,
    "salleMange" : salleMange
}

text = input("Ecrire la commande: ")
while True:
    try:
        commande = text.split(" ")
        inputCommande = commande[0]
        chambre =  commande[1]
        if inputCommande == 'insert' or inputCommande == 'delete':
            capteur = commande[2]
            valeur = commande[3]
            for i in chambres:
                if i == chambre:
                    if (capteur == "temperature"):
                        chambres[i].setTemperature(valeur)
                    elif (capteur == "lumiere"):
                        chambres[i].setLumiere(valeur)
                    elif (capteur == "humidite"):
                        chambres[i].setHumidite(valeur)     
        elif inputCommande == 'print':
            for i in chambres:
                if i == chambre:
                    if len(commande) == 2:
                        chambres[i].printData()
                    else: 
                        if (commande[2] == "temperature"):
                            print(chambres[i].getTemperature())
                        elif (commande[2] == "lumiere"):
                            print(chambres[i].getLumiere())
                        elif (commande[2] == "humidite"):
                            print(chambres[i].getHumidite())
        elif inputCommande == 'add_room':
            chambres[chambre] = Capteurs(0, 0, 0)
        elif inputCommande == 'del_room':
            del chambres[chambre]
        text = input("Ecrire la commande: ")
    except ValueError:
        print("Error")