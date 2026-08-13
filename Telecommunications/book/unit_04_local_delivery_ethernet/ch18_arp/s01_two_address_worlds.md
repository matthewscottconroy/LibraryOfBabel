# 18.1 Two Address Worlds

By the end of Unit VI there will be two address systems in use simultaneously, and
they will know nothing about each other. This section establishes why both are
necessary, why neither can be eliminated, and what the gap between them costs.

## The two systems

| | MAC address | IP address |
|---|---|---|
| Size | 48 bits | 32 (IPv4) / 128 (IPv6) |
| Structure | **flat** | **hierarchical** |
| Assigned by | manufacturer, at the factory | administrator or DHCP |
| Changes when the device moves? | **no** | **yes** |
| Scope | one link | global |
| Used by | Ethernet, Wi-Fi | IP |
| Aggregatable? | **no** | **yes** |

The two rows in bold are the whole story.

## Why the MAC address cannot be eliminated

Because **the hardware reads it**.

An Ethernet frame's destination field is 48 bits wide and contains a MAC address.
That is what the standard specifies (Chapter 15 §15.3), and it is what a network
interface compares against its own address to decide whether to accept a frame. A
switch's forwarding table is keyed on MAC addresses (Chapter 17 §17.2).

You cannot put an IP address in that field and expect anything to happen. The
hardware does not know what an IP address is, and — importantly — **it must not
need to**. Chapter 23 §23.1's hourglass depends on the link layer being ignorant of
what it carries; an Ethernet controller that understood IP would have to be revised
for IPv6, for IPX, for anything new, and the whole point is that it is not.

So: **nothing moves on a local segment without a MAC address.**

## Why the IP address cannot be eliminated

Because **MAC addresses cannot be routed**.

Chapter 15 §15.2 established that MAC addressing is flat: no rule summarises a set of
addresses, because the set has no structure to summarise. Two numerically adjacent
addresses may be on opposite continents.

Therefore any device that must know where an address lives must know it
**individually**. A global network on MAC addresses would require every router to
hold an entry for every device on Earth — twenty billion entries and rising, with no
possibility of aggregation, updated continuously as devices appear and vanish.

That is not expensive; it is arithmetically impossible. There is no memory in which
to hold it and no mechanism by which to distribute it.

The IP address is hierarchical, so a router can hold a rule covering
`203.0.113.0/24` — or `203.0.0.0/8`, or in the limit `0.0.0.0/0` — without knowing
any individual address in it. Chapter 26 §26.3's aggregation is what reduces the
global routing table from twenty billion entries to under a million.

So: **nothing crosses a network boundary without an IP address.**

## The gap

Both are necessary. Neither can do the other's job. And **they are assigned by
entirely different parties by entirely different mechanisms**, with no relationship
between them:

- A MAC address is burned in at manufacture, by a company that has never heard of
  your network.
- An IP address is assigned by your DHCP server or your configuration, according to
  a plan that has nothing to do with hardware.

There is no function from one to the other. Given `192.168.10.1`, nothing about the
number tells you what MAC address holds it.

**And a host that wants to send a packet needs both.**

## The concrete problem

Host A at `192.168.10.70/24` wants to send to `192.168.10.1`. It has:

- The destination **IP address** — that is what the application asked for, and it is
  what goes in the IP header.
- Its own MAC address.
- **No idea what MAC address to put in the frame's destination field.**

And it cannot proceed without one. The frame requires a 48-bit destination; the host
knows *who* it wants to reach and not *how to address the envelope*.

This is the problem ARP exists to solve, and it is worth noticing how narrow it is.
It is not a routing problem, not a discovery problem, not a naming problem. It is one
question: **given an IPv4 address on my own link, what MAC address holds it?**

## What the host does first

Before ARP can help, the host must answer a prior question, and getting this wrong is
the source of the confusion §18.2 addresses.

**The local-or-remote decision** (Chapter 25 §25.3), performed for every packet:

```
   my address    AND my mask   →  my network
   destination   AND my mask   →  destination's network
   equal?
```

