# Chapter 27 — The People

**Yakov Rekhter (1953–2023), Robert Moskowitz, Daniel Karrenberg, Geert Jan de Groot and
Eliot Lear.** Authors of **RFC 1918** (1996), *Address Allocation for Private Internets*.

The document is short and its reasoning is worth reading directly, because it is
unusually candid about the trade. It states plainly that private addressing **breaks the
end-to-end model**, that hosts using it cannot be reached from the Internet, and that
organisations choosing it accept a real limitation.

It recommends it anyway, and the judgement was right. **The alternative was running out
of addresses in about 1998.**

**Daniel Karrenberg** was RIPE NCC's founder and a central figure in European Internet
governance; his presence on the document reflects that the address registries were the
people watching consumption most closely and were the most alarmed.

**Kjeld Borch Egevang and Paul Francis.** RFC 1631 (NAT), which is RFC 1918's necessary
companion — private addressing is useless without translation at the boundary. Chapter 21
and Chapter 33 cover the architecture; the pairing is what actually deferred exhaustion.

**Stuart Cheshire and Bernard Aboba.** **RFC 3927** (2005), *Dynamic
Configuration of IPv4 Link-Local Addresses* — the standardisation of what Microsoft had
shipped as APIPA and Apple as part of Zeroconf.

Cheshire's larger project across three decades has been **networks that work without
configuration**: link-local addressing, mDNS, DNS service discovery, and Bonjour. The
argument is that requiring configuration is a design failure for anything a non-expert
must use, and it is why plugging two laptops together with a cable simply works, and why
a printer appears on a home network without anyone doing anything.

**The `169.254.x.x` diagnostic of §27.2 is a side effect of that work**, and a valuable
one: because the fallback is standardised and universal, its presence is unambiguous
evidence about what happened.

**Steve Deering.** **IP multicast** — his 1988 Stanford dissertation and
RFC 1112 (1989). Deering's design is elegant: a group is an address, joining is a local
operation, and senders need know nothing about receivers.

Its fate is one of this book's recurring lessons. **Technically excellent, deployed
enthusiastically within enterprises, and essentially absent from the public Internet** —
defeated not by any technical flaw but by an incentive structure in which the party who
must deploy it is not the party who benefits. §27.3 gives the reasons; Deering has
discussed them at length and without bitterness.

He went on to be the principal architect of **IPv6** (Chapter 28), where he applied the
lesson: IPv6 **removed broadcast entirely** and made multicast fundamental to the
protocol's own operation — so that multicast support is not optional, because the protocol
does not work without it. **Making a mechanism load-bearing is the most reliable way to
ensure it is implemented.**

**Van Jacobson (b. 1950), Steve Deering and the MBone.** The Multicast Backbone, from
1992: an overlay of tunnels carrying multicast between the parts of the Internet that
supported it, used for IETF meeting broadcasts and early Internet video.

It worked, it was genuinely exciting, and it did not become infrastructure. **A working
demonstration is not deployment**, and the MBone is the clearest case of the distinction
in this book.

**Craig Partridge, Trevor Mendez and Walter Milliken.** **RFC 1546** (1993), *Host
Anycasting Service* — the first statement of anycast.

What is striking about the paper is that anycast **needed no invention**. They observed
that if the same prefix were announced from several places, routing would already deliver
each packet to the nearest instance, and wrote down what that would mean. The
"protocol" is a consequence of existing behaviour.

**Compare with multicast**, which required a new address range, IGMP, PIM, MSDP, switch
features, and cooperation from every network in the path — and did not deploy. Anycast
required nothing and is now the foundation of the DNS root, of public resolvers, of every
CDN, and of every serious DDoS mitigation service.

> **The mechanism that required nothing new is the one that deployed.** This is Chapter
> 23's incremental-deployability argument in its purest form.

**Paul Vixie (b. 1963) and the root server operators.** The anycasting of the DNS root
from around 2002 onward, growing from thirteen physical machines to more than 1,900
instances. Chapter 39 covers DNS; the achievement here is that **the root became
essentially unattackable by volume** without any change to the protocol, to clients, or
to the thirteen published addresses.

**Bandwidth attacks against the root have been attempted repeatedly and have failed**,
and anycast is the reason.

**Paul Ferguson and Daniel Senie.** **RFC 2827 / BCP 38** (2000), *Network Ingress
Filtering*. Two pages: do not let packets leave your network with source addresses you do
not own.

**Universally recommended, incompletely deployed, twenty-five years on.** The economics
are the whole story — filtering costs the operator who does it and protects everyone
else — and it is the sharpest example in this book of Arkko's principle that **the party
who must act is rarely the party who is harmed**. Chapter 62 §62.4 covers what the
non-deployment enables.

**Jeremy Stretch and the NetBox community.** NetBox began at DigitalOcean as an internal
tool and became the de-facto open-source standard for network source-of-truth data.

Its significance for this chapter is that it made **rigorous address documentation
achievable for organisations that would never buy a commercial DDI system.** Before it,
the realistic choice for most networks was a spreadsheet or nothing.

Chapter 70 §70.3 returns to it, because the automation story depends entirely on having a
machine-readable source of truth — and **the documentation discipline of §27.4 turns out
to be the prerequisite for everything modern in network operations.**
