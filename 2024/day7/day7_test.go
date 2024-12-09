package day7

import (
	"fmt"
	"testing"
)

func TestP1(t *testing.T) {
	fmt.Println("Part 1")
	e := LoadData("./test_input.txt")
	fmt.Println("Equations", e)
	got := Part1(e, false)

	want := 3749

	if got != want {
		t.Errorf("Expected %d got %d", want, got)
	}
}

func TestP2(t *testing.T) {
	fmt.Println("Part 2")
	e := LoadData("./input.txt")
	fmt.Println("Equations", e)
	got := Part1(e, true)

	want := 11387

	if got != want {
		t.Errorf("Expected %d got %d", want, got)
	}
}
