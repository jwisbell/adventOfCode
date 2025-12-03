package day14

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func LoadData(fp string) []Robot {
	file, _ := os.Open(fp)

	defer file.Close()

	r := bufio.NewReader(file)

	robots := []Robot{}
	for {
		line, _, err := r.ReadLine()
		if len(line) > 0 {
			parts := strings.Split(string(line), " ")
			spos := strings.Split(parts[0][2:], ",")
			svel := strings.Split(parts[1][2:], ",")

			pos := []int{}
			vel := []int{}
			for i := range spos {
				v, err := strconv.Atoi(spos[i])
				if err == nil {
					pos = append(pos, v)
				}
				v, err = strconv.Atoi(svel[i])
				if err == nil {
					vel = append(vel, v)
				}
			}
			if len(vel) == 2 {
				robots = append(robots, Robot{pos, vel})
			}
		}
		if err != nil {
			break
		}
	}
	return robots
}

type Robot struct {
	pos []int
	vel []int
}

func (r *Robot) iterate(bounds []int, step int) []int {
	// integrate the velocity
	new_pos := []int{r.pos[0] + step*r.vel[0], r.pos[1] + step*r.vel[1]}
	// use modulus to keep in bounds since we have pacman rules
	new_pos[0] = (new_pos[0]%bounds[0] + bounds[0]) % bounds[0]
	new_pos[1] = (new_pos[1]%bounds[1] + bounds[1]) % bounds[1]

	// finally save the new pos
	return new_pos
}

func count_in_quadrants(robots []Robot, bounds []int) int {
	m := [][]int{}
	for y := 0; y < bounds[1]; y++ {
		row := []int{}
		for x := 0; x < bounds[0]; x++ {
			row = append(row, 0)
		}
		m = append(m, row)
	}

	quadrants := []int{0, 0, 0, 0}

	for _, r := range robots {
		m[r.pos[1]][r.pos[0]] += 1
		if r.pos[0] < bounds[0]/2 && r.pos[1] < bounds[1]/2 {
			quadrants[0] += 1
		} else if r.pos[0] > bounds[0]/2 && r.pos[1] < bounds[1]/2 {
			quadrants[1] += 1
		} else if r.pos[0] < bounds[0]/2 && r.pos[1] > bounds[1]/2 {
			quadrants[2] += 1
		} else if r.pos[0] > bounds[0]/2 && r.pos[1] > bounds[1]/2 {
			quadrants[3] += 1
		}
	}

	for _, row := range m {
		fmt.Println(row)
	}
	fmt.Println(quadrants)
	return quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3]
}

func Part1(robots []Robot, bounds []int) int {
	for i := range robots {
		new_pos := robots[i].iterate(bounds, 100)
		robots[i].pos = new_pos
	}
	return count_in_quadrants(robots, bounds)
}

func Part2(robots []Robot, bounds []int) int {
	step := 0
	for {
		m := [][]int{}
		for y := 0; y < bounds[1]; y++ {
			row := []int{}
			for x := 0; x < bounds[0]; x++ {
				row = append(row, 0)
			}
			m = append(m, row)
		}
		for i := range robots {
			new_pos := robots[i].iterate(bounds, 1)
			robots[i].pos = new_pos
			m[new_pos[1]][new_pos[0]] += 1
		}
		if identify_unique(m) {
			for _, row := range m {
				fmt.Println(row)
			}
			step += 1
			break
		}
		step += 1
		if step > 10000 {
			break
		}
	}
	return step
}

func identify_unique(data [][]int) bool {
	n_repeats := 0
	for _, row := range data {
		for _, val := range row {
			if val > 1 {
				n_repeats += 1
			}
		}
	}
	return n_repeats < 2
}
