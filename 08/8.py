w = 25
h = 6

def getWeirdProduct(entries: list):
    nbLayers = int(len(entries) / (w * h ))
    zerosCount = w * h
    weirdProduct = 0
    for layer in range(nbLayers):
        nbZeros = 0
        nbOnes  = 0
        nbTwos  = 0
        for idx in range(w * h):
            if entries[idx + layer * w * h] == 0:
                nbZeros += 1
            elif entries[idx + layer * w * h] == 1:
                nbOnes += 1
            elif entries[idx + layer * w * h] == 2:
                nbTwos += 1
        if nbZeros < zerosCount:
            zerosCount = nbZeros
            weirdProduct = nbOnes * nbTwos
    print(zerosCount, weirdProduct)

with open('input.txt') as f:
    entries  = [int(pix) for pix in f.readline().replace('\n', '')]
    finalImage = [2] * (w * h)
    nbLayers = int(len(entries) / (w * h ))
    for pxId in range(len(entries)):
        if finalImage[pxId % (w * h)] == 2:
            finalImage[pxId % (w * h)] = entries[pxId]
    message = ""
    for y in range(h):
        for x in range(w):
            message += "X" if finalImage[x + w * y] else " "
        message += "\n"
    print(message)
    
    
        
    
    
