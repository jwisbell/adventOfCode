package day9

import (
	"fmt"
	"testing"
)

func TestP1(t *testing.T) {
	fmt.Println("Part 1")
	file_blocks := LoadData("./test_input.txt")
	fmt.Println("Input", file_blocks)
	got := Part1(file_blocks)

	want := 1928

	if got != want {
		t.Errorf("Expected %d got %d", want, got)
	}
}

func TestP2(t *testing.T) {
	fmt.Println("Part 2")
	file_blocks := LoadData("./test_input.txt")
	// fmt.Println("Input", file_blocks)
	got := Part2(file_blocks)

	want := 2858

	if got != want {
		t.Errorf("Expected %d got %d", want, got)
	}
}
