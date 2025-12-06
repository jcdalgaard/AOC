package Solutions

import HelperFunctions.FileReader
import kotlin.Long
import kotlin.Pair
import kotlin.collections.component1
import kotlin.collections.component2
import kotlin.time.Duration.Companion.seconds
import kotlin.time.measureTime

class Day5(path: String) {

    var list: List<String> = FileReader().returnContentAsList(path + "data")
    var ranges = mutableListOf<Pair<Long, Long>>()
    var nums =  mutableListOf<Long>()

    fun run() {
        val runtimeChallenge1 = measureTime { println("Challenge 1: " + challenge1()) }
        println("Challenge 1 - Runtime: $runtimeChallenge1")
        val runtimeChallenge2 = measureTime { println("Challenge 2: " + challenge2()); }
        println("Challenge 2 - Runtime: $runtimeChallenge2")
    }

    fun challenge1(): Int {
        var index = 0

        list.forEach { x-> if (x.contains("-"))  x.split("-").let { (a,b) -> ranges.add(Pair(a.toLong(),b.toLong())) } else if (x !="") nums.add(x.toLong())   }
        for (x in nums){
            for(y in ranges){
                if( x>= y.first && x<= y.second){
                    index++
                    break
                }

            }

        }
        return index
    }
    fun challenge2(): Long {
        var newRanges =  mutableListOf<Pair<Long, Long>>()
        var sortedList = ranges.sortedBy { it.first }
        var f =  sortedList[0]
        for (x in sortedList) {
            if (x.first <= f.second) {
                f = Pair(f.first, maxOf(f.second, x.second))
            } else {
                newRanges.add(f)
                f = x
            }
        }
        newRanges.add(f)

        return newRanges.sumOf { x -> (1+x.second-x.first).toLong() }

    }
}