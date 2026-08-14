# Chapter 17 — The People

**Radia Perlman (b. 1951).** The transparent bridging model — learn from the source,
forward by the destination, flood when unknown — was standardised in IEEE 802.1D
alongside her spanning tree algorithm (Chapter 19), and the design goal was
**transparency**: stations should not know a bridge exists, should require no
configuration, and should behave exactly as they would on a single segment.

That goal is why learning is automatic and why there is no protocol between stations
and switches. It is also why she has been consistently critical of the large flat
Layer 2 networks her own work enabled, arguing that routing the fabric is the better
answer — a position the industry took thirty years to adopt with leaf-spine
(Chapter 67 §67.4). Her *Interconnections* is the book on which and when to bridge
versus route.

**Mark Kempf.** DEC engineer who designed the **LANBridge 100** (1986), the first
commercial transparent Ethernet bridge — the learn/forward/flood algorithm of
§17.2, implemented for the first time in a product stations did not know existed.

**Kalpana**, founded by Vinod Bhardwaj, produced the first Ethernet *switch* — the
EtherSwitch, in 1990. Kalpana's insight was that bridging in hardware at low cost
per port changed the *unit* of what you attached: segments became stations. They
also introduced **cut-through** switching, trading error containment for latency,
which is §17.4's specialist mode. Cisco acquired Kalpana in 1994 and folded the
products into the Catalyst line.

**Bob Metcalfe (b. 1946) and the 3Com/Grand Junction line.** Grand Junction Networks
produced early fast Ethernet switching and was acquired by Cisco in 1995; 3Com,
Metcalfe's company, was the other major early switching vendor. The commercial fight
between them through the mid-1990s is what drove the per-port price down to the point
where one-station-per-port became normal — which is the economic precondition for
everything in §17.1's final column.

**Rich Seifert.** IEEE 802.3 working group participant and author of *The Switch
Book*, which remains the standard reference for the mechanisms in this chapter. He is
unusually good on the gap between what the standard requires and what implementations
actually do, and on the buffer and queueing behaviour of §17.4 that datasheets do not
describe.

**Mart Molle.** His work on Ethernet performance measurement, and the broader
literature that Boggs, Mogul and Kent's 1988 paper belongs to, established that
Ethernet's real-world capacity substantially exceeded the analytical worst case. That
finding is part of why switching's benefits were initially undersold — the shared
medium was less of a bottleneck than theory predicted, so the case for switching
rested on fault isolation and full duplex rather than on raw throughput.

**Nick McKeown (b. 1963).** Stanford, and much of the foundational work on switch
architecture: **virtual output queueing**, scheduling algorithms for crossbar
fabrics, and the analysis showing that input-queued switches with VOQ can achieve
100% throughput where pure input queueing caps at about 58%. §17.4's head-of-line
blocking discussion is his subject, and essentially every serious switch built since
implements what his group analysed. He later co-founded the OpenFlow work
(Chapter 68).

**Guido Appenzeller, Isaac Keslassy and Nick McKeown.** Their 2004 paper *Sizing
Router Buffers* challenged the long-standing bandwidth-delay-product rule of thumb,
showing that a router carrying many flows needs far less buffer than the classical
rule specifies — smaller by a factor of √*n* for *n* flows. The practical consequence is §17.4's observation
that switch buffers are small deliberately, and the theoretical consequence fed
directly into the bufferbloat discussion.

**Jim Gettys.** Named bufferbloat and demonstrated it was endemic, which
is why §17.4 can state that more buffer is not better rather than merely suggesting
it. See Chapter 13's notes.

**Mohammad Alizadeh and the DCTCP authors (2010).** Their work on data-centre TCP
addressed the **incast** problem of §17.4 — many servers responding simultaneously
and overwhelming a switch's egress buffer — by using ECN marking to signal congestion
before the buffer overflows. It is one of the more successful transfers of a research
result into production infrastructure, and it underlies much of Chapter 71 §71.5's
lossless fabric work.
