# 13.2 Datagrams and Virtual Circuits

## Davies, and the other requirement

While Baran was designing for survivability, **Donald Davies** at the National
Physical Laboratory in Teddington was working on a different problem entirely.

Davies had visited the United States in 1965 and seen time-sharing systems, in which
many users at terminals shared one expensive computer. He noticed something about
the traffic: a user types for a few seconds, then thinks for thirty. A terminal
session is **overwhelmingly idle**, and reserving a circuit for it — as the telephone
network would — wastes almost all of the reserved capacity.

His conclusion was Chapter 9 §9.3's, arrived at independently:

**If traffic is bursty, reserving capacity for the peaks wastes it during the
troughs. Therefore capacity must not be reserved. Therefore each unit of data must
contend for the link when it has something to send. Therefore each unit must carry
its own destination address, because there is no reservation to identify it.**

The same mechanism, from a completely different requirement.

Davies presented the idea publicly in 1966 and learned of Baran's work shortly
afterwards. The designs were found to be substantially identical.

**And Davies supplied the word.** He wanted something short, ordinary, and
translatable, and he consulted a linguist about it — settling on **packet**, on the
grounds that it existed in French and German with the same sense and would survive
translation. Baran's "message block" did not.

Davies also built one: the NPL network became operational in 1969 and ran for a
decade, making it arguably the first working packet-switched network, though its
scale was a single building.

## Why the convergence matters

Two people, two continents, two entirely different requirements — survivability and
efficiency — and one mechanism.

That is a strong argument for the mechanism. A design that satisfies one demanding
requirement might be a special-purpose solution; a design that independently
satisfies two unrelated ones is more likely to be addressing something structural.

And the two requirements pull in the same direction for a shared reason:
**pre-established state is the enemy of both.** A reserved path is fragile under
attack and wasteful under bursty load. Removing it addresses both, and everything
else follows.

## The fork in the road

Having agreed that data should be chopped into addressed units, there remains a
substantial design choice, and the industry took both branches.

### Datagram (connectionless)

Each packet is **independent**. It carries a full destination address. Each router
makes an independent forwarding decision. No state is established beforehand and
none is retained afterwards.

Consequences:

- **No setup delay.** The first packet can be sent immediately.
- **Packets may take different paths** and may therefore arrive out of order.
- **Routers hold no per-conversation state**, so they scale and survive reboots.
- **A failed router affects only packets in flight**; subsequent packets route
  around it automatically.
- **No admission control**, and no guarantee of anything.

This is **IP** (Chapter 24), and it is the model that won the Internet.

### Virtual circuit (connection-oriented)

A **setup phase** establishes a path. Each switch along it records an entry mapping
an incoming label to an outgoing interface and label. Data packets then carry only a
short **label** rather than a full address, and each switch forwards by table lookup.
A teardown phase removes the state.

Consequences:

- **Setup delay** before the first packet.
- **All packets follow the same path**, so they arrive in order.
- **Switches hold per-circuit state**, which is memory and which must be rebuilt
  after a failure.
- **A failed switch destroys every circuit through it**, and each must be
  re-established.
- **Resources can be reserved at setup**, so guarantees become possible.
- **Labels are short**, so lookup is fast and headers are small.

This is X.25, Frame Relay, ATM and MPLS.

### The comparison

| | Datagram | Virtual circuit |
|---|---|---|
| Setup | none | required |
| Header | full address | short label |
| Path | may vary per packet | fixed |
| Ordering | not guaranteed | preserved |
| State in the network | **none** | per circuit |
| Failure of a node | packets reroute | circuits destroyed |
| Resource reservation | not possible | **possible** |
| Scaling limit | routing table size | state table size |
| Instances | **IP** | X.25, Frame Relay, ATM, MPLS |

## Why datagrams won the Internet

Four reasons, and they compound.

**Statelessness scales and survives.** A router holding no per-conversation state
can handle millions of simultaneous flows in hardware with no per-flow memory, and
can reboot without disrupting anything. Chapter 24 §24.1 identifies this as the
central architectural payoff.

