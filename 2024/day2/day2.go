package day2

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Report struct {
	values []int
}

func LoadData(fp string) []Report {
	file, _ := os.Open(fp)

	defer file.Close()

	r := bufio.NewReader(file)

	reports := []Report{}

	for {
		line, _, err := r.ReadLine()
		if len(line) > 0 {
			vals := strings.Fields(string(line))
			// convert each value to an int and put into a report
			int_vals := []int{}
			for _, v := range vals {
				iv, _ := strconv.Atoi(v)
				int_vals = append(int_vals, iv)
			}
			reports = append(reports, Report{values: int_vals})
		}
		if err != nil {
			break
		}
	}

	return reports
}

func reverseArray(arr []int) []int {
	for i, j := 0, len(arr)-1; i < j; i, j = i+1, j-1 {
		arr[i], arr[j] = arr[j], arr[i]
	}
	return arr
}

func slowSort(arr1 []int, ascending bool) ([]int, int) {
	// sort the array and return how many shifts it took
	nswap := 0
	new_arr := make([]int, len(arr1))
	copy(new_arr, arr1)
	for {
		swapped := false
		for i, v := range new_arr[:len(new_arr)-1] {
			if ascending {
				if v > new_arr[i+1] {
					swapped = true
					new_arr[i] = new_arr[i+1]
					new_arr[i+1] = v
					nswap += 1
				}
			} else {
				if v < new_arr[i+1] {
					swapped = true
					new_arr[i] = new_arr[i+1]
					new_arr[i+1] = v
					nswap += 1
				}
			}
		}
		if !swapped {
			// done
			break
		}
	}
	return new_arr, nswap
}

func checkArray(arr []int) int {
	// returns whether an array is sorted, optionally with tolerance for ONE shift
	nerr := 0
	prev := arr[0]

	for _, v := range arr[1:] {
		diff := v - prev
		abs_diff := max(diff, -diff)

		if abs_diff <= 3 && abs_diff != 0 {
			// okay
		} else {
			nerr += 1
		}
		prev = v

	}
	return nerr
}

func CheckReports(reports []Report, dampener int) int {
	// iterate over the entries of each report and check if
	// The levels are either all increasing or all decreasing.
	// Any two adjacent levels differ by at least one and at most three.

	count := 0
	for _, r := range reports {
		// sort the array and see if it (or the reverse) is equal to the original
		fmt.Println("report", r.values)
		s1, c1 := slowSort(r.values, true)
		s2, c2 := slowSort(r.values, false)

		// need to check if the default entries are correct
		// if not, remove up to one entry and check

		// initial checck
		sorted := make([]int, len(r.values))
		nerr := 0
		if c2 < c1 {
			sorted = s2
			nerr = c2
		} else {
			sorted = s1
			nerr = c1
		}

		nerr += checkArray(sorted)
		fmt.Println("errors", nerr)
		if nerr == 0 {
			count += 1
		} else if dampener > 0 {
			// now we have to iteratively check what happens when an element is removed
			for i := 0; i < len(r.values); i++ {
				temp_arr := []int{}
				temp_arr = append(temp_arr, r.values[:i]...)
				temp_arr = append(temp_arr, r.values[i+1:]...)

				fmt.Println(temp_arr, r.values, "what", r.values[i])
				s1, c1 := slowSort(temp_arr, true)
				s2, c2 := slowSort(temp_arr, false)

				sorted := make([]int, len(r.values))
				nerr := 0
				if c2 < c1 {
					sorted = s2
					nerr = c2
				} else {
					sorted = s1
					nerr = c1
				}
				nerr += checkArray(sorted)
				fmt.Println("errors", nerr)
				if nerr == 0 {
					count += 1
					break
				}
			}
		}

	}

	return count
}
