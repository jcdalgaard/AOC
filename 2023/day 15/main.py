def readinput()-> list[str]:
    with open('example.txt') as f:
     lines = f.readlines()
     return lines

def GetAsciiValue(x):
    return ord(x)
def Multiply(x, multiplier =17):
   return x* multiplier
def GetRemainder(x, divisor=256):
   return x%divisor

def HandleValue(value, char):
   value+= GetAsciiValue(char)
   value =  Multiply(value)
   value = GetRemainder(value)
   return value

lines = readinput()

string =  ""
for x in lines:
   string += x

arr  =  string.split(",")
vals = []
total = 0
for x in arr:
    currentValues = 0
    for y in x:
      currentValues = HandleValue(currentValues, y)
    vals.append(currentValues)
    total +=currentValues

print(total)
queues = []
for x in range(256): queues.append({})


for x in arr:
    currentValues = 0
    letters = ""
    minus = False
    focalStrength = 0
    if '-' in x:
       letters =  x[:-1]
       minus = True
    elif '=' in x:
       letters = x[:-2]
       minus = False
       focalStrength = int(x[-1])
    for y in letters:
         currentValues = HandleValue(currentValues, y)
    if minus:
      queues[currentValues].pop(letters,"No Value")
    elif not minus:
     queues[currentValues][letters] = focalStrength
val = 0
for x in range(len(queues)):
   c = 0
   i = 1
   for key, value in queues[x].items():
      c+=(x+1)*i*value
      i+=1
   val+=c   
print(val)