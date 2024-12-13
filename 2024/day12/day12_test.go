package day12

import (
	"fmt"
	"testing"
)

func TestP1(t *testing.T) {
	fmt.Println("Part 1")
	m := LoadData("test_input.txt")
	fmt.Println("Input", m)
	got := Part1(m)

	want := 1930

	if got != want {
		t.Errorf("Expected %d got %d", want, got)
	}
}

func TestP2(t *testing.T) {
	fmt.Println("Part 2")
	m := LoadData("test_input.txt")
	fmt.Println("Input", m)
	got := Part2(m)

	want := 1206

	if got != want {
		t.Errorf("Expected %d got %d", want, got)
	}
}
