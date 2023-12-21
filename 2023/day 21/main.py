from collections import deque

def readinput()-> list[str]:
    with open('example.txt') as f:
     lines = f.readlines()
     return lines
    
class Graph:
    def __init__(self):
        self.graph = {}
        self.ends =  set()
        self.visited = {}

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

        return distance,  
    def findNodesAtDistance(self, start, x):
        if x in self.visited.get(start,[]):
            return
        else:
            self.visited[start] = [x] + (self.visited.get(start,[]))
        val = x
        val +=-1
        if val == 0:
            self.ends.add(start)
            return
        else:
            edges = set(self.graph[start])
            for edge in edges:
                self.findNodesAtDistance(edge,val)   

    def getPossibleNodes(self, start, x):
        self.findNodesAtDistance(start, x)

        return list(self.ends)   

def readInGraph(data, graph:Graph):
    start = None
    locations = []
    for z in range(len(data)):
        data[z] = data[z].replace("\n","")
        locations.append(list(data[z]))
    i = 0
    for row in range(len(data)):
        for column in range(len(data[row])):
            locations[row][column] = i
            i+=1
    startingpoint = None
    for row in range(len(data)):
        for column in range(len(data[row])):
            c = data[row][column]
            if c == 'S':
                startingpoint = {'row':row, 'column':column}
            if c != '#':    
                try:
                    if data[row+1][column] == '.' or data[row+1][column] == 'S':
                        graph.add_edge(locations[row][column], locations[row+1][column])
                except:
                    None
                try:
                    if data[row-1][column] == '.' or data[row-1][column] == 'S':
                        graph.add_edge(locations[row][column], locations[row-1][column])
                except:
                    None
                try:
                    if data[row][column+1] == '.' or data[row][column+1] == 'S':
                        graph.add_edge(locations[row][column], locations[row][column+1])
                except:
                    None
                try:
                    if data[row][column-1] == '.' or data[row][column-1] == 'S':
                        graph.add_edge(locations[row][column], locations[row][column-1])
                except:
                    None                          
    start = locations[startingpoint['row']][startingpoint['column']]
   
    return start, locations, data


            
g = Graph()
start, loc, t = readInGraph(readinput(),g)
d = []
for x in t:
    d.append(list(x))
distances = g.getPossibleNodes(start,65)
for x in range(len(loc)):
    for y in range(len(loc[x])):
        temp = loc[x][y]
        if temp in distances:
            d[x][y] = 'O'
        elif loc[x][y] == start:
            d[x][y] = 'S'
           
for x in d:
    print("".join(x))
count = len(distances)


print("Gardens: ", count)
