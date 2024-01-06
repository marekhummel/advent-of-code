package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
	"sync"
)

func parseInput() []Instruction {
	// file, _ := os.Open("./2015/inputs/sample07.txt")
	file, _ := os.Open("./2015/inputs/input07.txt")
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

func compute(instructions []Instruction, broker *Broker) {
	wg := new(sync.WaitGroup)
	wg.Add(len(instructions))
	for _, inst := range instructions {
		go inst.Compute(broker, wg)
	}
}

func main_part1() uint16 {
	broker := NewBroker()
	go broker.Start()

	instructions := parseInput()

	resultCh := broker.Subscribe("a")
	compute(instructions, broker)
	result := <-resultCh

	broker.Stop()
	return result
}

func main_part2(a uint16) uint16 {
	broker := NewBroker()
	go broker.Start()

	instructions := parseInput()
	setup_inst := Instruction{output: "b", operation: "SIGNAL", args: []string{"46065"}}
	instructions = append([]Instruction{setup_inst}, instructions...)

	resultCh := broker.Subscribe("a")
	compute(instructions, broker)
	result := <-resultCh

	broker.Stop()
	return result
}

func main() {
	fmt.Println("Running Part 1...")
	result1 := main_part1()
	fmt.Printf(" -> Value of a: %d\n\n", result1)

	fmt.Println("Running Part 2...")
	result2 := main_part2(result1)
	fmt.Printf(" -> Value of a: %d\n\n", result2)
}
