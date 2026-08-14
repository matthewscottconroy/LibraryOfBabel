# Chapter 17 — Important Concepts

**Repeater** *(§17.1)* — Regenerates a signal between two segments. Operates on
symbols, not frames. Extends distance; changes nothing logical. Its latency counts
against the slot time, which is the origin of the **5-4-3 rule**.

**Hub** *(§17.1)* — A multi-port repeater. Physically a star, logically a bus.
One collision domain, shared bandwidth, half duplex only, and **no privacy** — every
frame reaches every station, so packet capture requires no special access. The
difficulty of capturing on a switched network is a *side effect of a performance
improvement*, not a designed security feature.

**Bridge** *(§17.1)* — Receives whole frames and makes a **forwarding decision**.
Perlman's characterisation: a device that improves the network by *not* forwarding
things. Each port becomes its own collision domain; traffic is contained; segments
may run at different speeds; the broadcast domain is unchanged.

**Switch** *(§17.1)* — A bridge with many ports in hardware. **No protocol
difference** — the same IEEE 802.1D algorithm. The transformation is economic: when a
port costs less than a metre of cable, you attach one *station* per port rather than
one *segment*, at which point every station has its own collision domain, full duplex
becomes possible, and CSMA/CD becomes vestigial.

**Layer 3 switch** *(§17.1)* — A router in switching silicon with many ports. It
**is** a router: routing table, longest-prefix match, TTL decrement, and it breaks up
broadcast domains. The name exists because "router" implied the slow software devices
of the early 1990s.

**The switch algorithm** *(§17.2)* — **Learn** the source against the arrival port;
**forward** by the destination out that port only; **flood** when the destination is
unknown. Three sentences, and everything in enterprise switching elaborates them.

Why learning is free *(§17.2)* — The switch extracts the source address from
frames it must handle anyway. A frame arriving on port *n* is proof its sender is
reachable through port *n*. No protocol, no configuration, no cooperation.

Why learning is the only option *(§17.2)* — MAC addresses are flat (Chapter 15
§15.2), so nothing can be computed and the table must be built by observation. The
finite table is a direct consequence of flat addressing.

**Ageing** *(§17.2)* — Entries expire after ~300 seconds of silence, because devices
move and the table is finite. A device silent for five minutes is forgotten, and
the next frame to it is flooded until it speaks — usually harmless, occasionally the
explanation for a puzzling capture.

Unicast flooding from asymmetric routing *(§17.2)* — If a host's replies leave by
another path, the switch never sees frames *from* it, never learns it, and floods
every frame *to* it permanently. Persistent unicast flooding to a live host means
the switch is not seeing that host's transmissions.

**Table exhaustion** *(§17.2)* — Capacities run 8,000 (small access) to 256,000+
(data centre). When full, the switch floods everything it cannot look up — it
degrades to a hub. Reachable legitimately by an over-large flat network including
virtual machines, or deliberately by **MAC flooding** (Chapter 62 §62.1).

The shape of the MAC flooding attack *(§17.2)* — It does not break the switch; it
degrades it to an earlier, weaker design. A surprising number of network attacks
have this form.

**Port security** *(§17.2)* — Limit learnable addresses per port. Two or three covers
a workstation plus a daisy-chained telephone plus a VM. Violation modes: `shutdown`
(err-disable, safe, generates a support call), `restrict` (drop and count, usually
right), `protect` (drop silently, no visibility).

Reading the MAC table *(§17.2)* — Locate a device physically; confirm presence;
spot a duplicate address on two ports; **spot a loop** by MAC flapping; and count
devices per port to find an unexpected hub or hypervisor.

**Collision domain** *(§17.3)* — The set of interfaces whose transmissions can
collide. One per switch port, one per hub (covering everything on it), one per
router interface.

**Broadcast domain** *(§17.3)* — The set of interfaces receiving a broadcast from any
of them. One per router interface, or one per VLAN. Switches and hubs create
none.

**The sentence** *(§17.3)* — A switch breaks up collision domains. It does not
break up broadcast domains. A router breaks up broadcast domains.

Why broadcast domain size matters *(§17.3)* — Every host processes every
broadcast, taking an interrupt whether or not it is relevant. ARP traffic grows
roughly with the **square** of host count in the worst case. And the **failure blast
radius** is the whole domain — one misbehaving device degrades every host sharing it.
The traditional few-hundred-hosts guidance is soft, dated and directionally correct;
modern segmentation is driven more by security than by broadcast volume.

**The modern asymmetry** *(§17.3)* — Many collision domains, few broadcast domains.
Collision domains are an exam topic and a historical artefact; broadcast domains
are a live design constraint.

**Exam technique** *(§17.3)* — Collision domains: count active switch ports, add one
per *hub*, add router interfaces. Broadcast domains: count router interfaces or
VLANs, ignore switches entirely. The distractors are "count the switches" and "count
the hub's ports".

**Store-and-forward** *(§17.4)* — Receive the whole frame, verify the FCS, forward.
~12 µs at 1 Gb/s, ~1.2 µs at 10 Gb/s. **Never forwards corruption**; permits rate
adaptation; the modern default.

**Cut-through** *(§17.4)* — Forward after the 6-byte destination address. ~500 ns,
independent of frame size. **Forwards corrupted frames**; cannot adapt rates.
Specialist — trading, some HPC. **Adaptive cut-through** falls back to
store-and-forward when a port's error rate rises.

**Why store-and-forward won** *(§17.4)* — As rates rise its latency penalty shrinks
(12 µs → 1.2 µs) while cut-through's advantage is fixed, and almost every real
topology needs rate adaptation.

**Buffers are small** *(§17.4)* — A 48-port switch with 4 MB shared has ~85 KB per
port, which is **680 µs** of transmission at 1 Gb/s. Deliberately so.

**Microbursts** *(§17.4)* — A link averaging 30% over five minutes can be at 100% for
tens of milliseconds. Standard SNMP polling cannot see this; the only evidence is
the **output drop counter**. An interface dropping on a lightly-utilised link is
reporting microbursts, not a fault.

**Head-of-line blocking** *(§17.4)* — With one FIFO per ingress port, a frame for a
congested egress blocks frames behind it destined for idle ports. Caps throughput at
about **58%**. **Virtual output queueing** — a separate queue per egress at each
ingress — is the fix, and distinguishes a well-engineered switch from a cheap one.

More buffer is not better *(§17.4)* — Large buffers hide congestion from TCP's
loss signal, so the window keeps growing and eventually every frame on the port is
delayed by the full queue depth. A buffer's job is to absorb a burst, not store a
backlog; active queue management handles the rest.

Switching capacity versus forwarding rate *(§17.4)* — Gb/s is the easy number;
**packets per second** is the binding one, because a switch is limited by lookups. A
1 Gb/s port at minimum frame size delivers **1.488 Mpps**. A switch chosen on its
Gb/s figure may fail under a small-frame load — which is exactly what a
denial-of-service attack generates.
