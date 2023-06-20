package main

import (
	"AoC/common"
	"fmt"
	"os"
	"strings"
)

var data string = `vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw`

func main() {

	year := "2022"
	day := "3"
	common.GetInput(year, day)
	dat, err := os.ReadFile(fmt.Sprintf("./%v/inputs/%v-%v.in", year, year, day))
	data = strings.TrimSuffix(strings.TrimSpace(string(dat)), "\n")
	if err != nil {
		fmt.Println(err)
	}

	var priority_sum int
	final := make(map[string]string)
	for i, rucksack := range strings.Split(data, "\n") {
		uniq := make(map[rune]int)
		p1 := make(map[rune]bool)
		p2 := make(map[rune]bool)
		for _, runee := range []rune(rucksack) {
			uniq[runee] = 0
		}
		for _, runee := range []rune(rucksack[:len(rucksack)/2]) {
			p1[runee] = false
		}
		for runee := range p1 {
			uniq[runee] += 1
		}
		for _, runee := range []rune(rucksack[(len(rucksack) / 2):]) {
			p2[runee] = false
		}
		for runee := range p2 {
			uniq[runee] += 1
		}
		keyval := 0
		for key, val := range uniq {
			if val >= 2 {
				final[fmt.Sprint(i)] = final[fmt.Sprint(i)] + string(key)
				offset := 38
				if int(key) > int('Z') {
					offset = 96
				}
				final[fmt.Sprint(i, "_", string(key))] = fmt.Sprint(int(key) - offset)
				keyval = int(key) - offset
				priority_sum += keyval
			}
		}
		// fmt.Printf("%v, %v, %v, %v, %v || ", len(rucksack), rucksack[:len(rucksack)/2], rucksack[(len(rucksack)/2)+1:], final[fmt.Sprint(i)], keyval)
		// fmt.Println(uniq)
	}

	var trip_priority_sum int
	// trip_final := make(map[string]string)
	split_data := strings.Split(data, "\n")
	for i := 0; i < len(split_data)-1; i += 3 {
		full_group := fmt.Sprintf("%v%v%v", split_data[i], split_data[i+1], split_data[i+2])
		// fmt.Printf("%v %v %v\n", split_data[i], split_data[i+1], split_data[i+2])
		uniq := make(map[rune]int)
		p1 := make(map[rune]bool)
		p2 := make(map[rune]bool)
		p3 := make(map[rune]bool)
		for _, runee := range []rune(full_group) {
			uniq[runee] = 0
		}
		for _, runee := range []rune(split_data[i]) {
			if _, ok := p1[runee]; !ok {
				uniq[runee] += 1
			}
			p1[runee] = false
		}
		for _, runee := range []rune(split_data[i+1]) {
			if _, ok := p2[runee]; !ok {
				uniq[runee] += 1
			}
			p2[runee] = false
		}
		for _, runee := range []rune(split_data[i+2]) {
			if _, ok := p3[runee]; !ok {
				uniq[runee] += 1
			}
			p3[runee] = false
		}

		keyval := 0
		for key, val := range uniq {
			if val >= 3 {
				final[fmt.Sprint(i)] = final[fmt.Sprint(i)] + string(key)
				offset := 38
				if int(key) > int('Z') {
					offset = 96
				}
				final[fmt.Sprint(i, "_", string(key))] = fmt.Sprint(int(key) - offset)
				keyval = int(key) - offset
				trip_priority_sum += keyval
			}
		}

		fmt.Printf("%v, %v, %v || %v \n", full_group, final[fmt.Sprint(i)], keyval, uniq)
	}
	// fmt.Println(final)

	fmt.Println(priority_sum)
	fmt.Println(trip_priority_sum)
}
