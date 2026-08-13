# Chapter 17 — Further Reading

## Primary sources

**IEEE 802.1D, *Media Access Control (MAC) Bridges*.**
The transparent bridging specification: the learning process, the forwarding
database, ageing, and flooding. Clause 7's description of the forwarding process is
the authoritative statement of §17.2's three-sentence algorithm. Now folded into
802.1Q, which is where current work lives.

**IEEE 802.1Q, *Bridges and Bridged Networks*.**
The current consolidated standard. Covers bridging, VLANs (Chapter 20), priority and
much else. Clause 8 covers the forwarding process and the filtering database.

**Boggs, D. R., Mogul, J. C. & Kent, C. A. (1988). "Measured Capacity of an Ethernet:
Myths and Reality." *ACM SIGCOMM*.**
Measured a real shared Ethernet and found it sustained far higher utilisation than
the folklore claimed. Relevant here because it shows why the case for switching
rested on fault isolation and full duplex rather than on the shared medium being a
throughput bottleneck.

**McKeown, N., Anantharam, V. & Walrand, J. (1996). "Achieving 100% Throughput in an
Input-Queued Switch." *IEEE INFOCOM*.**
The virtual output queueing result of §17.4. Demonstrates the 58% head-of-line
blocking ceiling and the scheduling that removes it.

**Appenzeller, G., Keslassy, I. & McKeown, N. (2004). "Sizing Router Buffers."
*ACM SIGCOMM*.**
Challenged the bandwidth-delay-product buffer rule, arguing a router carrying *n*
flows needs BDP/√*n*. The theoretical basis for §17.4's "buffers are small
deliberately".

**Alizadeh, M. et al. (2010). "Data Center TCP (DCTCP)." *ACM SIGCOMM*.**
The incast problem and an ECN-based answer. §2's characterisation of data-centre
traffic and the buffer pressure it creates is the clearest statement of the problem.

## Books

**Seifert, R. & Edwards, J. (2008). *The All-New Switch Book*, 2nd ed. Wiley.**
**The reference for this chapter.** Chapters 2–5 cover bridging, the forwarding
database, forwarding modes and buffer architectures in far more depth than any
summary, by someone who sat on the committees. Unusually good on the gap between what
the standard requires and what implementations do.

**Perlman, R. (1999). *Interconnections: Bridges, Routers, Switches, and
Internetworking Protocols*, 2nd ed. Addison-Wesley.**
Chapter 3 on transparent bridging, from the person who designed it. Her discussion of
*why* transparency was the goal, and what it cost, is not available elsewhere. Also
consistently sceptical about large bridged networks.

**Spurgeon, C. & Zimmerman, J. (2014). *Ethernet: The Definitive Guide*, 2nd ed.
O'Reilly.**
Chapters 17–19 on switching, with practical detail on configuration and on what the
counters mean.

**Dutt, D. G. (2019). *Cloud Native Data Center Networking.* O'Reilly.**
For the argument that Layer 2 switching should be minimised in favour of routing —
Perlman's position, thirty years on, made operational. Relevant background for
Chapter 67.

## Applied

**Any switch vendor's `show mac address-table` documentation**, and the platform's
buffer architecture guide.
The generic material in §17.2 and §17.4 is platform-independent; the specific
capacities, ageing defaults, violation behaviours and buffer allocations are not, and
the vendor documentation is what you consult during an incident.

**Cisco, "Troubleshooting Output Drops"** and equivalent notes from other vendors.
The practical companion to §17.4's microburst discussion, including how to configure
higher-resolution counters to see what five-minute averages hide.

**Arista and Cisco application notes on microbursts.**
Both vendors publish good material on measuring sub-second utilisation, because both
sell into markets where it matters. The measurement techniques are more useful than
the product conclusions.

**Port security configuration guides.**
Worth reading the violation-mode descriptions carefully: `shutdown`, `restrict` and
`protect` behave very differently in an incident, and choosing `protect` because it
sounds safest gives you a silent failure.

## Tools

**`show mac address-table`** on any managed switch, and `bridge fdb show` on Linux.
Look at a healthy table so you know what normal looks like. Then unplug a device,
wait five minutes, and watch its entry age out.

**Lab 03** in this book's [labs/](../../../labs/) directory builds the hub-versus-
switch comparison empirically — including capturing another station's traffic on a
hub and failing to on a switch, which makes §17.1's privacy point unforgettable.

**Lab 04** traces the learning process directly, clearing the table and watching it
repopulate frame by frame.

**`ip -s link` / `show interface`**, for the output drop counters of §17.4. An
interface with drops and no errors on a lightly-loaded link is the microburst
signature.

## For the certification-minded

Objective 1.2 expects the device taxonomy and switch operation. Objective 1.6 expects
collision and broadcast domains. Objective 5.1 expects using the MAC table to locate
a device.

Four things worth over-learning:

1. **The three-sentence algorithm**, verbatim.
2. **"A switch breaks up collision domains, not broadcast domains; a router breaks
   up broadcast domains."** This is among the most reliably examined sentences in the
   certification.
3. **The counting method** — switch ports for collision domains, router interfaces
   or VLANs for broadcast domains — and the two standard distractors.
4. **Output drops are congestion, not corruption.** Different counter, different
   cause, different remedy from CRC errors.

And one that is not examined and matters in practice: **utilisation graphs cannot
show microbursts**, so an interface dropping frames at 20% average utilisation is
reporting something real rather than something impossible.
