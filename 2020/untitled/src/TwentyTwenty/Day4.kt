package TwentyTwenty

import HelperFunctions.FileReader
import kotlin.time.measureTime

class Day4(path: String) {

    var list: List<String> = FileReader().concatStringBasedOnEmptyLines(path + "data")

    fun run() {
        val runtimeChallenge1 = measureTime {   println("Challenge 1: " + challenge1())}
        println("Challenge 1 - Runtime: $runtimeChallenge1")
        val runtimeChallenge2 = measureTime {   println("Challenge 2: " + challenge2());}
        println("Challenge 2 - Runtime: $runtimeChallenge2")
    }

    fun challenge1(): Int {
        var counter = 0;
        val passportItems = listOf<String>("ecl:", "pid:", "eyr:", "hcl:", "byr:", "iyr:", /*"cid:",*/ "hgt:")
        list.forEach { x -> if (passportItems.all { x.contains(it) }) {counter++} }
        return counter

    }
    fun challenge2(): Int {
        var counter = 0;
        list.forEach { x -> if (validator(x)) counter++}
        return counter
    }
    fun validator(element: String): Boolean {

        var isValid =  true;

        val elements =  element.split(" ");
        val passportItems = listOf<String>("ecl:", "pid:", "eyr:", "hcl:", "byr:", "iyr:", /*"cid:",*/ "hgt:")
        if (passportItems.all { element.contains(it) }) isValid = true else return false

        for (x in elements){
            if (x.contains("byr:")) {
                isValid = byr(x.split(":")[1])
            } else if (x.contains("iyr:")) {
                isValid = iyr(x.split(":")[1])
            } else if (x.contains("eyr:")) {
                isValid = eyr(x.split(":")[1])
            } else if (x.contains("hgt:")) {
                isValid = hgt(x.split(":")[1])
            } else if (x.contains("ecl:")) {
                isValid = ecl(x.split(":")[1])
            } else if (x.contains("pid:")) {
                isValid = pid(x.split(":")[1])
            } else if (x.contains("hcl:")) {
                isValid = hcl(x.split(":")[1])
            }
            if (!isValid) return false
        }
        return true;
    }
    fun byr(element:String): Boolean{
        try {
            val el =  element.toInt();
            return el in 1920..<2003
        } catch (e: Exception){
            return false

        }
    }
    fun iyr(element:String): Boolean{
        return try {
            element.toInt() in 2010..<2021
        } catch (e: Exception){
            false
        }
    }
    fun eyr(element:String): Boolean{
        return try {
            element.toInt() in 2020..<2031
        } catch (e: Exception){
            false
        }
    }
    fun hgt(element:String): Boolean {
        if (element.contains("cm")) {
            if (element.length != 5) {
                return false
            }
            val cm = element.take(3).toInt();
            return cm in 150..<194
        } else if (element.contains("in")) {
            if (element.length != 4) {
                return false
            }
            val inch = element.take(2).toInt();
            return inch in 59..<77
        } else return false

    }

    fun ecl(element:String): Boolean{
        val eyeColors = listOf<String>("amb", "blu", "brn", "gry", "grn", "hzl", "oth")
        return eyeColors.contains(element);
    }

    fun hcl(element:String): Boolean{
        val regex = """#[a-f\d]{6}""".toRegex();
        return regex.matches(element);
    }
    fun pid(element:String): Boolean{
        if (element.length !=9)return false
        try {
            val convert = element.toInt()
            return true
        } catch (e: Exception) {
            return false
        }
    }
}