package Solutions

import HelperFunctions.FileReader
import kotlin.time.measureTime

class Day4(path: String) {

    var list = FileReader().returnMap(path + "data") as MutableList<MutableList<String>>
    var coordinatesToBeRemoved = mutableListOf<Pair<Int,Int>>()
    val relativeCoordinates = listOf<Pair<Int,Int>>(Pair(-1,0),Pair(1,0),Pair(1,1),Pair(-1,-1),Pair(-1,1), Pair(1,-1), Pair(0,1), Pair(0,-1))

    fun run() {
        val runtimeChallenge1 = measureTime { println("Challenge 1: " + challenge1()) }
        println("Challenge 1 - Runtime: $runtimeChallenge1")
        val runtimeChallenge2 = measureTime { println("Challenge 2: " + challenge2()); }
        println("Challenge 2 - Runtime: $runtimeChallenge2")
    }

    fun challenge1(): Int {
        (0..list.size-1).forEach { x -> (0..list[x].size-1).forEach { y->
        if(list[x][y]=="@" &&checkSurrounding(Pair(x,y))<4){
                            coordinatesToBeRemoved.add(Pair(x, y))
                }
            }
        }
        return coordinatesToBeRemoved.size
    }

    fun checkSurrounding(coordinate: Pair<Int, Int>): Int{
        var count = 0
        for(x in relativeCoordinates){
            try {
                if (list[coordinate.first+x.first][coordinate.second+x.second] == "@") {
                    count++
                }
            } catch (e: Exception){
                continue
            }

        }
        return count
    }
    fun challenge2(): Int {
        coordinatesToBeRemoved.clear()
        var i = 0
       while(challenge1()>0) {
                i+=coordinatesToBeRemoved.size
                coordinatesToBeRemoved.forEach { x -> list[x.first][x.second] ="x" }
                coordinatesToBeRemoved.clear()
            i
        }
        return i
    }
}