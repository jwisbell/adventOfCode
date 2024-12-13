package day12

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
			parts := strings.Split(string(line), "")
			row := []string{}
			for _, p := range parts {
				row = append(row, p)
			}
			m = append(m, row)
		}
		if err != nil {
			break
		}
	}
	return m
}

type Loc struct {
	group    string
	Perim    int
	n_corner int
}

func DFS(start []int, m [][]string, locations map[string]Loc, group string) {
	cs := fmt.Sprintf("%d,%d", start[0], start[1])
	_, ok := locations[cs]
	if ok {
		return
	}

	l := Loc{Perim: 0, group: group, n_corner: 0}
	locations[cs] = l
	// check up, down, left, right
	// if increment is 1, recursively continue
	current_val := m[start[1]][start[0]]
	perim := 0

	sides := make([]bool, 4)
	// up
	if start[1] > 0 {
		if m[start[1]-1][start[0]] == current_val {
			DFS([]int{start[0], start[1] - 1}, m, locations, group)
		} else {
			perim += 1
			sides[0] = true
		}
	} else {
		perim += 1
		sides[0] = true
	}
	// down
	if start[1] < len(m)-1 {
		if m[start[1]+1][start[0]] == current_val {
			DFS([]int{start[0], start[1] + 1}, m, locations, group)
		} else {
			perim += 1
			sides[1] = true
		}
	} else {
		perim += 1
		sides[1] = true
	}

	// left
	if start[0] > 0 {
		if m[start[1]][start[0]-1] == current_val {
			DFS([]int{start[0] - 1, start[1]}, m, locations, group)
		} else {
			perim += 1
			sides[2] = true
		}
	} else {
		perim += 1
		sides[2] = true
	}
	// right
	if start[0] < len(m[0])-1 {
		if m[start[1]][start[0]+1] == current_val {
			DFS([]int{start[0] + 1, start[1]}, m, locations, group)
		} else {
			perim += 1
			sides[3] = true
		}
	} else {
		perim += 1
		sides[3] = true
	}

	n_corner := 0
	// now i need to figure out if it's a corner
	if perim == 2 {
		if (sides[0] && sides[2]) || (sides[0] && sides[3]) || (sides[1] && sides[2]) || (sides[1] && sides[3]) {
			n_corner = 1
		}
	} else if perim == 3 {
		n_corner = 2
	} else if perim == 4 {
		n_corner = 4
	}
	if perim >= 1 {
		// now it gets tricky...
		// check clockwise around interior angles so that they aren't double counted
		if sides[0] && start[0] > 0 && start[1] > 0 {
			if m[start[1]-1][start[0]-1] == current_val {
				n_corner += 1
			}
		}
		if sides[1] && start[0] < len(m[0])-1 && start[1] < len(m)-1 {
			if m[start[1]+1][start[0]+1] == current_val {
				n_corner += 1
			}
		}
		if sides[2] && start[0] > 0 && start[1] < len(m)-1 {
			if m[start[1]+1][start[0]-1] == current_val {
				n_corner += 1
			}
		}
		if sides[3] && start[1] > 0 && start[0] < len(m[0])-1 {
			if m[start[1]-1][start[0]+1] == current_val {
				n_corner += 1
			}
		}
	}

	locations[cs] = Loc{Perim: perim, group: group, n_corner: n_corner}
}

func Part1(m [][]string) int {
	cost := 0

	//want to find the area and perimeter of arbitrary regions
	//will probably do a DFS to complete a group
	//each member will store the number of adjacent plots that are NOT in the group, for perimeter calc
	//it doesnt look like diagonals are allowed
	//
	//need to use a stack/heap to prevent processing squares more than once
	//or use a map where map[coords] = Loc{group:1, perim:0}
	//maybe also/instead use a map[group] = []Loc to keep track of members

	locations := make(map[string]Loc)
	for _, row := range m {
		fmt.Println(row)
	}

	group_number := 0
	for y, row := range m {
		for x := range row {
			str_coords := fmt.Sprintf("%d,%d", x, y)
			_, ok := locations[str_coords]
			if !ok {
				group_number += 1
				DFS([]int{x, y}, m, locations, fmt.Sprintf("%d", group_number))
			}
		}
	}

	groups := make(map[string][]int)
	for _, loc := range locations {
		_, ok := groups[loc.group]
		if ok {
			groups[loc.group][0] += 1
			groups[loc.group][1] += loc.Perim
		} else {
			groups[loc.group] = []int{1, loc.Perim}
		}
	}

	// finally calculate the cost for each group
	for name, stats := range groups {
		fmt.Println("Name", name, " cost ", stats[1]*stats[0])
		cost += stats[1] * stats[0]
	}

	fmt.Println("test", locations)

	return cost
}

func Part2(m [][]string) int {
	cost := 0

	//want to find the area and perimeter of arbitrary regions
	//will probably do a DFS to complete a group
	//each member will store the number of adjacent plots that are NOT in the group, for perimeter calc
	//it doesnt look like diagonals are allowed
	//
	//need to use a stack/heap to prevent processing squares more than once
	//or use a map where map[coords] = Loc{group:1, perim:0}
	//maybe also/instead use a map[group] = []Loc to keep track of members

	locations := make(map[string]Loc)
	for _, row := range m {
		fmt.Println(row)
	}

	group_number := 0
	for y, row := range m {
		for x := range row {
			str_coords := fmt.Sprintf("%d,%d", x, y)
			_, ok := locations[str_coords]
			if !ok {
				group_number += 1
				DFS([]int{x, y}, m, locations, fmt.Sprintf("%d", group_number))
			}
		}
	}

	groups := make(map[string][]int)
	for cs, loc := range locations {
		_, ok := groups[loc.group]
		if loc.group == "2" {
			fmt.Println(cs, loc.n_corner)
		}
		if ok {
			groups[loc.group][0] += 1
			groups[loc.group][1] += loc.n_corner
			// basically instead of adding to the perim, i need to count the number of corners
		} else {
			groups[loc.group] = []int{1, loc.n_corner}
		}
	}

	// finally calculate the cost for each group
	for name, stats := range groups {
		fmt.Println("Name", name, " cost ", stats[1])
		cost += stats[1] * stats[0]
	}

	fmt.Println("test", locations)

	return cost
}
