package main

import (
    "fmt"
    "log"
    "os"
    "sort"
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

func part1(input string) (int, error) {
    leftList, rightList, err := parseInput(input)
    if err != nil {
        return 0, err
    }

    sort.Ints(leftList)
    sort.Ints(rightList)

    var total int
    for i := range leftList {
        total += abs(leftList[i] - rightList[i])
    }

    return total, nil
}

func abs(x int) int {
    if x < 0 {
        return -x
    }
    return x
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
