# Chapter 29 — Further Reading

## Primary sources

**RFC 1812 — Baker, F. (1995). *Requirements for IP Version 4 Routers.***
**What a router must do**, in normative language. §5.2.4 is the forwarding algorithm and
§5.2.4.3 is the route-matching rule. The companion to RFC 1122's host requirements, and
the authoritative statement of §29.1's eight steps.

**RFC 1519 / RFC 4632 — CIDR.**
Where longest-prefix match becomes the fundamental operation rather than a special case.

**RFC 2644 / BCP 34 — Senie, D. (1999).** and **RFC 1122 §3.3.1** on host routing.
The host side of §29.1's "every host is a router".

**RFC 2328 §11 (OSPF) and RFC 4271 §9.1 (BGP)** on route selection.
How each protocol decides what to install — the input to §29.3's tie-breakers. Read them
after Chapters 31 and 32.

## On the algorithm

**Degermark, M., Brodnik, A., Carlsson, S. & Pink, S. (1997). "Small Forwarding Tables
for Fast Routing Lookups." *ACM SIGCOMM*.**
Compressing a full routing table to fit in cache. A clean example of a systems result
where the win came from fitting the working set into faster memory rather than from a
better algorithm.

**Nilsson, S. & Karlsson, G. (1999). "IP-Address Lookup Using LC-Tries." *IEEE JSAC*.**
The structure the Linux kernel uses for IPv4 lookups. Worth reading because it is running
on the machine you are reading this on.

**Gupta, P. & McKeown, N. (2001). "Algorithms for Packet Classification." *IEEE
Network*.**
The systematic survey: what is achievable, at what cost, in hardware and in software.

**Varghese, G. (2004). *Network Algorithmics.* Morgan Kaufmann.**
**The book on how routers are actually built fast.** Chapter 11 on prefix lookup is the
definitive treatment, and the whole book is an education in the discipline of making a
thing go at line rate.

## Books

**Doyle, J. & Carroll, J. (2005). *Routing TCP/IP, Volume 1*, 2nd ed. Cisco Press.**
**The reference for this unit.** Chapter 3 on static routing and the routing table is
directly this chapter, worked in far more detail, with the platform behaviour spelled out.

**Perlman, R. (1999). *Interconnections*, 2nd ed.**
Chapter 12 onwards on routing, and the general argument about hierarchy that underlies
§29.3.

**Stevens, W. R. (1994). *TCP/IP Illustrated, Volume 1*, chapter 9.**
IP routing from the host's point of view, with real tables. Good on the host-is-a-router
point.

**Medhi, D. & Ramasamy, K. (2017). *Network Routing: Algorithms, Protocols, and
Architectures*, 2nd ed.**
Comprehensive and mathematical. The router-architecture chapters cover TCAM and lookup
hardware properly.

## Applied

**Read your own routing table** and account for every entry — exercise F1, and the most useful thing in this list. `ip route`, `route print`, `netstat -rn`.

**`ip route get <destination>`.** Learn this now. It performs the real lookup including
policy rules and reports what would actually happen, which is different from what reading
the table suggests more often than you would expect.

**`ip rule show`** — the policy rules consulted *before* the table. Check this when
traffic goes somewhere the table says it should not.

**`show ip cef <destination>`** on Cisco, and `ip route get` on Linux — the FIB rather
than the RIB. **When the table looks right and forwarding is wrong, this is where to
look.**

**FRRouting** (frrouting.org) or **BIRD** on a Linux box or in containers. Free, real
implementations of OSPF, BGP and IS-IS. **Build a three-router lab and watch tables
converge** — this is the fastest way to make Unit VII concrete, and it costs nothing.

**GNS3 or Containerlab**, for topologies with real vendor images or with FRR/BIRD
containers. Containerlab in particular makes a twelve-router topology a single YAML file.

**`bgp.potaroo.net`** for the current global table size and its growth. §29.1's TCAM
argument becomes concrete when you compare the number with a platform's datasheet.

**Lab 16** in this book's [labs/](../../../labs/) directory builds a four-router topology
with deliberately overlapping prefixes of different lengths, and requires predicting each
lookup before verifying it — which is exercise F3, done with a worksheet.

## For the certification-minded

Objective 2.2 expects routing concepts, the routing table, and longest-prefix match.
Objective 1.2 expects the router as a device. Objective 5.5 expects `route`/`ip route`.

Seven things worth over-learning:

1. **Forwarding is destination-based.** The source is not consulted.
2. **A router knows only the next hop.** Nothing knows the whole path.
3. **Longest prefix wins.** `/32` beats everything; `0.0.0.0/0` loses to everything.
4. **Ties break on administrative distance, then metric, then ECMP.**
5. **Administrative distance is the first bracketed number, metric the second**, and they
   are not comparable across protocols.
6. **Directly connected versus via a next hop** — and connected routes are created by
   configuring an address, never by hand.
7. **"Network is unreachable" means no route matched**, which is a different fault from a
   timeout.

Expect a table-and-destination question requiring you to apply longest-prefix match by
counting bits. **Do it by prefix length, not by reading down the list** — the exam's
distractor is always a route that appears first and is shorter.

And the operational habit worth more than the objective: **when traffic goes somewhere
unexpected, do not read the table — ask the kernel.** `ip route get` answers in one
command what ten minutes of squinting will get wrong.
