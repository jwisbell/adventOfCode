package day1

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

func LoadLists(fp string) ([]int, []int) {
	file, _ := os.Open(fp)

	defer file.Close()

	r := bufio.NewReader(file)

	list1 := []int{}
	list2 := []int{}

	for {
		line, _, err := r.ReadLine()
		if len(line) > 0 {
			vals := strings.Fields(string(line))
			i1, _ := strconv.Atoi(vals[0])
			i2, _ := strconv.Atoi(vals[1])
			list1 = append(list1, i1)
			list2 = append(list2, i2)
		}
		if err != nil {
			break
		}
	}

	return list1, list2
}

func SumDistances(list1, list2 []int) int {
	// need to match the smallest in each list
	// then 2nd smallest, etc
	// and compute the difference between them
	// naive idea is to sort them
	sort.Ints(list1)
	sort.Ints(list2)
	// the above are in ascending order
	// differences := []int{}
	sum := 0

	for i := 0; i < len(list1); i++ {
		v1 := list1[i] - list2[i]
		sum += max(v1, -v1)
		// differences = append(differences,  max(v1, -v1) ) //store-brand absolute value
	}

	return sum
}

func SimilarityScore(list1, list2 []int) int {
	//sum up values in left list x frequency in right list
	// i will use a map
	//
	//initialize the map

	left_freq := make(map[int]int)
	right_freq := make(map[int]int)

	for _, lv := range list1 {
		val, ok := left_freq[lv]
		if !ok {
			left_freq[lv] = 1
			right_freq[lv] = 0
		} else {
			left_freq[lv] = val + 1
		}

	}

	for _, rv := range list2 {
		val, ok := right_freq[rv]
		if ok {
			right_freq[rv] = val + 1
		}
	}

	sum := 0
	for k, v := range left_freq {
		sum += k * v * right_freq[k]
	}

	fmt.Println("Here is the map: ", left_freq, right_freq)

	return sum
}

func main() {
	LoadLists("./test_input.txt")
}
