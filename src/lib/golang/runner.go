package lib

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

// Part returns a typed result or an error.
type Part func(input []string, sample bool) (Result, error)

// Solution contains the two callable parts of a day. Construct it with NewSolution.
type Solution struct {
	part1 Part
	part2 Part
}

// NewSolution requires both part functions; nil is a registration error.
func NewSolution(part1, part2 Part) Solution {
	if part1 == nil || part2 == nil {
		panic("A solution must provide both part functions")
	}
	return Solution{part1: part1, part2: part2}
}

// Run parses DAY PART MODE (real or sample), loads input and prints one answer to stdout.
// Invalid arguments exit with code 2; execution errors are printed to stderr
// and exit with code 1. Unregistered days and unimplemented parts are informational.
// Batch execution and verification belong to the scripts.
func Run(year int, solutions map[int]Solution) {
	args := os.Args[1:]
	if len(args) != 3 || (args[2] != "real" && args[2] != "sample") {
		usage()
	}
	day, dayError := strconv.Atoi(args[0])
	part, partError := strconv.Atoi(args[1])
	if dayError != nil || partError != nil || day < 1 || day > 25 || part < 1 || part > 2 {
		usage()
	}
	solution, ok := solutions[day]
	if !ok {
		fmt.Println(Unsolved)
		return
	}
	solve := solution.part1
	if part == 2 {
		solve = solution.part2
	}
	sample := args[2] == "sample"
	prefix := "input"
	if sample {
		prefix = "sample"
	}
	filename := fmt.Sprintf("inputs/%d/%s%02d.txt", year, prefix, day)
	data, err := os.ReadFile(filename)
	if os.IsNotExist(err) {
		filename = fmt.Sprintf("inputs/%d/%s%02d_%d.txt", year, prefix, day, part)
		data, err = os.ReadFile(filename)
	}
	if err != nil {
		fail(err)
	}
	lines := strings.Split(strings.TrimSuffix(string(data), "\n"), "\n")
	for index := range lines {
		lines[index] = strings.TrimSuffix(lines[index], "\r")
	}
	result, err := solve(lines, sample)
	if err != nil {
		fail(err)
	}
	fmt.Println(result)
}

func usage() {
	fmt.Fprintln(os.Stderr, "Usage: mainYEAR DAY PART MODE (real|sample)")
	os.Exit(2)
}

func fail(err error) {
	fmt.Fprintln(os.Stderr, "Error:", err)
	os.Exit(1)
}
