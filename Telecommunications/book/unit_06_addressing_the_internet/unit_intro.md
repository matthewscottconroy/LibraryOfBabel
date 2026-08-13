# Unit VI — Addressing the World

Unit IV built a network that works beautifully and does not scale at all.

Ethernet's addressing is flat, and Chapter 15 §15.2 explained the consequence: a
switch must learn every address individually, because there is no rule that
summarises a group of them. On a segment of two hundred hosts this is fine. On a
network of twenty billion devices it is arithmetically impossible — there is no
memory in which to hold the table, and no mechanism by which the table could be
distributed.

So we need a second addressing system, and it must have the property that
Ethernet's lacks: **an address must tell you something about where the thing is**,
so that a forwarding device can make a decision about a whole set of addresses
without knowing any of them individually.

That is the entire subject of this unit, and the mechanism is hierarchy.

## The analogy that actually works

A postal address is hierarchical: country, then city, then street, then number. A
sorting office in Melbourne handling a letter to Lyon does not consult a list of
French addresses. It applies one rule — *France goes in that bag* — and forgets the
rest. Every subsequent office refines the decision using a more specific part of
the address, and no office anywhere needs global knowledge.

An IP address is the same idea in binary. Its leading bits identify a network, its
trailing bits identify a host within that network, and the boundary between them is
movable. A router in Sydney holding a packet for `203.0.113.47` need not know that
address exists; it needs only a rule covering `203.0.113.0/24`, or `203.0.0.0/8`, or
in the limit `0.0.0.0/0`.

The consequence, and it is a large one: **the global routing table holds roughly a
million entries rather than twenty billion.** That factor of twenty thousand is
what hierarchy buys, and it is the only reason the Internet functions.

## Why this unit is the hardest and the most valuable

Subnetting is where students most often lose their footing, and where practitioners
most often have a shaky floor under years of working knowledge. There is a reason,
and it is Chapter 2's fault as much as anyone's.

Dotted decimal notation — `192.168.10.70` — presents a 32-bit integer as four
decimal numbers, which is legible to humans and actively obstructive to the one
operation that matters. The operation is *masking off a prefix*, and it is a bitwise
operation that has nothing to do with decimal digits. Students who learn subnetting
as decimal recipes can handle /24 boundaries and fall apart at /27, because the
recipe does not generalise and the underlying structure was never visible.

This unit refuses the recipes. Chapter 26 works `192.168.10.70/27` out in binary,
completely, before showing any shortcut — and then shows the shortcuts as
*consequences* of the binary, so that when a shortcut is forgotten it can be
rederived rather than looked up.

The investment is worth making precisely once. Subnetting done properly is a skill
you have permanently; subnetting done by chart is a skill you lose within months of
the exam.

## What the unit contains

**Chapter 24 — The Internet Protocol.** Best-effort delivery as a deliberate
choice rather than a limitation; the IPv4 header field by field; fragmentation and
why it is a design error we still pay for; and the TTL that keeps loops from being
fatal.

**Chapter 25 — IPv4 Addresses and Masks.** Dotted decimal as a costume over a
32-bit integer; the network/host split; the mask as a bitwise operator; and the
classful era, which ended in 1993 and whose ghosts still haunt default
configurations and interview questions.

**Chapter 26 — Subnetting, CIDR, and VLSM.** The core chapter of the unit and
arguably of the book. Borrowing bits; a fully worked example in binary; the
shortcuts derived rather than asserted; CIDR and aggregation; and variable-length
subnet masking, which is how a real address plan is built.

**Chapter 27 — Address Plans in Practice.** RFC 1918 private space; the special
ranges (loopback, link-local, documentation, CGNAT) and what each is for; unicast,
broadcast, multicast and anycast; and what a defensible address plan document
actually contains — which is the deliverable your semester project will produce.

**Chapter 28 — IPv6.** Why it exists, with the exhaustion dates; the notation and
the address types; SLAAC and DHCPv6; and the transition mechanisms, plus an honest
account of why a protocol standardised in 1998 is still not universal in 2026.

## A note on the arithmetic

You will do a lot of binary in this unit. Do it on paper.

There are excellent subnet calculators, and you will use them professionally, and
you should — checking work by machine is good practice. But a calculator used
*instead of* understanding produces an engineer who cannot look at a routing table
and see that two entries overlap, cannot tell at a glance that a host has been given
the network address by mistake, and cannot design an address plan that summarises.
Those are not exam skills. They are the daily work.

Chapter 2 §2.2's octet place values — 128, 64, 32, 16, 8, 4, 2, 1 — and the block
size rule are the whole toolkit. If they are not automatic yet, go back for twenty
minutes before starting Chapter 26. It will save you an afternoon.