**It matches the end-to-end argument.** If reliability must be implemented at the
endpoints anyway (Chapter 23 §23.4), then the network's guarantees are redundant for
correctness — so the network should be as simple as possible and the complexity
should live at the edges.

**It permits heterogeneity.** A virtual circuit requires every switch along the path
to participate in setup and maintain state, which means every network the circuit
crosses must implement the same signalling. IP requires only that each network can
carry a datagram from one attached device to another — the lowest-common-denominator
argument of Chapter 14 §14.2 — which is what let it run over Ethernet, radio,
satellite, and everything since.

**Setup cost is intolerable for short exchanges.** A DNS query is one small packet
and one small reply. Establishing a virtual circuit for it costs more than the
transaction. Chapter 38 §38.2's arithmetic applies here at a lower layer.

## Why virtual circuits won inside carriers

And yet the model did not die. It thrived, in a specific place, for specific
reasons.

**Traffic engineering.** A carrier does not want traffic taking the shortest path;
it wants traffic taking the path the carrier chose, for commercial and capacity
reasons. A label-switched path can be **placed** deliberately, with reserved
bandwidth. Chapter 50 §50.4 develops this, and it is MPLS's real justification.

**Quality of service.** Reserving resources at setup permits guarantees, which is
what a carrier sells and what a datagram network cannot offer.

**Separation of customers.** Labels stack. An outer label routes across the
carrier's core and an inner label identifies the customer, so many customers with
overlapping RFC 1918 address space share one infrastructure with complete
separation. MPLS L3VPN became the standard enterprise WAN product for two decades on
the strength of this.

**Fast reroute.** A pre-computed backup path with pre-installed state can be
switched to in under 50 ms — approaching SONET's figure (Chapter 11 §11.2) on packet
infrastructure.

So the honest summary is: **the Internet's edge and core are datagram, and the
carrier networks underneath them are frequently virtual circuit**, and a packet
crossing a wide-area link is often riding inside a label-switched path without
knowing it.

## The historical sequence

Worth knowing, because the names appear constantly in older material and in carrier
price lists.

**X.25** (1976) — the first widely deployed public packet network standard. Virtual
circuits with per-hop error correction and flow control, because the underlying
links were noisy analog circuits and the network could not assume they were reliable.
Slow, and thorough. Superseded once links became good.

**Frame Relay** (early 1990s) — X.25 with the per-hop error correction removed, on
the grounds that links were now digital and reliable and the endpoints could handle
errors. Substantially faster and cheaper, and it dominated enterprise WANs through
the 1990s.

**ATM** (1990s) — fixed 53-byte cells, virtual circuits, designed to carry voice,
video and data on one network with quality guarantees. Technically ambitious; the
5-byte header on a 48-byte payload is a 9.4% overhead that became known as the "cell
tax". It was going to carry everything, from the desktop to the backbone, and
Ethernet ate it from below while IP ate it from above. It survives in some DSL
aggregation.

**MPLS** (from 1997) — labels over IP, keeping the datagram model at the edges and
adding virtual circuits in the core. The synthesis, and the one that stuck.

The trajectory is instructive: each generation removed function that the endpoints
could handle better, which is the end-to-end argument being applied incrementally
by an industry that had not read the paper.

## What breaks here

**Assuming packets arrive in order.** They need not, on a datagram network, and
application developers who assume otherwise are relying on behaviour that is common
rather than guaranteed.

**A virtual circuit's state lost on failure.** Every circuit through a failed switch
must be re-established, which is a control-plane storm at exactly the worst moment.

**Confusing MPLS with a datagram protocol.** It is a virtual circuit mechanism, and
its properties — placed paths, reserved bandwidth, customer separation — follow from
that.

**Applying virtual-circuit thinking to IP.** "Where is the connection?" is not a
question IP answers. TCP has connections; IP does not, and the distinction is the
subject of Chapter 23 §23.2.

> **Network+ note.** Objective 1.6 expects the packet-switched versus
> circuit-switched distinction; objective 1.2 expects MPLS. The connectionless
> versus connection-oriented distinction reappears at Layer 4 as UDP versus TCP
> (Chapters 36 and 37), and recognising it as the *same* design choice at a
> different layer is worth more than either instance alone.
