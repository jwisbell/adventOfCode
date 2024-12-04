package day4

import (
	"fmt"
	"testing"
)

func TestP1(t *testing.T) {
	fmt.Println("Part 1")
	array := LoadData("./test_input.txt")
	fmt.Println("memory", array)
	got := Part1(array)

	want := 18

	if got != want {
		t.Errorf("Expected %d got %d", want, got)
	}
}

func TestP2(t *testing.T) {
	fmt.Println("Part 2")
	array := LoadData("./input.txt")
	fmt.Println("memory", array)
	got := Part2(array)

	want := 9

	if got != want {
		t.Errorf("Expected %d got %d", want, got)
	}
}
