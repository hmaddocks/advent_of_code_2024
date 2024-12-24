package main

import (
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func parseLine(line string) []int {
	strNums := strings.Fields(line)
	nums := make([]int, 0, len(strNums))

	for _, strNum := range strNums {
		num, err := strconv.Atoi(strNum)
		if err != nil {
			continue
		}
		nums = append(nums, num)
	}

	return nums
}

func parseInput(input string) [][]int {
	lines := strings.Split(strings.TrimSpace(input), "\n")
	reports := make([][]int, 0, len(lines))

	for _, line := range lines {
		lineNums := parseLine(line)
		if len(lineNums) > 0 {
			reports = append(reports, lineNums)
		}
	}

	return reports
}

func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func ascending(nums []int) bool {
	ascending := true
	for i := 1; i < len(nums); i++ {
		if nums[i] <= nums[i-1] {
			ascending = false
			break
		}
	}

	return ascending
}

func descending(nums []int) bool {
	descending := true
	for i := 1; i < len(nums); i++ {
		if nums[i] >= nums[i-1] {
			descending = false
			break
		}
	}

	return descending
}

func safe(report []int) bool {
	if len(report) < 2 {
		return false
	}

	for i := 0; i < len(report)-1; i++ {
		first := report[i]
		second := report[i+1]
		if abs(first-second) < 1 || abs(first-second) > 3 {
			return false
		}

	}

	return ascending(report) || descending(report)
}

func count_safe_reports(reports [][]int) int {
	var safe_reports = 0
	for _, report := range reports {
		if safe(report) {
			safe_reports++
		}
	}

	return safe_reports
}

func part1(input string) (int, error) {
	reports := parseInput(input)
	return count_safe_reports(reports), nil
}

// go build -o bin/part1 ./part1/main.go
// ./bin/part1
func main() {
	inputBytes, err := os.ReadFile("input.txt")
	if err != nil {
		log.Fatalf("Failed to read input file: %v", err)
	}
	input := string(inputBytes)

	result, err := part1(input)
	if err != nil {
		log.Fatalf("Error processing input: %v", err)
	}
	fmt.Println(result)
}
