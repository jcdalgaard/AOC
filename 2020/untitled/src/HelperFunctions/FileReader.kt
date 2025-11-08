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


}


