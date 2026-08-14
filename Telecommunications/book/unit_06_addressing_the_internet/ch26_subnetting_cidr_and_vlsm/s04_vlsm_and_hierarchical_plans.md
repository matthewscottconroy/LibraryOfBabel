# 26.4 VLSM and Hierarchical Plans

**Variable Length Subnet Masking** means using different prefix lengths within one
address block, so that each subnet is sized for what it holds.

It sounds obvious. It was impossible until 1987, it is the difference between an
address plan that works and one that runs out, and doing it well requires a specific
discipline that this section sets out.

## The problem it solves

**A network:**

| Segment | Hosts needed |
|---|---|
| Head office | 100 |
| Branch A | 50 |
| Branch B | 25 |
| Branch C | 10 |
| WAN link 1 | 2 |
| WAN link 2 | 2 |
| WAN link 3 | 2 |

**Total: 191 hosts.** You have `192.168.1.0/24` — 254 addresses. It should fit
comfortably.

**Without VLSM** every subnet must use the same mask, sized for the largest:

- Head office needs 100 → **/25** (126 hosts)
- Every subnet must therefore be a /25
- A /24 contains **two** /25s

**Two subnets. You need seven.** It does not fit, and it is not close.

**With VLSM**, each subnet gets what it needs:

| Segment | Need | Prefix | Capacity |
|---|---|---|---|
| Head office | 100 | /25 | 126 |
| Branch A | 50 | /26 | 62 |
| Branch B | 25 | /27 | 30 |
| Branch C | 10 | /28 | 14 |
| WAN 1 | 2 | /30 | 2 |
| WAN 2 | 2 | /30 | 2 |
| WAN 3 | 2 | /30 | 2 |

**It fits, with room left over.** That is VLSM.

## What it requires

**The routing protocol must carry the mask** in its updates.

This is the entire technical requirement, and it is why VLSM was impossible before
RFC 1009 (1987) and impractical until classless routing protocols were widely deployed
in the early 1990s. A protocol like RIPv1 that advertises `192.168.1.64` with no mask
leaves the receiver to guess — and it guesses the classful default, which is wrong.

| Protocol | Carries the mask? | VLSM |
|---|---|---|
| RIPv1 | **no** | ✗ |
| IGRP | **no** | ✗ |
| RIPv2 | yes | ✓ |
| EIGRP | yes | ✓ |
| OSPF | yes | ✓ |
| IS-IS | yes | ✓ |
| BGP | yes | ✓ |

**Everything current supports it.** The question exists only as history and as an exam
item.

## The method

Five steps. Follow them in order; the order is what prevents fragmentation.

### Step 1 — List every requirement, largest first

**Sorting descending is the critical step**, and doing it out of order is the most
common VLSM mistake.

| Segment | Hosts | +2 | Round up | Prefix |
|---|---|---|---|---|
| Head office | 100 | 102 | 128 | **/25** |
| Branch A | 50 | 52 | 64 | **/26** |
| Branch B | 25 | 27 | 32 | **/27** |
| Branch C | 10 | 12 | 16 | **/28** |
| WAN 1 | 2 | 4 | 4 | **/30** |
| WAN 2 | 2 | 4 | 4 | **/30** |
| WAN 3 | 2 | 4 | 4 | **/30** |

### Step 2 — Allocate the largest first, from the start of the block

**Head office, /25**, block size 128:

```
   192.168.1.0/25      →  .0   – .127     (126 usable)
```

Remaining: `192.168.1.128` – `192.168.1.255`.

### Step 3 — Continue in descending size

**Branch A, /26**, block size 64. The next boundary at or after 128 that is a multiple of
64 is **128**:

```
   192.168.1.128/26    →  .128 – .191     (62 usable)
```

**Branch B, /27**, block size 32. Next multiple of 32 at or after 192 is **192**:

```
   192.168.1.192/27    →  .192 – .223     (30 usable)
```

**Branch C, /28**, block size 16. Next multiple of 16 at or after 224 is **224**:

```
   192.168.1.224/28    →  .224 – .239     (14 usable)
```

**WAN links, /30**, block size 4, starting from 240:

```
   192.168.1.240/30    →  .240 – .243     (2 usable)
   192.168.1.244/30    →  .244 – .247     (2 usable)
   192.168.1.248/30    →  .248 – .251     (2 usable)
```

