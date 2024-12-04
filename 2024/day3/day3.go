package day3

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
)

func LoadData(fp string) []string {
	file, _ := os.Open(fp)

	defer file.Close()

	r := bufio.NewReader(file)

	instruction_string := []string{}

	for {
		line, _, err := r.ReadLine()
		if len(line) > 0 {
			instruction_string = append(instruction_string, string(line))
		}
		if err != nil {
			break
		}
	}

	return instruction_string
}

func Part1Regex(lines []string) int {
	sum := 0

	for _, line := range lines {
		// Updated regular expression to match strictly formatted mul(X,Y)
		regex := `mul\(-?\d+,-?\d+\)`

		// Compile the regular expression
		re := regexp.MustCompile(regex)

		// Find all matches
		matches := re.FindAllString(line, -1)

		// Print the matches
		fmt.Println("Matches:", matches)

		for _, match := range matches {
			val := parse_match(match)
			sum += val
		}

	}

	return sum
}

func parse_match(text string) int {
	split1 := strings.Split(text, ",")
	splitleft := strings.Split(split1[0], "(")
	splitright := strings.Split(split1[1], ")")

	num1, err := strconv.Atoi(splitleft[len(splitleft)-1])
	if err != nil {
		// do nothing
	}
	num2, err := strconv.Atoi(splitright[0])
	if err != nil {
		// do nothing
	}

	return num1 * num2
}

func Part2(lines []string) int {
	sum := 0

	concat := strings.Join(lines, "")
	temp := []string{concat}

	for _, line := range temp {
		mul_regex := `mul\(-?\d+,-?\d+\)`
		do_regex := `do\(\)`
		dont_regex := `don't\(\)`

		re := regexp.MustCompile(mul_regex)
		mul_matches := re.FindAllString(line, -1)
		mul_positions := re.FindAllStringIndex(line, -1)

		do_re := regexp.MustCompile(do_regex)
		do_matches := do_re.FindAllString(line, -1)
		do_positions := do_re.FindAllStringIndex(line, -1)

		dont_re := regexp.MustCompile(dont_regex)
		dont_matches := dont_re.FindAllString(line, -1)
		dont_positions := dont_re.FindAllStringIndex(line, -1)

		// Print the matches
		fmt.Println("Matches:", mul_matches, mul_positions)
		fmt.Println("Matches:", do_matches, do_positions)
		fmt.Println("Matches:", dont_matches, dont_positions)

		mul_starts := []int{}
		do_starts := []int{0}
		dont_starts := []int{}

		for _, val := range mul_positions {
			mul_starts = append(mul_starts, val[0])
		}

		for _, val := range do_positions {
			do_starts = append(do_starts, val[0])
		}

		for _, val := range dont_positions {
			dont_starts = append(dont_starts, val[0])
		}

		mask := []bool{}

		good := true
		// make a mask using the previous do or dont value
		for i := 0; i < len(line); i++ {
			if len(do_starts) > 0 && i == do_starts[0] {
				good = true
				do_starts = do_starts[1:]
			} else if len(dont_starts) > 0 && i == dont_starts[0] {
				good = false
				dont_starts = dont_starts[1:]
			}
			mask = append(mask, good)
		}
		// iterate through the matches and parse those that are in the good part of the mask
		for idx, start := range mul_starts {
			if mask[start] {
				sum += parse_match(mul_matches[idx])
			}
		}

	}

	return sum
}
