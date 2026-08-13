# Chapter 13 — Important Concepts

**Baran's three topologies** *(§13.1)* — **Centralised** (one hub; destroy it and
everything stops), **decentralised** (several hubs; fragments), **distributed** (a
mesh; no node essential). The telephone network was the second, which is why it was
the target.

**The redundancy result** *(§13.1)* — A distributed network at redundancy level
**three or four** survives destruction of a large fraction of its nodes. Extreme
survivability requires only *modest* redundancy in the right topology, which is the
counterintuitive and useful finding.

**Baran's derivation** *(§13.1)* — No node may be essential → no path can be
pre-established → each unit must carry its own destination → each node forwards
independently → units must be small and standardised. **This chain is why packets
have headers and routers are stateless.**

**Hot-potato routing** *(§13.1)* — Hold a block briefly and pass it to whichever
neighbour currently offers the best path, updating continuously from neighbours'
reports. A distance-vector protocol, described in 1962.

**Davies's requirement** *(§13.2)* — Terminal traffic is overwhelmingly idle, so
reserving a circuit wastes almost all of it. Same conclusion, entirely different
motivation. He supplied the word **packet**, chosen after consulting a linguist for
something short and translatable.

**Why the convergence matters** *(§13.2)* — Two unrelated requirements
(survivability, efficiency) producing one mechanism is stronger evidence than either
alone. Both are defeated by **pre-established state**, which is the deeper common
factor.

**Datagram (connectionless)** *(§13.2)* — Each packet independent, carrying a full
address; independent forwarding at each hop; **no state in the network**. No setup
delay, possible reordering, routers scale and survive reboots, no guarantees. This
is **IP**.

**Virtual circuit (connection-oriented)** *(§13.2)* — Setup establishes per-hop
state mapping labels to interfaces; data carries a short **label**. Setup delay,
fixed path, ordering preserved, per-circuit state, **resource reservation possible**.
X.25, Frame Relay, ATM, MPLS.

**Why datagrams won the Internet** *(§13.2)* — Statelessness scales and survives; it
matches the end-to-end argument; it permits heterogeneity (any network that can move
a datagram can carry IP); and setup cost is intolerable for short exchanges.

**Why virtual circuits won inside carriers** *(§13.2)* — **Traffic engineering**
(paths can be *placed* with reserved bandwidth), quality of service, customer
separation via stacked labels, and sub-50 ms fast reroute. Hence the edge is datagram
and the carrier core is often virtual circuit.

**The historical sequence** *(§13.2)* — X.25 (per-hop error correction, because
links were noisy) → Frame Relay (correction removed, because links became reliable)
→ ATM (fixed 53-byte cells, 9.4% "cell tax", ambitious and displaced) → MPLS (labels
over IP; the synthesis). Each generation removed function the endpoints could handle
better — the end-to-end argument applied incrementally.

**Store-and-forward** *(§13.3)* — Receive the whole packet, verify it, then forward.
Adds *n* × *L*/*R* of delay across *n* hops. Necessary for error checking (the FCS
covers the whole frame), rate adaptation and contention.

**Cut-through and fragment-free** *(§13.3)* — Forward after the destination address
(sub-microsecond, forwards corrupted frames) or after 64 bytes (catches most
collision-induced corruption). Latency against error containment.

**The queue** *(§13.3)* — Wherever a packet may wait. Three outcomes: transmit
immediately, wait, or **discard**.

**Loss as a signal, not a fault** *(§13.3)* — On a congested link, discard is how
the network tells senders to slow down. A network that never dropped a packet would
have no congestion signal. Corruption-induced loss is the fault case, and the two are
distinguished by whether loss correlates with **load** or with the **physical
layer**.

**The ρ/(1−ρ) relationship** *(§13.3)* — Mean queueing delay ∝ ρ/(1−ρ). Raising
utilisation from 50% to 90% multiplies delay **ninefold**; 0.95 gives 19×, 0.99
gives 99×. A cliff whose approach is invisible on a utilisation graph.

**Self-similar traffic** *(§13.3)* — Leland, Taqqu, Willinger and Wilson (1993)
showed from Ethernet traces that real traffic is bursty at **every** timescale,
unlike Poisson. Queues are therefore **longer** at a given mean utilisation than
M/M/1 predicts — the model's shape is right and its optimism is not.

**Bufferbloat** *(§13.3)* — Large buffers hide congestion rather than preventing it.
TCP receives no loss signal, keeps growing its window, and the queue fills — after
which **every** packet on the link is delayed by the full queue depth, including
latency-sensitive flows that did not cause it. **More buffer is not better buffer.**
Active queue management (CoDel, FQ-CoDel, PIE, CAKE) drops on *sojourn time* rather
than occupancy.

**The efficiency case** *(§13.4)* — 100 users, 1 Mb/s, 5% active: 100 Mb/s reserved
versus ~20 Mb/s statistical, a **5× gain**, rising to 16× at 10,000 users. Applied to
national infrastructure, this is a different business rather than an optimisation.

**The other three arguments** *(§13.4)* — Setup cost is intolerable for short
transactions; heterogeneity requires only that each network move a datagram; and
**innovation without permission**, which mattered most and was least visible at the
time.

**What was given up** *(§13.4)* — Guaranteed bandwidth; constant delay; in-order
delivery; **admission control**; immunity to congestion collapse; freedom from
per-packet header overhead.

**Admission control, and why it is the most-regretted loss** *(§13.4)* — A busy
signal is an honest refusal delivered before effort is invested, and **calls already
in progress are unaffected**. A packet network accepts everything and degrades
everyone, including flows that were running acceptably. It cannot offer the choice
because it has no notion of admitting a flow.

**Buying it back** *(§13.4)* — Virtual circuits, IntServ/RSVP, DiffServ, MPLS-TE,
lossless data-centre Ethernet, TSN, 5G network slicing. Every one restores, for some
traffic, a property abandoned in 1964, and every one is harder than it looks because
the architecture was designed around not having it.

**The recurring pattern** *(§13.4)* — A general-purpose substrate absorbs a
specialised one because its economics improve faster, then spends decades
reimplementing the specialist's guarantees imperfectly. The specialists are usually
right about the list of losses and wrong about the outcome.

**Two corrections to the popular account** *(§13.1)* — The **ARPANET was not built to
survive nuclear war**; that was Baran's motivation, not the ARPANET's, which was
resource sharing. And Baran did not invent packet switching alone; Davies arrived
independently and named it.