> **A note on /30 versus /31.** This example uses /30 because it is what you will meet
> in existing networks and on the exam. **A /31 is the better answer** (Chapter 25 §25.2,
> RFC 3021): two devices need no broadcast address, so a /31 gives the same two usable
> addresses in half the space. This book's [tools/netcalc.py](../../../tools/netcalc.py)
> allocates /31 for two-host requirements for exactly that reason, so its output for this
> problem differs from the table above in the last three rows. Both are correct; the /31
> is better engineering and the /30 is what is deployed.

### Step 4 — Record what is left

```
   192.168.1.252/30    →  .252 – .255     UNUSED
```

**Four addresses spare out of 256.** Write it down; unrecorded free space becomes
unusable space, because nobody dares assign from a range they cannot prove is free.

### Step 5 — Verify

| Check | |
|---|---|
| No overlaps | Each network starts where the previous ended + 1 ✓ |
| Every network on a boundary | 0/128, 128/64, 192/32, 224/16, 240/4, 244/4, 248/4 ✓ |
| Every requirement satisfied | ✓ |
| Total accounted for | 128+64+32+16+4+4+4+4 = **256** ✓ |

That last check — **the sizes must sum to the parent block** — catches almost every
arithmetic error.

## Why largest-first matters

The rule exists because of alignment, and the failure is worth seeing.

**Allocate a /30 first:**

```
   192.168.1.0/30      →  .0 – .3
```

**Now place the /25.** A /25 must start at a multiple of 128 — either **0 or 128**. Zero
is taken. So the /25 goes at 128.

```
   192.168.1.0/30      →  .0 – .3        WAN 1
   192.168.1.128/25    →  .128 – .255    Head office
```

**And `.4` through `.127` — 124 addresses — is now a hole.** It can hold a /26 at 64
and smaller pieces below, but the block is fragmented and the fragments may not fit what
remains.

> **Large blocks have few valid starting positions. Small blocks fit almost anywhere.
> Place the constrained things first.**

This is the same principle as memory allocation, disk partitioning, and packing a
suitcase, and it generalises: **allocate in decreasing order of alignment constraint.**

## Designing for aggregation

VLSM sized correctly is only half the job. A plan should also **aggregate** (§26.3), and
that requires thinking about the hierarchy before assigning anything.

**A poor plan** — correct, efficient, and unsummarisable:

```
   10.1.0.0/24     Site A, floor 1
   10.2.0.0/24     Site B, floor 1
   10.3.0.0/24     Site A, floor 2        ← Site A is now discontiguous
   10.4.0.0/24     Site C
   10.5.0.0/24     Site A, floor 3
```

Site A's networks are `10.1`, `10.3` and `10.5`. **Nothing summarises**, and Site A's
router must advertise three prefixes forever — and four when floor 4 is built.

**A good plan** — allocate a contiguous block per site, then subdivide within it:

```
   10.0.0.0/8                          the organisation
     ├── 10.1.0.0/16    Site A         ← advertises ONE prefix
     │     ├── 10.1.1.0/24   floor 1
     │     ├── 10.1.2.0/24   floor 2
     │     ├── 10.1.3.0/24   floor 3
     │     └── 10.1.x.0/24   room for 250 more
     ├── 10.2.0.0/16    Site B
     └── 10.3.0.0/16    Site C
```

**Site A advertises `10.1.0.0/16` and nothing else, ever.** Floors can be added,
removed, resized and renumbered internally, and no router outside Site A ever hears
about it.

**The discipline:** *allocate blocks by location or function, from the top down, and
subdivide within them.* It costs some address space — Site A holds 65,536 addresses for
perhaps 800 hosts — and with RFC 1918 space, **address efficiency is nearly free and
aggregation is not.** Spend the addresses.

## A full worked plan

**The organisation:** three sites, `10.0.0.0/8` available.

| Site | Users | Servers | Voice | Guests | Management |
|---|---|---|---|---|---|
| HQ | 400 | 60 | 400 | 100 | 40 |
| Branch 1 | 80 | 10 | 80 | 30 | 10 |
| Branch 2 | 45 | 5 | 45 | 20 | 10 |

**Top level — one /16 per site**, leaving room for many more sites:

