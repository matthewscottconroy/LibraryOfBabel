# 15.2 The MAC Address

Forty-eight bits, assigned at the factory, globally unique, and telling you nothing
whatever about where the device is. That last property is the important one, and
this section is mostly about its consequences.

## Structure

```
   00:1b:44:11:3a:b7
   └───┬───┘ └───┬──┘
     OUI      device
   (24 bits)  (24 bits)
```

The first three bytes are the **Organisationally Unique Identifier**, purchased from
the IEEE Registration Authority. The remaining three are assigned by the
manufacturer, which is responsible for not repeating them.

- **2²⁴ = 16,777,216** OUIs available
- **2²⁴ = 16,777,216** devices per OUI
- **2⁴⁸ ≈ 281 trillion** addresses in total

Manufacturers buy OUIs — a large one holds dozens — and the registry is public and
searchable. Looking up an OUI from a capture tells you what kind of device sent a
frame, which is a genuinely useful diagnostic when an unexpected device appears on
a network.

## Two bits in the first byte

The first byte carries two flags in its low-order bits, and both matter.

```
   byte 0:  x x x x x x I G
                        │ │
                        │ └── I/G: 0 = unicast, 1 = multicast
                        └──── U/L: 0 = universally administered
                                    1 = locally administered
```

**The I/G bit** (bit 0, the least significant) distinguishes **individual** from
**group** addresses. If it is 1, the frame is multicast and every station examines
it. The broadcast address `ff:ff:ff:ff:ff:ff` is the degenerate case in which every
bit is set.

Practical consequence: any address whose **first byte is odd** is a multicast
address. `01:00:5e:...` is IPv4 multicast, `01:80:c2:00:00:00` is the spanning tree
BPDU address (Chapter 19), `33:33:...` is IPv6 multicast. Recognising an odd first
byte in a capture immediately tells you the frame is not addressed to one station.

**The U/L bit** (bit 1) distinguishes universally administered addresses — from the
OUI registry — from **locally administered** ones, which an administrator or an
operating system has assigned. Set it and you are guaranteeing only that you have
not collided with the registry; you are responsible for uniqueness yourself.

Locally administered addresses appear in virtual machines, in load balancers using
virtual MACs, in FHRP virtual routers (Chapter 56 §56.2), and — increasingly — in
privacy randomisation.

## Flat addressing, and the consequence

Here is the property that shapes Units VI and VII.

**A MAC address is a name, not a coordinate.** It identifies a device and says
nothing about where the device is. Two numerically adjacent addresses may be on
opposite sides of the planet, in different organisations, on different continents.

Compare a postal address, which is hierarchical: country, city, street, number. A
sorting office in Melbourne handling a letter to Lyon consults no list of French
addresses; it applies one rule — *France goes in that bag* — and forgets the rest.
Hierarchy permits **aggregation**, and aggregation is what makes large systems
scale.

Flat addressing permits none. There is no rule that summarises a set of MAC
addresses, because the set has no structure to summarise. Therefore:

> **Any device that must know where a MAC address lives must know it
> individually, by observation, in a table.**

Everything follows from that sentence:

- **Switches learn by observation** (Chapter 17 §17.2), because there is nothing to
  compute.
- **The table is finite** — 8,000 entries on a small access switch, 128,000 on a
  data-centre switch — because memory is finite and every entry is independent.
- **The table can be exhausted deliberately**, which is Chapter 62 §62.1's MAC
  flooding attack.
- **Broadcast domains have a size limit**, because every station must process every
  broadcast and the switches must hold every address.
- **VLANs exist** (Chapter 20) partly to bound broadcast domains.
- **And IP had to be invented** with hierarchy built in (Chapter 24), because a flat
  space of 2⁴⁸ addresses cannot be routed globally at any price.

## So why choose flat addressing at all?

Because it requires **no administration**.

Plug in a device and it works. It has a unique address, nobody assigned it, no
server was consulted, no configuration was entered, and no two devices anywhere will
collide. For a network in one building in 1980, that was exactly the right trade.

And it is *still* the right trade at local scale, which is why we have both address
systems rather than having replaced one with the other — and why Chapter 18's ARP
exists to reconcile them.

The general principle, which recurs: **flat addressing buys zero-configuration and
costs scalability; hierarchical addressing buys scalability and costs
administration.** Neither is better; they solve different problems, and large systems
use both at different layers.

## MAC address randomisation

A recent development that broke a great deal of established practice, and it is worth
understanding both sides.

**The problem.** A device's MAC address is a permanent, unique identifier that it
broadcasts constantly — in probe requests when scanning for Wi-Fi networks, and in
every frame it sends. Anyone with a receiver can log it. Retail analytics companies
deployed sensors in shopping centres to track individual devices between shops and
across visits; the same technique works in airports, stations and city streets, and
it worked without consent or awareness.

**The response.** Since roughly 2014 for probe requests, and since iOS 14 and
Android 10 for association, mobile devices use **randomised, locally administered
MAC addresses**, frequently a different one per network and rotating over time.

**What it broke:**

- **MAC-based access control lists**, which were never a security control anyway
  (Chapter 59 §59.2's MAB discussion) and are now not even an inventory control.
- **DHCP reservations** keyed on MAC address.
- **Device tracking for legitimate purposes** — capacity planning, locating a device
  reported as lost, correlating a support ticket with a session.
- **Captive portal session persistence**, so users are asked to log in repeatedly.
- **Wi-Fi analytics** for genuine operational purposes.

**The correct response** is to stop using the MAC address as an identity. 802.1X
with certificates or credentials (Chapter 59 §59.2) identifies a *user* or a
*device* cryptographically, which is what you actually wanted, and it is unaffected
by randomisation.

Organisations that respond by instructing users to disable randomisation have
misunderstood: they are asking users to give up a privacy protection to preserve a
mechanism that was never providing security.

## What breaks here

**Assuming a MAC address identifies a device permanently.** Randomisation, virtual
machines, locally administered addresses and cloned addresses all break it.

**MAC filtering as a security control.** Addresses are trivially spoofed —
`ip link set dev eth0 address ...` is one command — and are transmitted in clear in
every frame. It is an inventory mechanism at best.

**A DHCP reservation that stops working** after a phone updates its operating
system. Randomisation.

**Duplicate MAC addresses**, which do occur — manufacturing errors, cloned virtual
machines, and deliberate spoofing. The symptom is a switch's MAC table flapping the
address between two ports, and traffic to that address arriving unpredictably at one
or the other.

**Exhausting a switch's MAC table**, whether by attack (Chapter 62 §62.1) or by a
genuinely large flat network. The switch floods everything, performance collapses,
and eavesdropping becomes possible.

> **Network+ note.** Objective 1.4 expects MAC addressing and the OUI. Two things
> to over-learn: **an odd first byte means multicast**, which is immediately useful
> in a capture; and **flat addressing does not scale**, which is the reason IP
> exists and the most useful idea in this section.
