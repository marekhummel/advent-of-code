package main

import (
	"aoc/src/2024/golang/solutions/day24"
	lib "aoc/src/lib/golang"
	// aoc:imports
)

func main() {
	lib.Run(2024, map[int]lib.Solution{
		24: {Part1: day24.Part1, Part2: day24.Part2},
		// aoc:solutions
	})
}
