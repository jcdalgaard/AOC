import sys
import os

filename = "6.December/bf.txt"
	# for arg in sys.argv:
	# 	if arg.endswith('.'+suffix):
	# 		filename = arg

if(filename != None):
	if os.path.exists(filename):
		with open(filename, 'r') as f:
			    inf = f.read()
              

            

                


def convert(x):
    valuee = ""
    if x == "[":
        valuee ="ooo"
    elif x == "]":
        valuee ="wee"
    elif x == ">":
        valuee ="eeo"
    elif x == "<":
        valuee ="oee"
    elif x == "+":
        valuee ="oow"
    elif x=="-":
        valuee ="woo"
    elif x==".":
        valuee ="eee"
    elif x==",":
            valuee ="eew"
    else:
        return "Error"
    return valuee

counter = 0 
holder = []
n = len(inf)
for x in inf:
        if counter %8 == 0 and counter !=0:
            print((" ").join(holder))
            holder = []    
        holder.append(convert(x))
        counter += 1
print((" ").join(holder))