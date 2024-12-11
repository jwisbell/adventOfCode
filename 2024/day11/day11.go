package day11

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func LoadData(fp string) []int {
	file, _ := os.Open(fp)

	defer file.Close()

	r := bufio.NewReader(file)

	stones := []int{}

	for {
		line, _, err := r.ReadLine()
		if len(line) > 0 {
			parts := strings.Split(string(line), " ")
			row := []int{}
			for _, p := range parts {
				i, err := strconv.Atoi(p)
				if err != nil {
					fmt.Println("Something went wrong")
				} else {
					row = append(row, i)
				}
			}
			stones = row
		}
		if err != nil {
			break
		}
	}
	return stones
}

func dfs(stone, turn, stop_turn int, memoize map[int][]int) int {
	n_stones := 0
	if turn == stop_turn {
		return 1
	}

	new_stones := []int{}

	// process the current stone and generate any new stones if needed based on the rules
	if stone == 0 {
		new_stones = append(new_stones, 1)
	} else {
		val := strconv.Itoa(stone)
		if len(val)%2 == 0 && len(val) >= 2 {
			// split it
			left := val[:len(val)/2]
			right := val[len(val)/2:]
			l, err := strconv.Atoi(left)
			if err == nil {
				new_stones = append(new_stones, l)
			}
			r, err := strconv.Atoi(right)
			if err == nil {
				new_stones = append(new_stones, r)
			}
		} else {
			new_stones = append(new_stones, stone*2024)
		}
	}

	for _, s := range new_stones {

		val, ok := memoize[s]
		if ok {
			if val[turn] != 0 {
				// fmt.Println("wow a repeat")
				n_stones += val[turn]
			} else {
				res := dfs(s, turn+1, stop_turn, memoize)
				memoize[s][turn] = res
				n_stones += res
			}
		} else {
			res := dfs(s, turn+1, stop_turn, memoize)
			memoize[s] = make([]int, stop_turn)
			memoize[s][turn] = res
			n_stones += res
		}

	}

	return n_stones
}

func Part1(stones []int, stop_turn int) int {
	n_stones := 0
	memoize := make(map[int][]int)
	turn := 0
	for _, stone := range stones {
		n_stones += dfs(stone, turn, stop_turn, memoize)
	}

	return n_stones
}
