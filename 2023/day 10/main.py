from collections import deque

def readinput()-> list[str]:
    with open('example.txt') as f:
     lines = f.readlines()
     return lines

    
class Graph:
    def __init__(self):
        self.graph = {}

    def add_edge(self, u, v):
        if u not in self.graph:
            self.graph[u] = []
        if v not in self.graph:
            self.graph[v] = []
        
        self.graph[u].append(v)
        self.graph[v].append(u)

    def bfs(self, start):
        visited = {node: False for node in self.graph}
        distance = {node: float('inf') for node in self.graph}
        queue = deque([start])

        visited[start] = True
        distance[start] = 0

        while queue:
            current = queue.popleft()
            
            for neighbour in self.graph[current]:
                if not visited[neighbour]:
                    queue.append(neighbour)
                    visited[neighbour] = True
                    distance[neighbour] = distance[current] + 1

        return distance


def readInGraph(data, graph:Graph):
    start = None
    locations = []
    locations2 =[]
    for z in range(len(data)):
        data[z] = data[z].replace("\n","")
        locations.append(list(data[z]))
        locations2.append(list(data[z]))
    i = 0
    for row in range(len(data)):
        for column in range(len(data[row])):
            locations[row][column] = i
            locations2[row][column] = "."
            i+=1
    startingpoint = None
    for row in range(len(data)):
        for column in range(len(data[row])):
            c = data[row][column]
            if c == 'S':
                startingpoint = {'row':row, 'column':column}
    locations2[startingpoint['row']][startingpoint['column']] = "X"
    visited= set()
    row = startingpoint['row']
    column = startingpoint['column']-1
    test = data[row][column]
   
    start = locations[startingpoint['row']][startingpoint['column']]
    graph.add_edge(locations[row][column], locations[startingpoint['row']][startingpoint['column']])
    visited.add(locations[startingpoint['row']][startingpoint['column']])
    nexto = {'row' : row ,'column': column}
    while locations[nexto['row']][nexto['column']] not in visited:
        locations2[nexto['row']][nexto['column']] = "X"
        visited.add(locations[nexto['row']][nexto['column']])
        nexto = CharDef(graph,locations,nexto['row'],nexto['column'], data[nexto['row']][nexto['column']],visited)
    
    with open('output.txt', 'w') as file:
        for element in locations2:
            file.write(str(element) + '\n')  
    return start

def CharDef(graph, locations, row, column, c, visited):
    edge1 =  {'row' : row ,'column': column}
    edge2 =  {'row' : row ,'column': column}

    if c == '|':
            edge1['row'] = row+1
     
            edge2['row'] = row-1
    elif c == '-':
            edge1['column'] = column+1

            edge2['column'] = column-1
    elif c == 'L':

            edge1['row'] = row-1
         
            edge2['column'] = column+1

    elif c == 'J':
            edge1['row'] = row-1

            edge2['column'] = column-1

    elif c == '7':
            edge1['row'] = row+1

            edge2['column'] = column-1

    elif c == 'F':
            edge1['row'] = row+1

            edge2['column'] = column+1

    elif c == '.':
        None
    elif c == 'S':
      start = locations[row][column]

    if locations[edge1['row']][edge1['column']] in visited:
        graph.add_edge(locations[row][column],locations[edge2['row']][edge2['column']])
        return edge2
    else:
        graph.add_edge(locations[row][column],locations[edge1['row']][edge1['column']])
        return edge1
            
g = Graph()
start = readInGraph(readinput(),g)
distances = g.bfs(start)

max_value = max(value for value in distances.values() if value != float('inf'))

print("Maximum value", max_value)
b = 1