- **Equal → the destination is on my link.** ARP for **the destination itself**, and
  send the frame directly to it.
- **Not equal → the destination is elsewhere.** ARP for **the default gateway**, and
  send the frame to the router — **with the IP destination still set to the final
  target**.

That last clause is the one students most often miss, and it is worth stating
separately because it reveals the architecture:

> **The frame's destination MAC changes at every hop. The packet's destination IP
> does not change at all.**

A packet crossing five routers is carried by five different frames, each addressed to
the next hop's MAC address, each stripped and rebuilt. The IP header rides through
unchanged (except the TTL and checksum).

**The MAC address is a hop-by-hop identifier; the IP address is an end-to-end
identifier.** They operate at different scopes because they answer different
questions, and once that is clear the two-address-system arrangement stops looking
redundant.

## Traced

Host A (`192.168.10.70`) sends to a server at `198.51.100.25`, via router R
(`192.168.10.1`).

```
   A ──────── switch ──────── R ──────── … ──────── Server
   192.168.10.70          192.168.10.1          198.51.100.25
   aa:aa:aa:aa:aa:aa      rr:rr:rr:rr:rr:rr      ss:ss:ss:ss:ss:ss
```

**On A's link:**

| Field | Value |
|---|---|
| Dest MAC | `rr:rr:rr:rr:rr:rr` ← **the router**, found by ARP |
| Src MAC | `aa:aa:aa:aa:aa:aa` |
| Dest IP | `198.51.100.25` ← **the server**, unchanged |
| Src IP | `192.168.10.70` |

**On the server's link, after several hops:**

| Field | Value |
|---|---|
| Dest MAC | `ss:ss:ss:ss:ss:ss` ← the server, found by ARP **on that link** |
| Src MAC | the last router's |
| Dest IP | `198.51.100.25` ← **still unchanged** |
| Src IP | `192.168.10.70` ← **still unchanged** |

Every router along the way performed its own ARP, on its own link, for its own next
hop. The IP addresses are constant end to end; the MAC addresses are replaced at
every hop.

## Where ARP sits

Awkwardly, and the awkwardness is instructive.

ARP is carried **directly in Ethernet frames**, with EtherType `0x0806` — **not
inside IP**. It has to be: ARP must work *before* IP communication is possible, so it
cannot depend on IP.

Which means it fits the OSI model badly. It serves Layer 3 by resolving Layer 3
addresses; it is carried by Layer 2 and knows about Layer 2 addressing; it belongs to
neither. The conventional designation is **"Layer 2.5"**, offered with some
embarrassment.

Chapter 22 §22.3 makes the general point: layering is a useful model, not a law of
nature, and ARP is the clearest early demonstration. A student who cannot place ARP
in the OSI model has understood correctly.

## The IPv6 alternative

IPv6 does not use ARP. It uses **Neighbor Discovery Protocol**, carried inside
**ICMPv6** — which is inside IP.

That looks like a contradiction of the argument above, and it is not. NDP uses
**link-local addresses** (`fe80::/10`), which every IPv6 interface configures
automatically without any external input (Chapter 28 §28.2). So IP communication *is*
available before any global addressing exists, and NDP can ride on it.

The IPv6 designers, having watched ARP for sixteen years, restructured the whole
arrangement — and §18.4 covers what they changed and why.

## What breaks here

**Confusing the two decisions.** "Which MAC address?" is ARP's question. "Is this
local or remote?" is answered first, by the mask, and determines *what* to ARP for.

**Assuming the destination MAC identifies the destination host.** On any packet that
has crossed a router, it identifies the last router. This surprises people reading
their first capture of remote traffic.

**Expecting a relationship between the two addresses.** There is none, in either
direction.

**Trying to place ARP in the OSI model.** It does not fit. This is a property of the
model, not a gap in your understanding.

> **Network+ note.** Objective 1.4 expects ARP's function. The framing worth
> carrying: **MAC is hop-by-hop, IP is end-to-end**, and the local-or-remote decision
> determines which address a host resolves. Chapter 25 §25.3's arithmetic and this
> section's consequence are examined together constantly.
