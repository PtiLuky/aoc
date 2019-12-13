# global variables to keep track of values between execs (machine state)
base = 0
inputOutput = []
cirRef = [0]
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
    entries[p1] = inputOutput.pop(0)
    return cir + 2

# 4 Output
def doOutput(cir: int, entries: list):
    p1 = getIndex(cir, entries, 0)
    inputOutput.insert(0, (entries[p1]))
    return cir + 2

# 5, 6 jump
def jumpIf(cir: int, entries: list, conditionToJump: bool):
    p1, p2 = getIndex(cir, entries, 0), getIndex(cir, entries, 1)
    return entries[p2] if bool(entries[p1]) == conditionToJump else cir + 3
    
# 9
def setBase(cir: int, entries: list):
    global base
    p1 = getIndex(cir, entries, 0)
    base += entries[p1]
    return cir + 2

def execute():
    cir = cirRef[0]
    outputGenerated = 0
    while cir < len(entries):
        cirRef[0] = cir
        if outputGenerated == 2:
            return 0

        if entries[cir] % 100 == 99:
            return 1
        elif entries[cir] % 100 == 1:
            cir = doBinaryOpt(cir, entries, (lambda x, y : x + y))
        elif entries[cir] % 100 == 2:
            cir = doBinaryOpt(cir, entries, (lambda x, y : x * y))
        elif entries[cir] % 100 == 3:
            cir = doInput(cir, entries)
        elif entries[cir] % 100 == 4:
            cir = doOutput(cir, entries)
            outputGenerated += 1
        elif entries[cir] % 100 == 5:
            cir = jumpIf(cir, entries, True)
        elif entries[cir] % 100 == 6:
            cir = jumpIf(cir, entries, False)
        elif entries[cir] % 100 == 7:
            cir = doBinaryOpt(cir, entries, (lambda x, y : 1 if x < y else 0))
        elif entries[cir] % 100 == 8:
            cir = doBinaryOpt(cir, entries, (lambda x, y : 1 if x == y else 0))
        elif entries[cir] % 100 == 9:
            cir = setBase(cir, entries)
        else:
            print("Unknow instruction")
            return 1

def draw(pixels):
    xMin = xMax = yMin = yMax = 0
    for px in pixels:
        if px[0] < xMin:
            xMin = px[0]
        if px[0] > xMax:
            xMax = px[0]
        if px[1] < yMin:
            yMin = px[1]
        if px[1] > yMax:
            yMax = px[1]
    w = xMax - xMin + 1
    h = yMax - yMin + 1
    grid = [0] * (w * h)
    for px in pixels:
        grid[(px[0] - xMin) + (px[1] - yMin) * w] = pixels[px]
        if px[0] == 0 and px[1] == 0:
            grid[(px[0] - xMin) + (px[1] - yMin) * w] = -1
    for l in range(h):
        line = ''
        for c in range(w):
            line += ' ' if grid[c + l * w] == 0 else ('█' if grid[c + l * w] == 1 else '+')
        print(line)


def init():
    with open('input.txt') as f:
        global entries
        entries = [int(instr) for instr in f.readline().split(',')]
        entries.extend([0] * 10000)

def runRobot():
    global inputOutput
    init()
    pos = (0, 0)
    pixels = {(0, 0): 1}
    direction = 0 # 0 top, 1 right, 2 down, 3 left
    finished = 0
    while finished == 0:
        # read color
        color = 0
        if pos in pixels:
            color = pixels[pos]
        inputOutput = [color]

        # think
        finished = execute()

        if finished == 1:
            break

        # paint
        pixels[pos] = inputOutput[1]

        # move
        direction += inputOutput[0] * 2 - 1
        direction = direction % 4
        lpos = list(pos)
        if direction == 0:
            lpos[1] -= 1
        elif direction == 1:
            lpos[0] += 1
        elif direction == 2:
            lpos[1] += 1
        elif direction == 3:
            lpos[0] -= 1
        pos = tuple(lpos)

    draw(pixels)
    print(len(pixels))

runRobot()
