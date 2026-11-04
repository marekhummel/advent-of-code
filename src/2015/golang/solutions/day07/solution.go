package day07

import (
	lib "aoc/src/lib/golang"
	"strconv"
	"strings"
	"sync"
)

func parseInput(input []string) []Instruction {
	instructions := make([]Instruction, 0, len(input))
	for _, line := range input {
		instructions = append(instructions, parseLine(line))
	}
	return instructions
}

func parseLine(inst_str string) Instruction {
	trimmed := strings.TrimSpace(inst_str)
	sides := strings.Split(trimmed, "->")
	output := strings.TrimSpace(sides[1])
	instructions := strings.Fields(sides[0])

	switch len(instructions) {
	case 1:
		return Instruction{output: output, operation: "SIGNAL", args: instructions}
	case 2:
		return Instruction{output: output, operation: "NOT", args: []string{instructions[1]}}
	case 3:
		return Instruction{output: output, operation: instructions[1], args: []string{instructions[0], instructions[2]}}
	default:
		panic("Weird instruction")
	}
}

func compute(instructions []Instruction, broker *lib.Broker[uint16]) {
	wg := new(sync.WaitGroup)
	wg.Add(len(instructions))
	for _, inst := range instructions {
		go inst.Compute(broker, wg)
	}
}

func signal(instructions []Instruction, target string) uint16 {
	broker := lib.NewBroker[uint16]()
	go broker.Start()
	defer broker.Stop()

	resultCh := broker.Subscribe(target)
	compute(instructions, broker)
	return <-resultCh
}

func Part1(input []string, sample bool) (any, error) {
	target := "a"
	if sample {
		target = "i"
	}
	return signal(parseInput(input), target), nil
}

func Part2(input []string, sample bool) (any, error) {
	target := "a"
	if sample {
		target = "i"
	}
	instructions := parseInput(input)
	first := signal(instructions, target)
	rewired := []Instruction{}
	for _, instruction := range instructions {
		if instruction.output != "b" {
			rewired = append(rewired, instruction)
		}
	}
	rewired = append(rewired, Instruction{output: "b", operation: "SIGNAL", args: []string{strconv.Itoa(int(first))}})
	return signal(rewired, target), nil
}
