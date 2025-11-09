package TwentyTwenty

import HelperFunctions.FileReader
import kotlin.math.max
import kotlin.time.measureTime

class Day5(path: String) {

    var list: List<String> = FileReader().returnContentAsList(path + "data")

    fun run() {
        val runtimeChallenge1 = measureTime { println("Challenge 1: " + challenge1()) }
        println("Challenge 1 - Runtime: $runtimeChallenge1")
        val runtimeChallenge2 = measureTime { println("Challenge 2: " + challenge2()); }
        println("Challenge 2 - Runtime: $runtimeChallenge2")
    }

    fun challenge1(): Int {
        var highestId = 0;
        for (x in list) {
            var row = findRow(x)
            var column =  findSeat(x)
            var id =  row * 8 + column

            highestId = max(highestId,id)
        }
        return highestId

    }

    fun findRow(s: String): Int {

        var seatCombo = s.substring(0, 7);
        var l = (0..127).toList();
        for (x in seatCombo) {
            var size = l.size / 2;
            if (x == 'B') {
                l = l.slice(size..l.size - 1)
            } else {
                l = l.slice(0..size - 1)
            }
        }
        return l[0];
    }

    fun findSeat(s: String): Int {
        var seatCombo = s.substring(7,10);
        var l = (0..7).toList();
        for (x in seatCombo) {
            var size = l.size / 2;
            if (x == 'R') {
                l = l.slice(size..l.size - 1)
            } else {
               l =  l.slice(0..size - 1)
            }
        }
        return l[0];
    }

    fun challenge2(): Int {

        val l = (0..127).flatMap { x -> (0..7).map { y -> Pair(x,y) } }.toMutableList()
        list.forEach { x-> l.remove(Pair(findRow(x), findSeat(x))) }
        val filtered = l.groupBy { it.first }.filterValues { it.size == 1 }.values.flatten()
        return  filtered[0].first * 8 + filtered[0].second

    }
}