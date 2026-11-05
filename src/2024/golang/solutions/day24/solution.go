package day24

import (
	lib "aoc/src/lib/golang"
	"sort"
	"strings"
	"sync"
)

func parseInput(input []string) []Instruction {
	instructions := make([]Instruction, 0, len(input))
	for _, line := range input {
		if len(line) > 0 {
			instructions = append(instructions, parseLine(line))
		}
	}
	return instructions
}

func parseLine(inst_str string) Instruction {
	trimmed := strings.TrimSpace(inst_str)

	if strings.Contains(trimmed, ":") {
		// First set of lines are initial values for certain registers
		sides := strings.Split(trimmed, ":")
		target := strings.TrimSpace(sides[0])
		init_value := strings.TrimSpace(sides[1])
		return Instruction{output: target, operation: "SET", args: []string{init_value}}
	} else {
		// Actual instructions
		sides := strings.Split(trimmed, "->")
		output := strings.TrimSpace(sides[1])
		tokens := strings.Fields(sides[0])
		args := []string{tokens[0], tokens[2]}
		sort.Strings(args)
		return Instruction{output: output, operation: tokens[1], args: args}
	}
}

func compute(instructions []Instruction, broker *lib.Broker[Val]) {
	wg := new(sync.WaitGroup)
	wg.Add(len(instructions))
	for _, inst := range instructions {
		go inst.Compute(broker, wg)
	}
}

func Part1(input []string, _ bool) (any, error) {
	broker := lib.NewBroker[Val]()
	go broker.Start()
	defer broker.Stop()

	instructions := parseInput(input)

	// Subscribe to z channels and compute them
	zChans := make(map[string]chan Val)
	for _, inst := range instructions {
		if strings.HasPrefix(inst.output, "z") {
			zChans[inst.output] = broker.Subscribe(inst.output)
		}
	}
	compute(instructions, broker)

	// Sort z channels and create output decimal
	zs := make([]string, 0, len(zChans))
	for k := range zChans {
		zs = append(zs, k)
	}
	sort.Sort(sort.Reverse(sort.StringSlice(zs)))

	decimal := uint64(0)
	for _, z := range zs {
		result := <-zChans[z]
		decimal = (decimal << 1) + uint64(result)
	}

	return decimal, nil
}

func Part2(_ []string, _ bool) (any, error) {
	return lib.Unsolved, nil
}
