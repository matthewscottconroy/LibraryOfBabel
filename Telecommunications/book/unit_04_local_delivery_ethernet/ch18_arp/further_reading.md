# Chapter 18 — Further Reading

## Primary sources

**RFC 826 — Plummer, D. (1982). *An Ethernet Address Resolution Protocol.***
**Read this one.** Four pages, never revised, and the clearest demonstration in the
RFC series of how much can be specified when the problem is stated precisely. Twenty
minutes well spent, and the pseudocode at the end is still exactly what implementations
do.

**RFC 1027 — Carl-Mitchell, S. & Quarterman, J. (1987). *Using ARP to Implement
Transparent Subnet Gateways.***
Proxy ARP. Worth reading for the transitional problem it solved and as an example of a
mechanism that outlived its purpose.

**RFC 5227 — Cheshire, S. (2008). *IPv4 Address Conflict Detection.***
Formalises what gratuitous ARP had been doing informally: how to probe before claiming
an address, and how to behave on discovering a conflict.

**RFC 4861 — Narten, T., Nordmark, E., Simpson, W. & Soliman, H. (2007). *Neighbor
Discovery for IP version 6.***
The NDP specification. Long, but §7 (address resolution and NUD) is the core and is
readable on its own. The state machine in §7.3.2 is what §18.4 summarises.

**RFC 4862 — Thomson, S., Narten, T. & Jinmei, T. (2007). *IPv6 Stateless Address
Autoconfiguration.***
Duplicate Address Detection and the `tentative` state, in full.

**RFC 4890 — Davies, E. & Mohacsi, J. (2007). *Recommendations for Filtering ICMPv6
Messages in Firewalls.***
**Read this before writing any IPv6 firewall rule.** It states exactly which ICMPv6
types must be permitted and why, and it exists because people kept breaking IPv6 by
applying IPv4 habits.

**RFC 3971 — Arkko, J. et al. (2005). *SEcure Neighbor Discovery (SEND)*** and
**RFC 3972 — Aura, T. (2005). *Cryptographically Generated Addresses (CGA).***
The authentication solution that was not deployed. RFC 3972 is the elegant one — the
idea that an address can be its own credential is worth understanding regardless of
deployment.

**RFC 5082 — Gill, V., Heasley, J., Meyer, D., Savola, P. & Pignataro, C. (2007).
*The Generalized TTL Security Mechanism (GTSM).***
The hop-limit-255 trick, generalised. Short, and the reasoning is clean.

## Books

**Stevens, W. R. (1994). *TCP/IP Illustrated, Volume 1*, chapters 4 and 5.**
The classic treatment of ARP, with real captures. Stevens's method — state the
protocol, then show it happening — is what §18.2's traced exchange imitates.

**Fall, K. & Stevens, W. R. (2011). *TCP/IP Illustrated, Volume 1*, 2nd ed.,
chapter 4.**
The revision, which covers NDP alongside ARP and is the better single reference for
this chapter.

**Hagen, S. (2014). *IPv6 Essentials*, 3rd ed. O'Reilly.**
Chapter 4 on ICMPv6 and NDP. Practical, with configuration and capture examples, and
good on the operational differences NDP creates.

**Perlman, R. (1999). *Interconnections*, 2nd ed.**
Her discussion of address resolution as a general problem — not merely ARP — is worth
reading for the framing: what has to be true for any scheme that maps one address space
onto another.

## On the security

**Song, D. (1999). dsniff.** The tool suite, and the accompanying writing.
Historically important as the moment "switched networks are secure" became untenable.

**Ornaghi, A. & Valleri, M. (2003). "Man in the Middle Attacks."** Blackhat Europe.
The ettercap authors' systematic treatment. Dated in its particulars and still the
clearest taxonomy of what a position in the middle of a conversation permits.

**Cisco, "Dynamic ARP Inspection" configuration guides**, and the equivalent from
other vendors.
The practical defence. Read the DHCP snooping dependency carefully, and read the
static-binding and ARP-ACL sections before enabling anything, because the failure mode
of a partial deployment is a total outage.

**Nikander, P., Kempf, J. & Nordmark, E. (2004). RFC 3756, *IPv6 Neighbor Discovery
Trust Models and Threats.***
The threat analysis that motivated SEND. Useful independently of SEND, as a model of
how to enumerate what a protocol assumes.

## Applied

**`ip neigh`, `arp -a`, `show ip arp`, `show ipv6 neighbors`.**
Look at yours now, and again after clearing it, and again after ten minutes of
silence. The ageing behaviour is much easier to believe once observed.

**`arping` and `arping -D`.**
The underused tool of §18.3. `-D` finds duplicate addresses in seconds; plain `arping`
distinguishes a missing host from a host with a broken IP stack or a filtering
firewall.

**`ndisc6`, `rdisc6`, `rltraceroute6`** (the `ndisc6` package).
The IPv6 equivalents. `rdisc6 eth0` dumps router advertisements in full, which is the
fastest way to see what a segment is telling its hosts.

**`ping6 ff02::1%eth0` and `ping6 ff02::2%eth0`.**
All nodes and all routers on the link. The second has no useful IPv4 equivalent and
answers "what is routing this segment?" instantly.

**Wireshark filters:** `arp`, `arp.opcode == 1`, `arp.duplicate-address-detected`,
`icmpv6.type == 135 || icmpv6.type == 136`, `icmpv6.type == 134`.
Wireshark flags duplicate-address conditions automatically, which is worth knowing
before you need it.

**Lab 05** in this book's [labs/](../../../labs/) directory captures a full ARP
exchange, clears the cache and watches it rebuild, then demonstrates ARP spoofing on
an isolated segment and the effect of enabling DAI.

## For the certification-minded

Objective 1.4 expects ARP and NDP. Objective 2.3 expects SLAAC. Objective 4.2 expects
ARP spoofing/poisoning as an attack and DAI as the mitigation. Objective 5.5 expects
the `arp` command.

Five things worth over-learning:

1. **A host never ARPs for an off-subnet address.** It ARPs for its gateway. An
   off-subnet ARP request proves a wrong mask.
2. **MAC is hop-by-hop, IP is end-to-end.** The destination MAC changes at every
   router; the destination IP does not change at all.
3. **ARP has no authentication**, and **DAI depends on DHCP snooping**. The second is
   examined because people enable DAI alone.
4. **NDP uses ICMPv6**, uses **multicast rather than broadcast**, and makes DAD
   mandatory.
5. **Blocking ICMPv6 breaks IPv6 entirely.** Examined, and also the most common
   real-world IPv6 mistake.

And one that is not examined and is worth more than several that are: **`arping` tells
you whether a host exists at Layer 2 when everything above it is filtered**, which
collapses a large part of the troubleshooting tree in one command.
