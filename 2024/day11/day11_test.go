package day11

import (
	"fmt"
	"testing"
)

func TestP1(t *testing.T) {
	fmt.Println("Part 1")
	stones := LoadData("input.txt")
	fmt.Println("Input", stones)
	got := Part1(stones, 25)

	want := 55312

	if got != want {
		t.Errorf("Expected %d got %d", want, got)
	}
}

func TestP2(t *testing.T) {
	fmt.Println("Part 2")
	stones := LoadData("./input.txt")
	fmt.Println("Input", stones)
	got := Part1(stones, 75)

	want := 55312

	if got != want {
		t.Errorf("Expected %d got %d", want, got)
	}
}
