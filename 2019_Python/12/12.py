import copy

def getGravity1D(moon1, moon2, axis, velAxis):
    if moon1[axis] < moon2[axis]:
        moon1[velAxis] += 0.5
        moon2[velAxis] -= 0.5
    elif moon1[axis] > moon2[axis]:
        moon1[velAxis] -= 0.5
        moon2[velAxis] += 0.5

def getGravityMoons(moon1, moon2):
    getGravity1D(moon1, moon2, 'x', 'vx')
    getGravity1D(moon1, moon2, 'y', 'vy')
    getGravity1D(moon1, moon2, 'z', 'vz')

def applyGravitySystem(system):
    for m1 in system:
        for m2 in system:
            getGravityMoons(m1, m2)
    for m in system:
        m['x'] += m['vx']
        m['y'] += m['vy']
        m['z'] += m['vz']

def getPotentialEn(moon):
    return abs(moon['x']) + abs(moon['y']) + abs(moon['z'])
 
def getKineticEn(moon):
    return abs(moon['vx']) + abs(moon['vy']) + abs(moon['vz']) 

def getEn(moon):
    return getPotentialEn(moon) * getKineticEn(moon)

def getSystemEn(system):
    en = 0
    for m in system:
        en += getEn(m)
    return en

def isInHisto(histo, system):
    for old in histo:
        if isSame(old[0], system[0]) and isSame(old[1], system[1]) and isSame(old[2], system[2]) and isSame(old[3], system[3]):
            return True
    return False

def isSame(m1, m2):
    return m1['x'] == m2['x'] and m1['y'] == m2['y'] and m1['z'] == m2['z'] and m1['vx'] == m2['vx'] and m1['vy'] == m2['vy'] and m1['vz'] == m2['vz']


def runSim():
    history = []
    moons = [
        {'x':-7,  'y':-1,  'z':6,   'vx':0, 'vy':0, 'vz':0},
        {'x':6,   'y':-9,  'z':-9,  'vx':0, 'vy':0, 'vz':0},
        {'x':-12, 'y':2,   'z':-7,  'vx':0, 'vy':0, 'vz':0},
        {'x':4,   'y':-17, 'z':-12, 'vx':0, 'vy':0, 'vz':0}]

    for step in range(10000):
        applyGravitySystem(moons)
        print(toSimId(moons[0]))

        if isInHisto(history, moons):
            print("Found !", step)
            break

        stepHisto = []
        for moon in moons:
            stepHisto.append(copy.deepcopy(moon))
        history.append(stepHisto)
        

runSim()
