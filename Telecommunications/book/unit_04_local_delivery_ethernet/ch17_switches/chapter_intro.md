# Chapter 17 — Switches

There is a single mechanism at the heart of this chapter, it fits in a paragraph,
and it is probably the highest ratio of consequence to complexity in the entire
book.

A switch receives a frame on a port. It reads the frame's **source** address and
records: *this address is reachable through this port*. It then reads the frame's
**destination** address and consults the same table. If the destination is in the
table, it sends the frame out that port and no other. If it is not, it sends the
frame out every port except the one it arrived on, and waits to learn.

That is the whole algorithm. Learn from the source, forward by the destination,
flood when you do not know. Everything else — spanning tree, VLANs, the entire
enterprise switching industry — is elaboration on those three sentences.

## What it replaced, and why the improvement is so large

Before switches there were **hubs**, and a hub is a device with no intelligence
whatsoever: a signal arriving on any port is regenerated onto every other port.
Electrically, a hub-based network is identical to the shared coaxial cable of
Chapter 16, merely rearranged into a physically convenient star (Chapter 11 §11.3).

That means every station hears every frame. Which in turn means:

- **The bandwidth is shared.** Twelve stations on a 10 Mb/s hub have 10 Mb/s
  between them, not each.
- **Collisions are inevitable.** All twelve are in one collision domain, contending
  under CSMA/CD, and utilisation suffers accordingly.
- **Only half duplex is possible.** A station must listen while transmitting to
  detect collisions, so it cannot use the return path simultaneously.
- **There is no privacy at all.** Every frame is delivered to every station's
  network interface, which discards those not addressed to it — unless placed in
  promiscuous mode, which takes one command. Packet capture on a hub network
  requires no special access, and this is exactly why network monitoring in the
  1990s was so much easier than it is now.

Switching removes all four. Each port becomes its own collision domain; the frame
goes only where it is needed; the link can run full duplex because there is nobody
to collide with; and a station no longer receives frames addressed to others. A
12-port switch does not divide 10 Mb/s twelve ways — it provides 10 Mb/s per port,
and can forward several frames simultaneously between different pairs of ports.

The performance improvement from replacing a hub with a switch is not incremental.
It is often an order of magnitude, and it is why hubs disappeared entirely once
switch prices fell.

## The two domains, and the sentence to memorise

The chapter's most examinable content, and its most frequently confused:

> **A switch breaks up collision domains. It does not break up broadcast domains.
> A router breaks up broadcast domains.**

A **collision domain** is the set of interfaces that can collide with one another —
the region within which CSMA/CD arbitration applies. Each switch port is one.

A **broadcast domain** is the set of interfaces that receive a broadcast frame sent
by any of them. A switch floods broadcasts out every port by construction, so an
entire switched network — however many switches, however many ports — is a single
broadcast domain until something stops it.

The two things that stop it are a router (Chapter 29) and a VLAN (Chapter 20), and
a VLAN is best understood as a way of making one physical switch behave as several
independent switches, which is to say several broadcast domains.

This matters practically because broadcast domains have a size limit. Every host
must process every broadcast, ARP traffic (Chapter 18) grows with the square of
host count in the worst case, and a single misbehaving device can saturate the
entire domain. The traditional guidance of "a few hundred hosts per broadcast
domain" is soft, dated, and directionally correct.

## The table, and its limits

The MAC address table is finite. A small access switch might hold 8,000 entries; a
data-centre switch, 128,000 or more. Entries age out, typically after 300 seconds
of silence.

Both facts have consequences. Ageing means a silent device is eventually forgotten
and frames for it are flooded again until it speaks — which is normally harmless
and occasionally the explanation for a puzzling traffic pattern. Finiteness means
the table can be **filled deliberately**: an attacker generating frames from
hundreds of thousands of fabricated source addresses can exhaust it, at which
point the switch, unable to learn, floods everything — converting itself back into
a hub and restoring the eavesdropping capability that switching removed. This is
**MAC flooding** or **CAM table overflow**, the mitigation is port security, and
Chapter 62 covers both.

Note the structure of that attack: it does not break the switch, it degrades it to
an earlier, weaker design. A surprising number of network attacks have that shape,
and recognising it is a useful instinct.

## What this chapter does

§17.1 traces the evolution from repeater to hub to bridge to switch as one
continuous line, and explains what each added.

§17.2 covers the MAC address table in detail: learning, ageing, flooding,
unicast/multicast/broadcast handling, and how to read a real `show mac
address-table` output.

§17.3 develops collision and broadcast domains rigorously, with worked examples of
counting each in a described network — a standard exam task and a genuinely useful
skill.

§17.4 covers forwarding modes (store-and-forward, cut-through, fragment-free),
their latency and error-propagation tradeoffs, buffer architecture, head-of-line
blocking, microbursts, and why a switch with plenty of headroom can still drop
frames.

## By the end you will be able to

- State the switch algorithm in three sentences and trace it through a sequence of
  frames on a multi-switch network, predicting exactly which ports each frame
  exits.
- Count collision domains and broadcast domains in any described topology.
- Explain MAC table ageing and predict the traffic pattern it produces.
- Compare forwarding modes and choose one for a stated requirement.
- Explain MAC flooding, why it works, and how port security prevents it.
- Explain why a switch reporting low average utilisation can still be dropping
  frames.
