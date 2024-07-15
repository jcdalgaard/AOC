package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)


type Stack []string

// IsEmpty: check if stack is empty
func (s *Stack) IsEmpty() bool {
	return len(*s) == 0
}

// Push a new value onto the stack
func (s *Stack) Push(str string) {
	*s = append(*s, str) // Simply append the new value to the end of the stack
}

// Remove and return top element of stack. Return false if stack is empty.
func (s *Stack) Pop() (string, bool) {
	if s.IsEmpty() {
		return "", false
	} else {
		index := len(*s) - 1 // Get the index of the top most element.
		element := (*s)[index] // Index into the slice and obtain the element.
		*s = (*s)[:index] // Remove it from the stack by slicing it off.
		return element, true
	}
}

func fillStack() (s Stack, str string)   {

   if str != ""{
      s.Push(str)
   }
}

func main() {
   // get file from terminal
   inputFile := "t.txt"
   // read the whole content of file and pass it to file variable, in case of error pass it to err variable
   file, err := ioutil.ReadFile(inputFile)
   if err != nil {
      fmt.Printf("Could not read the file due to this %s error \n", err)
   }
     var n int
     n=9
   var stackHolder[9]Stack
   for i:=0; i<n; i++ {
      
      stackHolder[i] = Stack{}
   }
   
   //s := {"H","B","V",}




   

   
   

   // convert the file binary into a string using string
   fileContent := string(file)
   // print file content

   s := strings.Split(fileContent, "\n")

   counter := 0
   reader := true
   for reader {

      

      if s[counter] == "\r" {
         reader = false
         
      }else 
      {
        fillStack()

      }


      counter +=1



   }

   fmt.Println(s)
}