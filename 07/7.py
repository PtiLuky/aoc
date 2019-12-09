import itertools

# Direct or indirect mode
def getIndex(cir: int, entries: list, paramIdx: int):
    tenPowerToCheck = 100 * pow(10, paramIdx)   # 00100 to check first param, 01000 to check second one...
    return cir + paramIdx + 1 if entries[cir] % (tenPowerToCheck * 10) >= tenPowerToCheck else entries[cir + paramIdx + 1] 

# General binary operation
def doBinaryOpt(cir: int, entries: list, opt):
    p1, p2, p3 = getIndex(cir, entries, 0), getIndex(cir, entries, 1), getIndex(cir, entries, 2)
    entries[p3] = opt(entries[p1], entries[p2])
    return cir + 4

# 3 Input
def doInput(cir: int, entries: list):
    entries[getIndex(cir, entries, 0)] = int(input())
    return cir + 2
def doInput2(cir: int, entries: list, params: list):
    entries[getIndex(cir, entries, 0)] = params.pop(0)
    return cir + 2

# 4 Output
def doOutput(cir: int, entries: list):
    print(entries[getIndex(cir, entries, 0)])
    return cir + 2
def doOutput2(cir: int, entries: list, params: list):
    params.insert(1, entries[getIndex(cir, entries, 0)]) 
    return cir + 2

# 5, 6 jump
def jumpIf(cir: int, entries: list, conditionToJump: bool):
    p1, p2 = getIndex(cir, entries, 0), getIndex(cir, entries, 1)
    return entries[p2] if bool(entries[p1]) == conditionToJump else cir + 3

# One program, starting at instruction position cirsList[i], will update this value
def execute(entries: list, cirsList: list, params: list, idxOfAmp: int):
    cir     = cirsList[idxOfAmp]
    stopped = False
    while cir < len(entries):
        if entries[cir] % 100 == 99:
            stopped = True
            break
        elif entries[cir] % 100 == 1:
            cir = doBinaryOpt(cir, entries, (lambda x, y : x + y))
        elif entries[cir] % 100 == 2:
            cir = doBinaryOpt(cir, entries, (lambda x, y : x * y))
        elif entries[cir] % 100 == 3:
            cir = doInput2(cir, entries, params)
        elif entries[cir] % 100 == 4:
            cir = doOutput2(cir, entries, params)
            break
        elif entries[cir] % 100 == 5:
            cir = jumpIf(cir, entries, True)
        elif entries[cir] % 100 == 6:
            cir = jumpIf(cir, entries, False)
        elif entries[cir] % 100 == 7:
            cir = doBinaryOpt(cir, entries, (lambda x, y : True if x < y else False))
        elif entries[cir] % 100 == 8:
            cir = doBinaryOpt(cir, entries, (lambda x, y : True if x == y else False))
        else:
            print("Unknow instruction")
            stopped = True
            break
    cirsList[idxOfAmp] = cir
    return 1 if stopped else 0


def execAllAmp(params: list, firstVal: int, onlyOnce : bool = True):
    with open('input.txt') as f:
        entries  = [int(instr) for instr in f.readline().replace('\n', '').split(',')]       # original program, parsed
        programs = [entries, entries.copy(), entries.copy(), entries.copy(), entries.copy()] # copies of program, one per Amp
        cirs     = [0] * 5          # copies of cir, one per Amp
        retValue = -1               # return value to know when to stop
        inputs   = list(params)     # amp parameters
        inputs.insert(1, firstVal)  # write input value in input-value-list

        while retValue == -1 or (retValue == 0 and not onlyOnce):
            for i in range(5):
                retValue = execute(programs[i], cirs, inputs, i)
                
        return inputs[0]
    
def findMaxThrust(onlyOnce: bool):
    maxSignal = 0
    permuts = itertools.permutations(range(5)) if onlyOnce else itertools.permutations(range(5, 10))
    for permut in permuts:
        signal = execAllAmp(permut, 0, onlyOnce)
        if signal > maxSignal:
            maxSignal = signal
    return maxSignal
    
print(findMaxThrust(True))
