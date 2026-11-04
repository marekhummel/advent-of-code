package main

import (
	"aoc/src/2016/golang/solutions/day10"
	lib "aoc/src/lib/golang"
	// aoc:imports
)

func main() {
	lib.Run(2016, map[int]lib.Solution{
		10: {Part1: day10.Part1, Part2: day10.Part2},
		// aoc:solutions
	})
}
