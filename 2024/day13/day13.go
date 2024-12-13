package day13

import (
	"bufio"
	"math"
	"os"
	"strconv"
	"strings"
)

func LoadData(fp string) []Game {
	file, _ := os.Open(fp)

	defer file.Close()

	r := bufio.NewReader(file)

	games := []Game{}

	temp_a := []int{}
	temp_b := []int{}
	temp_p := []int{}
	for {
		line, _, err := r.ReadLine()
		if len(line) > 0 {
			parts := strings.Split(string(line), ":")
			// if a
			if string(parts[0][len(parts[0])-1]) == "A" {
				nums := strings.Split(parts[1], ",")
				vals := []int{}
				for _, n := range nums {
					p := strings.Split(n, "+")
					v, err := strconv.Atoi(p[len(p)-1])
					if err == nil {
						vals = append(vals, v)
					}
				}
				temp_a = vals
			}
			// if b
			if string(parts[0][len(parts[0])-1]) == "B" {
				nums := strings.Split(parts[1], ",")
				vals := []int{}
				for _, n := range nums {
					p := strings.Split(n, "+")
					v, err := strconv.Atoi(p[len(p)-1])
					if err == nil {
						vals = append(vals, v)
					}
				}
				temp_b = vals
			}
			// if prize
			if string(parts[0][len(parts[0])-1]) == "e" {
				nums := strings.Split(parts[1], ",")
				vals := []int{}
				for _, n := range nums {
					p := strings.Split(n, "=")
					v, err := strconv.Atoi(p[len(p)-1])
					if err == nil {
						vals = append(vals, v)
					}
				}
				temp_p = vals
			}
		} else {
			// game is finished, push it
			games = append(games, Game{A: temp_a, B: temp_b, Prize: temp_p})
		}
		if err != nil {
			break
		}
	}
	return games
}

type Game struct {
	A     []int
	B     []int
	Prize []int
}

func det(A [][]int) int {
	return -1 * (A[0][0]*A[1][1] - A[0][1]*A[1][0])
}

func solve_linear_eq(A [][]int, y []int) []int {
	// Ax = y  -> x = A^-1 y
	// invert the 2x2 matrix A
	d := float64(det(A))

	// because they're 2x2, here are the combined steps
	a := float64(y[1]*A[1][0]-y[0]*A[1][1]) / d
	b := -1 * float64(y[1]*A[0][0]-y[0]*A[0][1]) / d

	// check if integer value
	if a == math.Trunc(a) && b == math.Trunc(b) {
		// valid
		return []int{int(a), int(b)}
	}

	return []int{-1, -1}
}

func Part1(games []Game, offset int) int {
	cost := 0

	// now we have our matrices, and we can compute the answers
	for _, game := range games {
		test := solve_linear_eq([][]int{game.A, game.B}, []int{game.Prize[0] + offset, game.Prize[1] + offset})
		if test[0] >= 0 && test[1] >= 0 {
			cost += 3*test[0] + test[1]
		}
	}

	return cost
}
