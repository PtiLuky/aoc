import math

recips = {}

def decodeComponent(entry: str):
    entry = entry.strip().split(' ')
    return int(entry[0]), entry[1]

def findPrimaries(toProduce: dict):
    while not(len(toProduce) == 1 and 'ORE' in toProduce):
        print(toProduce)
        input()
        compo = list(toProduce)[0]
        if compo == 'ORE':
            compo = list(toProduce)[1]

        neededQuantity = toProduce.pop(compo)
        productionFactor = int(math.ceil(neededQuantity / recips[compo]['count']))

        for ingr in recips[compo]['ingr']:
            if ingr[1] not in toProduce:
                toProduce[ingr[1]] = 0
            toProduce[ingr[1]] += productionFactor * ingr[0]
    return toProduce

with open('ex1.txt') as f:
    for line in f:
        splitted = line.replace('\n', '').split('=>')
        productCount, product = decodeComponent(splitted[1])
        ingredients = []
        for ingred in splitted[0].split(','):
            ingredients.append(decodeComponent(ingred))
        recips[product] = {'count': productCount, 'ingr': ingredients}
        
    toProduce = {'FUEL' : 1}
    result = findPrimaries(toProduce)

    print(result)
