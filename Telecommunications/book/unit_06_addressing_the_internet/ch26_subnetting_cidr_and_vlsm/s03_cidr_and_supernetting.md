# 26.3 CIDR and Supernetting

Subnetting divides a block into smaller ones. **Supernetting** — aggregation, summarisation,
route summarisation — does the reverse: it combines several blocks into one shorter
prefix.

Same arithmetic, opposite direction, and it is the mechanism that keeps the global
routing table from collapsing.

## The idea

```
   Subnetting:      one /24  →  four /26
                    more networks, fewer hosts each

   Supernetting:    four /24  →  one /22
                    fewer routes, more addresses each
```

**Aggregation is a routing operation.** Nobody assigns a /22 to a segment; a router
**advertises** a /22 in place of four /24s, and its neighbours hold one entry instead of
four.

## Why it matters

Chapter 25 §25.4 gave the history. The arithmetic:

**The global BGP routing table** currently holds roughly **950,000 IPv4 prefixes**. Every
Internet router with a full table holds all of them, in fast memory, and must look up a
destination in that table for every packet at line rate.

**Without aggregation, the number would be in the tens of millions.** Router memory —
specifically TCAM, which is expensive, power-hungry and physically limited — would not
hold it. Chapter 32 §32.4 covers what happens when the table exceeds a platform's
capacity, and the answer is that the router stops forwarding correctly.

> **Aggregation is not an optimisation. It is what makes a global routing table
> possible.**

## Summarising by hand

**The problem:** given several networks, find the shortest prefix that covers exactly
them.

**The method: find the longest common prefix in binary.**

### Example — four /24s

```
   192.168.4.0/24    11000000 10101000 00000100 00000000
   192.168.5.0/24    11000000 10101000 00000101 00000000
   192.168.6.0/24    11000000 10101000 00000110 00000000
   192.168.7.0/24    11000000 10101000 00000111 00000000
                     └──────── common ─────────┘└─ vary ─┘
                            22 bits
```

The third octets are 4, 5, 6, 7 = `00000100`, `00000101`, `00000110`, `00000111`. **The
first six bits are identical**; the last two vary through all four combinations.

$$8 + 8 + 6 = 22 \text{ common bits}$$

**Summary: `192.168.4.0/22`**

**Verify:** a /22 covers 4 × 256 = 1024 addresses, from `192.168.4.0` to
`192.168.7.255`. Exactly the four networks, nothing more.

### Example — eight /24s

```
   10.1.8.0/24   through   10.1.15.0/24

   8  = 00001000
   9  = 00001001
   …
   15 = 00001111
        └──5──┘└3┘
        common  vary
```

$$8 + 8 + 5 = 21$$

**Summary: `10.1.8.0/21`** — covering `10.1.8.0` to `10.1.15.255`.

### The shortcut

**Number of networks combined → bits given back:**

| Networks | Bits | Prefix change |
|---|---|---|
| 2 | 1 | /24 → /23 |
| 4 | 2 | /24 → /22 |
| 8 | 3 | /24 → /21 |
| 16 | 4 | /24 → /20 |
| 32 | 5 | /24 → /19 |
| 64 | 6 | /24 → /18 |
| 256 | 8 | /24 → /16 |

$$\text{new prefix} = \text{old prefix} - \log_2(\text{number of networks})$$

## The two conditions

Aggregation is only correct when both hold.

### 1. The blocks must be contiguous

`192.168.4.0/24` through `192.168.7.0/24` — yes, no gaps.

`192.168.4.0/24`, `192.168.5.0/24`, `192.168.9.0/24` — **no**. Summarising these to a
/22 would advertise `192.168.6.0/24` and `192.168.7.0/24` as well, which you do not
have.

**Advertising what you do not have is not merely untidy.** It attracts traffic destined
for someone else's network, which you will then drop. At Internet scale this is a
**route hijack** (Chapter 32 §32.4), whether or not it was intended, and it has taken
significant portions of the Internet offline more than once.

### 2. The summary must start on a boundary

**The starting network must be a multiple of the block size.**

`192.168.4.0/22` — the block covers 4 units of /24, so it must start at a multiple of 4.
4 ÷ 4 = 1. ✓

`192.168.5.0/22` — 5 is **not** a multiple of 4. × Not a valid /22.

**Check by looking at the binary:** the network address must have all zeros in the host
portion of the summary prefix. If it does not, the prefix is not valid.

**The most common summarisation error** is combining a set that is contiguous but does
not begin on a boundary, for example `192.168.5.0/24` through `192.168.8.0/24`. Four
networks, contiguous, and no single prefix covers exactly them:

```
   5, 6, 7, 8
   5 = 00000101
   8 = 00001000
       └─4─┘
   Only 4 bits common → /20, which covers 0–15.
   That is sixteen networks, not four.
```

