#!/usr/bin/env python3
import sys
import os

def nextword(mode,line):
	if mode == "t":
		return input(">> ")
	elif mode == "f" :
		global words
		global number
		if line<len(words):
			number = line
			return words[line]
		else:
			return "exit"	

def interp(word,mode,line):
	global keywords
	global array
	global ptr
	global words

	oline = line

	while word != "exit":
		if (ptr == 21 or ptr == 37 or array[ptr]==-1):
			hola = "ola?"
		if(mode == "f" and line > len(words)):
			return
	
		word = nextword(mode,line)					
		if word == "ooo":
			line+=1
			while array[ptr] != 0:
				inc = interp(nextword(mode,line),mode,line)
			line+=inc		
		elif word=="wee":
			if number == 383:
				print("Time to die.....")
			return line-oline
		elif word=="eeo":
			ptr+=1
		elif word=="oee":
			ptr-=1
		elif word=="oow":
			array[ptr]+=1
		elif word=="woo":
			array[ptr]-=1
		elif word=="eee":
			print(chr(array[ptr]),end="")
		elif word=="eew":
			array[ptr] = ord(sys.stdin.read(1))	
		elif word == "exit":
			exit(0)
		elif word == None:
			a=0
		else:
			print('Ru Retarded ?\nWhy is there a "%s" in ur code.'% word)	
			sys.exit(0)
		
		line+=1

		if(mode == "t"):
			print(word+" : "+"{",end=" ")
			for x in range(0,10):
				if x==ptr:
					print("\033[4m"	+ str(array[x]) + "\033[0m",end=" ")
				else:
					print(array[x],end=" ")
			print("}")

	

suffix = "Wee"
array = [0]*30000
ptr = 0
words = []
number = 0

if __name__ == '__main__':

	filename = "6.December/t.txt"
	# for arg in sys.argv:
	# 	if arg.endswith('.'+suffix):
	# 		filename = arg

	if(filename != None):
		if os.path.exists(filename):
			with open(filename, 'r') as f:
				inf = f.read()
		else:
			print("WHERE IS D FILE !!\n")
			exit(0)
				
		words = inf.split()
		interp(words[0],"f",0)

	else:
		print("WHERE IS D FILE !!\nAlso Where R my Testicles")
		exit(0)