| Site | Block |
|---|---|
| HQ | `10.1.0.0/16` |
| Branch 1 | `10.2.0.0/16` |
| Branch 2 | `10.3.0.0/16` |
| WAN links | `10.255.0.0/16` |
| Reserved | `10.4.0.0` – `10.254.0.0` |

**Within HQ (`10.1.0.0/16`)** — by function, so that policy is expressible by prefix:

| Purpose | Need | Prefix | Network | Capacity |
|---|---|---|---|---|
| Users | 400 | /22 | `10.1.0.0/22` | 1,022 |
| Voice | 400 | /22 | `10.1.4.0/22` | 1,022 |
| Servers | 60 | /24 | `10.1.8.0/24` | 254 |
| Guests | 100 | /24 | `10.1.9.0/24` | 254 |
| Management | 40 | /26 | `10.1.10.0/26` | 62 |
| Reserved | — | — | `10.1.11.0` – `10.1.255.255` | — |

**Within Branch 1 (`10.2.0.0/16`)** — same structure, smaller:

| Purpose | Prefix | Network |
|---|---|---|
| Users | /24 | `10.2.0.0/24` |
| Voice | /24 | `10.2.1.0/24` |
| Servers | /26 | `10.2.2.0/26` |
| Guests | /26 | `10.2.2.64/26` |
| Management | /27 | `10.2.2.128/27` |

**WAN links (`10.255.0.0/16`)**, one /30 each:

| Link | Prefix |
|---|---|
| HQ – Branch 1 | `10.255.0.0/30` |
| HQ – Branch 2 | `10.255.0.4/30` |
| Branch 1 – Branch 2 | `10.255.0.8/30` |

**What this achieves:**

- **Each site advertises one /16.** The WAN routing table has four entries.
- **The structure is readable.** `10.2.1.50` is Branch 1, voice. You can tell at a
  glance, from an address in a log, without consulting documentation.
- **Policy is expressible by prefix.** "Guests may not reach servers" is one rule per
  site, or — if the second octet were function rather than site — one rule total.
- **Growth is provided for.** 250 more sites, and large reserved ranges within each.

**The cost:** HQ consumes 65,536 addresses for about 1,000 hosts. **Irrelevant.** RFC
1918 gives 16.7 million in `10.0.0.0/8`, and there is no prize for using them
efficiently. **Optimise for comprehensibility and aggregation; the addresses are free.**

## When to break the rules

Two cases where the neat plan is wrong.

**Public address space.** Public IPv4 is expensive and scarce (Chapter 27 §27.1), so
efficiency matters again. Public allocations are sized tightly and VLSM'd carefully.

**Very large scale.** A cloud provider or a large ISP will exhaust even 10/8 and must
plan much more carefully — which is one of several reasons such organisations moved to
IPv6 internally, where the address space genuinely is unlimited.

## Documenting it

**A plan that exists only in the running configuration is not a plan.** Chapter 27 §27.4
covers IPAM properly. The minimum, for each block:

| Field | Why |
|---|---|
| Prefix | the allocation |
| Purpose | so nobody reuses it |
| VLAN ID | the correspondence of Chapter 20 §20.4 |
| Gateway | the convention, stated |
| DHCP range | which part is dynamic |
| Static range | which part is not |
| Reserved | explicitly, so it is not "free" |
| Site / owner | who to ask |

**The reserved rows matter most.** Undocumented free space is space nobody will use,
because assigning from a range you cannot prove is free is how you cause an outage.

## What breaks here

**Allocating smallest first.** Fragments the block so the large subnets do not fit.

**Forgetting alignment.** A /26 cannot start at `.100`. Multiples of 64 only.

**Overlapping subnets.** The failure is intermittent and depends on which router learned
what first, which makes it very hard to diagnose.

**A plan that does not aggregate.** Correct, efficient, and it will churn the routing
table forever.

**Sizing with no headroom.** 30 hosts in a /27 fits exactly and renumbering is
expensive.

**Undocumented allocations.** The next engineer assigns the same range.

> **Network+ note.** Objective 1.7 expects VLSM. Over-learn: **VLSM means different mask
> lengths within one block**; **it requires a classless routing protocol that carries the
> mask**; **allocate largest first**; and **each subnet must start on a boundary that is
> a multiple of its own block size**. Expect a design question giving several host
> requirements and one block.
