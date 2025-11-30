package TwentyTwenty

import HelperFunctions.FileReader
import TwentyTwenty.DataClasses.WeightedEdges
import kotlin.time.measureTime

class Day7(path: String) {

    var list: List<String> = FileReader().returnContentAsList(path + "data")
    var elementToCheck =  "shiny gold"

    var validBags: Map<String, Int> = mutableMapOf()
    var checkedEdges =  mutableListOf<WeightedEdges>()
    var graph =  createGraph();

    fun run() {
        val runtimeChallenge1 = measureTime { println("Challenge 1: " + challenge1()) }
        println("Challenge 1 - Runtime: $runtimeChallenge1")
        val runtimeChallenge2 = measureTime { println("Challenge 2: " + challenge2()); }
        println("Challenge 2 - Runtime: $runtimeChallenge2")
    }

    fun challenge1(): Int {
        graph.forEach { x -> checkBags( x.bagFrom) }
        return validBags.size
    }

    fun checkBags( edge: String) {
        var connections =  graph.filter { x -> x.bagFrom == edge }
        for (x in connections) {
            if (!checkedEdges.contains(x) && x.bagFrom != elementToCheck ) {
                if (validBags[x.bagTo] != null || x.bagTo == elementToCheck || validBags[x.bagFrom] != null ) {
                    validBags += mapOf<String, Int>(Pair(x.bagFrom, x.amount))
                } else {
                    checkBags(x.bagTo)
                    if (validBags[x.bagTo] != null || x.bagTo == elementToCheck || validBags[x.bagFrom] != null) validBags += mapOf<String, Int>(Pair(x.bagFrom, x.amount))
                }
            }
            checkedEdges.add(x)
        }
    }

    fun checkBags2(edges: List<WeightedEdges>, edge: String):Int {
        var amount = 0
         edges.filter { y -> y.bagFrom == edge }.forEach{x-> amount += checkBags2(edges, x.bagTo) * x.amount }
        return 1+ amount;
    }

    fun createGraph(): List<WeightedEdges>{
        var edges = mutableListOf<WeightedEdges>()
        for (x in list){
            var y = x.split("bags contain")
            y[1].split(",").map { z-> if ( !z.contains("no other bags")) edges.add(WeightedEdges(y[0].trim(), z.trim().replace(".","").replace("bags","").replace("bag","").trim().substring(2), z.trim()[0].toString().toInt()))  }
        }
        return edges
    }

    fun challenge2(): Int {
        return checkBags2(graph, elementToCheck)-1
    }
}