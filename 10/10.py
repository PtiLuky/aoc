import math

def angle(ast1, ast2):
    return math.atan2(ast1['y'] - ast2['y'], ast1['x'] - ast2['x'])

def distSq(ast1, ast2):
    return (ast1['x'] - ast2['x']) * (ast1['x'] - ast2['x']) + (ast1['y'] - ast2['y']) * (ast1['y'] - ast2['y'])

def loadMap(text):
    asteroids = []
    x = 0
    y = 0
    for line in text:
        x = 0
        for char in line:
            if char == '#':
                asteroids.append({'x':x, 'y':y, 'inView': {}})
            x += 1
        y += 1
    return asteroids

def editMap(text, x, y, newChar):
    new = list(text[y])
    new[x] = newChar
    text[y] = ''.join(new)

def listInView(watcher):
    watcher['inView'] = {}
    for other in asteroids:
        if watcher != other:
            a = angle(watcher, other)
            d = distSq(watcher, other)
            if a in watcher['inView']:
                if watcher['inView'][a]['d'] > d:
                    watcher['inView'][a] = {'d':d, 'x':other['x'], 'y':other['y']}
            else:
                watcher['inView'][a] = {'d':d, 'x':other['x'], 'y':other['y']}

def findBestAst(asteroids: list):
    maxInView = 0
    for ast in asteroids:
        listInView(ast)

        if len(ast['inView']) > maxInView:
            maxInView = len(ast['inView'])
            bestAst = ast

    return bestAst

def destroy(textMap, watcher, currAng):
    bestDAng = 1000000
    nextAngle = currAng
    for otherAng in watcher['inView']:
        dAng = (otherAng - currAng) % (2 * math.pi)
        if dAng < bestDAng and dAng != 0:
            bestDAng = dAng
            nextAngle = otherAng

    destroyed = watcher['inView'][nextAngle]
    editMap(textMap, destroyed['x'], destroyed['y'], '*')
    print(destroyed)

    return nextAngle

with open('input.txt') as f:
    textMap = f.read().split('\n')
    asteroids = loadMap(textMap)
    bestAst = findBestAst(asteroids)

    editMap(textMap, bestAst['x'], bestAst['y'], '0')
    for line in textMap:
        print(line)
    
    a = math.pi/ 2 - 0.000001
    for i in range(0, 200):
        a = destroy(textMap, bestAst, a)
        asteroids = loadMap(textMap)
        listInView(bestAst)
        for line in textMap:
            print(line)
