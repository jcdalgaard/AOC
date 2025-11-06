package TwentyTwenty

import HelperFunctions.FileReader

class Day1(path: String) {
    var list: List<Int>

    init {
        list =
            FileReader().returnContentAsListOfNumbers(path+"data");
    }

    fun run() {

        println("Challenge 1: " + challenge1(2020));
        println("Challenge 2: " + challenge2());

    }

    fun challenge1(number: Int): Int {
        list.forEach({ x ->
            for (y in list) {
                if (x + y == number) {
                    return (x * y)
                }
            }
        })
        return 0;
    }

    fun challenge2(): Int {

        for (x in list) {
            var temp = challenge1(2020 - x);
            if (temp > 0) return temp * x
        }
        return 0;
    }
}

