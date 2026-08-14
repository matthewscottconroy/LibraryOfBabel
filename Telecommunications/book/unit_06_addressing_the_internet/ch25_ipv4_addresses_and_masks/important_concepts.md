# Chapter 25 — Important Concepts

An IPv4 address is a 32-bit integer *(§25.1)* — Dotted decimal is presentation.
`192.168.10.70`, `11000000 10101000 00001010 01000110` and `3232238150` are the same
thing. **The largest obstacle to learning subnetting is learning the dots first and the
integer never.**

The mask octet table *(§25.1)* — 0, 128, 192, 224, 240, 248, 252, 254, 255. The
only values a mask octet can take. The highest-value memorisation in the subject —
every subnetting question depends on it. Any other value is a malformed mask.

Powers of two to 2¹⁰ *(§25.1)* — 1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024.
Required for host counts and block sizes.

**4,294,967,296 addresses** *(§25.1)* — Extravagant in 1981 when the world had a few
hundred networked computers; inadequate now, and the usable count is far lower after
reservations and allocation waste.

The dots are punctuation *(§25.1)* — Nothing in IP cares about octet boundaries.
A /26 splits mid-octet, a /12 splits mid-second-octet, and every subnetting difficulty
traces back to treating the dots as boundaries.

Leading zeros are dangerous *(§25.1)* — `192.168.010.70` is parsed as **octal** by
some libraries, making that octet 8. This has caused real security incidents.

`169.254.x.x` means DHCP failed *(§25.1)* — Recognise it instantly; it localises a
fault in one glance.

**The network/host split** *(§25.2)* — **Left part identifies a network, right part
identifies a host on it.** Everything in Chapters 25, 26 and 30 follows from this
sentence.

Why the split exists *(§25.2)* — It makes **aggregation** possible: a router needs
one entry for `203.0.113.0/24`, not 254. **20 billion devices → under 1 million routing
entries.** The Internet works not because routers are fast but because the address
structure means they do not have to know much.

The split falls anywhere *(§25.2)* — Any of 33 positions, set by the mask. **The /24
boundary is a convention because it aligns with an octet**, and nothing else.

Usable hosts = 2^h − 2 *(§25.2)* — All-zeros in the host portion is the **network
address; all-ones is the broadcast address**. Neither can be assigned.

/30 gives 2, /31 gives 2 (RFC 3021), /32 gives 1 *(§25.2)* — A /31 on a
point-to-point link is legal and widely underused; a /30 wastes half its addresses for
no reason. A /32 is a **host route**, used for router loopbacks and anycast.

`192.168.10.70/26` is not the 70th host of anything *(§25.2)* — It is the 7th host
of the network starting at 64. The dotted notation actively misleads here.

An address alone does not determine its network *(§25.2)* — `192.168.10.70` is on
seven different networks depending on the mask. Always write the mask. Documentation
listing addresses without masks is nearly useless.

A mask is a run of ones then a run of zeros *(§25.3)* — Nothing else. **The ones
must be contiguous and first**, because longest-prefix match depends on a prefix being a
leading run. Non-contiguous masks are invalid.

The mask is a stencil *(§25.3)* — AND preserves bits where the mask is 1 and erases
them where it is 0. It keeps the network bits and erases the host bits. Once the
mask is a stencil rather than a mysterious number, Chapter 26 becomes mechanical.

**The local-or-remote decision** *(§25.3)* — Performed before every packet:
AND both addresses with the sender's own mask and compare. Equal → ARP for the
destination. Different → ARP for the gateway, with the IP destination unchanged.

Only the sender's mask matters *(§25.3)* — The host does not know the destination's
mask, cannot know it, and does not need it. **This asymmetry is what makes a mask
mismatch produce one-way connectivity.**

**The mask mismatch** *(§25.3)* — The most confusing common fault in IP networking.
A `/24` host sees a `/25` neighbour as local and ARPs directly; the `/25` host sees the
`/24` host as remote and sends to the gateway. Works one way, not the other, or
erratically. Symptoms: selective failure with no pattern, one-way ping, ARP for
off-subnet addresses, same-switch traffic traversing the router. **Diagnosis is always
"compare the masks", takes ten seconds, and is skipped constantly.**

Wildcard masks are inverted *(§25.3)* — 0 means must-match, 1 means don't-care.
Convert by subtracting each octet from 255. `/26` → `0.0.0.63`. `host` = `0.0.0.0`,
`any` = `255.255.255.255`. They exist because a wildcard **may be non-contiguous**,
expressing matches a prefix cannot.

Reading a mask quickly *(§25.3)* — **Count the ones** for the prefix; **subtract from
256 for the block size (the "magic number"); check every octet is in the table** and
that nothing follows a zero.

**Classful addressing** *(§25.4)* — The class determined the mask, encoded in the
leading bits, because RFC 791 had no room for a mask field. A: 1–126 (/8), B: 128–191
(/16), C: 192–223 (/24), D: 224–239 multicast, E: 240–255 reserved. **127 is loopback.**

**Why it failed** *(§25.4)* — (1) **Three sizes only** — an organisation with 300 hosts
took a class B and wasted 65,234 addresses; **class B exhaustion was projected for
1994**. (2) **Routing tables grew unmanageably** because sixteen contiguous class Cs
could not be expressed as one entry — **the more urgent problem, since address shortage
is inconvenient and routing table collapse is fatal**. (3) It could not be fixed
incrementally.

**Subnetting (RFC 950, 1985)** *(§25.4)* — Let an organisation divide its **own**
allocation. The outside world still saw one class B. **It solved neither global problem
— and it introduced the mask as an explicit object, which made the class redundant.**

CIDR (RFC 1519, 1993) *(§25.4)* — Abolish classes, carry the prefix length
explicitly. **Right-sized allocations** cut waste from 99% to 40%; **aggregation** turns
sixteen entries into one; **provider-based allocation** means the routing table grows
with the number of providers rather than the number of organisations. **The growth curve
bent visibly in 1994.**

**The ghosts** *(§25.4)* — **Classful defaults** when a mask is omitted (always specify
it); **classful routing protocols** (RIPv1, IGRP — the test is *does it carry the
mask?*); **auto-summarisation** breaking discontiguous networks, which is why
`no auto-summary` was on every configuration for fifteen years; **the vocabulary**
("a class C" meaning /24); and **RFC 1918's ranges**, which are on class boundaries.

`172.16.0.0/12` is a fossil *(§25.4)* — Sixteen class Bs, which is why the range is
16–31 and the prefix is the awkward /12. The most commonly forgotten private range,
and `172.32.5.1` is the standard exam distractor.
