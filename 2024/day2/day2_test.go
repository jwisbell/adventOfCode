package day2

import (
	"fmt"
	"testing"
)

func TestP1(t *testing.T) {
	fmt.Println("Part 1")
	reports := LoadData("./input_test.txt")
	fmt.Println("reports", reports)
	got := CheckReports(reports, 0)

	want := 2

	if got != want {
		t.Errorf("Expected %d got %d", want, got)
	}
}

func TestP2(t *testing.T) {
	reports := LoadData("./input.txt")
	fmt.Println("Part 2")
	got := CheckReports(reports, 1)

	want := 4

	if got != want {
		t.Errorf("Expected %d got %d", want, got)
	}
}
