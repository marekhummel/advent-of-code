package lib

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

// Solution contains the two parts of a day. Each returns an answer or an error.
type Solution struct {
	Part1 func(input []string, sample bool) (any, error)
	Part2 func(input []string, sample bool) (any, error)
}

// Run parses DAY PART [--sample], loads input and prints one answer to stdout.
// Invalid arguments exit with code 2; execution errors are printed to stderr
// and exit with code 1. Batch execution and verification belong to the scripts.
func Run(year int, solutions map[int]Solution) {
	args := os.Args[1:]
	if len(args) < 2 || len(args) > 3 || (len(args) == 3 && args[2] != "--sample") {
		usage()
	}
	day, dayError := strconv.Atoi(args[0])
	part, partError := strconv.Atoi(args[1])
	if dayError != nil || partError != nil || day < 1 || day > 25 || part < 1 || part > 2 {
		usage()
	}
	solution, ok := solutions[day]
	if !ok {
		fail(fmt.Errorf("no solution for %d day %02d", year, day))
	}
	solve := solution.Part1
	if part == 2 {
		solve = solution.Part2
	}
	if solve == nil {
		fail(fmt.Errorf("part %d is not implemented", part))
	}
	sample := len(args) == 3
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
	if result == nil {
		fail(fmt.Errorf("part %d returned no answer", part))
	}
	fmt.Println(result)
}

func usage() {
	fmt.Fprintln(os.Stderr, "Usage: mainYEAR DAY PART [--sample]")
	os.Exit(2)
}

func fail(err error) {
	fmt.Fprintln(os.Stderr, "Error:", err)
	os.Exit(1)
}
