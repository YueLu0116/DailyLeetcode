package main

import "fmt"

// data structer: hash map
// golang:
//     1. map and slice
//     2. loop an array/slice
//     3. basic structer of a golang project
func twoSum(nums []int, target int) []int {
	var num_map = make(map[int]int)
	for id, n := range nums {
		id_tar, ok := num_map[target-n]
		if ok && (id_tar != id) {
			return []int{id, id_tar}
		}
		num_map[n] = id
	}
	return nil
}

func main() {
	nums := []int{3, 3}
	target := 6
	ids := twoSum(nums, target)
	fmt.Println(ids)
}
