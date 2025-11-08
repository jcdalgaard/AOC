package TwentyTwenty

import HelperFunctions.FileReader
import kotlin.time.measureTime

class Day3(path: String) {

    var list: List<List<String>> = FileReader().returnMap(path + "data")

    fun run() {
        val runtimeChallenge1 = measureTime {   println("Challenge 1: " + challenge1(Pair(3,1)))}
        println("Challenge 1 - Runtime: $runtimeChallenge1")
        val runtimeChallenge2 = measureTime {   println("Challenge 2: " + challenge2());}
        println("Challenge 2 - Runtime: $runtimeChallenge2")
    }

    fun challenge1(slope: Pair<Int, Int>): Int {
        var counter = 0;    
        var indexX =  0;
        var indexY = 0;

        while (indexY < list.size-1){
            indexX = if (indexX+slope.first >= list[0].size)  indexX+slope.first -list[0].size else indexX +slope.first
            indexY += slope.second
            if (list[indexY][indexX] == "#"){
                counter++
            }
        }
        return counter

    }
    fun challenge2(): Int {
        var counter = 1;
        val slopes = listOf( Pair(1,1), Pair(3,1), Pair(5,1), Pair(7,1), Pair(1,2))
        slopes.forEach { x -> counter *= challenge1(x) }

        return counter

    }
}