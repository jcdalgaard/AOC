package HelperFunctions

import java.io.File

class FileReader {

    fun returnContentAsList(filePath: String): List<String> {
        val file = File(filePath)
        if (!file.exists()) {
            throw IllegalArgumentException("File not found: $filePath")
        }
        return file.readLines()
    }

    fun returnContentAsListOfNumbers(filePath: String): List<Int> {
        try {
            return returnContentAsList(filePath).map({ it.toInt() });
        } catch (e: Exception) {
            throw e;
        }
    }
    fun returnMap(filePath: String): List<List<String>>{
        return returnContentAsList((filePath)).map { it.split("").filter { x -> x.isNotBlank() } };
    }
    fun concatStringBasedOnEmptyLines(filePath: String) : List<String>{
        var list = returnContentAsList(filePath)
        var newList =  emptyList<String>().toMutableList();
        var string ="";
        for(x in list){
            if (x.isNotBlank()){
                string += "$x ";

            } else {
                newList.add(string)
                string = "";
            }
        }
        if (string.isNotBlank()) newList.add(string)
        return newList;
    }
    fun returnLineSplit(filePath: String, delimter: String): List<String>{
        return returnContentAsList(filePath)[0].split(delimter);
    }

    fun returnMap(filePath: String,splitBy: String): List<List<String>>{
        return returnContentAsList((filePath)).map { it.split(splitBy).filter { x -> x.isNotBlank() } };
    }


}


