package main

import (
	"AoC/common"
	"fmt"
	"os"
	"strconv"
	"strings"
)

var data string = `2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8`

func main() {
	defer common.Timer("main")()
	year := "2022"
	day := "4"
	common.GetInput(year, day)
	dat, err := os.ReadFile(fmt.Sprintf("./%v/inputs/%v-%v.in", year, year, day))
	data = strings.TrimSuffix(strings.TrimSpace(string(dat)), "\n")
	if err != nil {
		fmt.Println(err)
	}

	result := []string{}
	contain_count := 0
	overlap_count := 0
	for _, section := range strings.Split(data, "\n") {
		p1, _ := strconv.Atoi(strings.Split(section[:strings.Index(section, ",")], "-")[0])
		p2, _ := strconv.Atoi(strings.Split(section[:strings.Index(section, ",")], "-")[1])
		p3, _ := strconv.Atoi(strings.Split(section[strings.Index(section, ",")+1:], "-")[0])
		p4, _ := strconv.Atoi(strings.Split(section[strings.Index(section, ",")+1:], "-")[1])

		if (p1 <= p3 && p2 >= p4) || (p3 <= p1 && p4 >= p2) {
			contain_count += 1
		}
		if (p1 <= p3 && p2 >= p3) || (p3 <= p1 && p4 >= p1) {
			// result = append(result, section)
			overlap_count += 1
		}
	}
	fmt.Println(result)
	fmt.Println(contain_count)
	fmt.Println(overlap_count)
}
