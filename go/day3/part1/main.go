package main

import (
	"fmt"
	"log"
	"os"
	"regexp"
	"strconv"
)

func part1(input string) (int, error) {
	regex, error := regexp.Compile(`mul\((\d{1,3}),(\d{1,3})\)`)
	if error != nil {
		return 0, error
	}

	matches := regex.FindAllStringSubmatch(input, -1)
	sum := 0
	for _, match := range matches {
		match1, _ := strconv.Atoi(match[1])
		match2, _ := strconv.Atoi(match[2])
		sum += match1 * match2
	}
	return sum, nil
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
