package main

import "fmt"

func main() {
	fmt.Println("Hello")

	// var number1 = calculateOptions(7, 9)
	// var number2 = calculateOptions(15, 40)
	// var number3 = calculateOptions(30, 200)
	// var number1 = calculateOptions(59, 597)
	// var number2 = calculateOptions(79, 1234)
	// var number3 = calculateOptions(65, 1032)
	// var number4 = calculateOptions(75, 1328)
	// fmt.Println(number1 * number2 * number3 * number4)
	// var number1 = calculateOptions(71530, 940200)
	var number1 = calculateOptions(59796575, 597123410321328)
	fmt.Println(number1)
}
func calculateOptions(time int64, longestDistance int64) int64 {

	var count int64 = 0
	var i int64 = 0
	for i < time {

		distanceCalculations := calculateDistance(i, time)
		if distanceCalculations > longestDistance {

			count = count + 1
		}
		i++
	}
	return count

}

func calculateDistance(secondsHeld int64, time int64) int64 {
	timeleft := time - secondsHeld
	return timeleft * secondsHeld

}
