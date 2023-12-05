fun main(args: Array<String>) {

 val data =  FileReader.GetFileContent("src/main/resources/example.txt")

    val lotteryCalculator = LotteryCalculator()
    lotteryCalculator.addCards(data);

    println(lotteryCalculator.getSumOfAllCards())

    println(lotteryCalculator.calculateCardNumber())
    println("Done")

}