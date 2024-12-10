package day10

import (
	"fmt"
	"testing"
)

func TestP1(t *testing.T) {
	fmt.Println("Part 1")
	topo := LoadData("./input.txt")
	fmt.Println("Input", topo)
	got := Part1(topo)

	want := 36

	if got != want {
		t.Errorf("Expected %d got %d", want, got)
	}
}

func TestP2(t *testing.T) {
	fmt.Println("Part 2")
	topo := LoadData("./input.txt")
	fmt.Println("Input", topo)
	got := Part2(topo)

	want := 81

	if got != want {
		t.Errorf("Expected %d got %d", want, got)
	}
}
