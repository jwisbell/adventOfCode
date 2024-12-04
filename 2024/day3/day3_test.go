package day3

import (
	"fmt"
	"testing"
)

func TestP1(t *testing.T) {
	fmt.Println("Part 1")
	memory := LoadData("./test_input.txt")
	fmt.Println("memory", memory)
	got := Part1Regex(memory)

	want := 161

	if got != want {
		t.Errorf("Expected %d got %d", want, got)
	}
}

func TestP2(t *testing.T) {
	fmt.Println("Part 2")
	memory := LoadData("./input.txt")
	fmt.Println("memory", memory)
	got := Part2(memory)

	want := 48

	if got != want {
		t.Errorf("Expected %d got %d", want, got)
	}
}
