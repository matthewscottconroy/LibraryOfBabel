# 29.3 Longest-Prefix Match

Several routes may match one destination. **The most specific wins.** This section
derives why that is the only sensible rule, works it in detail, and draws out the
consequences — one of which is the mechanism behind the most damaging class of Internet
outage.

## The problem

A router holds:

```
   0.0.0.0/0        via A
   10.0.0.0/8       via B
   10.1.0.0/16      via C
   10.1.5.0/24      via D
   10.1.5.64/26     via E
```

A packet arrives for **`10.1.5.70`**. Which route applies?

**All five of them.** The address is inside every one of those prefixes:

| Route | Contains `10.1.5.70`? |
|---|---|
| `0.0.0.0/0` | ✓ (everything) |
| `10.0.0.0/8` | ✓ |
| `10.1.0.0/16` | ✓ |
| `10.1.5.0/24` | ✓ |
| `10.1.5.64/26` | ✓ (`.64`–`.127`) |

**The rule: use the longest prefix.** Here that is `/26`, so the packet goes **via E**.

## Why longest wins

Not arbitrary. **A longer prefix is a more specific statement, and a more specific
statement is later, better informed, or deliberately excepted.**

Consider what each route is saying:

| Route | Says |
|---|---|
| `0.0.0.0/0` | "I know nothing; try this way" |
| `10.0.0.0/8` | "Everything in 10-space is over there" |
| `10.1.0.0/16` | "That particular site is over there" |
| `10.1.5.0/24` | "That particular subnet is over there" |
| `10.1.5.64/26` | "**Those specific hosts** are over there" |

**Each is more informed than the one above it.** Ignoring the specific in favour of the
general would mean discarding the better information — which would make aggregation
impossible, because an aggregate could never have an exception.

**The general principle:** *a general rule with specific exceptions* is expressible only
if specific beats general. This is why longest-prefix match makes hierarchy work
(Chapter 26 §26.3), and it is the same rule as a firewall's most-specific-match, a CSS
selector's specificity, and a filesystem's deepest-path match.

> **Longest-prefix match is what lets you say "everything that way, except this."**

## Working it in binary

The mechanism, so the rule is not merely asserted.

**Destination `10.1.5.70`:**

```
   00001010 00000001 00000101 01000110
```

**Test each route** by comparing the first *n* bits, where *n* is the prefix length:

```
   /8    00001010 | ...                                match on 8 bits    ✓
   dest  00001010 | 00000001 00000101 01000110

   /16   00001010 00000001 | ...                       match on 16 bits   ✓
   dest  00001010 00000001 | 00000101 01000110

   /24   00001010 00000001 00000101 | ...              match on 24 bits   ✓
   dest  00001010 00000001 00000101 | 01000110

   /26   00001010 00000001 00000101 01 | 000000        match on 26 bits   ✓
   dest  00001010 00000001 00000101 01 | 000110
                                     ↑
                              26 bits agree
```

**All match. The /26 matches on the most bits, so it wins.**

**Equivalently, by masking** — which is Chapter 25 §25.3's arithmetic:

$$\text{route matches} \iff (\text{destination} \operatorname{AND} \text{mask}) = \text{network}$$

```
   10.1.5.70 AND 255.255.255.192 = 10.1.5.64  =  10.1.5.64   ✓
   10.1.5.70 AND 255.255.255.0   = 10.1.5.0   =  10.1.5.0    ✓
   10.1.5.70 AND 255.255.0.0     = 10.1.0.0   =  10.1.0.0    ✓
```

**This is why masks must be contiguous** (Chapter 25 §25.3). "Longest" is only meaningful
if a prefix is a leading run of bits; with a non-contiguous mask there is no length to
compare.

## Worked lookups

Same table. Trace each:

```
   0.0.0.0/0        via A
   10.0.0.0/8       via B
   10.1.0.0/16      via C
   10.1.5.0/24      via D
   10.1.5.64/26     via E
```

| Destination | Matches | Longest | Via |
|---|---|---|---|
| `10.1.5.70` | /0 /8 /16 /24 /26 | **/26** | **E** |
| `10.1.5.200` | /0 /8 /16 /24 | **/24** | **D** |
| `10.1.9.1` | /0 /8 /16 | **/16** | **C** |
| `10.99.1.1` | /0 /8 | **/8** | **B** |
| `172.16.1.1` | /0 | **/0** | **A** |
| `10.1.5.63` | /0 /8 /16 /24 | **/24** | **D** |

**Note `10.1.5.63` and `10.1.5.200`.** Both fall outside the /26 (`.64`–`.127`), so they
fall back to the /24. **The /26 is a hole punched in the /24**, and everything outside it
still follows the general rule.

That pattern — an aggregate with specific exceptions carved out — is how essentially every
real routing table is structured.

## Ties

Two routes with the **same** prefix length matching the same destination. Longest-prefix
match cannot decide, so the tie-breakers run:

**1. Administrative distance** — trust the source more (Chapter 30 §30.2).

```
   10.1.0.0/16 [1/0]   via A     static, AD 1     ← wins
   10.1.0.0/16 [110/20] via B    OSPF,   AD 110
```

