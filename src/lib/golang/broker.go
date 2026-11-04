package lib

type SubMessage[V any] struct {
	topic   string
	channel chan V
}

type PublishMessage[V any] struct {
	topic  string
	signal V
}

type Broker[V any] struct {
	subs      map[string]map[chan V]struct{}
	stopCh    chan struct{}
	publishCh chan PublishMessage[V]
	subCh     chan SubMessage[V]
	unsubCh   chan chan V
}

func NewBroker[V any]() *Broker[V] {
	return &Broker[V]{
		subs:      make(map[string]map[chan V]struct{}),
		stopCh:    make(chan struct{}),
		publishCh: make(chan PublishMessage[V], 1),
		subCh:     make(chan SubMessage[V], 1),
		unsubCh:   make(chan chan V, 1),
	}
}

func (b *Broker[V]) Start() {
	for {
		select {
		case <-b.stopCh:
			return
		case msgSub := <-b.subCh:
			// Register new subbed channel
			if _, ok := b.subs[msgSub.topic]; !ok {
				b.subs[msgSub.topic] = make(map[chan V]struct{})
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

func (b *Broker[V]) Publish(wire string, msg V) {
	b.publishCh <- PublishMessage[V]{topic: wire, signal: msg}
}

func (b *Broker[V]) Subscribe(wire string) chan V {
	msgCh := make(chan V, 3)
	b.subCh <- SubMessage[V]{topic: wire, channel: msgCh}
	return msgCh
}

func (b *Broker[V]) Unsubscribe(msgCh chan V) {
	b.unsubCh <- msgCh
}

func (b *Broker[V]) Stop() {
	close(b.stopCh)
}
