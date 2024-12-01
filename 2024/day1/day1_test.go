package day1

import (
	"testing"
)

func TestP1(t *testing.T) {
	list1, list2 := LoadLists("./test_input.txt")

	got := SumDistances(list1, list2)

	want := 11
	if want != got {
		t.Errorf("got %d want %d", got, want)
	}
}

func TestP2(t *testing.T) {
	list1, list2 := LoadLists("./input.txt")

	got := SimilarityScore(list1, list2)

	want := 31

	if want != got {
		t.Errorf("got %d want %d", got, want)
	}
}
