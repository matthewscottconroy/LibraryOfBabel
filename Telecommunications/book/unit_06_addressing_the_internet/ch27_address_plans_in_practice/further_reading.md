# Chapter 27 — Further Reading

## Primary sources

**RFC 1918 — Rekhter, Y. et al. (1996). *Address Allocation for Private Internets.***
Nine pages. Read it for the candour: it states plainly that private addressing breaks the
end-to-end model and recommends it anyway. The judgement was right.

**RFC 6598 — Weil, J. et al. (2012). *IANA-Reserved IPv4 Prefix for Shared Address
Space.***
Carrier-grade NAT space, and the argument for why a fourth private range was necessary.
Read it if you want to understand what your mobile provider is doing to you.

**RFC 5737 — Arkko, J., Cotton, M. & Vegoda, L. (2010). *IPv4 Address Blocks Reserved
for Documentation.***
Two pages. Use these ranges in your own diagrams.

**RFC 3927 — Cheshire, S., Aboba, B. & Guttman, E. (2005). *Dynamic Configuration of
IPv4 Link-Local Addresses.***
APIPA, standardised. The conflict-detection procedure is the interesting part.

**RFC 1112 — Deering, S. (1989). *Host Extensions for IP Multicasting.***
The original multicast specification. Elegant, and worth reading alongside §27.3's
account of why it did not deploy on the public Internet.

**RFC 1546 — Partridge, C., Mendez, T. & Milliken, W. (1993). *Host Anycasting
Service.***
**Read this one.** Nine pages describing a mechanism that required no invention — the
authors observed that routing already produces anycast behaviour and wrote down what it
would mean. Compare with RFC 1112 and draw the conclusion.

**RFC 4786 — Abley, J. & Lindqvist, K. (2006). *Operation of Anycast Services.***
The practical guide: how to actually run an anycast service, including the TCP caveats
of §27.3.

**RFC 2827 / BCP 38 — Ferguson, P. & Senie, D. (2000). *Network Ingress Filtering.***
Two pages, universally recommended, incompletely deployed. Read it and then read
Chapter 62 §62.4 for what the non-deployment costs.

**RFC 2644 — Senie, D. (1999). *Changing the Default for Directed Broadcasts in
Routers.***
The Smurf attack's remedy, and a good short example of a security fix delivered as a
default change.

## Books

**Doyle, J. & Carroll, J. *Routing TCP/IP*, Volume 1, chapter 3, and Volume 2 for
multicast.**
The multicast material in Volume 2 is the best available treatment for people who will
actually configure PIM and IGMP.

**Limoncelli, T., Hogan, C. & Chalup, S. (2016). *The Practice of System and Network
Administration*, 3rd ed.**
Chapters on documentation, naming and IP address management. **The best available
argument for why the unglamorous discipline of §27.4 matters**, written by people who
have suffered from its absence.

**Kurose, J. & Ross, K. *Computer Networking*, chapter 4.**
Good, brief treatment of the four delivery models with the multicast/CDN comparison.

**Bush, R. & Meyer, D. RFC 3439, *Some Internet Architectural Guidelines and
Philosophy*.**
Relevant to the whole chapter: the argument that operational simplicity beats
architectural elegance, which is exactly why anycast beat multicast.

## On IPAM

**NetBox documentation** (netbox.dev).
Install it and model your own network. The data model itself is instructive — the fields
it insists on are §27.4's fields, arrived at independently by people solving the same
problem.

**Infoblox and BlueCat white papers on DDI.**
Vendor material, and the argument for integrating DNS, DHCP and IPAM is sound
independently of whose product you buy. Read for the reasoning, not the conclusion.

**RIPE, ARIN and APNIC address policy documents.**
How addresses are actually allocated at the registry level, and what an organisation must
justify to receive public space. Worth reading once to understand why public IPv4 is
expensive and why the transfer market exists.

## Applied

**`ip addr` on every device you can reach.** Find one with a `169.254.x.x` address —
there is usually one somewhere — and work out why.

**`ss -tlnp`** and read the bind addresses. Anything on `0.0.0.0` that should be on
`127.0.0.1` is a finding. Do this on your own machine first.

**`dig CHAOS TXT id.server @8.8.8.8`** and the same from a different network. Different
answers, same address — that is anycast, visible in one command. Also try
`dig +nsid`.

**`traceroute 1.1.1.1`** from two networks in different countries and compare hop counts.
Both reach "the same" server in a handful of hops.

**`ip maddr show`** and `netstat -g` for multicast group membership on a host. Look at
what your machine has already joined without being asked.

**Capture with `tcpdump -i eth0 multicast`** on a quiet segment. There is more than you
expect — mDNS, SSDP, routing protocol hellos — and identifying each one is a good
exercise.

**Lab 15** in this book's [labs/](../../../labs/) directory covers multicast with and
without IGMP snooping, measuring the flooding difference, and includes an anycast
demonstration using two hosts announcing the same /32 over a routing protocol.

**[tools/netcalc.py](../../../tools/netcalc.py) `local`** prints your machine's networks
with their classification — private, public, link-local, loopback — which is a quick way
to check what you are actually on.

## For the certification-minded

Objective 1.7 expects private ranges, APIPA, loopback and the delivery models.
Objective 3.1 expects documentation and IPAM as operational practice.

Eight things worth over-learning:

1. **`10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`** — and that **`172.32.x.x` is
   public**.
2. **`169.254.x.x` means DHCP failed.** Expect a scenario question.
3. **`127.0.0.0/8` is loopback** — the whole /8.
4. **`100.64.0.0/10` is carrier-grade NAT.**
5. **Multicast is `224.0.0.0/4`.**
6. **Broadcast does not cross a router**, and **IPv6 has no broadcast**.
7. **Anycast is one-to-nearest** — the definition most often confused with multicast.
8. **`255.255.255.255` is never forwarded.**

The scenario that appears most: **a user with an APIPA address — what is wrong?** The
answer is that no DHCP reply arrived, and the most common enterprise cause is the wrong
VLAN on the switch port.

And the practice worth more than any exam item: **document the reserved ranges
explicitly.** Address space that nobody can prove is free is address space nobody will
use, and every network that ran out of addresses while holding thousands of unused ones
ran out for exactly that reason.
