def digitize(n):
    list = []
    while n!= 0:
        list.append(n%10)
        n = n//10
    return list

n = 12345
print(digitize(n))