**2. Metric**, within the same protocol.

**3. Equal-cost multipath.** If distance *and* metric tie, **both routes are installed**
and traffic is shared:

```
   10.1.0.0/16 [110/20] via A
   10.1.0.0/16 [110/20] via B
```

**ECMP distributes by hashing**, exactly as link aggregation does (Chapter 19 §19.4), and
for exactly the same reason: **round-robin would reorder packets within a flow, and TCP
reads reordering as loss.**

So the same consequence applies: **a single flow uses one path and cannot exceed its
capacity.** Four ECMP paths of 10 Gb/s give one TCP connection 10 Gb/s, not 40.

**And the diagnostic consequence:** ECMP is why `traceroute` may show different routers
for successive probes at the same hop (Chapter 24 §24.4), and why an intermittent problem
may affect some flows and not others — they are on different paths. `paris-traceroute`
exists to hold the flow constant so the path is stable.

## The consequences that matter

### The default route always loses

`0.0.0.0/0` is the **shortest possible prefix**, so **any other matching route beats
it.** That is exactly the desired behaviour — "if nothing else matches, send it here" —
and it is achieved with no special case at all. §29.4 develops it.

### A /32 always wins

A host route is the **longest possible prefix** and beats everything. Used to:

- Pull one server's traffic down a specific path
- Blackhole a single address (`ip route 203.0.113.66 255.255.255.255 Null0`)
- Advertise an anycast service (Chapter 27 §27.3)
- Reach a router's loopback (Chapter 27 §27.4)

### More-specific hijacking

**The dangerous consequence, and the mechanism behind the worst Internet outages.**

If you announce a **more specific prefix** than the legitimate holder, **you attract their
traffic** — because longest-prefix match is doing exactly what it is supposed to.

```
   Legitimate:  203.0.113.0/24   announced by the real owner
   Attacker:    203.0.113.0/25   and  203.0.113.128/25

   Every router on the Internet now prefers the attacker's /25s.
```

**Nothing is broken.** Every router applies the rule correctly. The protocol has no way
to know which announcement is legitimate, because **BGP has no authentication of who owns
what** (Chapter 32 §32.4).

**This is not theoretical:**

| Incident | Effect |
|---|---|
| **Pakistan Telecom / YouTube, 2008** | A /24 more specific than YouTube's /22 took YouTube offline globally for about two hours |
| **China Telecom, 2010** | ~15% of Internet prefixes briefly routed via China |
| **Amazon Route 53 / MyEtherWallet, 2018** | More-specific announcement enabled DNS redirection and cryptocurrency theft |
| **Rostelecom, 2020** | 8,800 prefixes including major providers |

**Providers filter announcements more specific than /24** as a partial defence, which is
why /24 is the effective minimum announceable unit on the Internet — a convention with no
protocol basis, enforced by mutual filtering.

**RPKI** (Chapter 32 §32.4) is the real fix: cryptographic attestation of which AS may
announce which prefix. Deployment is now substantial and incomplete.

### Aggregation with exceptions

The benign version of the same mechanism, and the reason hierarchical plans work:

```
   Advertise:  10.1.0.0/16      the whole site
   Plus:       10.1.5.0/24      the one subnet that moved elsewhere
```

**One aggregate plus one exception**, rather than 256 individual routes. Chapter 26
§26.3's claim that "aggregation does not have to be perfect to be valuable" is exactly
this.

## How it is done at speed

§29.1 gave the constraint: **6.7 ns per packet at 100 Gb/s.** You cannot iterate a
million routes.

| Technique | How |
|---|---|
| **TCAM** | compares against **all entries at once**, returns the longest match in one lookup, constant time |
| **Trie / radix tree** | walk the address bit by bit; software routers and the Linux kernel |
| **Compressed tries** (LC-trie, Poptrie) | fewer memory accesses per lookup |
| **Hash per prefix length** | one hash table per possible length, probe longest-first |

**TCAM is why longest-prefix match is affordable in hardware**, and its cost and power
consumption are why routing table size is a real, physical limit rather than a
theoretical one. §29.1's table applies.

## What breaks here

**Traffic taking an unexpected path.** A more specific route exists somewhere. **Look for
the longest match, not the one you configured.**

**A static route "not working".** A more specific dynamic route is winning, or a
better administrative distance is.

**Blackholing more than intended.** A `Null0` route with too short a prefix swallows a
whole range.

**A single flow not using all ECMP paths.** Working as designed — one flow, one path.

**Traceroute showing different paths per probe.** ECMP.

**Someone else's traffic arriving at you.** You announced something more specific than
you hold. Check your aggregation (Chapter 26 §26.3).

> **Network+ note.** Objective 2.2 examines longest-prefix match directly, usually as a
> table plus a destination. Over-learn: **most specific wins**; **`/32` beats everything
> and `0.0.0.0/0` loses to everything**; **ties break on administrative distance, then
> metric, then ECMP**; and **ECMP means one flow takes one path**. Work the lookup by
> counting prefix bits, not by reading down the table.
