# 1.3 Hosts, Links, and Protocols

A network is built from exactly three kinds of thing. Everything else in this
book — every acronym, every standard, every vendor's product line — is a
specialisation of one of them.

## Hosts, nodes, endpoints

A **host** is a device that originates or consumes messages. Your laptop, a web
server, a printer, a thermostat, an industrial sensor. The defining property is
that a host is *where the application lives*. It is the source and destination of
Shannon's diagram, wearing a chassis.

A **node** is any device attached to the network, whether or not it originates
traffic. All hosts are nodes; not all nodes are hosts. A switch is a node and is
not a host, because it exists to move other people's messages, not to send its
own. (It does send some of its own — spanning tree BPDUs, management traffic —
and this is exactly the kind of edge case that makes precise terminology worth
having.)

An **intermediate system** or **relay** is a node whose purpose is forwarding:
repeaters, hubs, bridges, switches, routers, firewalls. The ISO documents use
"intermediate system" and "end system," which is clunky but unambiguous, and if
you ever read a routing standard you will meet IS-IS, whose name is literally
*Intermediate System to Intermediate System*.

An **endpoint** is a modern security-flavoured word for a host, emphasising that
it sits at the edge of the network and is the thing an attacker wants. When a
vendor says "endpoint," they usually mean "laptop we would like to sell you
software for."

The distinctions matter because the answer to *what does this device do with a
frame it did not ask for* differs completely between them, and that question is
the whole of Chapter 17.

## Links

A **link** is a communication channel that directly connects two or more nodes,
with no intervening node. "Directly" is doing work in that sentence: a link may
contain amplifiers, repeaters, media converters and a thousand kilometres of
glass, and still be one link, provided nothing along the way makes a *forwarding
decision*. The moment a device inspects an address and chooses an output, the
link ends and a new one begins.

Links come in two topological flavours, and the distinction shapes Unit II and
Unit IV:

**Point-to-point.** Exactly two nodes. A fibre pair between two switches; a
serial line; a modern Ethernet cable from a PC to a switch port. Nobody else can
hear you. Nobody else can interrupt. This is the easy case, and much of the
history of networking is the story of engineers working very hard to make shared
media *behave* like point-to-point links.

**Multi-access (shared).** Three or more nodes on one medium, all of whom can
hear each other. Original coaxial Ethernet; a Wi-Fi channel; a cable-modem
segment; a satellite transponder. Multi-access links are enormously more
complicated, because they require rules about who may transmit when — the
**medium access control** problem, which is Chapter 16's whole subject and which
recurs in a different costume in Chapter 44.

Links also differ in **directionality**. A **simplex** link carries traffic one
way only (broadcast television; a temperature sensor's data feed). A **half
duplex** link can carry traffic in either direction but only one at a time (a
walkie-talkie; classic shared Ethernet; every Wi-Fi radio ever made). A **full
duplex** link carries traffic both ways simultaneously (a modern switched
Ethernet port, which uses separate pairs or separate wavelengths for each
direction).

That last taxonomy will earn its keep in Chapter 66, where **duplex mismatch** —
one end of a link full duplex, the other half — produces one of the most
characteristic and misdiagnosed performance failures in the field. The link works.
Ping succeeds. Throughput is a tenth of what it should be, and the interface
counters tell you exactly why, if you know to look.

## Protocols

A **protocol** is an agreement between two or more parties about the format and
meaning of what they exchange, and about the sequence in which they exchange it.

That definition has three clauses and each one matters.

**Format** — the **syntax**. Where do the fields sit, how many bits is each one,
what byte order, how is the end marked? If A sends a 32-bit address big-endian
and B reads it little-endian, both parties are following a protocol; they are
just not following the *same* one, and the result is not an error message but
nonsense delivered with confidence.

**Meaning** — the **semantics**. What does a 1 in this bit position *mean*? What
is the receiver obliged to do about it? A protocol specification that defines
only the format is a data structure, not a protocol.

**Sequence** — the **timing** or state machine. What may be sent when? What is a
valid reply to this message? How long do I wait before deciding you are not going
to answer? An enormous fraction of protocol complexity — and essentially all of
TCP's, as we will see in Chapter 37 — lives in this clause. The message formats
of TCP fit on one page; its state machine does not.

Protocols in this book range from the almost trivially simple to the genuinely
baroque. ARP (Chapter 18) is two message types and one table. BGP (Chapter 32) is
a path-vector protocol with a policy language, a finite state machine, and a
sufficiently rich configuration surface that a single typo in one autonomous
system can and repeatedly has removed large parts of the Internet from the
Internet.

## Composing the three

Given hosts, links, and protocols, a network is simply what you get when you
compose them:

> A **network** is a set of nodes connected by links, cooperating under a set of
> protocols to deliver messages between hosts.

And an **internetwork** — the word from which *Internet* is contracted — is what
you get when you apply the same construction one level up, treating whole
networks as the things being connected. That recursion is not a metaphor. It is
the literal architecture of the global Internet, and Chapter 48 traces it from
your laptop out to the autonomous systems that constitute the public core.

## A worked decomposition

Take one concrete action and name every piece. You type `example.com` into a
browser on a laptop in a coffee shop.

- **Hosts:** your laptop; a DNS resolver; the web server; a dozen CDN caches.
- **Links:** laptop→AP over 5 GHz Wi-Fi (multi-access, half duplex); AP→router
  over Cat5e copper (point-to-point, full duplex); router→ISP over a fibre PON
  (shared, and Chapter 49 explains how that shares); then a sequence of long-haul
  optical links.
- **Intermediate systems:** the access point; a switch; your router doing NAT; a
  dozen ISP routers; possibly a firewall and a load balancer at the far end.
- **Protocols:** 802.11 for the air; Ethernet for the copper; ARP to find the
  router's hardware address; DHCP, earlier, to get your address at all; DNS to
  turn the name into an address; IP to route; TCP to make the byte stream
  reliable; TLS to make it private; HTTP to ask for the page. Nine protocols
  before a single pixel appears, and we have not counted the ones the ISP is
  running on your behalf.

Every one of those nine is a chapter in this book. What you should take from the
list is not intimidation but structure: each protocol exists because the ones
below it left a specific problem unsolved, and each is replaceable without
disturbing its neighbours. That property — and the discipline that produces it —
is what Unit V is about.
