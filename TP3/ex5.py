from microbit import display

electromenager = {
    "mixer": 105,
    "fer a repasser": 220,
    "refrigerateur": 999
}

while True:
    text = input("Ecrire le produit: ")
    if text  == "stop":
        break
    else:
        try:
            print(electromenager[text])
        except KeyError:
            print("Error")