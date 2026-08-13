# Chapter 27 — Address Plans in Practice

Chapter 26 gave you the arithmetic. This chapter is about the judgement, which is
the part that is not on the exam and is most of the job.

An address plan is a document. It is a durable artefact that will outlive the
person who wrote it, that will be consulted by people who were not in the room, and
that — if it is any good — will still make sense when the organisation has three
times as many sites and a cloud presence that nobody anticipated. Producing one is
the deliverable of Week 6 of the semester project, and it is the single most
common piece of design work a network engineer is actually asked to do.

## Private space, and the RFC that shaped the modern Internet

In February 1996, RFC 1918 reserved three ranges for private use:

| Range | CIDR | Addresses | Old name |
|---|---|---|---|
| 10.0.0.0 – 10.255.255.255 | 10.0.0.0/8 | 16,777,216 | one Class A |
| 172.16.0.0 – 172.31.255.255 | 172.16.0.0/12 | 1,048,576 | sixteen Class Bs |
| 192.168.0.0 – 192.168.255.255 | 192.168.0.0/16 | 65,536 | 256 Class Cs |

These addresses are not routable on the public Internet. Every provider drops them.
Any organisation may use any of them, simultaneously, without coordination — which
is precisely the point, and which is why your home network and several hundred
million others are all on `192.168.1.0/24` without incident.

The consequences were larger than intended. RFC 1918 combined with NAT (Chapter 33)
removed most of the pressure that IPv4 exhaustion was applying, which bought two
decades and — as §27.1 argues — is a substantial part of why IPv6 deployment took
thirty years. A problem that has been made survivable is a problem that does not get
solved.

There is also a practical trap worth stating early. `10.0.0.0/8` is enormous and
free, so people use it casually — and then merge with a company that did the same,
and discover that both organisations built on `10.1.0.0/16`. Overlapping private
space after a merger is a genuinely miserable problem whose only clean solutions are
renumbering (expensive, disruptive) or double NAT (fragile, opaque). **Choosing an
unusual slice of 10/8 costs nothing today and is worth real money later**, and this
is the kind of judgement a plan document should record along with its reasoning.

## The special ranges

Beyond RFC 1918, a set of ranges are reserved for specific purposes, and knowing
them prevents both configuration errors and a class of diagnostic confusion:

- **127.0.0.0/8** — loopback. The whole /8, not merely `127.0.0.1`; a packet to any
  of the sixteen million never leaves the host.
- **169.254.0.0/16** — link-local, **APIPA**. Self-assigned when DHCP fails. Seeing
  one of these on a client is a diagnosis, not a configuration: it means *no DHCP
  server answered*, and Chapter 40 §40.3 is where you go next. Recognising a
  169.254 address on sight is one of the highest-value pattern matches in
  troubleshooting.
- **100.64.0.0/10** — carrier-grade NAT space (RFC 6598), for providers to use
  between their customers and their own NAT. Increasingly visible on home
  connections, and a source of surprise when someone expects a public address.
- **192.0.2.0/24, 198.51.100.0/24, 203.0.113.0/24** — documentation (RFC 5737).
  Used throughout this book, and the correct thing to use in your own diagrams
  instead of someone's real addresses.
- **224.0.0.0/4** — multicast.
- **255.255.255.255** — limited broadcast, never forwarded by a router.

## Four ways to address a destination

The chapter's conceptual core, because the four are routinely confused and the
distinctions determine real behaviour:

**Unicast** — one sender, one recipient. Nearly all traffic.

**Broadcast** — one sender, every host on the subnet. Delivered to all, processed
by all, and therefore expensive: every host's CPU is interrupted. This is why
broadcast domain size matters (Chapter 17 §17.3). IPv6 has no broadcast at all; the
designers concluded it was a mistake and replaced it with scoped multicast.

**Multicast** — one sender, a *subscribed group*. Hosts join a group; the network
delivers one copy per link rather than one per recipient. This is how IPTV, market
data feeds, and some conferencing work, and it is enormously more efficient than
unicast replication for genuinely one-to-many traffic. It is also operationally
demanding — IGMP, snooping, PIM — which is why it is deployed far less than its
efficiency would justify.

**Anycast** — one address, *many* hosts, and the routing system delivers to the
nearest. This sounds exotic and is completely routine: it is how the DNS root
servers work (thirteen addresses, well over a thousand physical servers), how
public resolvers like `8.8.8.8` and `1.1.1.1` work, and how every CDN steers users
to a nearby edge (Chapter 52). Anycast is a *routing* trick rather than an
addressing feature — nothing in the packet is special — and that is worth
understanding, because it means anycast failover works at BGP speed rather than DNS
speed.

## What a plan document contains

§27.4 specifies the deliverable, and it is worth previewing because it is what your
project will be graded against:

1. **The blocks held**, with source and status.
2. **The allocation hierarchy** — how the space is divided by site, then by
   function, then by VLAN — chosen so that each level summarises.
3. **The per-subnet table**: prefix, purpose, VLAN ID, gateway, DHCP range,
   reserved static range, and current utilisation.
4. **The conventions**: gateway always `.1`; infrastructure in the low range;
   DHCP pool in the middle; static servers in a reserved band; and — critically —
   the reasoning behind each convention, so a successor does not violate it by
   accident.
5. **Growth headroom**, stated explicitly, with the assumption behind it.
6. **The IPv6 plan**, even if not yet deployed.

The convention that matters most and is most often skipped: **leave gaps
deliberately.** A plan that packs subnets perfectly against each other has no room
for the site that grows, and renumbering a live subnet is a genuinely disruptive
operation. Space in 10/8 is free; disruption is not.

## By the end you will be able to

- Choose an appropriate private range for an organisation and justify it against
  the merger scenario.
- Identify any special-purpose range on sight and state what its presence implies.
- Distinguish unicast, broadcast, multicast and anycast, and choose correctly for a
  stated requirement.
- Diagnose a 169.254 address immediately and know the next three things to check.
- Produce a complete, defensible address plan document for a multi-site
  organisation.
