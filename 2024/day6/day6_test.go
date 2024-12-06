package day6

import (
	"fmt"
	"testing"
)

func TestP1(t *testing.T) {
	fmt.Println("Part 1")
	m := LoadData("./test_input.txt")
	fmt.Println("map", m)
	got, _ := Part1(m)

	want := 41

	if got != want {
		t.Errorf("Expected %d got %d", want, got)
	}
}

func TestP2(t *testing.T) {
	fmt.Println("Part 2")
	m := LoadData("./input.txt")
	fmt.Println("map", m)
	_, traversed_m := Part1(m)
	m = LoadData("./input.txt") // reload to be safe
	got := Part2(m, traversed_m)

	want := 6

	if got != want {
		t.Errorf("Expected %d got %d", want, got)
	}
}
