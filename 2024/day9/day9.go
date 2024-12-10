package day9

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func LoadData(fp string) []int {
	file, _ := os.Open(fp)

	defer file.Close()

	r := bufio.NewReader(file)

	entries := []int{}

	for {
		line, _, err := r.ReadLine()
		if len(line) > 0 {
			parts := strings.Split(string(line), "")
			for _, p := range parts {
				i, err := strconv.Atoi(p)
				if err != nil {
					fmt.Println("Something went wrong")
				} else {
					entries = append(entries, i)
				}

			}

		}
		if err != nil {
			break
		}
	}
	return entries
}

func Part1(disk_map []int) int {
	checksum := 0
	// now we need to go through each entry and parse whether it's a file block or empty space
	// we then need to build up ids and locations of file blocks (ids start from 0)
	// length of free space and files alternate

	memory := []int{}
	current_id := 0
	num_empty := 0
	for idx, val := range disk_map {
		if idx%2 == 0 {
			// this is a file block
			for i := 0; i < val; i++ {
				memory = append(memory, current_id)
			}
			current_id++
		} else {
			for i := 0; i < val; i++ {
				memory = append(memory, -1)
				num_empty += 1
			}
		}
	}
	fmt.Println("This is the uncompressed memory representation", memory, num_empty)

	compressed := []int{}
	// now i need to iteratively take the last file block entry and put it into the FIRST available memory space (-1)
	// this generates a new, "compressed" memory
	for {
		if len(memory) > 0 {
			// fmt.Println("memory", memory)
			val := memory[0]
			if val == -1 {
				new_val := memory[len(memory)-1]
				memory = memory[:len(memory)-1]
				if new_val != -1 {
					compressed = append(compressed, new_val)
					memory = memory[1:]
				}
			} else {
				compressed = append(compressed, val)
				memory = memory[1:]
			}
		} else {
			break
		}
	}

	fmt.Println("This is the compressed memory representation", compressed)
	// finally calculate the checksum
	for idx, val := range compressed {
		checksum += idx * val
	}

	return checksum
}

type File struct {
	Id    int
	Size  int
	After int
	Next  int
	Idx   int
}

type Item struct {
	Id     int
	Before int
	Size   int
}

func Part2(disk_map []int) int {
	checksum := 0
	// now we need to go through each entry and parse whether it's a file block or empty space
	// we then need to build up ids and locations of file blocks (ids start from 0)
	// length of free space and files alternate

	memory := []int{}
	files := []File{}
	ll := []Item{}

	current_id := 0
	num_empty := 0
	prev := -1

	for idx, val := range disk_map {
		if idx%2 == 0 {
			ll = append(ll, Item{Id: current_id, Before: prev, Size: val})
			// this is a file block
			for i := 0; i < val; i++ {
				memory = append(memory, current_id)
			}
			prev = current_id
			current_id++
		} else {
			// files = append(files, File{Id: current_id - 1, Size: disk_map[idx-1], After: val, Next: current_id, Idx: 0})
			ll = append(ll, Item{Id: -current_id, Before: prev, Size: val})
			for i := 0; i < val; i++ {
				memory = append(memory, -1)
				num_empty += 1
			}
			prev = -current_id
		}
	}
	if len(disk_map)%2 != 0 {
		files = append(files, File{Id: current_id - 1, Size: disk_map[len(disk_map)-1], After: -1, Idx: 0})
	}
	fmt.Println("This is the uncompressed memory representation", ll)

	// compressed := []int{}
	// the below isn't working -- i tried to be clever, but now I'll brute force it
	// fmt.Println(files)
	for {
		if len(files) > 0 {
			next := files[len(files)-1]
			fmt.Println("Processing", next)
			files = files[:len(files)-1]
			gap_size := 0
			gap_start := 0
			for idx := 0; idx < next.Idx; idx++ {
				// search for a big enough gap
				if memory[idx] == -1 {
					if gap_size == 0 {
						gap_start = idx
					}
					gap_size += 1
				} else {
					if gap_size >= next.Size {
						// place the file block here
						for jdx := gap_start; jdx < gap_start+next.Size; jdx++ {
							memory[jdx] = next.Id
						}
						for jdx := next.Idx; jdx < next.Idx+next.Size; jdx++ {
							memory[jdx] = -1
						}
						fmt.Println("found a gap! ", gap_start, memory)
						break
					}
					gap_size = 0
					gap_start = 0
				}
			}
		} else {
			break
		}
	}

	// i really think a linked list is the way to go ...

	//fmt.Println(memory)
	/*
		for {
			did_move := false
			// start from the back, find the leftmost place it can fit
			for idx := len(files) - 1; idx > 0; idx-- {
				current := files[idx]
				// fmt.Println("current", current)
				// fmt.Println("prev", files[idx-1])
				for jdx, val := range files[:idx] {
					if val.After >= current.Size {
						// move current to follow val and update the "after" fields
						// fmt.Println("current goes after val", current.Id, val.Id, "next val", current.Next, val.Next)
						files = append(files[:idx], files[idx+1:]...)
						// fmt.Println("After removal", files)
						files = append(files[:jdx+1], files[jdx:]...)
						files[jdx+1] = current
						// fmt.Println(files)
						files[jdx].After = 0
						files[jdx+1].After = val.After - current.Size
						files[idx].After += files[jdx+1].Size + current.After
						// fmt.Println(files)
						did_move = true
						break
					}
				}
			}
			if !did_move {
				// everything is in place??
				break
			}
			break
		}
		// fmt.Println(files)
		for _, f := range files {
			for i := 0; i < f.Size; i++ {
				compressed = append(compressed, f.Id)
			}
			for i := 0; i < f.After; i++ {
				compressed = append(compressed, -1)
			}
		}

		fmt.Println("This is the compressed memory representation")
		fmt.Println("what")
		fmt.Println(compressed)
	*/
	// finally calculate the checksum
	for idx, val := range memory {
		if val != -1 {
			checksum += idx * val
		}
	}
	return checksum
}
