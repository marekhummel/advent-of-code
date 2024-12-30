package main

import (
	lib "aoc/lib/golang"
	"sort"
	"strconv"
	"strings"
	"sync"
)

type Instruction struct {
	source  string
	targets []string
}

func (inst Instruction) Compute(broker *lib.Broker[uint16], wg *sync.WaitGroup) {
	// Receive value for args, might block until args are ready
	values := inst.getChips(broker, wg)
	sort.Slice(values, func(i, j int) bool { return values[i] < values[j] })

	for i := range inst.targets {
		broker.Publish(inst.targets[i], values[i])
	}

	if len(values) == 2 && values[0] == 17 && values[1] == 61 {
		bot, _ := strconv.ParseUint(strings.TrimPrefix(inst.source, "bot"), 10, 16)
		broker.Publish("result1", uint16(bot))
	}
}

func (inst Instruction) getChips(broker *lib.Broker[uint16], wg *sync.WaitGroup) []uint16 {
	if val, err := strconv.ParseUint(inst.source, 10, 16); err == nil {
		wg.Done()
		wg.Wait()
		return []uint16{uint16(val)}
	} else {
		valCh := broker.Subscribe(inst.source)
		wg.Done()
		wg.Wait()
		values := []uint16{}
		for i := 0; i < len(inst.targets); i++ {
			values = append(values, <-valCh)
		}
		broker.Unsubscribe(valCh)
		return values
	}
}
