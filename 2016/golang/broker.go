package main

type SubMessage struct {
	topic   string
	channel chan uint16
}

type PublishMessage struct {
	topic  string
	signal uint16
}

type Broker struct {
	subs      map[string]map[chan uint16]struct{}
	stopCh    chan struct{}
	publishCh chan PublishMessage
	subCh     chan SubMessage
	unsubCh   chan chan uint16
}

func NewBroker() *Broker {
	return &Broker{
		subs:      make(map[string]map[chan uint16]struct{}),
		stopCh:    make(chan struct{}),
		publishCh: make(chan PublishMessage, 1),
		subCh:     make(chan SubMessage, 1),
		unsubCh:   make(chan chan uint16, 1),
	}
}

func (b *Broker) Start() {
	for {
		select {
		case <-b.stopCh:
			return
		case msgSub := <-b.subCh:
			// Register new subbed channel
			if _, ok := b.subs[msgSub.topic]; !ok {
				b.subs[msgSub.topic] = make(map[chan uint16]struct{})
			}
			b.subs[msgSub.topic][msgSub.channel] = struct{}{}
		case msgUnsub := <-b.unsubCh:
			// Remove channel
			for _, subs := range b.subs {
				delete(subs, msgUnsub)
			}
		case msgPub := <-b.publishCh:
			// Distribute message to all listeners
			if _, ok := b.subs[msgPub.topic]; ok {
				for msgCh := range b.subs[msgPub.topic] {
					select {
					case msgCh <- msgPub.signal:
					default:
					}
				}
			}
		}
	}
}

func (b *Broker) Publish(wire string, msg uint16) {
	b.publishCh <- PublishMessage{topic: wire, signal: msg}
}

func (b *Broker) Subscribe(wire string) chan uint16 {
	msgCh := make(chan uint16, 3)
	b.subCh <- SubMessage{topic: wire, channel: msgCh}
	return msgCh
}

func (b *Broker) Unsubscribe(msgCh chan uint16) {
	b.unsubCh <- msgCh
}

func (b *Broker) Stop() {
	close(b.stopCh)
}
