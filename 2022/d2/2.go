package main

import (
	"AoC/common"
	"fmt"
	"os"
	"strings"
)

var data string = `A Y
B X
C Z`

func main() {
	year := "2022"
	day := "2"
	common.GetInput(year, day)
	dat, err := os.ReadFile(fmt.Sprintf("./%v/inputs/%v-%v.in", year, year, day))
	data = strings.TrimSuffix(strings.TrimSpace(string(dat)), "\n")
	if err != nil {
		fmt.Println(err)
	}

	vals := map[string]int{
		"Rock":     1,
		"Paper":    2,
		"Scissors": 3,
	}

	A_Mapping := map[string]string{
		"A": "Rock",
		"B": "Paper",
		"C": "Scissors",
	}
	B_Mapping := map[string]string{
		"X": "Rock",
		"Y": "Paper",
		"Z": "Scissors",
	}

	ALT_Mapping := map[string]string{
		"X": "Lose",
		"Y": "Draw",
		"Z": "Win",
	}

	relationships := map[string]string{
		"Rock":     "Scissors",
		"Paper":    "Rock",
		"Scissors": "Paper",
	}

	var score int
	var alt_score int
	for _, rd := range strings.Split(data, "\n") {
		l := strings.Split(rd, " ")[0]
		r := strings.Split(rd, " ")[1]
		if A_Mapping[l] == B_Mapping[r] {
			score += 3
		} else if A_Mapping[l] == relationships[B_Mapping[r]] {
			score += 6
		}
		score += vals[B_Mapping[r]]

		var hand string
		switch ALT_Mapping[r] {
		case "Win":
			hand = relationships[relationships[A_Mapping[l]]]
		case "Lose":
			hand = relationships[A_Mapping[l]]
		case "Draw":
			hand = A_Mapping[l]
		}
		if A_Mapping[l] == hand {
			alt_score += 3
		} else if A_Mapping[l] == relationships[hand] {
			alt_score += 6
		}
		alt_score += vals[hand]
	}
	fmt.Printf("Final score %v\n", score)
	fmt.Printf("Alt score %v\n", alt_score)
}
