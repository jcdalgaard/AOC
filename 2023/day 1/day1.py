with open('example.txt') as f:
    lines = f.readlines()
result =  0
textNumbers =  ['one','two','three','four','five','six','seven','eight','nine']

def firstLetternumber(text):
    Minn = None
    minNumbertext = -1
    maxNumbertext = -1
    Maxx = None
    for z in textNumbers:
      num = text.find(z)
      if num != -1:
            if Minn == None:
              Minn = num, minNumbertext = textNumbers.index(z) +1 
     
            if Minn != min(Minn,num):   
                Minn = min(Minn,num)
                minNumbertext = textNumbers.index(z) +1 
      num = text.rfind(z)
      if num != -1:
          if Maxx == None:
              Maxx = num
              maxNumbertext = textNumbers.index(z) +1

          if Maxx != max(Maxx,num):   
             Maxx = max(Maxx,num)  
             maxNumbertext = textNumbers.index(z) +1
    if Minn == None or Maxx == None:
        return text
    s = text[:Minn] + str(minNumbertext) + text[Minn + 1:]
    l = s[:Maxx] + str(maxNumbertext) + s[Maxx + 1:]   
    return l
for z in range(len(lines)):
    lines[z] = firstLetternumber(lines[z])
for x in lines: 
    Minimum =None 
    Maximum = None
    for y in range(len(x)):
       if x[y].isdigit():
            if Minimum == None:
               Minimum = y
            if Maximum == None:
                Maximum = y
            Minimum = min(Minimum, y)
            Maximum = max(Maximum, y)
    result += int(str(x[Minimum])+str(x[Maximum]))
print(result)