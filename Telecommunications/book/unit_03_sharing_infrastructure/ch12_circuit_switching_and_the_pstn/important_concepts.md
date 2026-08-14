# Chapter 12 — Important Concepts

**Circuit** *(§12.1)* — A dedicated end-to-end path, established before
communication, held for its duration, released afterwards. Three phases: **setup,
transfer, teardown**. Between the first two, the path is yours exclusively — which
is what it buys and what it costs.

**The Strowger switch** *(§12.1)* — The first automatic exchange (patented 1891, in
service 1892), built by an undertaker who believed the operator was diverting his
calls. A stepping selector advanced by dial pulses. Variants ran into the 1990s.

Dial pulsing as a protocol *(§12.1)* — Interrupting loop current a defined
number of times, with specified make/break ratios and inter-digit timing: syntax,
semantics and timing, and the first protocol most people ever used.

Crossbar and common control *(§12.1)* — Separating the switching **matrix** from
the **control** that selects a path through it. Path selection can then consider the
whole matrix rather than a single predetermined route, control logic is shared and
can therefore be more capable, and reliability improves. Stored-program control
(1ESS, 1965) made the control a computer, after which network features became
software.

**The exchange hierarchy** *(§12.1)* — Local → tandem → toll → sectional →
regional, with **aggregation** (many lines share fewer trunks), **summarisation**
(the telephone number is a routing hierarchy read left to right), and
**containment**. Chapter 11 §11.4's argument, seventy years earlier.

**Alternate routing** *(§12.1)* — Offering a call to the most direct trunk group
first and to progressively less direct paths on finding it full. Load-dependent
inter-exchange routing, in production from the 1950s, prefiguring Chapter 31.

**The local loop** *(§12.1)* — A dedicated pair from exchange to premises, present
in copper to virtually every building in the developed world and already paid for.
The reason DSL exists and the single most consequential piece of infrastructure in
this book.

Regeneration versus amplification, in its original setting *(§12.2)* — Analog
amplification compounds noise across spans, so quality fell with distance; digital
regeneration discards it, making quality **independent of distance**. The largest
single improvement in the network's history.

**Companding** *(§12.2)* — Compressing before quantisation and expanding after, so
step size is proportional to amplitude — fine steps for quiet sounds. **µ-law**
(North America, Japan) and **A-law** (elsewhere) give roughly 12–13 bits of dynamic
range in 8 bits, and are mutually incompatible.

**The DS0** *(§12.2)* — 300–3,400 Hz speech, sampled at **8 kHz** (Nyquist plus
filter headroom), quantised to **8 bits** with companding: **64 kb/s**. G.711 *is*
the DS0, and every voice codec since is measured against it.

**T1** *(§12.2)* — 24 channels × 8 bits + 1 framing bit = 193 bits/frame ×
8,000 frames/s = **1.544 Mb/s**. Deployed 1962, using existing copper with repeaters
at the loading-coil spacing.

Robbed-bit signalling and the 56 kb/s figure *(§12.2)* — T1 originally stole the
least significant bit of every sixth frame for supervision. Inaudible in voice,
fatal to data, so a data channel used 7 bits: 8,000 × 7 = **56 kb/s**. This is the
origin of a number that pervades older networking material.

**E1** *(§12.2)* — 32 slots (30 voice, slot 0 framing, slot 16 signalling) ×
8 bits × 8,000 = **2.048 Mb/s**. Having a dedicated signalling slot, it never needed
robbed-bit signalling and its channels are clear 64 kb/s by design.

**In-band signalling** *(§12.3)* — Control information on the same channel as the
conversation. Natural, economical, and structurally flawed: users can reach the
control channel.

The 2,600 Hz vulnerability *(§12.3)* — Whistling the trunk-idle tone made the
far exchange release while the local one kept billing state, after which
multi-frequency digits routed a free call. The **blue box** automated it. The
general principle: *if control information travels on the same channel as user data,
users can generate control information* — the same class as SQL injection, XSS,
format-string attacks and stack smashing.

**SS7** *(§12.3)* — Common channel signalling from 1975: a separate packet-switched
network carrying call setup, routing and management. SSP (exchange), **STP**
(signalling router), **SCP** (database). Bought immunity to phreaking, much faster
setup, trunks not held during ringing, and the database lookups that made toll-free
numbers, portability, caller ID and roaming possible.

The telephone network's control plane was packet-switched twenty years before its
data plane *(§12.3)*.

**The SIP/RTP parallel** *(§12.3)* — Signalling negotiates and media carries, on
different paths with different requirements and different transports. SIP and RTP
reimplement SS7's separation deliberately.

**SS7's security assumption** *(§12.3)* — Designed for a few dozen mutually trusting
licensed monopolies, so no inter-carrier authentication. Deregulation multiplied
participants without changing the assumption, enabling **SMS interception**
(defeating SMS-based two-factor authentication), location tracking, and call
interception — demonstrated publicly since 2014 and exploited in real account
takeovers. Mitigations are partial and unevenly deployed.

**The general lesson** *(§12.3)* — A protocol's security rests on assumptions about
who can participate, and those assumptions have a shelf life. ARP, BGP, DNS, SMTP
and SS7 all share this history.

**The erlang** *(§12.4)* — Offered traffic: the average number of circuits in
simultaneous use. *A* = λ*h*/3600 for λ calls per hour of mean holding time *h*
seconds.

**Erlang B** *(§12.4)* — Blocking probability for a lost-calls-cleared system, with
the recurrence *B*(*n*,*A*) = *A·B*(*n*−1,*A*) / (*n* + *A·B*(*n*−1,*A*)),
*B*(0,*A*) = 1. Ten erlangs needs eighteen circuits for better than 1%
blocking — not ten, and not hundreds.

Where the curve bends *(§12.4)* — Blocking falls sharply up to about 1% and then
flattens. Grade-of-service targets sit at the knee because beyond it you pay
substantially for improvements nobody perceives.

**Trunking efficiency** *(§12.4)* — Larger groups are dramatically more efficient at
the same grade of service: 1 erlang needs 5 circuits (20% utilisation), 100 erlangs
needs 117 (85%). Hence combining trunk groups saves real money, and splitting
them for administrative convenience is an invisible waste.

**Erlang C** *(§12.4)* — For systems where blocked demand **queues** rather than
clearing: call centres, server pools, connection pools, checkout lanes. Different
formula, same framework.

**Admission control** *(§12.4)* — Erlang's model includes it: the 19th call is
refused and the other 18 are unaffected. Packet networks accept everything and
degrade everyone, which Chapter 13 §13.4 identifies as the most-regretted loss and
which QoS and network slicing partly attempt to recover.

**Busy hour** *(§12.4)* — Erlang calculations use busy-hour traffic, not the daily
average. Sizing on the average blocks heavily at peak.
