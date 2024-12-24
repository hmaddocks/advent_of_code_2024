package main

import (
    "testing"
)

func TestParseLine(t *testing.T) {
    testCases := []struct {
        name     string
        input    string
        expected []int
    }{
        {
            name:     "Valid input with multiple numbers",
            input:    "5 10 15",
            expected: []int{5, 10, 15},
        },
        {
            name:     "Input with non-numeric values",
            input:    "5 abc 10",
            expected: []int{5, 10},
        },
        {
            name:     "Empty input",
            input:    "",
            expected: []int{},
        },
    }

    for _, tc := range testCases {
        t.Run(tc.name, func(t *testing.T) {
            result := parseLine(tc.input)

            if len(result) != len(tc.expected) {
                t.Errorf("Expected %d numbers, got %d", len(tc.expected), len(result))
            }

            for i := range tc.expected {
                if result[i] != tc.expected[i] {
                    t.Errorf("Expected %d at index %d, got %d", tc.expected[i], i, result[i])
                }
            }
        })
    }
}

func TestParseInput(t *testing.T) {
    testCases := []struct {
        name     string
        input    string
        expected [][]int
    }{
        {
            name:     "Multiple lines with numbers",
            input:    "5 10\n15 20 25\n30",
            expected: [][]int{{5, 10}, {15, 20, 25}, {30}},
        },
        {
            name:     "Empty input",
            input:    "",
            expected: [][]int{},
        },
    }

    for _, tc := range testCases {
        t.Run(tc.name, func(t *testing.T) {
            result := parseInput(tc.input)

            if len(result) != len(tc.expected) {
                t.Errorf("Expected %d lines, got %d", len(tc.expected), len(result))
            }

            for i := range tc.expected {
                if len(result[i]) != len(tc.expected[i]) {
                    t.Errorf("Expected %d numbers at line %d, got %d", len(tc.expected[i]), i, len(result[i]))
                }

                for j := range tc.expected[i] {
                    if result[i][j] != tc.expected[i][j] {
                        t.Errorf("Expected %d at line %d, index %d, got %d", tc.expected[i][j], i, j, result[i][j])
                    }
                }
            }
        })
    }
}

func TestCountSafeReports(t *testing.T) {
    testCases := []struct {
        name     string
        reports  [][]int
        expected int
    }{
        {
            name:     "Multiple safe reports",
            reports:  [][]int{{1, 2}, {3, 4}, {5}},
            expected: 2,
        },
        {
            name:     "No safe reports",
            reports:  [][]int{{1}, {2}, {3}},
            expected: 0,
        },
        {
            name:     "Empty input",
            reports:  [][]int{},
            expected: 0,
        },
    }

    for _, tc := range testCases {
        t.Run(tc.name, func(t *testing.T) {
            result := count_safe_reports(tc.reports)

            if result != tc.expected {
                t.Errorf("Expected %d safe reports, got %d", tc.expected, result)
            }
        })
    }
}

func TestPart1(t *testing.T) {
    testCases := []struct {
        name     string
        input    string
        expected int
    }{
        {
            name:     "Simple input",
            input:    "5 10\n15 20\n1",
            expected: 0,
        },
        {
            name:     "Empty input",
            input:    "",
            expected: 0,
        },
    }

    for _, tc := range testCases {
        t.Run(tc.name, func(t *testing.T) {
            result, err := part1(tc.input)

            if err != nil {
                t.Errorf("Unexpected error: %v", err)
            }

            if result != tc.expected {
                t.Errorf("Expected %d, got %d", tc.expected, result)
            }
        })
    }
}