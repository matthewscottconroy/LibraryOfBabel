# Chapter 24 — Further Reading

## Primary sources

**RFC 791 — Postel, J. (1981). *Internet Protocol.***
**Read it.** Forty-five pages, never revised, and it specifies the header in §24.2
exactly as it still is. The appendices with worked examples are unusually good, and the
prose is clear in a way modern specifications rarely manage.

**RFC 1812 — Baker, F. (1995). *Requirements for IP Version 4 Routers.***
What a router must do, including the TTL clarification that formalised what everyone was
already doing. The companion to RFC 1122's host requirements.

**RFC 1191 — Mogul, J. & Deering, S. (1990). *Path MTU Discovery.***
The mechanism, and — worth noting — the authors' own discussion of what happens when the
ICMP does not return. They anticipated the black hole; the fallback was not implemented.

**RFC 4821 — Mathis, M. & Heffner, J. (2007). *Packetization Layer Path MTU
Discovery.***
The robust alternative that does not depend on ICMP. The correct fix, still
under-deployed.

**RFC 8900 — Bonica, R. et al. (2020). *IP Fragmentation Considered Fragile.***
The formal verdict of §24.3. Collects thirty-nine years of operational knowledge into a
citable document, which is a real and under-recognised kind of contribution.

**RFC 2474 (DSCP) and RFC 3168 (ECN).**
The two redefinitions of the TOS byte. RFC 3168 is worth reading for its careful
treatment of how to deploy something incrementally when middleboxes are hostile.

**RFC 3514 — Bellovin, S. (2003). *The Security Flag in the IPv4 Header.***
The evil bit. Two pages, published 1 April, and the point it makes is one that security
proposals keep needing to be told.

**RFC 1393 / the traceroute history.** Jacobson's tool has no RFC of its own, which is
itself informative — it was a program that worked, and documentation followed usage.

## Books

**Stevens, W. R. (1994). *TCP/IP Illustrated, Volume 1*, chapters 3, 8 and 11.**
IP, traceroute, and fragmentation, with real captures. Chapter 8's traceroute
explanation is the clearest anywhere.

**Fall, K. & Stevens, W. R. (2011). *TCP/IP Illustrated, Volume 1*, 2nd ed.,
chapters 5 and 8.**
The revision, covering IPv6 and PMTUD's modern failure modes.

**Comer, D. *Internetworking with TCP/IP, Volume 1.***
Chapters 7–9 on IP, fragmentation and error handling. More formal than Stevens and good
on the reasoning behind the design.

**Bellovin, S., Cheswick, W. & Rubin, A. (2003). *Firewalls and Internet Security*,
2nd ed.**
For §24.3's fragmentation attacks and for the ICMP filtering argument, from the people
best placed to make it. Their position — that blocking all ICMP is wrong — is worth
reading in full, because it is the counter to a very widespread bad practice.

## On the operational side

**Huston, G.** — APNIC's chief scientist, and his ongoing measurement work on
fragmentation loss rates, path MTU behaviour, and IPv6 deployment. Published monthly at
the APNIC blog and at potaroo.net. **The empirical foundation under most of this unit's
claims.**

**Cloudflare and Google engineering blogs**, on path MTU and fragmentation in
production. Both operate at a scale where §24.3's problems are measurable rather than
anecdotal, and both publish the data.

**"Ping of Death", "Teardrop", and the fragmentation attack literature.**
Historical, mostly patched, and worth reading once to understand why firewalls treat
fragments the way they do.

## Applied

**Decode a header by hand.** Capture a packet, dump it in hex (`tcpdump -X`), and work
through all twenty bytes before checking against Wireshark. **Exercise F1**, and it is
the most valuable exercise in this chapter — after doing it three times you can
read a header in a capture when the tools are unhelpful.

**`ping -M do -s N`** — the path MTU binary search. Learn this before you need it; the
day you need it, you will be under pressure.

**`tracepath`** does the search automatically and reports the result.

**`traceroute`, `traceroute -I`, `traceroute -T -p 443`, `tracert`, `mtr`.**
Run all five to the same destination and compare. The differences are §24.4's point, and
seeing them once makes the "traceroute shows nothing but the site works" scenario
instantly recognisable.

**`mtr --report --report-cycles 100`** to a distant host. Read the per-hop loss column
and identify which hops are lossy versus which merely rate-limit ICMP. **Exercise F5**,
and it is a skill worth having.

**`ip route get 8.8.8.8`** and `ip link show` for local MTU. Then check every tunnel
interface, and notice how much smaller they are.

**Lab 12** in this book's [labs/](../../../labs/) directory builds a deliberate MTU
mismatch, reproduces the black hole by blocking ICMP, diagnoses it with the tools above,
and then fixes it three different ways.

**`tools/perfcalc.py`** in this book computes the loss-throughput relationship —
use `perfcalc.py loss` to see exactly what fragment-multiplied loss does to TCP
throughput, which makes §24.3's arithmetic concrete.

## For the certification-minded

Objective 1.4 expects the IPv4 header and its key fields. Objective 5.2 expects MTU and
fragmentation as a troubleshooting topic. Objective 5.5 expects `traceroute`/`tracert`.

Seven things worth over-learning:

1. **IP is connectionless and best-effort; TCP adds reliability; UDP does not.**
2. **TTL is a hop count**, decremented at each router, ICMP Time Exceeded at zero.
3. **Protocol numbers: 1 ICMP, 6 TCP, 17 UDP.**
4. **Ethernet MTU is 1500.**
5. **DF drives path MTU discovery**, and **blocking ICMP breaks it**.
6. **The IP header checksum covers the header only.**
7. **`tracert` uses ICMP; Unix `traceroute` uses UDP by default.**

The scenario that appears most often, essentially verbatim: **connectivity works, small
transfers work, large transfers hang — what is wrong?** The answer is MTU, and the
mechanism is the PMTUD black hole.

And the one worth more than any of them operationally: **`* * *` in traceroute does not
mean the path is broken.** Enormous amounts of time are wasted by people who believe it
does.
