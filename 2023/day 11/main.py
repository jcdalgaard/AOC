def readinput()-> list[str]:
    with open('example.txt') as f:
     lines = f.readlines()
     return lines
    
def handleInput(data:list[str])->list[list[str]] and list[dict] and list[int] and list[int]:
    graph = []
    locations = []
    number = 1
    emptyrowsToReturn = []
    emptyColumnsToReturn = []
    setupGraph(data, graph, number, emptyrowsToReturn)   
    findEmptyColums(graph, emptyColumnsToReturn)
    findNodeLocations(graph, locations)
    return graph,locations,emptyColumnsToReturn,emptyrowsToReturn

def findNodeLocations(graph, locations):
    for x in range(len(graph)):
        for y in range(len(graph[x])):
            if graph[x][y]!='.':
                locations.append({'element':  graph[x][y], 'x': x, 'y': y})

def findEmptyColums(graph, emptyColumnsToReturn):
    for z in range(len(graph[0])):
       containsGalaxy = False
       for zz in range(len(graph)):
            containsGalaxy = True if graph[zz][z] != '.' else containsGalaxy               
       if not containsGalaxy:
        emptyColumnsToReturn.append(z)

def setupGraph(data, graph, number, emptyrowsToReturn):
    for x in range(len(data)):
      line = list(data[x].replace("\n",""))
      containsGalaxy = False
      for y in range(len(line)):
         if line[y] == "#":
            line[y] = number
            number += 1
            containsGalaxy = True
      if not containsGalaxy:
            emptyrowsToReturn.append(x)
      graph.append(line)

def calculateDistances(graph:list[list[str]], locations:list[dict],timesGreater:int, columns:list[int], rows:list[int]) -> list[dict]:
    distances = []
    for x in locations:
       for y in locations:
          distances.append(calculatedistance(x,y,timesGreater,columns,rows))
    return distances       
def calculatedistance(nodeOne:dict, nodeTwo:dict,timesGreater:int, columns:list[int], rows:list[int]):
    penalty = calculatePenalty(nodeOne['x'], nodeTwo['x'], timesGreater, rows)
    penalty += calculatePenalty(nodeOne['y'], nodeTwo['y'], timesGreater, columns)
    return {'from': nodeOne['element'],
            'to': nodeTwo['element'],
            'x': abs(nodeTwo['x']-nodeOne['x']),
            'y': abs(nodeTwo['y']-nodeOne['y']),
            'dis': abs(nodeTwo['x']-nodeOne['x']) + abs(nodeTwo['y']-nodeOne['y']) + penalty
            }

def calculatePenalty(nodeOne, nodeTwo, timesGreater, arr):
    p = 0
    for x in arr:
       if (nodeOne < x and x < nodeTwo) or (nodeOne > x and x > nodeTwo) :
          p +=timesGreater-1
    return p


graph,locations,columns, rows = handleInput(readinput())
distances = calculateDistances(graph,locations,1000000, columns, rows)
sum = 0
for x in distances: sum+= x['dis']

print(sum/2)

