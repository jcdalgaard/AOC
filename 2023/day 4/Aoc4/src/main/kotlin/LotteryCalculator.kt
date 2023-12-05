import java.util.*

class LotteryCalculator() {
    val cards: MutableList<Card> = mutableListOf()
    val cardIdsToProcess: Queue<Int> = LinkedList()
    var totalCards: Long = 0


    fun addCards(strings: MutableList<String>) {

        strings.forEach { line -> cards.add(addCard(line)) }
        initCardIdsToProcess()

    }

    fun calculateCardNumber(): Long {

        while (cardIdsToProcess.size > 0) {
            addCardToProcess((cards[cardIdsToProcess.remove()]))
            totalCards++
        }

        return totalCards

    }

    fun addCardToProcess(card: Card) {
        val elementNumber = card.id.split(" ")[1].toInt()
        for (i in elementNumber + 1..elementNumber + (card.matches ?: 0)) {
            cardIdsToProcess.add(i - 1)
        }
    }


    fun initCardIdsToProcess() {
        for (i in 0..cards.size - 1) {
            cardIdsToProcess.add(i)
        }
    }


    fun addCard(cardString: String): Card {

        val cleanedStr = cardString.replace(Regex("\\s+"), " ")
        var firstSplit = cleanedStr.split(":")
        var secondSplit = firstSplit[1].split("|");

        val card: Card =
            Card(firstSplit[0], secondSplit[0].trim().split(" "), secondSplit[1].trim().split(" "), null, 0)

        return calculateCardScore(card);
    }

    fun calculateCardScore(card: Card): Card {

        val yourWinningNumbers = card.numbers.intersect(card.winnerNumbers)
        var j = 0
        for (i in 0..yourWinningNumbers.size - 1) {
            j = 1.takeIf { i < 1 } ?: j * 2
        }
        card.score = j
        card.matches = yourWinningNumbers.size
        return card
    }

    fun getSumOfAllCards(): Int {
        var sum = 0

        cards.forEach { card -> sum += card.score ?: 0 }

        return sum
    }
}