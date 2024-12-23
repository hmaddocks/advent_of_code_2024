package main

import (
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func parseLine(line string) (int, int, error) {
	parts := strings.Fields(line)
	if len(parts) != 2 {
		return 0, 0, fmt.Errorf("invalid number of elements in line")
	}

	left, err := strconv.Atoi(parts[0])
	if err != nil {
		return 0, 0, fmt.Errorf("failed to parse left number: %s", parts[0])
	}

	right, err := strconv.Atoi(parts[1])
	if err != nil {
		return 0, 0, fmt.Errorf("failed to parse right number: %s", parts[1])
	}

	return left, right, nil
}

func parseInput(input string) ([]int, []int, error) {
	var leftList, rightList []int

	for lineNum, line := range strings.Split(strings.TrimSpace(input), "\n") {
		left, right, err := parseLine(line)
		if err != nil {
			return nil, nil, fmt.Errorf("failed to parse line %d: %v", lineNum+1, err)
		}
		leftList = append(leftList, left)
		rightList = append(rightList, right)
	}

	return leftList, rightList, nil
}

func part2(input string) (int, error) {
	leftList, rightList, err := parseInput(input)
	if err != nil {
		return 0, err
	}

	rightMap := map[int]int{}

	for _, right := range rightList {
		rightMap[right]++
	}

	var count int = 0
	for _, left := range leftList {
		if rightCount, exists := rightMap[left]; exists {
			count += left * rightCount
		}
	}

	return count, nil
}

func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

// go build -o bin/part2 ./part2/main.go
// ./bin/part1
func main() {
	inputBytes, err := os.ReadFile("input.txt")
	if err != nil {
		log.Fatalf("Failed to read input file: %v", err)
	}
	input := string(inputBytes)

	result, err := part2(input)
	if err != nil {
		log.Fatalf("Error processing input: %v", err)
	}
	fmt.Println(result)
}
