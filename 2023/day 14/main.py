def readinput()-> list[str]:
    with open('example.txt') as f:
     lines = f.readlines()
     return lines
def moveRocks(arr):
   for y in range(len(arr[0])):
      for x in range(len(arr)):
        if arr[x][y] == 'O' and x != 0:
           moveRock(x,y,arr,x)


   return arr.copy()


def moveRock(x, y,arr,rock):
   move = False
   moveTo = arr[x][y]
   if checkMove(arr[x-1][y]) and x != 0: 
        moveRock(x-1,y,arr,rock)
   else:
    arr[rock][y] = '.'
    arr[x][y] = 'O'

   
def checkMove(value)-> bool: 
    return value == '.'
def calculateScore(arr):
   score =  0
   for x in range(len(arr)):
      numberOfOs =  arr[x].count('O')
      score += (numberOfOs * (len(arr)-x))

   return score

inp = readinput()


lists =  [list(x.replace("\n","")) for x in inp]
arr =  lists
x = 1


while x < 20445:

        #north
        arr = moveRocks(arr)
       
        #west
        arr = [list(row) for row in zip(*arr[::-1])]
        arr = moveRocks(arr)
        arr = [list(row) for row in zip(*arr[::-1])]
        arr = [list(row) for row in zip(*arr[::-1])]
        arr = [list(row) for row in zip(*arr[::-1])]
        
        #print("south")
        arr = [list(row) for row in zip(*arr[::-1])]
        arr = [list(row) for row in zip(*arr[::-1])]
        arr = moveRocks(arr)
        arr = [list(row) for row in zip(*arr[::-1])]
        arr = [list(row) for row in zip(*arr[::-1])]
        #print("east")
        arr = [list(row) for row in zip(*arr[::-1])]
        arr = [list(row) for row in zip(*arr[::-1])]
        arr = [list(row) for row in zip(*arr[::-1])]
        arr = moveRocks(arr)
        arr = [list(row) for row in zip(*arr[::-1])]
        print("x: " + str(x))
        var = calculateScore(arr) 
        print(var)

        x = x +1



