package main

import (
	"strconv"
	"sync"
)

type Instruction struct {
	output    string
	operation string
	args      []string
}

func (inst Instruction) Compute(broker *Broker, wg *sync.WaitGroup) {
	// Receive value for args, might block until args are ready
	values := inst.getArgs(broker, wg)

	// Compute outgoing value
	result := uint16(0)
	switch inst.operation {
	case "AND":
		arg1 := values[inst.args[0]]
		arg2 := values[inst.args[1]]
		result = arg1 & arg2
	case "OR":
		arg1 := values[inst.args[0]]
		arg2 := values[inst.args[1]]
		result = arg1 | arg2
	case "LSHIFT":
		arg1 := values[inst.args[0]]
		arg2 := values[inst.args[1]]
		result = arg1 << arg2
	case "RSHIFT":
		arg1 := values[inst.args[0]]
		arg2 := values[inst.args[1]]
		result = arg1 >> arg2
	case "NOT":
		arg := values[inst.args[0]]
		result = ^arg
	case "SIGNAL":
		arg := values[inst.args[0]]
		result = arg
	}

	// Publish output
	broker.Publish(inst.output, result)
}

func (inst Instruction) getArgs(broker *Broker, wg *sync.WaitGroup) map[string]uint16 {
	// Get input values or prepare channels for that
	values := make(map[string]uint16)
	valChs := make(map[string]chan uint16)
	for _, arg := range inst.args {
		if val, err := strconv.ParseUint(arg, 10, 16); err == nil {
			values[arg] = uint16(val)
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
