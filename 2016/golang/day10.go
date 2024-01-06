package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
	"sync"
)

func parseInput() []Instruction {
	// Change condition for result1 in instruction.go as well, when switching to sample
	// file, _ := os.Open("./2016/inputs/sample10.txt")
	file, _ := os.Open("./2016/inputs/input10.txt")
	defer file.Close()

	instructions := []Instruction{}
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		inst := parseLine(scanner.Text())
		instructions = append(instructions, inst)
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

func compute(instructions []Instruction, broker *Broker) {
	wg := new(sync.WaitGroup)
	wg.Add(len(instructions))
	for _, inst := range instructions {
		go inst.Compute(broker, wg)
	}
}

func main() {
	broker := NewBroker()
	go broker.Start()

	instructions := parseInput()

	result1 := broker.Subscribe("result1")
	output0 := broker.Subscribe("output0")
	output1 := broker.Subscribe("output1")
	output2 := broker.Subscribe("output2")

	compute(instructions, broker)

	fmt.Printf("Bot with (17, 61):  %d\n", <-result1)
	result2 := (<-output0) * (<-output1) * (<-output2)
	fmt.Printf("Output product:     %d\n", result2)

	broker.Stop()
}
