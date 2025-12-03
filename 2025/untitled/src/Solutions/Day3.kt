package Solutions

import HelperFunctions.FileReader
import java.math.BigInteger
import kotlin.time.measureTime

class Day3(path: String) {

    var list: List<String> = FileReader().returnContentAsList(path + "data")

    fun run() {
        val runtimeChallenge1 = measureTime { println("Challenge 1: " + challenge1()) }
        println("Challenge 1 - Runtime: $runtimeChallenge1")
        val runtimeChallenge2 = measureTime { println("Challenge 2: " + challenge2()); }
        println("Challenge 2 - Runtime: $runtimeChallenge2")
    }

    fun challenge1(): Long {
        return list.sumOf { x -> removeNumbers(x,2) }
    }

    fun removeNumbers(element: String, len:Int):Long{
        var newele = element
        while (newele.length>len){
            var numbers =  mutableMapOf<BigInteger, Int>()
            for(x in 0..newele.length-1){
                numbers[BigInteger(newele.removeRange(x,x+1))] = x
            }
            var index: Int = numbers[numbers.keys.max()]!!
            newele = newele.removeRange(index, index+1)
        }
        return newele.toLong()
    }

    fun challenge2(): Long {
        return list.sumOf { x -> removeNumbers(x,12) }
    }
}