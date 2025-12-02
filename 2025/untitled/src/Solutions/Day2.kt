package Solutions

import HelperFunctions.FileReader
import kotlin.time.measureTime

class Day2(path: String) {

    var list: List<String> = FileReader().returnLineSplit(path + "testdata", ",")

    fun run() {
        val runtimeChallenge1 = measureTime { println("Challenge 1: " + challenge1()) }
        println("Challenge 1 - Runtime: $runtimeChallenge1")
        val runtimeChallenge2 = measureTime { println("Challenge 2: " + challenge2()); }
        println("Challenge 2 - Runtime: $runtimeChallenge2")
    }

    fun challenge1(): Long {
      return patternCounter(true)
    }

    fun challenge2(): Long {
        return patternCounter(false)
    }
    fun patternCounter(onlyHalf: Boolean): Long{
        var count: Long = 0;
        for (x in list) {
            val split = x.split("-")
            for (y in split[0].trim().toLong()..split[1].trim().toLong()) {
                val s = y.toString()
                if(onlyHalf && s.length%2 !=0) continue
                for(x in (if (onlyHalf) listOf<Int>(s.length/2) else (1..s.length/2).filter { x-> s.length % x == 0})){
                    if (s.take(x).toRegex().findAll(s).count() == s.length / x) {
                        count += y
                        break
                    }

                }
            }
        }
        return count
    }

}