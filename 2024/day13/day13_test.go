package day13

import (
	"fmt"
	"testing"
)

func TestP1(t *testing.T) {
	fmt.Println("Part 1")
	games := LoadData("input.txt")
	fmt.Println("Input", games)
	got := Part1(games, 0)

	want := 480

	if got != want {
		t.Errorf("Expected %d got %d", want, got)
	}
}

func TestP2(t *testing.T) {
	fmt.Println("Part 2")
	games := LoadData("input.txt")
	fmt.Println("Input", games)
	got := Part1(games, 10000000000000)

	want := 480

	if got != want {
		t.Errorf("Expected %d got %d", want, got)
	}
}
