
def readinput()-> list[str]:
    with open('example.txt') as f:
     lines = f.readlines()
     return lines
    
def createPyramid(line:list[int])->list[list[int]]:
   liste = []
   liste.append(line)
   newList = []
   last = True
   for x in range(len(line)-1):
      v = line[x+1]-line[x]
      newList.append(v)
      if v!=0: last = False
   if not last:
      liste =  liste + createPyramid(newList)
   else:
      liste.append(newList)
      
   
   return liste
def getnumbers(data:list[str]) -> int:
   sum = 0
   for x in data:
      d = [int(item) for item in x.replace("\n","").split(" ")]
      d.reverse() ## Part 2...
      pyramid =  createPyramid(d)
      pyramid.reverse()
      for x in range(len(pyramid)):
         if x == 0:
            pyramid[x].append(0)
         else:
            pyramid[x].append(pyramid[x][-1]+pyramid[x-1][-1])

      sum+= pyramid[-1][-1]
   return sum
data = readinput()

print(getnumbers(data))