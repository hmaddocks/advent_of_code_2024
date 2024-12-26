package main

import (
	"fmt"
	"log"
	"os"
	"regexp"
	"strconv"
)

func part2(input string) (int, error) {
	regex, error := regexp.Compile(`do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\)`)
	if error != nil {
		return 0, error
	}

	enabled := true
	sum := 0

	matches := regex.FindAllStringSubmatch(input, -1)
	for _, match := range matches {
		fmt.Println(match)
		if match[0] == "do()" {
			enabled = true
		} else if match[0] == "don't()" {
			enabled = false
		} else if enabled {
			match1, _ := strconv.Atoi(match[1])
			match2, _ := strconv.Atoi(match[2])
			sum += match1 * match2
		}
	}
	return sum, nil
}

func main() {
	input, err := os.ReadFile("input.txt")
	if err != nil {
		log.Fatalf("Error reading input file: %v", err)
	}
	fmt.Println(part2(string(input)))
}
