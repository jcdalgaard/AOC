
import java.io.File

    object FileReader {

        fun GetFileContent(fileLocation:String) :MutableList<String>{
            val fileContent: MutableList<String> = mutableListOf();
            val file = File(fileLocation)
            file.forEachLine { line -> fileContent.add((line)) }

            return fileContent
        }



    }

