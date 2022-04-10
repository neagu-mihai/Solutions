def firstPrimes(n):
    for i in range(1, n + 1):
        if i > 1:
            ok = True
            for j in range(2, i):
                if(i % j == 0):
                    ok = False
                    break
            if(ok):
                print(i)    

print("First prime numbers lower than 15 are: ")
firstPrimes(15)      
