package day10

import (
	lib "aoc/src/lib/golang"
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
	instructions := strings.Fields(trimmed)

	if strings.Contains(trimmed, "goes to") {
		return Instruction{source: instructions[1], targets: []string{instructions[4] + instructions[5]}}
	} else if strings.Contains(trimmed, "gives") {
		return Instruction{source: instructions[0] + instructions[1], targets: []string{instructions[5] + instructions[6], instructions[10] + instructions[11]}}
	}

	panic("Weird instruction")
}

func compute(instructions []Instruction, broker *lib.Broker[uint16], chips [2]uint16) {
	wg := new(sync.WaitGroup)
	wg.Add(len(instructions))
	for _, inst := range instructions {
		go inst.Compute(broker, wg, chips)
	}
}

func Part1(input []string, sample bool) (any, error) {
	broker := lib.NewBroker[uint16]()
	go broker.Start()
	defer broker.Stop()

	chips := [2]uint16{17, 61}
	if sample {
		chips = [2]uint16{2, 5}
	}
	result := broker.Subscribe("result1")
	compute(parseInput(input), broker, chips)
	return <-result, nil
}

func Part2(input []string, _ bool) (any, error) {
	broker := lib.NewBroker[uint16]()
	go broker.Start()
	defer broker.Stop()

	output0 := broker.Subscribe("output0")
	output1 := broker.Subscribe("output1")
	output2 := broker.Subscribe("output2")
	compute(parseInput(input), broker, [2]uint16{17, 61})
	return uint64(<-output0) * uint64(<-output1) * uint64(<-output2), nil
}
