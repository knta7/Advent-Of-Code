package main

import (
	"AoC/common"
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

var data string = `1000
2000
3000

4000

5000
6000

7000
8000
9000

10000`

func decode_input(in string) []int {
	out := []int{}
	for _, elf := range strings.Split(in, "\n\n") {
		total := 0
		for _, cal := range strings.Split(elf, "\n") {
			int_cal, _ := strconv.Atoi(cal)
			total += int_cal
		}
		out = append(out, total)
	}
	return out
}

func getMax(in []int) (m int) {
	for i, e := range in {
		if i == 0 || e > m {
			m = e
		}
	}
	return
}

func getMax_sort(in []int) int {
	sort.Ints(in)
	return in[len(in)-1]
}

func getMaxThree_sort(in []int) (out int) {
	sort.Ints(in)
	for i := len(in) - 1; i >= len(in)-3; i-- {
		out += in[i]
	}
	return
}

func main() {
	year := "2022"
	day := "1"
	common.GetInput(year, day)
	dat, err := os.ReadFile(fmt.Sprintf("./%v/inputs/%v-%v.in", year, year, day))
	data = strings.TrimSuffix(strings.TrimSpace(string(dat)), "\n")
	if err != nil {
		fmt.Println(err)
	}
	decoded_input := decode_input(string(dat))
	fmt.Println(getMax(decoded_input))
	fmt.Println(getMax_sort(decoded_input))
	fmt.Println(getMaxThree_sort(decoded_input))
}
