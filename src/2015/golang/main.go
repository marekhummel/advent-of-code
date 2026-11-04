package main

import (
	"aoc/src/2015/golang/solutions/day07"
	lib "aoc/src/lib/golang"
	// aoc:imports
)

func main() {
	lib.Run(2015, map[int]lib.Solution{
		7: {Part1: day07.Part1, Part2: day07.Part2},
		// aoc:solutions
	})
}
