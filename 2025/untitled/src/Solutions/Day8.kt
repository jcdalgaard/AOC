package Solutions

import DataClasses.Edge
import DataClasses.Point
import HelperFunctions.FileReader
import java.math.BigInteger
import kotlin.math.pow
import kotlin.math.sqrt
import kotlin.time.measureTime

class Day8(path: String) {

    var list: List<Point> = FileReader().returnListOfZPoints(path + "data", ",")
    var map = mutableMapOf<Point, MutableList<Point>>()
    var potentielEdges = mutableListOf<Edge>()
    var edges = mutableListOf<Edge>()
    var checked = mutableSetOf<Point>()
    var timesToRun = 1000
    fun run() {
        val runtimeChallenge1 = measureTime { println("Challenge 1: " + challenge1()) }
        println("Challenge 1 - Runtime: $runtimeChallenge1")
        val runtimeChallenge2 = measureTime { println("Challenge 2: " + challenge2()); }
        println("Challenge 2 - Runtime: $runtimeChallenge2")
    }

    fun fillMap() {
        var filledEdges = mutableMapOf<Point, Boolean>()
        for (x in 0..<list.size) {
            filledEdges[list[x]] = true
            for (y in 0..<list.size) {
                if (filledEdges[list[y]] != null) continue
                else {
                    var weight = calculateDistance(list[x], list[y])
                    potentielEdges.add(Edge(list[x], list[y], weight))
                }
            }
        }
        potentielEdges = potentielEdges.distinct().sortedBy { it.weight } as MutableList<Edge>
    }

    fun calculateDistance(p1: Point, p2: Point): Double =  sqrt(((p2.x - p1.x).pow(2.0) + (p2.y - p1.y).pow(2.0) + (p2.z!! - p1.z!!).pow(2.0)))

    fun findConnections(s: Point): Long {
        var stack = ArrayDeque<Point>()
        stack.add(s)
        checked.add(s)
        var count = 0.toLong()
        while (stack.isNotEmpty()) {
            var curremt = stack.removeLast()
            count++
            for (x in map[curremt] !!) {
                if (checked.add(x)) {
                    stack.add(x)
                }
            }
        }
        return count
    }
    fun addConnections(x:Int){
        edges.add(potentielEdges[x])
        if (map[potentielEdges[x].from] == null) {
            map[potentielEdges[x].from] = mutableListOf<Point>()
        }
        map[potentielEdges[x].from]?.add(potentielEdges[x].to)
        if (map[potentielEdges[x].to] == null) {
            map[potentielEdges[x].to] = mutableListOf<Point>()
        }
        map[potentielEdges[x].to]?.add(potentielEdges[x].from)
    }

    fun challenge1(): Long {
        fillMap()
        for (x in 0..<timesToRun) {
            addConnections(x)
        }
        var counts = mutableListOf<Long>()
        for (x in map.keys) {
            if (!checked.contains(x)) {
                counts.add(findConnections(x))
            }
        }
        return counts.sortedByDescending { it }.take(3).reduce(Long::times)
    }

    fun challenge2(): Long {
        var x = timesToRun
        var counts = mutableListOf<Long>(1)
        while (counts.max() < list.size) {
            counts.clear()
            addConnections(x)
            checked.clear()
            map.keys.forEach {y-> if (!checked.contains(y)) counts.add(findConnections(y)) }
            x++
        }
        return edges[edges.size-1].from.x.toLong() * edges[edges.size-1].to.x.toLong()
    }
}