package TwentyTwenty

import HelperFunctions.FileReader
import kotlin.time.measureTime

class Day8(path: String) {

    var l: List<String> = FileReader().returnContentAsList(path + "data")


    fun run() {
        val runtimeChallenge1 = measureTime { println("Challenge 1: " + challenge1()) }
        println("Challenge 1 - Runtime: $runtimeChallenge1")
        val runtimeChallenge2 = measureTime { println("Challenge 2: " + challenge2()); }
        println("Challenge 2 - Runtime: $runtimeChallenge2")
    }

    fun challenge1(): Int {

        return runner(l).first
    }

    fun runner(list:List<String>):Pair<Int, Int>{
        val run = true
        var index = 0
        var acc = 0
        val cache =  mutableMapOf<Int,String>()
        try {


        while(run){
            if(cache[index]!=null){
                throw Exception("Error")
            }
            cache[index] = ""

            var ope =  list[index].split(" ")
            var operation = ope[0]
            if (operation=="nop"){
                index++
                continue;
            } else if (operation=="acc") {
                var argument =  if (ope[1].trim()[0]=='+') 0+(ope[1].trim().substring(1).toInt()) else 0-(ope[1].trim().substring(1).toInt())
                acc+= argument
                index++
            }
            else if (operation=="jmp"){
                var argument =  if (ope[1].trim()[0]=='+') 0+(ope[1].trim().substring(1).toInt()) else 0-(ope[1].trim().substring(1).toInt())

                var temp = if (index+argument>=list.size) index+argument % list.size else if (index +argument <0) list.size - index+(argument % list.size) else index+argument
                index = temp
            }
        if (index==list.size){
            println("Wow...")
            return Pair(acc, index)
        }
        }

    } catch (e: Exception) {
        return Pair(acc, index)
    }
        return TODO("Provide the return value")
    }

    fun challenge2(): Int {

        for (x in 0..l.size-1){

            val newList = mutableListOf<String>()
            newList.addAll(l)
            if (newList[x].contains("nop")){
                newList[x] = newList[x].replace("nop","jmp")

                var v =  runner(newList)
                if(v.second==l.size){
                    return v.first
                }
            } else if (newList[x].contains("jmp")){

                newList[x] = newList[x].replace("jmp","nop")
                var v = runner(newList)
                if(v.second==l.size){
                 return v.first
                }
            } else {
                continue
            }



        }


        return 1
    }
}