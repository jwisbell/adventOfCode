package day4

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

	array := [][]string{}

	for {
		line, _, err := r.ReadLine()
		if len(line) > 0 {
			array = append(array, strings.Split(string(line), ""))
		}
		if err != nil {
			break
		}
	}

	return array
}

func search(x, y int, letter string, grid [][]string, direction string) bool {
	// search for the next letter around the position x,y in the direction
	found := false
	next_letter := ""
	if x < 0 || y < 0 {
		return false
	} else if x >= len(grid[0]) || y >= len(grid) {
		return false
	}

	switch letter {
	case "X":
		next_letter = "M"
	case "M":
		next_letter = "A"
	case "A":
		next_letter = "S"
	case "S":
		next_letter = ""
	default:
		return true
	}

	switch direction {
	case "up":

		if grid[max(y-1, 0)][x] == letter {
			found = search(x, y-1, next_letter, grid, direction)
		}
	case "down":
		if grid[min(y+1, len(grid)-1)][x] == letter {
			found = search(x, y+1, next_letter, grid, direction)
		}
	case "left":
		if grid[y][max(x-1, 0)] == letter {
			found = search(x-1, y, next_letter, grid, direction)
		}
	case "right":
		if grid[y][min(x+1, len(grid[0])-1)] == letter {
			found = search(x+1, y, next_letter, grid, direction)
		}
	case "ur":
		if grid[max(y-1, 0)][min(x+1, len(grid[0])-1)] == letter {
			found = search(x+1, y-1, next_letter, grid, direction)
		}
	case "lr":
		if grid[min(y+1, len(grid)-1)][min(x+1, len(grid[0])-1)] == letter {
			found = search(x+1, y+1, next_letter, grid, direction)
		}
	case "ul":
		if grid[max(0, y-1)][max(0, x-1)] == letter {
			found = search(x-1, y-1, next_letter, grid, direction)
		}
	case "ll":
		if grid[min(y+1, len(grid)-1)][max(x-1, 0)] == letter {
			found = search(x-1, y+1, next_letter, grid, direction)
		}
	}
	return found
}

func lookAround(x, y int, grid [][]string) int {
	count := 0
	// fmt.Println("starting vals ", x, y)
	for _, dx := range []int{-1, 0, 1} {
		for _, dy := range []int{-1, 0, 1} {
			if dx == -1 && x == 0 {
				continue
			} else if dx == 1 && x >= len(grid[0])-1 {
				continue
			}
			if dy == -1 && y == 0 {
				continue
			} else if dy == 1 && y >= len(grid)-1 {
				continue
			}
			if dx == 0 && dy == 0 {
				continue
			}

			// fmt.Println("Vals ", x+dx, y+dy)
			if grid[y+dy][x+dx] == "M" {
				// found! figure out the direction
				// fmt.Println("found an M?", dx, dy)
				if dx == -1 && dy == 0 {
					fmt.Println("left")
					complete := search(x, y, "M", grid, "left")
					if complete {
						count += 1
					}
				} else if dx == 1 && dy == 0 {
					fmt.Println("right")
					complete := search(x, y, "M", grid, "right")
					if complete {
						count += 1
					}
				} else if dx == 0 && dy == -1 {
					complete := search(x, y, "M", grid, "up")
					if complete {
						count += 1
					}
				} else if dx == 0 && dy == 1 {
					complete := search(x, y, "M", grid, "down")
					if complete {
						count += 1
					}
				} else if dx == 1 && dy == 1 {
					complete := search(x, y, "M", grid, "lr")
					if complete {
						count += 1
					}
				} else if dx == -1 && dy == 1 {
					complete := search(x, y, "M", grid, "ll")
					if complete {
						count += 1
					}
				} else if dx == 1 && dy == -1 {
					complete := search(x, y, "M", grid, "ur")
					if complete {
						count += 1
					}
				} else if dx == -1 && dy == -1 {
					complete := search(x, y, "M", grid, "ul")
					if complete {
						count += 1
					}
				}
			}
		}
	}

	return count
}

func lookAroundA(x, y int, grid [][]string) int {
	count := 0

	// center of the x can't be on the edge
	if x == 0 || x == len(grid[0])-1 {
		return 0
	}
	if y == 0 || y == len(grid)-1 {
		return 0
	}

	// we have a limited number of cases

	/*
	  [M, ., M]
	  [., A, .]
	  [S, ., S]
	*/
	if grid[y-1][x-1] == "M" && grid[y-1][x+1] == "M" && grid[y+1][x-1] == "S" && grid[y+1][x+1] == "S" {
		count += 1
	}

	/*
	  [M, ., S]
	  [., A, .]
	  [M, ., S]
	*/

	if grid[y-1][x-1] == "M" && grid[y-1][x+1] == "S" && grid[y+1][x-1] == "M" && grid[y+1][x+1] == "S" {
		count += 1
	}
	/*
	  [S, ., M]
	  [., A, .]
	  [S, ., M]
	*/

	if grid[y-1][x-1] == "S" && grid[y-1][x+1] == "M" && grid[y+1][x-1] == "S" && grid[y+1][x+1] == "M" {
		count += 1
	}
	/*
	  [S, ., S]
	  [., A, .]
	  [M, ., M]
	*/
	if grid[y-1][x-1] == "S" && grid[y-1][x+1] == "S" && grid[y+1][x-1] == "M" && grid[y+1][x+1] == "M" {
		count += 1
	}
	return count
}

func Part1(grid [][]string) int {
	// need to look for XMAS spelled in any direction (incl diagonals and backwards)
	// idea is to find each X and then look around it recursively for the next letter
	count := 0

	//test_amount := lookAround(4, 1, grid)
	//test_found := search(4, 1, "M", grid, "left")
	//fmt.Println("did it work?", test_found, test_amount)
	//
	pretty_grid := [][]string{}
	for _, row := range grid {
		new_row := []string{}
		for range row {
			new_row = append(new_row, ".")
		}
		pretty_grid = append(pretty_grid, new_row)
	}

	for y, row := range grid {
		for x, val := range row {
			if val == "X" {
				// figure out if an M is adjacent and in which direction
				c := lookAround(x, y, grid)
				count += c
				if c > 0 {
					pretty_grid[y][x] = "X"
				} else {
					pretty_grid[y][x] = "."
				}
			} else {
				pretty_grid[y][x] = "."
			}
		}
	}

	for _, row := range pretty_grid {
		fmt.Println(row)
	}

	return count
}

func Part2(grid [][]string) int {
	// need to look for MAS in an X shape

	count := 0

	// similar to part 1, look for a letter and then continue outward
	// in all cases, there's an A in the middle -- so limit search to As that have two M and two S nearby

	pretty_grid := [][]string{}
	for _, row := range grid {
		new_row := []string{}
		for range row {
			new_row = append(new_row, ".")
		}
		pretty_grid = append(pretty_grid, new_row)
	}

	for y, row := range grid {
		for x, val := range row {
			if val == "A" {
				// figure out if adjacent letters work
				c := lookAroundA(x, y, grid)
				count += c
				if c > 0 {
					pretty_grid[y][x] = "A"
				} else {
					pretty_grid[y][x] = "."
				}
			} else {
				pretty_grid[y][x] = "."
			}
		}
	}
	return count
}
