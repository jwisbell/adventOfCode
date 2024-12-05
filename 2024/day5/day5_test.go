package day5

import (
	"fmt"
	"testing"
)

func TestP1(t *testing.T) {
	fmt.Println("Part 1")
	rules, instructions := LoadData("./test_input.txt")
	fmt.Println("rules", rules)
	fmt.Println("instructions", instructions)
	got := Part1(rules, instructions)

	want := 143

	if got != want {
		t.Errorf("Expected %d got %d", want, got)
	}
}

func TestP2(t *testing.T) {
	fmt.Println("Part 2")
	rules, instructions := LoadData("./input.txt")
	fmt.Println("rules", rules)
	fmt.Println("instructions", instructions)
	got := Part2(rules, instructions)

	want := 123

	if got != want {
		t.Errorf("Expected %d got %d", want, got)
	}
}
