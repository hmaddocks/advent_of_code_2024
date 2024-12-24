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

func count_safe_reports(reports [][]int) int {
	var safe_reports int = 0
	for _, report := range reports {
		if len(report) >= 2 {
			safe_reports++
		}
	}

	return safe_reports
}

func part1(input string) (int, error) {
	reports := parseInput(input)
	count_safe_reports(reports)
	return 0, nil
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
