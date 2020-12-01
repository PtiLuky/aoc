base = 0

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
	
# *
def setBase(cir: int, entries: list):
	global base
	p1 = getIndex(cir, entries, 0)
	base += entries[p1]
	return cir + 2

def execute():
	with open('input.txt') as f:
		cir = 0
		entries = [int(instr) for instr in f.readline().split(',')]
		entries.extend([0] * 10000)
		while cir < len(entries):
			if entries[cir] % 100 == 99:
				break;
			elif entries[cir] % 100 == 1:
				cir = doBinaryOpt(cir, entries, (lambda x, y : x + y))
			elif entries[cir] % 100 == 2:
				cir = doBinaryOpt(cir, entries, (lambda x, y : x * y))
			elif entries[cir] % 100 == 3:
				cir = doInput(cir, entries)
			elif entries[cir] % 100 == 4:
				cir = doOutput(cir, entries)
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
				break
		return entries[0]

execute()
