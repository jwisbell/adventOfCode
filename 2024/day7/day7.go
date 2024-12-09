package day7

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func LoadData(fp string) []Equation {
	file, _ := os.Open(fp)

	defer file.Close()

	r := bufio.NewReader(file)

	rows := []Equation{}

	for {
		line, _, err := r.ReadLine()
		if len(line) > 0 {
			parts := strings.Split(string(line), ":")
			answer, err := strconv.Atoi(parts[0])
			if err != nil {
				fmt.Println("Something wrong in this line")
			}
			str_vals := strings.Split(parts[1], " ")
			int_vals := []int{}
			for _, sv := range str_vals {
				i, err := strconv.Atoi(sv)
				if err == nil {
					int_vals = append(int_vals, i)
				}
			}
			rows = append(rows, Equation{Answer: answer, Values: int_vals})

		}
		if err != nil {
			break
		}
	}
	return rows
}

type Equation struct {
	Answer int
	Values []int
}

func concat(num1, num2 int) int {
	// cast to string, concat, and return the int
	s1 := strconv.Itoa(num1)
	s2 := strconv.Itoa(num2)

	result, err := strconv.Atoi(s2 + s1)
	if err != nil {
		fmt.Println("Couldn't concat ", num1, num2)
	}
	fmt.Println("Did concat ", result)
	return result
}

func recursive_check(target, val int, next_vals []int, allow_concat bool) bool {
	// valid := false
	if len(next_vals) == 0 && target != val {
		return false
	} else if len(next_vals) == 0 && target == val {
		return true
	}
	valid1 := false
	valid2 := false
	valid3 := false
	// fmt.Println("Test", target, val, next_vals)
	if val != 0 && target%val == 0 && target/val >= next_vals[len(next_vals)-1] {
		valid1 = recursive_check(target/val, next_vals[len(next_vals)-1], next_vals[:len(next_vals)-1], allow_concat)
	}
	if target-val >= 0 && target-val >= next_vals[len(next_vals)-1] {
		valid2 = recursive_check(target-val, next_vals[len(next_vals)-1], next_vals[:len(next_vals)-1], allow_concat)
	}
	if allow_concat {
		s1 := strconv.Itoa(target)
		s2 := strconv.Itoa(val)
		if len(s2) < len(s1) {
			if s1[len(s1)-len(s2):] == s2 {
				s_new := s1[:len(s1)-len(s2)]
				new_targ, err := strconv.Atoi(s_new)
				if err != nil {
					fmt.Println("Something went wrong")
				}
				valid3 = recursive_check(new_targ, next_vals[len(next_vals)-1], next_vals[:len(next_vals)-1], allow_concat)
			}
		}
	}
	return valid1 || valid2 || valid3
}

func Part1(rows []Equation, allow_concat bool) int {
	count := 0

	//for each equation, figure out if a combination of + and * operators can be used to
	//make it work. The equations are always read left to right, not in order of operations
	//
	//the algorithm is straightforward and recursive
	//1. Let T be the target value and Vals be the individual numbers we can use
	//2. Starting from the LAST value, x, in Vals, try to divide T/x
	//3. If it's an integer AND >= the next value, continue trying (recursively)
	//4. Else, try to subtract T-x. If it's >= the next value, continue recursively
	//5. Stop when there are no more numbers to use or if the result is LESS than the remaining numbers
	//rows = []Equation{{Answer: 7290, Values: []int{6, 8, 6, 15}}}
	for _, eq := range rows {
		targ := eq.Answer
		vals := eq.Values
		if recursive_check(targ, vals[len(vals)-1], vals[:len(vals)-1], allow_concat) {
			count += eq.Answer
		} else {
			fmt.Println("What am i missing? ", eq)
		}
	}

	return count
}
