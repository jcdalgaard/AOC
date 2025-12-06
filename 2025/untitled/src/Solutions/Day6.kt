package Solutions

import HelperFunctions.FileReader
import kotlin.time.measureTime

class Day6(path: String) {

    var list: List<List<String>> = FileReader().returnMap(path + "data"," ")
    var pureList: MutableList<String> = FileReader().returnContentAsList(path+"data") as MutableList

    fun run() {
        val runtimeChallenge1 = measureTime { println("Challenge 1: " + challenge1()) }
        println("Challenge 1 - Runtime: $runtimeChallenge1")
        val runtimeChallenge2 = measureTime { println("Challenge 2: " + challenge2()); }
        println("Challenge 2 - Runtime: $runtimeChallenge2")
    }
    fun operation(str: String, num1: Long, num2: Long): Long =
         when(str){
            "+" -> num1+num2
            "*" -> num1*num2
             else -> {num1+num2}
         }

    fun operation(str: String, lst: List<Long>): Long =
         when(str){
            "+" -> lst.sumOf { it }
            "*" -> lst.reduce  { acc, it -> acc * it }
            else -> {0}
        }


    fun challenge1(): Long {
        var index: Long = 0
        var len = list[0].size

        for(x in 0..len-1){

            var ops = list[list.size-1][x]
            var num : Long = 0

            for (y in 0..list.size-2){
                if (num == 0.toLong()) {num = list[y][x].toLong()}
                else { num =  operation(ops, num, list[y][x].toLong())}
            }
            index += num
        }

        return index
    }

    fun challenge2(): Long {
        var index: Long = 0
        var ll =  mutableListOf<Int>()
        var lastEle =  pureList.last()
        var count = 0
        for(x in 0..<lastEle.length){
            if (x==0)continue
            if (lastEle[x] !=' ') {
                ll.add(count)
                count = 0
            }
                else {count+=1}
        }
        ll.add(3)

        for( x in 0..ll.size-1){
            var o = pureList.last()[0].toString()
            var numbers = mutableListOf<Long>()
                for(z in 0..<ll[x]){
                    numbers.add((0..pureList.size - 2).joinToString("") { pureList[it][z].toString() }.trim().toLong())
                }
            if (x != ll.size - 1) (0 .. pureList.size-1).forEach { y -> pureList[y] = pureList[y].substring(ll[x] + 1) }
            index+= operation(o, numbers)
        }
        return index
    }
}