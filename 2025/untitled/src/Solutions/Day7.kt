package Solutions

import HelperFunctions.FileReader
import kotlin.time.measureTime

class Day7(path: String) {

    var list: MutableList<MutableList<String>> = FileReader().returnMap(path + "data") as  MutableList<MutableList<String>>
    var graph =  mutableMapOf<Pair<Long,Long>, Long>()

    fun run() {
        val runtimeChallenge1 = measureTime { println("Challenge 1: " + challenge1()) }
        println("Challenge 1 - Runtime: $runtimeChallenge1")
        val runtimeChallenge2 = measureTime { println("Challenge 2: " + challenge2()); }
        println("Challenge 2 - Runtime: $runtimeChallenge2")
    }

    fun challenge1(): Long {
        var index = 0.toLong()

        for (x in 0..list.size-2){
            for(y in 0..<list[x].size){
                if (list[x][y] ==  "S" ){
                    list[x+1][y] = "|"
                    graph[Pair(x+1.toLong(),y.toLong())] =1
                } else if (list[x][y] ==  "|" ){

                    var t  =  graph[Pair(x.toLong(),y.toLong())] ?: 0.toLong()

                    if (list[x+1][y] == "^"){
                        var l = graph[Pair(x+1.toLong(),y-1.toLong())] ?: 0.toLong()
                        var u = graph[Pair(x+1.toLong(),y+1.toLong())] ?: 0.toLong()

                        graph[Pair(x+1.toLong(),y-1.toLong())] = l + t
                        graph[Pair(x+1.toLong(),y+1.toLong())] = u + t
                        index++
                        list[x+1][y-1] = "|"
                        list[x+1][y+1]="|"
                    } else{
                        list[x+1][y] ="|"
                        var u = graph[Pair(x+1.toLong(),y.toLong())] ?: 0.toLong()
                        graph[Pair(x+1.toLong(),y.toLong())]= t + u
                    }
                }
            }
        }   
        return index
    }
    fun challenge2(): Long {
        return graph.keys.filter { x-> x.first == graph.keys.maxOf { it.first } }.sumOf { graph[it]!! }
    }
}