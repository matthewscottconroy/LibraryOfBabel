# 25.4 Classful History and Its Ghosts

Classful addressing was abolished in 1993. It is still in the exam objectives, still in
equipment defaults, and still the reason certain configurations behave in ways that make
no sense otherwise.

This section covers what it was, why it failed, and where its ghosts remain — which is
the only reason it is worth your time.

## The original design

RFC 791 (1981) divided the address space into classes by the leading bits, so that
**the class determined the mask**. There was no separate mask field; you read the first
few bits and knew the split.

| Class | Leading bits | First octet | Network / host bits | Networks | Hosts each |
|---|---|---|---|---|---|
| **A** | `0` | 1–126 | 8 / 24 | 126 | **16,777,214** |
| **B** | `10` | 128–191 | 16 / 16 | 16,384 | **65,534** |
| **C** | `110` | 192–223 | 24 / 8 | 2,097,152 | **254** |
| **D** | `1110` | 224–239 | multicast | — | — |
| **E** | `1111` | 240–255 | reserved | — | — |

`127` is absent from class A because `127.0.0.0/8` is loopback.

**Why classes at all?** Because in 1981 there was no room in the header for a mask, and
the designers needed *some* way to divide network from host. Encoding the split in the
leading bits was free — no field, no signalling, no configuration.

It was a reasonable engineering decision for a network of a few hundred hosts, made by
people who expected the address space to be plentiful indefinitely.

## Why it failed

Three problems, and the third was fatal.

### 1. The granularity was hopeless

Three sizes. That is all there was.

**An organisation with 300 hosts:**

| Option | Result |
|---|---|
| One class C (254) | **too small** |
| Two class Cs | two networks, needing a router between them, and no aggregation |
| One class B (65,534) | **65,234 addresses wasted** |

Everyone took the class B. It was easier, it was free, and there was no reason not to.

**An organisation with 2,000 hosts** took a class B and wasted 63,000 addresses. An
organisation with 300,000 took a class A and wasted sixteen million.

**By 1992, class B exhaustion was projected for 1994.** Not address exhaustion — there
were plenty of class Cs left — but exhaustion of the only size anyone could use.

### 2. Routing tables grew unmanageably

Without classes, an organisation with sixteen contiguous class C networks
(`200.1.0.0` through `200.1.15.0`) would appear in the global routing table as **one
entry**.

**With classes, they appear as sixteen**, because a class C is a class C and there was
no way to express "these sixteen together".

The global routing table was growing faster than router memory, and the projections
showed it exceeding what was buildable within a few years. **This was the more urgent
problem**, because address exhaustion is inconvenient and routing table collapse is
fatal.

### 3. It could not be fixed incrementally

The class was implied by the address. There was no field to change, no option to
negotiate, and no way for two routers to disagree gracefully about whether a given
address was classful.

## Subnetting — the first repair

**RFC 950 (1985)** introduced the subnet mask, and it is worth being precise about what
it did.

It allowed an organisation to divide its **own** allocation internally. A class B holder
could use `255.255.255.0` internally and get 256 subnets of 254 hosts each — separate
broadcast domains, separate segments, sensible sizes.

**But the outside world still saw one class B.** The mask was local; the class was
global. This helped the organisation enormously and did nothing at all for the two
global problems.

**It also introduced the concept that would replace classes**, which is the historical
point: once the mask existed as an explicit object, the class was redundant.

## CIDR — the real fix

**RFC 1519, September 1993.** *Classless Inter-Domain Routing*.

**One idea:** abolish classes and carry the prefix length explicitly.

```
   Classful:   203.0.113.0  is class C, therefore /24.  No choice.
   Classless:  203.0.113.0/26  or  /24  or  /22  —  whatever you say it is.
```

**What this solved:**

**Right-sized allocations.** An organisation with 300 hosts gets a `/23` — 510
addresses. Not 254, not 65,534. **Waste falls from 99% to 40%.**

**Aggregation.** Sixteen contiguous class Cs become one `/20`:

