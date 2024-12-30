package main

import (
	lib "aoc/lib/golang"
	"bufio"
	"fmt"
	"os"
	"sort"
	"strings"
	"sync"
)

func parseInput(realInput bool) []Instruction {
	var filename string
	if realInput {
		filename = "input"
	} else {
		filename = "sample"
	}
	// file, _ := os.Open("./2024/inputs/sample24.txt")
	file, _ := os.Open(fmt.Sprintf("./2024/inputs/%s24.txt", filename))
	defer file.Close()

	instructions := []Instruction{}
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		if len(line) == 0 {
			continue
		}
		inst := parseLine(line)
		instructions = append(instructions, inst)
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

func main_part1(realInput bool) uint64 {
	broker := lib.NewBroker[Val]()
	go broker.Start()
	defer broker.Stop()

	instructions := parseInput(realInput)

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

	return decimal
}

func main() {
	fmt.Println("Running Part 1...")
	fmt.Printf(" -> Sample: %d\n", main_part1(false))
	fmt.Printf(" -> Real:   %d\n\n", main_part1(true))
	fmt.Println("Running Part 2 not solved here.")
}
