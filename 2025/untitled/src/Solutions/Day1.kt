package Solutions

import HelperFunctions.FileReader
import kotlin.time.measureTime

class Day1(path: String) {

    var list: List<String> = FileReader().returnContentAsList(path + "data")
    
    fun run() {
        val runtimeChallenge1 = measureTime { println("Challenge 1: " + challenge1()) }
        println("Challenge 1 - Runtime: $runtimeChallenge1")
        val runtimeChallenge2 = measureTime { println("Challenge 2: " + challenge2()); }
        println("Challenge 2 - Runtime: $runtimeChallenge2")
    }

    fun challenge1(): Int {
        var index = 50
        return list.sumOf { x ->  index = if (x[0]== 'L') if (index-(x.substring(1).toInt()% 100) <0) 100+index-(x.substring(1).toInt()% 100) else index-(x.substring(1).toInt()% 100) else if (index+(x.substring(1).toInt()% 100) >100-1) index+(x.substring(1).toInt()% 100)-100 else index+(x.substring(1).toInt()% 100);if (index==0) 1 else 0  }
    }
    fun challenge2(): Int {
        var index = 50
        return list.sumOf { x -> (0..<(x.substring(1).toInt())).toList().sumOf { z -> (if (index+(if(x[0]=='L') -1 else 1)<0) index = 99 else if(index+(if(x[0]=='L') -1 else 1)>99) index = 0 else index+=(if(x[0]=='L') -1 else 1)); if (index==0) 1 else 0 } }
    }
}