package day6

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func LoadData(fp string) [][]string {
	file, _ := os.Open(fp)

	defer file.Close()

	r := bufio.NewReader(file)

	m := [][]string{}

	for {
		line, _, err := r.ReadLine()
		if len(line) > 0 {
			row := strings.Split(string(line), "")
			m = append(m, row)
		}
		if err != nil {
			break
		}
	}
	return m
}

func find_all(s string, m [][]string) [][]int {
	// return x, y of guard
	spots := [][]int{}
	for y, row := range m {
		for x, val := range row {
			if val == s {
				spots = append(spots, []int{x, y})
			}
		}
	}
	return spots
}

func find_guard(m [][]string) (int, int) {
	// return x, y of guard

	for y, row := range m {
		for x, val := range row {
			if val == "^" || val == "v" || val == ">" || val == "<" {
				return x, y
			}
		}
	}
	return -1, -1
}

func Part1(m [][]string) (int, [][]string) {
	n_uniq := 0

	// we have to make the guard traverse the map and record unique positions
	// go until she leaves the area

	startx, starty := find_guard(m)
	if startx < 0 || starty < 0 {
		return 0, nil
	}

	// start an infinite loop
	x := startx
	y := starty
	exited := false
	for {
		switch m[y][x] {
		case "^":
			if y == 0 {
				// we've reached the edge
				m[y][x] = "+"
				exited = true
			} else {
				if m[y-1][x] == "#" {
					// turn right
					m[y][x] = ">"
				} else {
					m[y][x] = "+"
					m[y-1][x] = "^"
					y = y - 1
				}
			}
		case ">":
			if x == len(m[0])-1 {
				// we've reached the edge
				m[y][x] = "+"
				exited = true
			} else {
				if m[y][x+1] == "#" {
					// turn right
					m[y][x] = "v"
				} else {
					m[y][x] = "+"
					m[y][x+1] = ">"
					x = x + 1
				}
			}
		case "v":
			if y == len(m)-1 {
				// we've reached the edge
				m[y][x] = "+"
				exited = true
			} else {
				if m[y+1][x] == "#" {
					// turn right
					m[y][x] = "<"
				} else {
					m[y][x] = "+"
					m[y+1][x] = "v"
					y = y + 1
				}
			}
		case "<":
			if x == 0 {
				// we've reached the edge
				m[y][x] = "+"
				exited = true
			} else {
				if m[y][x-1] == "#" {
					// turn right
					m[y][x] = "^"
				} else {
					m[y][x] = "+"
					m[y][x-1] = "<"
					x = x - 1
				}
			}
		}
		if exited {
			break
		}
	}

	for _, row := range m {
		fmt.Println(row)
		for _, val := range row {
			if val == "+" {
				n_uniq = n_uniq + 1
			}
		}
	}

	return n_uniq, m
}

func traverse(startx, starty int, initial_map [][]string) bool {
	// start an infinite loop
	x := startx
	y := starty
	exited := false
	m := make([][]string, len(initial_map))
	copy(m, initial_map)
	counts := [][]int{}
	for range m {
		counts = append(counts, make([]int, len(m[0])))
	}
	for idx, row := range counts {
		for jdx := range row {
			counts[idx][jdx] = 0
		}
	}

	counts[y][x] = 1
	cycle := false
	niter := 0

	for {
		if counts[y][x] > 10 {
			// we're in a cycle!
			cycle = true
			break
		}
		switch m[y][x] {
		case "^":
			if y == 0 {
				// we've reached the edge
				m[y][x] = "+"
				exited = true
			} else {
				if m[y-1][x] == "#" {
					// turn right
					m[y][x] = ">"
				} else {
					m[y][x] = "+"
					m[y-1][x] = "^"
					counts[y-1][x] += 1
					y = y - 1
				}
			}
		case ">":
			if x == len(m[0])-1 {
				// we've reached the edge
				m[y][x] = "+"
				exited = true
			} else {
				if m[y][x+1] == "#" {
					// turn right
					m[y][x] = "v"
				} else {
					m[y][x] = "+"
					m[y][x+1] = ">"
					counts[y][x+1] += 1
					x = x + 1
				}
			}
		case "v":
			if y == len(m)-1 {
				// we've reached the edge
				m[y][x] = "+"
				exited = true
			} else {
				if m[y+1][x] == "#" {
					// turn right
					m[y][x] = "<"
				} else {
					m[y][x] = "+"
					m[y+1][x] = "v"
					counts[y+1][x] += 1
					y = y + 1
				}
			}
		case "<":
			if x == 0 {
				// we've reached the edge
				m[y][x] = "+"
				exited = true
			} else {
				if m[y][x-1] == "#" {
					// turn right
					m[y][x] = "^"
				} else {
					m[y][x] = "+"
					m[y][x-1] = "<"
					counts[y][x-1] += 1
					x = x - 1
				}
			}
		}
		niter += 1
		if exited {
			break
		}
		if niter > 60000 {
			break
		}
	}

	fmt.Println("Result of the run", cycle)
	for _, row := range m {
		fmt.Println(row)
	}
	return cycle
}

func Part2(m [][]string, traversed_m [][]string) int {
	n_pos := 0

	// we have to make the guard traverse the map and record unique positions
	// go until she leaves the area

	startx, starty := find_guard(m)
	if startx < 0 || starty < 0 {
		return 0
	}

	// first, identify all the spots from part 1 as possible obstacle locations
	spots := find_all("+", traversed_m)
	new_map := make([][]string, len(m))

	for _, spot := range spots {
		m = LoadData("./input.txt")
		copy(new_map, m)
		if spot[1] == starty && spot[0] == startx {
			continue
		}

		new_map[spot[1]][spot[0]] = "#"
		fmt.Println("testing spot", spot)
		for _, row := range new_map {
			fmt.Println(row)
		}

		if traverse(startx, starty, new_map) {
			n_pos += 1
		}

	}

	return n_pos
}
