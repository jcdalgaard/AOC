def readinput()-> list[str]:
    with open('example.txt') as f:
     lines = f.readlines()
     return lines

def handleInput(data):
    listResults = []
    listt = []
    smudgedlistResults = []
    for x in range(len(data)):
        data[x] = data[x].replace('\n','')
        if len(data[x]) == 0 or x == len(data): 
           listResults.append(findMirrors(listt,0))
           smudgedlistResults.append(findMirrors(listt,1))
           listt.clear()
        else:
           listt.append(list(data[x]))
    listResults.append(findMirrors(listt,0))
    smudgedlistResults.append(findMirrors(listt,1))
    listt.clear()
    return listResults,smudgedlistResults

def findMirrors(pat, diff, flipped =  False):
    pattern = pat.copy()
    for i in range(1,len(pattern)):
       arr1 =pattern[:i]
       arr2 = pattern[i:]
       if len(arr1) < len(arr2):
          arr2 =  arr2[:len(arr1)]
       elif len(arr2) < len(arr1):
          arr1 = arr1[-len(arr2):]
       arr2.reverse()
       errors =[]
       for x in range(len(arr1)):
         if len(errors)  > 1: break

         if arr1[x] != arr2[x]:
            for y in range(len(arr1[x])):
               if arr1[x][y]!=arr2[x][y]:
                  errors.append([x,y])
                  if len(errors) >1:break 

       if len(errors) == diff:
              return {
      'pattern' : pattern.copy(),
      'row': i,
      'flipped': flipped
   }
       
    transposed = [list(row) for row in zip(*pattern[::-1])]
    return findMirrors(transposed, diff, True)

def calculateAssignmentone(results):
   total = 0
   for x in results:
      multiplier =  100 if not x['flipped'] else 1
      total+= int(x['row'])*multiplier
   return total


listresults,smudgeResults = handleInput(readinput())
print(calculateAssignmentone(listresults))
print(calculateAssignmentone(smudgeResults))