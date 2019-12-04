def testOnePassword(pwd: int):
    if (pwd < 0 or pwd > 999999):
        return False

    twoAdjacentFound = False
    lastAdjacentCount = 1
    lastDigit = pwd % 10
    
    # from right to left
    for posDigit in range(1, 7):
        currDigit = int(pwd / pow(10, posDigit)) % 10

        if currDigit > lastDigit:
            return False

        if currDigit == lastDigit:
            lastAdjacentCount += 1
        else:
            if lastAdjacentCount == 2:
                twoAdjacentFound = True
            lastAdjacentCount = 1

        lastDigit = currDigit

    return twoAdjacentFound

start = 136818
end = 685979
okCount = 0
for pwd in range(start, end + 1):
    if testOnePassword(pwd):
        okCount += 1
print(okCount)
