package day10

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func LoadData(fp string) [][]int {
	file, _ := os.Open(fp)

	defer file.Close()

	r := bufio.NewReader(file)

	m := [][]int{}

	for {
		line, _, err := r.ReadLine()
		if len(line) > 0 {
			parts := strings.Split(string(line), "")
			row := []int{}
			for _, p := range parts {
				i, err := strconv.Atoi(p)
				if err != nil {
					fmt.Println("Something went wrong")
				} else {
					row = append(row, i)
				}
			}
			m = append(m, row)

		}
		if err != nil {
			break
		}
	}
	return m
}

func findall(targ int, arr [][]int) [][]int {
	// return the coords of all instances of targ
	coords := [][]int{}
	for y, row := range arr {
		for x, val := range row {
			if val == targ {
				coords = append(coords, []int{x, y})
			}
		}
	}
	return coords
}

func find_path(start []int, target int, m [][]int) [][]int {
	if m[start[1]][start[0]] == target {
		return [][]int{start}
	}
	locs := [][]int{}

	// check up, down, left, right
	// if increment is 1, recursively continue
	current_val := m[start[1]][start[0]]

	// up
	if start[1] > 0 {
		if m[start[1]-1][start[0]]-current_val == 1 {
			locs = append(locs, find_path([]int{start[0], start[1] - 1}, 9, m)...)
		}
	}
	// down
	if start[1] < len(m)-1 {
		if m[start[1]+1][start[0]]-current_val == 1 {
			locs = append(locs, find_path([]int{start[0], start[1] + 1}, 9, m)...)
		}
	}

	// left
	if start[0] > 0 {
		if m[start[1]][start[0]-1]-current_val == 1 {
			locs = append(locs, find_path([]int{start[0] - 1, start[1]}, 9, m)...)
		}
	}
	// right
	if start[0] < len(m[0])-1 {
		if m[start[1]][start[0]+1]-current_val == 1 {
			locs = append(locs, find_path([]int{start[0] + 1, start[1]}, 9, m)...)
		}
	}
	return locs
}

func count_unique(arr [][]int) map[string]int {
	u := make(map[string]int)
	for _, val := range arr {
		s := fmt.Sprintf("%d,%d", val[0], val[1])
		val, ok := u[s]
		if ok {
			val++
		} else {
			u[s] = 1
		}
	}
	return u
}

func Part1(topo_map [][]int) int {
	n_trailheads := 0

	starting_locs := findall(0, topo_map)

	for _, loc := range starting_locs {
		score := find_path(loc, 9, topo_map)
		set := count_unique(score)
		n_trailheads += len(set)
	}

	return n_trailheads
}

func Part2(topo_map [][]int) int {
	n_trailheads := 0

	starting_locs := findall(0, topo_map)

	for _, loc := range starting_locs {
		score := find_path(loc, 9, topo_map)
		// set := count_unique(score)
		n_trailheads += len(score)
	}

	return n_trailheads
}
