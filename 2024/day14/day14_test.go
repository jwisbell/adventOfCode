package day14

import (
	"fmt"
	"testing"
)

func TestP1(t *testing.T) {
	fmt.Println("Part 1")
	r := LoadData("input.txt")
	fmt.Println("Input", r)
	got := 0 // Part1(r, []int{101, 103})

	want := 12

	if got != want {
		t.Errorf("Expected %d got %d", want, got)
	}
}

func TestP2(t *testing.T) {
	fmt.Println("Part 2")
	r := LoadData("input.txt")
	fmt.Println("Input", r)
	got := Part2(r, []int{101, 103})

	want := 1

	if got != want {
		t.Errorf("Expected %d got %d", want, got)
	}
}
