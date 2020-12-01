base = 0
cir = 0
inQueue = []
outQueue = []
entries = []

# Direct or indirect mode
def getIndex(cir: int, entries: list, paramIdx: int):
    tenPow = 100 * pow(10, paramIdx) # 00100 to check first param, 01000 to check second one...
    parameterMode = entries[cir] % (tenPow * 10) // tenPow
    if parameterMode == 2:
        return entries[cir + paramIdx + 1] + base
    elif parameterMode == 1:
        return cir + paramIdx + 1
    else:
        return  entries[cir + paramIdx + 1] 

# General binary operation
def doBinaryOpt(cir: int, entries: list, opt):
    p1, p2, p3 = getIndex(cir, entries, 0), getIndex(cir, entries, 1), getIndex(cir, entries, 2)
    entries[p3] = opt(entries[p1], entries[p2])
    return cir + 4

# 3 Input
def doInput(cir: int, entries: list):
    p1 = getIndex(cir, entries, 0)
    #inQueue.append(int(input()))
    entries[p1] = inQueue.pop(0)
    return cir + 2

# 4 Output
def doOutput(cir: int, entries: list):
    p1 = getIndex(cir, entries, 0)
    outQueue.append(entries[p1])
    #print(entries[p1])
    return cir + 2

# 5, 6 jump
def jumpIf(cir: int, entries: list, conditionToJump: bool):
    p1, p2 = getIndex(cir, entries, 0), getIndex(cir, entries, 1)
    return entries[p2] if bool(entries[p1]) == conditionToJump else cir + 3
    
# *
def setBase(cir: int, entries: list):
    global base
    p1 = getIndex(cir, entries, 0)
    base += entries[p1]
    return cir + 2

def execute():
    global cir
    outputBreak = 3
    while cir < len(entries):
        if entries[cir] % 100 == 99:
            return 1;
        elif entries[cir] % 100 == 1:
            cir = doBinaryOpt(cir, entries, (lambda x, y : x + y))
        elif entries[cir] % 100 == 2:
            cir = doBinaryOpt(cir, entries, (lambda x, y : x * y))
        elif entries[cir] % 100 == 3:
            cir = doInput(cir, entries)
        elif entries[cir] % 100 == 4:
            cir = doOutput(cir, entries)
            if len(outQueue) == outputBreak:
                return 0
        elif entries[cir] % 100 == 5:
            cir = jumpIf(cir, entries, True)
        elif entries[cir] % 100 == 6:
            cir = jumpIf(cir, entries, False)
        elif entries[cir] % 100 == 7:
            cir = doBinaryOpt(cir, entries, (lambda x, y : True if x < y else False))
        elif entries[cir] % 100 == 8:
            cir = doBinaryOpt(cir, entries, (lambda x, y : True if x == y else False))
        elif entries[cir] % 100 == 9:
            cir = setBase(cir, entries)
        else:
            print("Unknow instruction")
            return 1

def init():
    with open('input.txt') as f:
        entries[:] = [int(instr) for instr in f.readline().split(',')]
        entries.extend([0] * 10000)
        entries[0] = 2

def countBlocks(screen: dict):
    counter = 0
    for px in screen:
        if screen[px] == 2:
            counter += 1
    return counter

def printGame(screen: dict):
    xMax = yMax = 0
    for px in screen:
        if px[0] > xMax:
            xMax = px[0]
        if px[1] > yMax:
            yMax = px[1]
    for y in range(yMax + 1):
        line = ""
        for x in range(xMax + 1):
            if (x, y) not in screen:
                continue
            if screen[(x, y)] == 0:
                line += " "
            elif screen[(x, y)] == 1:
                line += "|"
            elif screen[(x, y)] == 2:
                line += "X"
            elif screen[(x, y)] == 3:
                line += "X"
            elif screen[(x, y)] == 4:
                line += "o"
        print(line)

def updateInput(paddlePos, ballPos):
    if paddlePos[0] < ballPos[0]:
        inQueue[:] = [1]
    elif paddlePos[0] > ballPos[0]:
        inQueue[:] = [-1]
    else:
        inQueue[:] = [0]

def play():
    init()
    screen = {}
    statusCode = 0
    score = 0
    nbBlocks = 1000000
    ballPos = (0, 0)
    paddlePos = (0, 0)
    while statusCode == 0 or nbBlocks == 0: 
        # clean output
        outQueue[:] = []
        # run
        statusCode = execute()

        # check error/stop
        if statusCode == 1: 
            break
        
        # display
        if (outQueue[0], outQueue[1]) == (-1, 0):
            score = outQueue[2] 
            print(score)
        else:
            screen[(outQueue[0], outQueue[1])] = outQueue[2]
        
        if outQueue[2] == 3:
            paddlePos = (outQueue[0], outQueue[1])
            updateInput(paddlePos, ballPos)
        elif outQueue[2] == 4:
            ballPos = (outQueue[0], outQueue[1])
            updateInput(paddlePos, ballPos)
        
        #printGame(screen)

        # test end
        nbBlocks = countBlocks(screen)


play()

