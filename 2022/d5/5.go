package main

import (
	"AoC/common"
	"fmt"
	"os"
	"strconv"
	"strings"
)

var data string = `    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
`

func main() {
	defer common.Timer("main")()
	year := "2022"
	day := "5"
	common.GetInput(year, day)
	dat, err := os.ReadFile(fmt.Sprintf("./%v/inputs/%v-%v.in", year, year, day))
	data = strings.TrimSuffix(strings.TrimSpace(string(dat)), "\n")
	if err != nil {
		fmt.Println(err)
	}

	crate_map := strings.Split(data, "\n\n")[0]
	cols := crate_map[strings.LastIndex(crate_map, "\n"):]
	crate_map = crate_map[:strings.LastIndex(crate_map, "\n")]
	numCols, _ := strconv.Atoi(strings.TrimSpace(cols[strings.LastIndex(cols, "   "):]))
	instructions := strings.Split(data, "\n\n")[1]
	crate_arrangement_9000 := [][]string{}
	crate_arrangement_9001 := [][]string{}
	for i := 0; i < numCols; i++ {
		crate_arrangement_9000 = append(crate_arrangement_9000, []string{})
		crate_arrangement_9001 = append(crate_arrangement_9001, []string{})
	}
	for _, row := range strings.Split(crate_map, "\n") {
		for i := 0; i < numCols; i++ {
			crate_loc := (i * 4) + 1
			crate := string(row[crate_loc])
			if len(strings.TrimSpace(crate)) != 0 {
				crate_arrangement_9000[i] = append(crate_arrangement_9000[i], crate)
				crate_arrangement_9001[i] = append(crate_arrangement_9001[i], crate)
			}
		}
	}

	fmt.Println(visualize(crate_arrangement_9001))

	for _, instruct := range strings.Split(instructions, "\n") {
		amt, _ := strconv.Atoi(strings.Split(instruct, " ")[1])
		from, _ := strconv.Atoi(strings.Split(instruct, " ")[3])
		to, _ := strconv.Atoi(strings.Split(instruct, " ")[5])
		from -= 1
		to -= 1

		for i := 0; i < amt; i++ {
			crate_arrangement_9000[to] = append([]string{crate_arrangement_9000[from][0]}, crate_arrangement_9000[to]...)
			crate_arrangement_9000[from] = append(crate_arrangement_9000[from][:0], crate_arrangement_9000[from][1:]...)
		}
		fmt.Printf("Amt: %v, From: %v, To: %v\n", amt, from, to)
		// TODO: Figure out why slice magic doesn't work
		// crate_arrangement_9001[to] = append(crate_arrangement_9001[from][0:amt], crate_arrangement_9001[to]...)
		// crate_arrangement_9001[from] = append(crate_arrangement_9001[from][:0], crate_arrangement_9001[from][amt:]...)
		tmp_list := []string{}
		for _, crt := range crate_arrangement_9001[from][0:amt] {
			tmp_list = append(tmp_list, crt)
		}
		for y := len(crate_arrangement_9001[from][0:amt]) - 1; y >= 0; y-- {
			crate_arrangement_9001[to] = append([]string{crate_arrangement_9001[from][y]}, crate_arrangement_9001[to]...)
		}
		for i := 0; i < amt; i++ {
			crate_arrangement_9001[from] = append(crate_arrangement_9001[from][:0], crate_arrangement_9001[from][1:]...)
		}
	}
	fmt.Println(visualize(crate_arrangement_9000))
	fmt.Println(visualize(crate_arrangement_9001))
	topCrates_9000 := ""
	topCrates_9001 := ""
	for i := 0; i < len(crate_arrangement_9000); i++ {
		topCrates_9000 = fmt.Sprint(topCrates_9000, crate_arrangement_9000[i][0])
		topCrates_9001 = fmt.Sprint(topCrates_9001, crate_arrangement_9001[i][0])
	}
	fmt.Println(fmt.Sprint("9000: ", topCrates_9000))
	fmt.Println(fmt.Sprint("9001: ", topCrates_9001))
}

func visualize(crate [][]string) (out string) {
	crate_temp := make([][]string, len(crate))

	copy(crate_temp, crate)
	maxIdx := 0
	for i := 0; i < len(crate_temp); i++ {
		if len(crate_temp[i])-1 > maxIdx {
			maxIdx = len(crate_temp[i]) - 1
		}
	}

	for j := maxIdx; j >= 0; j-- {
		line := ""
		for i := 0; i < len(crate_temp); i++ {
			str_to_add := ""
			if len(crate_temp[i])-1 >= 0 {
				str_to_add = fmt.Sprint(" [", crate_temp[i][len(crate_temp[i])-1], "]")
				crate_temp[i] = crate_temp[i][:len(crate_temp[i])-1]
			} else {
				str_to_add = "    "
			}
			line = fmt.Sprint(line, str_to_add)
			line = strings.TrimLeft(line, " ")
		}
		line = fmt.Sprint(line, "\n")
		out = fmt.Sprint(line, out)
	}
	for i := 0; i < len(crate_temp); i++ {
		out = fmt.Sprint(out, " ", i, "  ")
	}
	out = fmt.Sprint(out, "\n")
	out = fmt.Sprint("---------------------------\n", out, "---------------------------")
	return
}
