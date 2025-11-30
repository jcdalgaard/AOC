package TwentyTwenty

import HelperFunctions.FileReader
import kotlin.math.max
import kotlin.text.iterator
import kotlin.time.measureTime

class Day6(path: String) {

    var list: List<String> = FileReader().returnContentAsList(path + "data")

    fun run() {
        val runtimeChallenge1 = measureTime { println("Challenge 1: " + challenge1()) }
        println("Challenge 1 - Runtime: $runtimeChallenge1")
        val runtimeChallenge2 = measureTime { println("Challenge 2: " + challenge2()); }
        println("Challenge 2 - Runtime: $runtimeChallenge2")
    }

    fun challenge1(): Int {
        var total = 0;

        var group = mutableSetOf<Char>();
        for (x in list){
            if (x==""){
                total += group.size;
                group = mutableSetOf();
            }
           group.addAll(x.toSet())
        }
        total += group.size;

        return total

    }



    fun challenge2(): Int {
        var total = 0;

        var group = mutableSetOf<Char>();
        var added = false

        for (x in list){
            if (x==""){
                total += group.size;
                group = mutableSetOf();
                added =  false

                continue
            }
            if (!added){
                group.addAll(x.toSet())
                added =  true
                continue
            }

            var y = mutableSetOf<Char>();


            x.forEach { z -> if(group.contains(z)) y.add(z) }
            group = y


        }
        total += group.size;

        return total
    }
}