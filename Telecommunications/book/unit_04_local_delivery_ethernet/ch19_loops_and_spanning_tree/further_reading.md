# Chapter 19 — Further Reading

## Primary sources

**Perlman, R. (1985). "An Algorithm for Distributed Computation of a Spanning Tree in
an Extended LAN." *ACM SIGCOMM Computer Communication Review*, 15(4).**
**The paper.** Short, clear, and contains *Algorhyme*. Worth reading in full for the
statement of constraints — the section explaining what the algorithm is *not allowed*
to assume is the part that makes the achievement visible.

**IEEE 802.1D, *Media Access Control (MAC) Bridges*.**
The standardised spanning tree. The 1998 edition is the classic algorithm; the 2004
edition incorporates RSTP and is the one to read. Clause 17 is the protocol.

**IEEE 802.1w (1998), *Rapid Reconfiguration of Spanning Tree*.**
RSTP, before it was folded into 802.1D-2004. The proposal/agreement description is
here.

**IEEE 802.1s, *Multiple Spanning Trees*.**
MSTP, later folded into 802.1Q. The region-matching rules that cause so much
operational confusion are stated precisely.

**IEEE 802.1AX (formerly 802.3ad), *Link Aggregation*.**
LACP and the distribution requirements. Worth reading §5.2 on the **frame ordering
requirement**, which is the constraint that forces hashing and produces §19.4's
single-flow limit.

**RFC 6325 — Perlman, R. et al. (2011). *RBridges: Base Protocol Specification*
(TRILL).**
Perlman's own answer to the limitations of her earlier work. Read at least the
introduction, which states the case against spanning tree more forcefully than a
textbook can.

**RFC 5880 — Katz, D. & Ward, D. (2010). *Bidirectional Forwarding Detection.***
The general solution to fast failure detection, of which UDLD and Loop Guard are
special cases.

## Books

**Perlman, R. (1999). *Interconnections: Bridges, Routers, Switches, and
Internetworking Protocols*, 2nd ed. Addison-Wesley.**
**The reference for this chapter**, chapter 3 in particular. Nobody else can explain
why the design is what it is, because nobody else made the decisions. Also the best
available treatment of the bridge-versus-route question.

**Seifert, R. & Edwards, J. (2008). *The All-New Switch Book*, 2nd ed. Wiley.**
Chapters 5–6 on spanning tree and 9 on aggregation. Detailed, practical, and good on
the difference between the standard and what vendors ship.

**Froom, R., Sivasubramanian, B. & Frahim, E. *Implementing Cisco IP Switched
Networks (SWITCH).***
The vendor-specific configuration in depth: PortFast, BPDU Guard, Root Guard, Loop
Guard, UDLD, EtherChannel and their interactions. Whichever edition you find; the
mechanisms are stable.

**Dutt, D. G. (2019). *Cloud Native Data Center Networking.* O'Reilly.**
The modern argument for routing the fabric rather than bridging it. Reads as the
practical vindication of Perlman's position and is a good companion to Chapter 67.

## Applied

**`show spanning-tree`**, on any managed switch you have access to.
Find the root. Check whether it is the switch you would have chosen. It very often is
not, and that single observation is worth more than reading about the election.

**`show spanning-tree detail`**, and look at the topology-change counters.
A network that recalculates repeatedly has a fault; the counter is where it shows.

**`show etherchannel summary`** / `show lacp neighbor` / `cat /proc/net/bonding/bond0`.
Look at member counts and at per-member traffic distribution. The imbalance of §19.4
is invisible in the aggregate figure and obvious per member.

**`mstpd` and Linux bridge STP** (`brctl stp` / `ip link set … type bridge stp_state 1`).
A cheap way to run real spanning tree between virtual machines and watch the election
and reconvergence without any hardware.

**Wireshark filter `stp`**, and look at a BPDU field by field alongside §19.2's table.
The `flags` byte carrying proposal and agreement is where RSTP's speed comes from, and
seeing the handshake in a capture makes it concrete.

**Lab 06** in this book's [labs/](../../../labs/) directory creates a real loop on an
isolated segment, measures the collapse, then enables STP and repeats — followed by
forcing a root election and observing reconvergence timing under 802.1D and RSTP.

## On the poem

**Kilmer, J. (1913). "Trees."**
The original. Two minutes, and it makes *Algorhyme* considerably funnier.

**Perlman, R. "Algorhyme v2."**
She wrote a second poem for TRILL, which is less well known and makes the argument for
routing the fabric in verse. Worth finding.

## For the certification-minded

Objective 2.3 expects spanning tree and its enhancements; objective 4.3 expects BPDU
Guard and Root Guard as hardening measures; objective 5.2 expects switching loops and
broadcast storms as a troubleshooting scenario.

Six things worth over-learning:

1. **An Ethernet frame has no TTL**, which is why a loop never stops on its own.
2. **MAC flapping in the logs means a loop.**
3. **Lowest bridge ID wins**: priority (default 32768) first, MAC address as tie-break.
4. **Root port = lowest cost to root; one designated port per segment; everything else
   blocks.**
5. **PortFast and BPDU Guard are a pair.** Never configure the first without the
   second.
6. **BPDU Guard = no BPDUs allowed here. Root Guard = BPDUs allowed, but you may not
   become root.** The distinction is examined repeatedly.

And one that is not examined and will save you an afternoon: **a single flow never
exceeds one member link's speed in a bundle.** When a stakeholder asks why the new
4 Gb/s bundle did not speed up the backup, this is the answer, and the system is
working exactly as designed.
