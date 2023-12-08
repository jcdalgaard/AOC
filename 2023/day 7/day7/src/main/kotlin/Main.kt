fun main(args: Array<String>) {
    val data = FileReader.GetFileContent("src/main/resources/example.txt")

    val c = CamelCards(data)

    c.fillHand()
    val sortedJohn: List<Hand>  = c.sort()

    println(c.calculateScore(sortedJohn))

}