```
   200.1.0.0/24                     ┐
   200.1.1.0/24                     │
   200.1.2.0/24                     ├──  200.1.0.0/20
   …                                │      one entry
   200.1.15.0/24                    ┘
```

**Sixteen routing entries become one.** Chapter 26 §26.3 develops this, and it is the
mechanism that keeps the global table at under a million entries instead of tens of
millions.

**Provider-based allocation.** A provider gets a large block and allocates from it to
customers. All customers aggregate into the provider's one announcement. **The routing
table grows with the number of providers, not the number of organisations** — which is a
change in the growth rate, not just the size.

CIDR was deployed over about two years. **It worked**, and the routing table growth
curve bent visibly in 1994 — one of the few times in this book where a protocol change
produced an immediately measurable global effect.

## The ghosts

Classes are gone. Their traces are everywhere, and each one is a real source of
confusion.

### Default masks in equipment

Configure an address without a mask and many devices apply the classful default:

```
Router(config-if)# ip address 10.1.1.1
! Some platforms assume 255.0.0.0 — a /8
```

**Always specify the mask explicitly.** This ghost causes real outages, because a /8
where a /24 was intended makes a router believe 16 million addresses are directly
attached.

### Classful routing protocols

**RIPv1** and **IGRP** carried no mask in their updates — they assumed the classful
default. This made VLSM (Chapter 26 §26.4) impossible and discontiguous networks broken.

Both are obsolete. **RIPv2, EIGRP, OSPF and IS-IS all carry the mask**, and "does this
protocol carry the mask?" is *the* question that separates a classful protocol from a
classless one.

### Auto-summarisation

Some protocols historically summarised to the classful boundary **automatically** at
major network borders, which broke **discontiguous networks** — two parts of `10.0.0.0/8`
separated by a different network would each advertise `10.0.0.0/8` and the routing would
be ambiguous.

`no auto-summary` was the standard first line of every EIGRP and RIP configuration for
fifteen years. It is now the default, and knowing why the command exists explains a
great deal of older configuration.

### The vocabulary

People still say "a class C network" meaning a /24, and "class B" meaning something
large. **It is imprecise and it is universal.** Understand it, use the precise form
yourself, and do not correct people mid-incident.

### The private ranges

**The clearest surviving ghost.** RFC 1918's private blocks (Chapter 27 §27.1) are
defined on class boundaries:

| Range | Prefix | Was |
|---|---|---|
| `10.0.0.0` – `10.255.255.255` | **/8** | one class A |
| `172.16.0.0` – `172.31.255.255` | **/12** | 16 class Bs |
| `192.168.0.0` – `192.168.255.255` | **/16** | 256 class Cs |

**The awkward `172.16.0.0/12` exists because it was defined as sixteen class Bs.** The
`/12` and the odd 16–31 range in the second octet are a direct fossil of classful
thinking, and it is the most-forgotten private range for exactly that reason.

### Exam objectives

Network+ still examines classes. It is history and it is on the test, and the ranges
must be known.

## What breaks here

**Configuring an address without a mask.** The device may assume a classful default and
believe something enormous is directly attached.

**Using a classful routing protocol with VLSM.** It cannot work; the mask is not
carried.

**Auto-summarisation with discontiguous networks.** Two parts of `10.0.0.0/8` in
different places, both advertising the /8. Routing becomes ambiguous.

**Assuming `172.x` is private.** Only `172.16.0.0` – `172.31.255.255`. `172.15.x.x` and
`172.32.x.x` are public and belong to somebody.

**Believing classes still determine anything.** They determine nothing. They explain
defaults.

> **Network+ note.** Objective 1.7 expects the classful ranges. Over-learn: **A =
> 1–126, B = 128–191, C = 192–223, D = 224–239 multicast, E = 240–255 reserved**; **127
> is loopback**; and **the three RFC 1918 ranges including the awkward 172.16–172.31**.
> Also expect "which of these is a valid private address?" with `172.32.5.1` as a
> distractor.
