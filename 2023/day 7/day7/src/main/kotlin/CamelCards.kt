

class CamelCards(val input: MutableList<String> ){

    val hands: MutableList<Hand> = mutableListOf()
    var arr = charArrayOf('A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2')
    val sorted: List<Hand> =  listOf()

    fun fillHand(){

        input.forEach { x ->
            var s = x.split(" ")
            var combi = s[0].replace('A', 'S').replace('K','R').replace('T', 'H').replace("J","0")
            var grouping =  combi.groupingBy { it }.eachCount()
            var key:Char = '0'
            if (combi!="00000") {
                key = grouping.filterKeys { x -> x != '0' }.maxBy { it.value }.key.toChar()
            }
            var combi2 = combi.replace('0',key)
            var grouping2 = combi2.groupingBy { it }.eachCount()
            var count = grouping2.maxBy { it.value}.value

                //var grouping2 = grouping.filterKeys { x -> x!='0' }
            if(count >= 3 ) {
                if(grouping2.containsValue(2)){
                    count = 5
                }else if (count>3){
                    count+=2
                } else {

                    count ++
                }

            } else if (count ==2 ){
                var group2 = grouping2.filterValues { x ->  x==2 }

                if (group2.count()==2){
                    count = 3
                }

            }

            hands.add(Hand(combi, s[1].toLong(),count.toLong()))
        }




    }
    fun sort() : List<Hand> {
      return hands.sortedWith(compareBy({it.numberOfCommon}, {it.Cards}))
    }

    fun  calculateScore(list:List<Hand>):Long{
        var score:Long = 0
        var i = 0
        list.forEach { element ->
            i = i+1
            score += element.number *i

        }
        return score
    }

}