The correct answer is **two summaries**: `192.168.5.0/24` + `192.168.6.0/23` +
`192.168.8.0/24`, or simply advertising the four separately.

## Hierarchy — why plans are designed for this

Aggregation works only if the addressing was **planned** to allow it. This is the strongest argument for designing an address plan before deploying, and it is what §26.4
develops.

```
   10.0.0.0/8                        one organisation
     ├── 10.1.0.0/16                 Site A
     │     ├── 10.1.0.0/20           Building 1
     │     │     ├── 10.1.0.0/24     Floor 1
     │     │     ├── 10.1.1.0/24     Floor 2
     │     │     └── 10.1.2.0/24     Floor 3
     │     └── 10.1.16.0/20          Building 2
     ├── 10.2.0.0/16                 Site B
     └── 10.3.0.0/16                 Site C
```

**What each level advertises:**

| Level | Advertises | Instead of |
|---|---|---|
| Floor router | `10.1.0.0/24` | — |
| Building router | `10.1.0.0/20` | 16 /24s |
| Site router | `10.1.0.0/16` | 256 /24s |
| Organisation to the Internet | `10.0.0.0/8` | 65,536 /24s |

**Each level's routing table stays small**, and — the operationally important part — **a
change inside a building does not propagate outside it.** A new subnet on floor 4 is
invisible to Site B, because Site B's route to Building 1 is a /20 that already covers
it.

> **Aggregation is not only about table size. It is about isolating change.** A network
> whose routing table churns every time a subnet is added is a network whose stability
> depends on nobody doing anything.

**Contrast with a plan that grew organically:**

```
   10.1.0.0/24    Building 1, floor 1
   10.5.0.0/24    Building 1, floor 2      ← assigned later, no plan
   10.2.0.0/24    Building 2
   10.9.0.0/24    Building 1, floor 3      ← later still
```

**Nothing aggregates.** Every subnet must be advertised individually, forever, and every
addition is a global event. This is extremely common in networks that grew without a
plan, and it is essentially unfixable without renumbering.

## Longest-prefix match, and why it makes exceptions work

Chapter 29 §29.3 covers this properly. The relevant consequence here:

**A router uses the most specific matching route.** Which means an aggregate and an
exception can coexist:

```
   10.1.0.0/16   →  next hop A     (the aggregate)
   10.1.5.0/24   →  next hop B     (the exception)
```

Traffic to `10.1.5.50` matches both, and **the /24 wins because it is longer**. Traffic
to `10.1.9.50` matches only the /16.

This is what makes hierarchical plans practical: you can aggregate broadly and still
carry a specific route for the one subnet that moved. **Aggregation does not have to be
perfect to be valuable.**

It is also the mechanism route hijacking exploits: announcing a **more specific** prefix
than the legitimate holder attracts the traffic, because longer always wins. Chapter 32
§32.4.

## The default route

The limiting case of aggregation:

```
   0.0.0.0/0
```

**Zero network bits.** It matches every address, and therefore — being the shortest
possible prefix — it loses to every other route under longest-prefix match. Which is
exactly the behaviour you want: *"if nothing else matches, send it here."*

**One route replacing the entire Internet.** It is why an office router with four
entries can reach any of four billion addresses, and it is aggregation taken to its
logical end.

## Working it in practice

```bash
# Summarise a list of networks
python3 tools/netcalc.py summarise 192.168.4.0/24 192.168.5.0/24 \
                                   192.168.6.0/24 192.168.7.0/24
# → 192.168.4.0/22

# Check a proposed summary covers what you think
python3 tools/netcalc.py subnet 192.168.4.0/22
```

The tool will refuse to produce a single summary when the set does not aggregate
cleanly, which is the check worth having.

## What breaks here

**Summarising non-contiguous blocks.** You advertise addresses you do not hold, attract
traffic you cannot deliver, and — externally — commit a route hijack.

**Summarising from a non-boundary start.** The result is not a valid prefix, or it
covers more than intended.

**Summarising too aggressively at the wrong place.** Aggregating at a point where the
components are reachable by different paths creates a black hole for part of the range.

**An address plan that cannot aggregate.** The commonest real-world problem, caused by
assigning subnets as they were requested rather than from a plan. Not fixable without
renumbering.

**Assuming aggregation is only about table size.** Its larger value is **isolating
change**.

> **Network+ note.** Objective 1.7 expects CIDR and summarisation. Over-learn: **four
> /24s summarise to a /22, eight to a /21, sixteen to a /20**; **the summary must start
> on a boundary**; **the blocks must be contiguous**; and **`0.0.0.0/0` is the default
> route and loses to everything under longest-prefix match**. Expect "which summary
> covers these networks?" questions with a non-boundary distractor.
