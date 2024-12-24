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
		lineNums := parseLine(line)
		if len(lineNums) > 0 {
			reports = append(reports, lineNums)
		}
	}

	return reports
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

func safeSequence(report []int) bool {
	if len(report) < 2 {
		return false
	}

	for i := 0; i < len(report)-1; i++ {
		diff := report[i] - report[i+1]
		if abs(diff) < 1 || abs(diff) > 3 {
			return false
		}
	}

	return isStrictMonotonic(report)
}

func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func safe(report []int) bool {
	if safeSequence(report) {
		return true
	}

	for i := range report {
		modified := make([]int, 0, len(report)-1)
		modified = append(modified, report[:i]...)
		modified = append(modified, report[i+1:]...)

		if safeSequence(modified) {
			return true
		}
	}

	return false
}

func countSafeReports(reports [][]int) int {
	safeCount := 0
	for _, report := range reports {
		if safe(report) {
			safeCount++
		}
	}
	return safeCount
}

func part1(input string) (int, error) {
	reports := parseInput(input)
	return countSafeReports(reports), nil
}

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
