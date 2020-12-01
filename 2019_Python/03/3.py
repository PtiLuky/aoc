minX = 0
minY = 0
maxX = 0
maxY = 0

width = 1;
height = 1;
arr = []

def toCoor(x, y):
    return (x - minX) + (y - minY) * width

def fillWithPath(directions: str):
    x = 0
    y = 0
    cost = 0

    for direction in directions.replace(' ','').split(','):
        dx = 0
        dy = 0
        steps = int(direction[1:])
        if (direction[0] == 'R'):
            dx = 1
        if (direction[0] == 'L'):
            dx = -1
        if (direction[0] == 'U'):
            dy = 1
        if (direction[0] == 'D'):
            dy = -1

        for ministep in range(steps):
            x += dx
            y += dy
            cost += 1
            idx = toCoor(x, y)
            if arr[idx] == 0:
                arr[idx] = cost


def lowestIntersection(directions: str):
    bestX = 0
    bestY = 0
    bestGlobalCost = 999999999999
    x = 0
    y = 0
    cost = 0

    for direction in directions.replace(' ','').split(','):
        dx = 0
        dy = 0
        steps = int(direction[1:])
        if (direction[0] == 'R'):
            dx = 1
        if (direction[0] == 'L'):
            dx = -1
        if (direction[0] == 'U'):
            dy = 1
        if (direction[0] == 'D'):
            dy = -1

        for ministep in range(steps):
            x += dx
            y += dy
            cost += 1
            idx = toCoor(x, y)
            if arr[idx] != 0 and arr[idx] + cost < bestGlobalCost:
                bestGlobalCost = arr[idx] + cost
                bestX = x
                bestY = y
    
    return bestGlobalCost


def expandWith(directions: str):
    global minX
    global minY
    global maxX
    global maxY

    x = 0
    y = 0

    for direction in directions.replace(' ','').split(','):
        steps = int(direction[1:])
        if (direction[0] == 'R'):
            x += steps
        if (direction[0] == 'L'):
            x -= steps
        if (direction[0] == 'U'):
            y += steps
        if (direction[0] == 'D'):
            y -= steps

        if (x < minX):
            minX = x
        if (y < minY):
            minY = y

        if (x > maxX):
            maxX = x
        if (y > maxY):
            maxY = y


with open('input.txt') as f:
    line1 = f.readline()
    line2 = f.readline()
    expandWith(line1)
    expandWith(line2)
    width = maxX - minX +1
    height = maxY - minY +1
    arr = [0] * width * height
    fillWithPath(line1)
    print(lowestIntersection(line2))
    
