package main

import (
	lib "aoc/lib/golang"
	"strconv"
	"sync"
)

type Instruction struct {
	output    string
	operation string
	args      []string
}

type Val = uint8

func (inst Instruction) Compute(broker *lib.Broker[Val], wg *sync.WaitGroup) {
	// Receive value for args, might block until args are ready
	values := inst.getArgs(broker, wg)

	// Compute outgoing value
	result := Val(0)
	switch inst.operation {
	case "AND":
		arg1 := values[inst.args[0]]
		arg2 := values[inst.args[1]]
		result = arg1 & arg2
	case "OR":
		arg1 := values[inst.args[0]]
		arg2 := values[inst.args[1]]
		result = arg1 | arg2
	case "XOR":
		arg1 := values[inst.args[0]]
		arg2 := values[inst.args[1]]
		result = arg1 ^ arg2
	case "SET":
		arg := values[inst.args[0]]
		result = arg
	}

	// Publish output
	broker.Publish(inst.output, result)
}

func (inst Instruction) getArgs(broker *lib.Broker[Val], wg *sync.WaitGroup) map[string]Val {
	// Get input values or prepare channels for that
	values := make(map[string]Val)
	valChs := make(map[string]chan Val)
	for _, arg := range inst.args {
		if val, err := strconv.ParseUint(arg, 10, 16); err == nil {
			values[arg] = Val(val)
		} else {
			valCh := broker.Subscribe(arg)
			valChs[arg] = valCh
		}
	}

	// Signal that all channels are ready
	wg.Done()

	// Wait until all channels from all instructions are ready
	wg.Wait()

	// Receive values and return
	for arg, valCh := range valChs {
		values[arg] = <-valCh
		broker.Unsubscribe(valCh)
	}
	return values
}
