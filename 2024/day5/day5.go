package day5

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func LoadData(fp string) (map[int][]int, [][]int) {
	file, _ := os.Open(fp)

	defer file.Close()

	r := bufio.NewReader(file)

	rules_map := make(map[int][]int)
	instructions := [][]int{}
	getting_instructions := false
	for {
		line, _, err := r.ReadLine()
		if len(line) > 0 {
			if getting_instructions {
				// parse the lines of pages
				vals := strings.Split(string(line), ",")
				int_vals := []int{}
				for _, v := range vals {
					i, err := strconv.Atoi(v)
					if err == nil {
						int_vals = append(int_vals, i)
					}
				}
				instructions = append(instructions, int_vals)
			} else {
				// parse the page rules
				vals := strings.Split(string(line), "|")
				int_vals := []int{}
				for _, v := range vals {
					i, err := strconv.Atoi(v)
					if err == nil {
						int_vals = append(int_vals, i)
					}
				}
				val, ok := rules_map[int_vals[0]]
				if ok {
					rules_map[int_vals[0]] = append(val, int_vals[1])
				} else {
					rules_map[int_vals[0]] = []int{int_vals[1]}
				}
			}
		} else {
			getting_instructions = true
		}
		if err != nil {
			break
		}
	}

	return rules_map, instructions
}

func contains(arr []int, v int) bool {
	for _, x := range arr {
		if x == v {
			return true
		}
	}
	return false
}

func Part1(rules map[int][]int, instructions [][]int) int {
	sum := 0
	for _, inst := range instructions {
		valid := true
		for idx, val := range inst {
			if idx == len(inst)-1 {
				break
			}
			next := inst[idx+1]
			r, ok := rules[val]
			if ok {
				if !contains(r, next) {
					valid = false
					break
				}
			} else {
				valid = false
				break
			}
		}

		if valid {
			fmt.Println(inst, " is valid ")
			// extract the relevant info for the challenge
			// in this case, the middle page number
			sum += inst[int(len(inst)/2)]
		}

	}

	return sum
}

func Part2(rules map[int][]int, instructions [][]int) int {
	sum := 0
	for _, inst := range instructions {
		valid := true
		for idx, val := range inst {
			if idx == len(inst)-1 {
				break
			}
			next := inst[idx+1]
			r, ok := rules[val]
			if ok {
				if !contains(r, next) {
					valid = false
					break
				}
			} else {
				valid = false
				break
			}
		}

		if !valid {
			// fix it so it is valid...
			// then add to the sum
			// the way to go about this is to recursively go through the pages and spit out the correct order
			// start from the first option, and iterate through if no valid combos are possible
			fmt.Println(inst, " is invalid")
			cp := make([]int, len(inst))

			for idx, val := range inst {
				// use val as the starting point
				copy(cp, inst)
				// fmt.Println(val, idx, cp, inst, "what")
				other_pages := append(cp[:idx], cp[idx+1:]...)
				sorted := recursive_sort(val, other_pages, rules)
				if len(sorted) == len(inst) {
					// fmt.Println("Did it work?", sorted)
					sum += sorted[int(len(sorted)/2)]
					break
				}

			}
		}

	}

	return sum
}

func recursive_sort(start int, remaining_pages []int, rules map[int][]int) []int {
	s := []int{start}
	cp := make([]int, len(remaining_pages))

	if len(remaining_pages) == 0 {
		return nil
	}

	val, ok := rules[start]
	if !ok {
		return nil
	}
	if ok && len(remaining_pages) == 1 && contains(val, remaining_pages[0]) {
		return []int{start, remaining_pages[0]}
	}

	for idx, p := range remaining_pages {
		copy(cp, remaining_pages)
		if contains(val, p) {
			rp := append(cp[:idx], cp[idx+1:]...)
			// fmt.Println("One step closer?", p, rp)
			test := recursive_sort(p, rp, rules)
			if len(test) == len(remaining_pages) {
				return append(s, test...)
			}
		}
	}

	return nil
}
