# Direct or indirect mode
def getIndex(cir: int, entries: list, paramIdx: int):
    tenPowerToCheck = 100 * pow(10, paramIdx) # 00100 to check first param, 01000 to check second one...
    return cir + paramIdx + 1 if entries[cir] % (tenPowerToCheck * 10) >= tenPowerToCheck else entries[cir + paramIdx + 1] 

# 1 Add
def doAdd(cir: int, entries: list):
    p1, p2, p3 = getIndex(cir, entries, 0), getIndex(cir, entries, 1), getIndex(cir, entries, 2)
    entries[p3] = entries[p1] + entries[p2]
    return cir + 4

# 2 Mult
def doMult(cir: int, entries: list):
    p1, p2, p3 = getIndex(cir, entries, 0), getIndex(cir, entries, 1), getIndex(cir, entries, 2)
    entries[p3] = entries[p1] * entries[p2]
    return cir + 4

# 3 Input
def doInput(cir: int, entries: list):
    p1 = getIndex(cir, entries, 0)
    entries[p1] = int(input())
    return cir + 2

# 4 Output
def doOutput(cir: int, entries: list):
    p1 = getIndex(cir, entries, 0)
    print(entries[p1])
    return cir + 2

# 5, 6 jump
def jumpIf(cir: int, entries: list, conditionToJump: bool):
    p1, p2 = getIndex(cir, entries, 0), getIndex(cir, entries, 1)
    return entries[p2] if bool(entries[p1]) == conditionToJump else cir + 3

# 7 less than
def lessThan(cir: int, entries: list):
    p1, p2, p3 = getIndex(cir, entries, 0), getIndex(cir, entries, 1), getIndex(cir, entries, 2)
    entries[p3] = 1 if entries[p1] < entries[p2] else 0
    return cir + 4

# 8 equals
def equals(cir: int, entries: list):
    p1, p2, p3 = getIndex(cir, entries, 0), getIndex(cir, entries, 1), getIndex(cir, entries, 2)
    entries[p3] = 1 if entries[p1] == entries[p2] else 0
    return cir + 4

def execute():
    with open('input2.txt') as f:
        cir = 0
        entries = [int(instr) for instr in f.readline().split(',')]
        while cir < len(entries):
            if entries[cir] % 100 == 99:
                break;
            elif entries[cir] % 100 == 1:
                cir = doAdd(cir, entries)
            elif entries[cir] % 100 == 2:
                cir = doMult(cir, entries)
            elif entries[cir] % 100 == 3:
                cir = doInput(cir, entries)
            elif entries[cir] % 100 == 4:
                cir = doOutput(cir, entries)
            elif entries[cir] % 100 == 5:
                cir = jumpIf(cir, entries, True)
            elif entries[cir] % 100 == 6:
                cir = jumpIf(cir, entries, False)
            elif entries[cir] % 100 == 7:
                cir = lessThan(cir, entries)
            elif entries[cir] % 100 == 8:
                cir = equals(cir, entries)
            else:
                print("Unknow instruction")
                break
        return entries[0]

execute()
