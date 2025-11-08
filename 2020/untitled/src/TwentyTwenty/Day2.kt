package TwentyTwenty

import HelperFunctions.FileReader
import kotlin.time.measureTime

class Day2(path: String) {
    var list: List<String> = FileReader().returnContentAsList(path + "data")


    fun run() {

        val runtimeChallenge1 = measureTime {   println("Challenge 1: " + challenge1())}


        println("Challenge 1 - Runtime: $runtimeChallenge1")

        val runtimeChallenge2 = measureTime {   println("Challenge 2: " + challenge2());}
        println("Challenge 2 - Runtime: $runtimeChallenge2")



    }

    fun challenge1(): Int {
        var counter = 0;
        for (x in list) {

            val (policy, pw) = x.split(":").let { it[0].trim() to it[1].trim() }
            val (range, character) = policy.split(" ").let { it[0].trim() to it[1].trim().single() }
            var (min, max) = range.trim().split("-").let { it[0].trim().toInt() to it[1].trim().toInt() }

            var occ = pw.count({ it == character })

            if (occ in min..max) {
                counter++;
            }

        }

        return counter

    }
    fun challenge2(): Int {
        var counter = 0;
        for (x in list) {

            val (policy, pw) = x.split(":").let { it[0].trim() to it[1].trim() }
            val (range, character) = policy.split(" ").let { it[0].trim() to it[1].trim().single() }
            var (min, max) = range.trim().split("-").let { it[0].trim().toInt() to it[1].trim().toInt() }


            if ((pw[min-1] ==character && pw[max-1] != character) || (pw[min-1] !=character && pw[max-1] == character)) {
                counter++
            }
        }

        return counter

    }
}