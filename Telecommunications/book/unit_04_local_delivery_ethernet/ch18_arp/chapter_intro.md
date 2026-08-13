# Chapter 18 — ARP

By the end of Unit VI you will have two address systems, and they will not know
about each other.

**MAC addresses** identify hardware. They are 48 bits, flat, factory-assigned, and
they are what an Ethernet frame actually uses to decide which interface should
accept it. Nothing on a local segment moves without one.

**IP addresses** identify a host's position in the global network. They are 32 bits
(or 128), hierarchical, administratively assigned, and they are what makes routing
across the world possible. Nothing crosses a network boundary without one.

Both are necessary and neither can replace the other. A frame cannot be delivered
on a local segment using an IP address, because Ethernet hardware does not read IP
addresses — the destination field in the frame is 48 bits and it is a MAC address,
full stop. And a packet cannot cross the Internet using a MAC address, because MAC
addresses are flat and cannot be aggregated, as Chapter 15 §15.2 argued.

So a host that wishes to send a packet to `192.168.10.1` and knows that this
address is on its own segment faces a concrete, immediate problem: **it must put a
frame on the wire, the frame requires a 48-bit destination address, and the host
does not have one.** It knows *who* it wants to talk to and not *how to address the
envelope*.

The **Address Resolution Protocol**, specified by David Plummer in RFC 826 in
November 1982, solves this in the most direct way imaginable: it asks everybody.

```
Host A broadcasts:  "Who has 192.168.10.1?  Tell 192.168.10.70"
Host B replies:     "192.168.10.1 is at 00:1b:44:11:3a:b7"
```

The question goes to the broadcast address `ff:ff:ff:ff:ff:ff`, so every station on
the segment receives it. Every station examines it; the one that recognises its own
IP address replies directly; everyone else discards it silently. Host A caches the
answer and gets on with its life.

RFC 826 is four pages long. It is one of the shortest specifications in the
Internet suite and one of the most consequential, and reading it is a genuinely
pleasant twenty minutes.

## The three things worth understanding beyond the mechanism

**ARP is the glue between two layers, and it belongs to neither.** It is carried
directly in Ethernet frames with EtherType `0x0806` — not inside IP — because it
must work before IP communication is possible. Attempts to place it neatly in the
OSI model (Chapter 22) fail; it is usually called "Layer 2.5" with some
embarrassment. This is a good early example of the fact that layering is a useful
model and not a law of nature.

**ARP has no authentication whatsoever, by design.** Any station may reply to any
request, and any station may send an unsolicited reply that receivers will cache.
In 1982, on a network of mutually trusting research institutions, this was
reasonable. Today it is the basis of **ARP spoofing**, one of the most
straightforward on-path attacks in existence: claim to be the default gateway, and
every host on the segment sends you their traffic. Chapter 62 covers the attack;
mitigations include Dynamic ARP Inspection and DHCP snooping, and the fact that
they are *add-ons* rather than protocol features tells you something about how
security arrived in this field.

**ARP failure produces a characteristic symptom.** If ARP resolution fails, the
host has a routing table entry, an IP address, a working cable and a link light,
and simply cannot communicate — with no error message, because ARP has no failure
notification. The signature is `ping` reporting "Destination Host Unreachable"
generated *locally*, and an ARP cache entry showing `incomplete`. Recognising that
signature saves hours, and Chapter 65 catalogues it.

## And the redesign

IPv6 does not use ARP. It uses **Neighbor Discovery Protocol**, defined in RFC 4861
and carried inside ICMPv6, and the changes are instructive because they are the
1998 designers correcting the 1982 designers with sixteen years of hindsight.

NDP replaces broadcast with *multicast* — a solicitation goes only to the
solicited-node multicast group, so uninterested hosts are not interrupted. It
integrates router discovery, prefix advertisement and address autoconfiguration
into the same protocol, so a host can obtain its address, its prefix and its
gateway from one mechanism (Chapter 28's SLAAC). And it has an optional
cryptographic authentication extension, SEND, which addresses the spoofing problem
and which almost nobody deploys — a recurring pattern worth noting.

## What this chapter does

§18.1 develops the two-address-worlds problem and why both systems are necessary.

§18.2 walks the request/reply exchange packet by packet, with the ARP packet format
and a real capture, including the case where the destination is *not* local and the
host must ARP for its gateway instead — which is the case that confuses people, and
which is essential preparation for Chapter 29.

§18.3 covers the ARP cache: timers, states, gratuitous ARP and its legitimate uses,
proxy ARP and why it is usually a mistake, and the failure modes with their
symptoms.

§18.4 covers NDP: solicitation and advertisement, the solicited-node multicast
address, duplicate address detection, and the comparison with ARP.

## By the end you will be able to

- Explain why two address systems exist and why neither can be eliminated.
- Trace an ARP exchange in a capture and identify every field.
- Predict correctly which address a host will ARP for, given a destination and a
  subnet mask — including the crucial off-subnet case.
- Read an ARP cache and identify an incomplete entry, a duplicate, and a spoof.
- Explain ARP spoofing and the mitigations available on a managed switch.
- Describe how NDP differs from ARP and what each difference was intended to fix.
