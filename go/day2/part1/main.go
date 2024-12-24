package main

import (
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func parseLine(line string) []int {
	nums := make([]int, 0, len(strings.Fields(line)))
	for _, strNum := range strings.Fields(line) {
		num, err := strconv.Atoi(strNum)
		if err == nil {
			nums = append(nums, num)
		}
	}
	return nums
}

func parseInput(input string) [][]int {
	lines := strings.Split(strings.TrimSpace(input), "\n")
	reports := make([][]int, 0, len(lines))

	for _, line := range lines {
		if lineNums := parseLine(line); len(lineNums) > 0 {
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

func isStrictMonotonic(nums []int) bool {
	if len(nums) < 2 {
		return false
	}

	ascending := nums[0] < nums[1]
	for i := 1; i < len(nums); i++ {
		if ascending && nums[i] <= nums[i-1] {
			return false
		}
		if !ascending && nums[i] >= nums[i-1] {
			return false
		}
	}
	return true
}

func isSafe(report []int) bool {
	if len(report) < 2 {
		return false
	}

	for i := 0; i < len(report)-1; i++ {
		first, second := report[i], report[i+1]
		if abs(first-second) < 1 || abs(first-second) > 3 {
			return false
		}
	}

	return isStrictMonotonic(report)
}

func countSafeReports(reports [][]int) int {
	safeReports := 0
	for _, report := range reports {
		if isSafe(report) {
			safeReports++
		}
	}

	return safeReports
}

func part1(input string) (int, error) {
	reports := parseInput(input)
	return countSafeReports(reports), nil
}

// go build -o bin/part1 ./part1/main.go
// ./bin/part1
func main() {
	inputBytes, err := os.ReadFile("input.txt")
	if err != nil {
		log.Fatalf("Failed to read input file: %v", err)
	}

	result, err := part1(string(inputBytes))
	if err != nil {
		log.Fatalf("Error processing input: %v", err)
	}
	fmt.Println(result)
}
