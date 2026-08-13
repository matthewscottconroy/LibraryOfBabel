# Chapter 54 — The People

**Jeffrey Case, Mark Fedor, Martin Schoffstall and James Davin.** **SNMP, 1988 — and it was
meant to be temporary.**

**The context is worth having.** **In 1987 there were three competing proposals** for managing
IP networks: **HEMS**, **SGMP** (a simple thing built quickly), and **CMIP over TCP** — the
OSI management framework, which was **complete, general, object-oriented and enormous.**

**The IETF's decision was to adopt SGMP as an interim measure, renamed SNMP, while CMOT
(CMIP over TCP) was developed as the real answer.**

> **CMOT was never deployed. SNMP is still running the world's device monitoring thirty-eight
> years later.** **Chapter 22 §22.1's OSI story, in miniature and with the same outcome** — the
> simple thing that shipped beat the complete thing that was designed.

**Case's design constraint was explicit:** **the agent must be small enough to run on the
equipment of the time**, which meant a few kilobytes of code on a device with very little
memory. **Everything about SNMP that is awkward follows from that** — **UDP rather than TCP, no
session state, a flat namespace of numbered objects, and no security to speak of.**

**And the security decision has a specific history.** **SNMPv2 was intended to fix it**, and the
working group **split irreconcilably over the security model** — producing **v2p, v2c, v2u and
v2\*** as competing variants. **The community settled on v2c, the one with no security at all**,
because it was the only one everybody could implement.

> **SNMPv3 (1998) eventually supplied what v2 could not agree on**, by which time **v2c was
> everywhere and worked.** **A security model that arrives ten years after the deployment is a
> security model that will not be deployed**, and this is the single clearest example of that
> pattern in the book.

**Marshall T. Rose.** **The MIB structure, SMI, and much of what makes SNMP navigable** — and,
later, **one of its more articulate critics.**

**Rose's *The Simple Book* (1990)** was the standard text, and **his subsequent writing about
what SNMP got wrong is unusually candid for someone who built it.** **The core criticism —
that a management framework built for retrieving individual scalar values is the wrong shape
for configuring devices** — **is exactly the argument that NETCONF and YANG were built to
answer**, fifteen years later.

**Eric Allman (b. 1955).** **Syslog, at Berkeley in the 1980s** — **and he wrote it as part of
sendmail, not as a system facility.**

**Allman needed somewhere for sendmail's diagnostics to go. He wrote a small daemon that
accepted messages over a socket, tagged them with a facility and a severity, and wrote them to
a file or forwarded them elsewhere.**

**Other programs began using it. Then the kernel. Then every Unix. Then every network device.**

> **Syslog was never designed as a protocol.** **It was standardised in 2001 (RFC 3164) by
> writing down what implementations already did** — the document is explicitly descriptive
> rather than normative — **and properly specified only in 2009 (RFC 5424), twenty-five years
> after it was written.**

**Which explains its oddities**: the priority arithmetic, the ambiguous timestamp format, the
absence of a year in the original format, the 1,024-byte message limit, **and the fact that it
runs over UDP with no acknowledgement.**

**Allman's larger contribution is sendmail itself**, and **the observation that most often
follows him is that he wrote both the most widely deployed mail system and the most widely
deployed logging system by solving his own immediate problems well.**

**Darren Kerr and Barry Bruins.** **NetFlow, at Cisco, 1996 — and it was originally a
forwarding optimisation, not a monitoring tool.**

**The idea was a flow cache.** **Rather than performing a full route lookup for every packet,
look it up once for the first packet of a flow and cache the result**, keyed by the five-tuple.
**Subsequent packets of that flow hit the cache** — much faster in the hardware of the time.

**The cache had to record, per flow, the addresses, ports, protocol, interfaces and a packet
count. Someone observed that this was exactly the data an operator would want.**

> **NetFlow's monitoring capability was a by-product of a forwarding optimisation**, and **the
> optimisation itself became obsolete** — modern hardware performs a full lookup per packet at
> line rate and does not need a cache. **The by-product outlived the purpose entirely**, which
> is a pattern this book has met before (Chapter 50 §50.4's MPLS).

**Peter Phaal and the sFlow team, at InMon.** **The alternative architecture, and a deliberate
one.**

**Phaal's argument was that a flow cache does not scale to switching silicon.** **Maintaining
per-flow state at 10 Gb/s per port across 48 ports is expensive; sampling one packet in N and
copying its header is nearly free** and can be done in the ASIC.

**The design accepts statistical error in exchange for being implementable everywhere**, and
**the mathematics of the trade is stated explicitly in the sFlow specification** — which is
unusual and admirable in a protocol document.

> **NetFlow and sFlow are the same disagreement as Chapter 52 §52.1's IntServ and DiffServ:
> exact per-flow state against cheap aggregate approximation.** **Both answers are correct for
> different scales**, and the industry runs both.

**Rob Enns, Andy Bierman, Martin Björklund and the NETCONF/YANG group.** **The intended
successor, from 2006 onward.**

**The argument was Rose's, made twenty years later with the benefit of the evidence:**
**a protocol for reading scalar values is not a protocol for managing configuration**, and
**every vendor had built a proprietary CLI-scraping mechanism because SNMP could not do the
job.**

**YANG (RFC 6020, Björklund)** supplies what MIBs lacked: **a modelling language with types,
constraints, groupings and a defined mapping to XML and JSON.** **gNMI then supplied a modern
transport**, and **OpenConfig — an operator-led group rather than a vendor one — supplied
vendor-neutral models.**

> **The OpenConfig detail matters.** **It was founded by network operators** — Google, AT&T,
> Microsoft and others — **because vendors had produced incompatible YANG models and the
> operators were the ones paying for the incompatibility.** **A standards effort driven by
> buyers rather than sellers is rare and it changes what gets standardised.**

## What this chapter's history shows

**Three of its four core technologies were accidents.**

**SNMP was an interim measure** pending a proper solution that never arrived. **Syslog was
sendmail's debug output.** **NetFlow was a cache side-effect.** **Only sFlow was designed for
the purpose it serves.**

> **And all four are still in production.** The pattern is not that accidents are good design;
> **it is that something which works and ships will be adopted, and adoption is very difficult
> to reverse.** Chapter 22's argument, Chapter 51's argument, and this one.

**The corollary for a practitioner is uncomfortable and worth stating:** **the tooling you
inherit was not designed for what you are using it for**, and understanding its origins is
frequently the fastest route to understanding its limitations.
