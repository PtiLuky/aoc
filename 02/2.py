def doAdd(cir: int, entries: list):
    entries[entries[cir + 3]] = entries[entries[cir + 1]] + entries[entries[cir + 2]]
    return cir + 4

def doMult(cir: int, entries: list):
    entries[entries[cir + 3]] = entries[entries[cir + 1]] * entries[entries[cir + 2]]
    return cir + 4

def execute(noun:int, verb:int):
    with open('input.txt') as f:
        cir = 0
        entries = [int(instr) for instr in f.read().replace('\n','').split(',')]
        entries[1] = noun
        entries[2] = verb
        while cir < len(entries):
            if entries[cir] == 99:
                break;
            elif entries[cir] == 1:
                cir = doAdd(cir, entries)
            elif entries[cir] == 2:
                cir = doMult(cir, entries)
        return entries[0]

# Part 1
#execute(12, 2)

# Part 2
for n in range(100):
    for v in range(100):
        if execute(n, v) == 19690720:
            print(n, v)
