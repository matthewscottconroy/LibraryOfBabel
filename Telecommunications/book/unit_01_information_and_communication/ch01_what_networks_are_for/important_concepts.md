# Chapter 1 — Important Concepts

**Communication system** *(§1.2)* — The six-part structure common to every act of
telecommunication: information source, transmitter, channel, noise source,
receiver, destination. Drawn by Shannon in 1948 and unrevised since. Its value is
diagnostic: any fault must live in one of the six, and naming which one is the
first step of every troubleshooting procedure in this book.

**Channel** *(§1.2)* — The physical medium a signal traverses. Its properties —
bandwidth, attenuation, propagation velocity, noise floor — are given by physics
and constrain everything built on top. Chapter 4 turns "the channel is good" into
a number.

**Noise** *(§1.2)* — Unwanted energy added to a signal in transit. Not a
malfunction but a thermodynamic guarantee: any conductor above absolute zero
generates it. Includes noise we manufacture ourselves (quantisation error, clock
jitter, intermodulation) as well as noise the world supplies.

**Propagation delay** *(§1.1)* — The time a signal takes to traverse a link,
determined by distance and the medium's propagation velocity. In fibre,
approximately 204 km/ms; in copper, 0.64–0.77 *c* depending on insulation. It is
irreducible by any amount of money and is the reason a New York–London round trip
cannot go below about 65 ms.

**Velocity factor** *(§1.1)* — The propagation velocity of a medium expressed as a
fraction of *c*. Appears on cable datasheets and matters for time-domain
reflectometry, where a cable tester converts a reflection's timing into a distance
to the fault.

**Intersymbol interference (ISI)** *(§1.1)* — The smearing of one transmitted
symbol into the time slot of its neighbours, caused by a channel that does not
pass all frequency components equally. The 1858 Atlantic cable's fundamental
limitation; still the limiting factor in high-speed copper and long-haul fibre.
Developed in Chapter 6, mitigated in Chapter 7.

**Host** *(§1.3)* — A device that originates or consumes messages; where the
application lives. Source and destination of Shannon's model in physical form.

**Node** *(§1.3)* — Any device attached to the network. All hosts are nodes; a
switch is a node and not a host.

**Intermediate system / relay** *(§1.3)* — A node whose function is forwarding
rather than originating. Repeaters, hubs, bridges, switches, routers, firewalls.
The presence of a forwarding decision is what ends one link and begins another.

**Link** *(§1.3)* — A channel directly connecting two or more nodes with no
intervening forwarding decision. May contain amplifiers, repeaters and thousands
of kilometres of glass and still be one link.

**Point-to-point link** *(§1.3)* — Exactly two nodes on the medium. No contention,
no ambiguity about the intended recipient. The easy case.

**Multi-access (shared) link** *(§1.3)* — Three or more nodes sharing one medium,
all able to hear one another. Requires rules about who transmits when — the medium
access control problem, solved differently by Ethernet (Chapter 16), Wi-Fi
(Chapter 44), and DOCSIS (Chapter 49).

**Simplex / half duplex / full duplex** *(§1.3)* — One-way; two-way but not
simultaneously; two-way simultaneously. A mismatch in the last two between the
ends of a link produces a characteristic and much-misdiagnosed performance
failure (Chapter 66).

**Protocol** *(§1.3)* — An agreement on the **syntax** (format), **semantics**
(meaning), and **timing** (permitted sequence) of exchanged messages. All three
clauses are required; a specification with only the first is a data structure.

**Network** *(§1.3)* — A set of nodes connected by links, cooperating under
protocols to deliver messages between hosts.

**Internetwork** *(§1.3)* — The same construction applied one level up, with whole
networks as the components. The word from which *Internet* is contracted, and a
literal description of the global Internet's architecture (Chapter 48).

**The organising question** *(§1.4)* — *How do we get information from one process
on one computer to another process on another computer, reliably, efficiently,
securely, and at scale?* Every technology in this book answers one clause of it;
knowing which clause is most of what it means to understand the technology.
