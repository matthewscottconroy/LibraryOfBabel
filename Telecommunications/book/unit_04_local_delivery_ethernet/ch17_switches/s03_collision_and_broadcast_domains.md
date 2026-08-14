# 17.3 Collision and Broadcast Domains

The most examined pair of concepts in this chapter, the most confused, and — once
the definitions are precise — the easiest to count.

## The definitions

> A **collision domain** is the set of interfaces whose transmissions can collide
> with one another. It is the region within which CSMA/CD arbitration applies.

> A **broadcast domain** is the set of interfaces that will receive a broadcast
> frame sent by any of them.

They are different sets, bounded by different devices, and the sentence that follows
is worth memorising verbatim:

> **A switch breaks up collision domains. It does not break up broadcast domains.
> A router breaks up broadcast domains.**

## Counting collision domains

The rule: **one per switch port**, plus one per hub (covering everything attached to
it), plus one per legacy shared segment.

| Device | Collision domains created |
|---|---|
| Repeater | 0 — joins two segments into one |
| Hub, *n* ports | 1 total |
| Switch, *n* ports | **1 per port** = *n* |
| Router, *n* interfaces | 1 per interface |

**Worked example.**

```
        ┌──────────────┐
        │  8-port      │
        │  SWITCH      │
        └┬──┬──┬──┬──┬─┘
         1  2  3  4  5
         │  │  │  │  └── HUB ──┬── PC-F
         │  │  │  │            ├── PC-G
         │  │  │  │            └── PC-H
       PC-A │ PC-C PC-D
          PC-B
```

Five ports in use. Ports 1–4 have one station each; port 5 has a hub with three
stations.

**Collision domains:** ports 1, 2, 3 and 4 give **four**; the hub and everything on
it — including switch port 5 — is **one** more. **Total: five.**

The three stations on the hub share their domain and contend under CSMA/CD; the four
directly attached stations each have a domain to themselves and run full duplex with
no contention at all.

**On a fully switched network with no hubs, the collision domain count equals the
number of active ports, and every domain contains exactly two interfaces — the
station and the switch port — so no collision is possible.** This is why CSMA/CD is
vestigial (Chapter 16 §16.4) and why a collision counter above zero is a fault
indication.

## Counting broadcast domains

The rule: **one per router interface, or one per VLAN.** Switches and hubs create
none.

| Device | Broadcast domains created |
|---|---|
| Hub | 0 |
| Switch, no VLANs | **0** — the whole switched network is one |
| Switch, *n* VLANs | *n* |
| Router, *n* interfaces | *n* |

**Worked example**, extending the topology above with a second switch and a router:

```
        Router
       ┌──┴───┐
    Fa0/0   Fa0/1
       │       │
   [SWITCH-1] [SWITCH-2]
    5 ports    6 ports
    in use     in use
```

**Broadcast domains: two.** One on each router interface. Every station on SWITCH-1
is in the first; every station on SWITCH-2 is in the second. Adding more switches to
either side changes nothing — cascade ten switches off Fa0/0 and it is still one
broadcast domain.

**Collision domains:** 5 + 6 = 11 switch ports, plus the two router interfaces =
**13**.

That asymmetry — many collision domains, few broadcast domains — is the normal shape
of a modern network and is exactly what the bolded sentence predicts.

## Why broadcast domain size matters

A broadcast is processed by **every host in the domain**, and processing means an
interrupt and CPU cycles on every machine, whether or not the broadcast is relevant
to it.

What generates broadcasts:

- **ARP requests** (Chapter 18) — the largest single source in most networks.
- **DHCP DISCOVER and REQUEST** (Chapter 40).
- **NetBIOS name resolution**, mDNS, SSDP, LLMNR — service discovery protocols,
  which are chatty by design.
- **Routing protocol hellos**, on some protocols.
- **Broadcast storms** when something goes wrong (Chapter 19 §19.1).

**The scaling problem** is that ARP traffic grows roughly with the *square* of the
host count in the worst case: *n* hosts, each occasionally needing to resolve any of
the other *n*−1, and every request seen by all *n*.

Two costs follow:

**Per-host CPU.** In a 1,000-host broadcast domain, every host processes every ARP
request for every other host. Most are discarded immediately, and the interrupt is
still taken.

**Failure blast radius.** A broadcast storm affects the entire domain. One
misbehaving device — a failed NIC transmitting continuously, a loop, a rogue DHCP
server — degrades every host that shares the domain with it.

**The traditional guidance is a few hundred hosts per broadcast domain**, and it is
soft, dated, and directionally correct. Modern hardware handles far more than 1990s
hardware, and the blast-radius argument has not changed at all — which is why
segmentation is now driven more by security (Chapter 60 §60.4) than by broadcast
volume.

## Reducing broadcast domain size

Two mechanisms, and Chapter 20 covers the second properly.

**A router.** Each interface is a separate broadcast domain by construction, because
a router does not forward broadcasts. Effective, and it costs a router interface and
a subnet per domain.

**A VLAN.** One physical switch behaves as several logical switches, each with its
own broadcast domain, without additional hardware (Chapter 20 §20.1). Traffic between
VLANs must then be routed — which is exactly the point, and which is the fact
students most often fail to internalise.

## Full duplex, and the disappearance of one of them

Worth stating explicitly because it makes half the section's material historical.

On a modern switched network with full duplex everywhere:

- There is **one collision domain per port**, each containing exactly two interfaces.
- **No collision is possible** in any of them.
- **CSMA/CD does not run** and was never specified above 1 Gb/s.
- The count is arithmetically correct and operationally uninteresting.

**Broadcast domains remain entirely relevant**, because broadcasts are a protocol
requirement rather than a medium artefact. ARP still broadcasts, DHCP still
broadcasts, and the domain's size still determines the blast radius.

So the honest modern summary: **collision domains are an exam topic and a historical
artefact; broadcast domains are a live design constraint.** Count both for the exam;
worry about the second in practice.

## Exam technique

These questions have a reliable shape, and the method is mechanical.

**"How many collision domains?"** Count active switch ports. Add one per hub (not per
hub port). Add one per legacy shared segment. Add router interfaces.

**"How many broadcast domains?"** Count router interfaces. Or count VLANs. Ignore
switches and hubs entirely.

**The classic distractor** presents a network with several switches and asks for
broadcast domains, expecting you to count switches. The answer is one, unless VLANs
or a router are present.

**The second distractor** presents a hub with eight stations and asks for collision
domains, expecting eight. The answer is one.

## What breaks here

**A flat network that has grown too large.** Broadcast traffic consuming host CPU,
and a blast radius covering everything. The remedy is segmentation, and the argument
for it is now usually security rather than broadcast volume.

**Collisions on a full-duplex port.** Should be zero. Any count means a duplex
mismatch (Chapter 16 §16.4).

**A hub discovered in production.** One collision domain covering everything on it,
shared bandwidth, half duplex, and no privacy. Replace it.

**VLANs created with no routing.** Broadcast domains successfully separated, and now
nothing can reach anything (Chapter 20 §20.4).

**A broadcast storm.** Every host in the domain saturated. The domain's boundary is
exactly the extent of the damage, which is the practical argument for keeping it
small.

> **Network+ note.** Objective 1.6 and objective 1.2 both touch this, and it is one
> of the most reliably examined topics in the certification. **Memorise the bolded
> sentence, practise the counting on a few diagrams, and know that the distractors
> are "count the switches" and "count the hub's ports".**
