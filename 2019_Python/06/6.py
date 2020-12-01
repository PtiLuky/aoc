def fillWithCount(graph: dict, parent: str):
    for child in graph[parent]['children']:
        graph[child]['orbCount'] = graph[parent]['orbCount'] + 1
        fillWithCount(graph, child)

def count(graph: dict):
    total = 0
    for node in graph:
        total += graph[node]['orbCount']
    return total

def pathToCenter(graph: dict, node: str):
    path = ['COM']
    while node != 'COM':
        path.insert(1, node)
        node = graph[node]['parent']
    return path

def commonLength(path1: list, path2: list):
    i = 0
    while i < min(len(path1), len(path2)) and path1[i] == path2[i]:
        i+=1
    return i
   
def exec():
    with open('input.txt') as f:
        graph = {} # {'node' : {'parent':'PARENT', 'children':[], 'orbCount':-1}}
        for line in f:
            splitted = line.replace('\n', '').split(')')
            if len(splitted) == 2:    

                if not splitted[0] in graph:
                    graph[splitted[0]] = {'parent': '', 'children':[], 'orbCount':0}
                if not splitted[1] in graph:
                    graph[splitted[1]] = {'parent': '', 'children':[], 'orbCount':0}

                graph[splitted[1]]['parent'] = splitted[0]
                graph[splitted[0]]['children'].append(splitted[1])

        fillWithCount(graph, 'COM')

        myPath = pathToCenter(graph, 'YOU')
        hisPath = pathToCenter(graph, 'SAN')
        pathLength = len(myPath) +  len(hisPath) - 2 * commonLength(myPath, hisPath) - 2
        print(pathLength)

exec()
                
                
