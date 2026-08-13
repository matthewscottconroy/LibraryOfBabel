# 25.2 Network and Host Portions

The single most important structural fact about an IP address:

> **Every IPv4 address is split into two parts. The left part identifies a network. The
> right part identifies a host on that network.**

Everything in Chapters 25, 26 and 30 follows from this one sentence.

## Why the split exists

Chapter 18 §18.1 established the argument, and it is worth restating because this is
where it becomes concrete.

MAC addresses are **flat**. To know where a MAC address lives, you must know that
address individually — there is no rule that covers a set of them. A global network
addressed that way would need one routing entry per device on Earth, which is not
expensive but arithmetically impossible.

**The split makes aggregation possible.** A router does not need to know where
`203.0.113.47` is. It needs to know where `203.0.113.0/24` is — one entry covering 254
hosts. And further up, one entry covering `203.0.0.0/8` — 16 million.

$$\text{20 billion devices} \longrightarrow \text{under 1 million routing entries}$$

**That reduction is what makes the Internet possible.** It is worth restating: the
Internet does not work because routers are fast. It works because the address structure
means they do not have to know very much.

## The split, drawn

```
   192.168.10.70  with mask  255.255.255.0

   11000000 10101000 00001010 . 01000110
   └────────── network ───────┘ └─ host ─┘
              24 bits              8 bits
```

**Network portion:** `192.168.10.0` — identifies the network.
**Host portion:** `.70` — identifies this host within it.

Two consequences that are the substance of the split:

**Every host on a network shares the network portion.** `192.168.10.1`,
`192.168.10.70` and `192.168.10.200` are all on `192.168.10.0/24`.

**Two hosts with different network portions are on different networks**, no matter how
similar the numbers look. `192.168.10.70/24` and `192.168.11.70/24` are on different
networks and **cannot reach each other without a router**, despite differing in one
digit.

## Where the split falls

**Anywhere.** This is the part that surprises people.

The boundary is set by the **mask**, and it may fall at any of 33 positions:

| Notation | Network bits | Host bits | Hosts per network |
|---|---|---|---|
| /8 | 8 | 24 | 16,777,214 |
| /16 | 16 | 16 | 65,534 |
| /24 | 24 | 8 | **254** |
| /25 | 25 | 7 | 126 |
| /26 | 26 | 6 | 62 |
| /27 | 27 | 5 | 30 |
| /28 | 28 | 4 | 14 |
| /29 | 29 | 3 | **6** |
| /30 | 30 | 2 | **2** |
| /31 | 31 | 1 | 2 (special, RFC 3021) |
| /32 | 32 | 0 | 1 (a single host) |

**The /24 boundary is a convention, not a rule.** It is common because it aligns with an
octet and is therefore easy for humans, and there is nothing else special about it.

## The host count formula

$$\text{usable hosts} = 2^{h} - 2 \quad \text{where } h = 32 - \text{prefix length}$$

**Why minus two?** Two host values in every network are reserved:

**All host bits zero — the network address.** It names the network itself. `192.168.10.0`
in a /24. You cannot assign it to a host, because it is what routers put in their tables
to refer to the whole network.

**All host bits one — the broadcast address.** `192.168.10.255` in a /24. A packet
addressed here goes to every host on that network. Chapter 27 §27.3 covers what it is
for.

```
   192.168.10.0/24

   192.168.10.0      network address    ← reserved
   192.168.10.1      first usable
   192.168.10.2      …
      …
   192.168.10.254    last usable
   192.168.10.255    broadcast          ← reserved

   256 total, 254 usable
```

**The two exceptions:**

**/31** — RFC 3021 permits a /31 on point-to-point links, giving **two usable
addresses** rather than zero. There are only two devices and no need for a broadcast, so
the reservation is pointless. Widely supported and widely underused; a /30 on every
point-to-point link wastes half its addresses for no reason.

**/32** — a single address, a **host route**. Used for loopback interfaces on routers, for
anycast (Chapter 27 §27.3), and for pointing at one specific host in a routing table.

## Worked examples

Do these until they are automatic.

**`10.1.2.3/8`**

| | |
|---|---|
| Network bits | 8 |
| Host bits | 24 |
| Network | `10.0.0.0` |
| Broadcast | `10.255.255.255` |
| Usable | `10.0.0.1` – `10.255.255.254` |
| Count | 2²⁴ − 2 = **16,777,214** |

**`172.16.50.100/16`**

| | |
|---|---|
| Network | `172.16.0.0` |
| Broadcast | `172.16.255.255` |
| Usable | `172.16.0.1` – `172.16.255.254` |
| Count | 2¹⁶ − 2 = **65,534** |

**`192.168.10.70/26`**

Here the split falls mid-octet, so binary is required:

```
   192.168.10.70   =  11000000 10101000 00001010 . 01 000110
   /26 mask        =  11111111 11111111 11111111 . 11 000000
                                                    └┬┘ └─┬─┘
                                            network ─┘    └─ host
```

| | |
|---|---|
| Host bits | 6 |
| Network | `01` + zeros = `01000000` = **`192.168.10.64`** |
| Broadcast | `01` + ones = `01111111` = **`192.168.10.127`** |
| Usable | `192.168.10.65` – `192.168.10.126` |
| Count | 2⁶ − 2 = **62** |

**Note that `192.168.10.70` is not the 70th host of anything.** It is the 7th host of
the network starting at 64. The dotted notation actively misleads here, which is §25.1's
warning made concrete.

**`203.0.113.130/30`**

```
   .130  =  10000010
   /30   =  11111100
```

| | |
|---|---|
| Network | `10000000` = **`.128`** |
| Broadcast | `10000011` = **`.131`** |
| Usable | `.129` and `.130` — **two addresses** |
| Count | 2² − 2 = **2** |

The classic point-to-point link allocation. Four addresses consumed for two usable, which
is why /31 exists.

## The two operations

Two things a host or router does constantly. Both are bitwise AND with the mask.

**1. Find the network of an address.**

$$\text{network} = \text{address} \operatorname{AND} \text{mask}$$

**2. Decide whether two addresses are on the same network.**

$$(\text{A} \operatorname{AND} \text{mask}) \stackrel{?}{=} (\text{B} \operatorname{AND} \text{mask})$$

This is the **local-or-remote decision** of Chapter 18 §18.1, performed for every packet
a host sends, and §25.3 develops it.

## Different masks, same address

The point that makes masks feel strange at first: **an address alone does not tell you
its network.**

`192.168.10.70` is on:

| With mask | Network | Range |
|---|---|---|
| /24 | `192.168.10.0` | .0 – .255 |
| /25 | `192.168.10.0` | .0 – .127 |
| /26 | `192.168.10.64` | .64 – .127 |
| /27 | `192.168.10.64` | .64 – .95 |
| /28 | `192.168.10.64` | .64 – .79 |
| /29 | `192.168.10.64` | .64 – .71 |
| /30 | `192.168.10.68` | .68 – .71 |

Seven different networks, same address.

> **An IP address is meaningless without its mask.** Always write them together —
> `192.168.10.70/26`, never `192.168.10.70`.

This is why documentation that lists addresses without masks is nearly useless, and why
`ip addr` always shows both.

## What breaks here

**Assuming /24.** It is a convention. Read the mask.

**Forgetting the minus two.** A /26 has 64 addresses and **62 usable**.

**Assuming .1 is always the gateway.** A convention, and a common one, and not a rule.
In `192.168.10.64/26` the first usable address is `.65`.

**Assuming a /24 boundary in a non-/24 network.** `192.168.10.70/26` is not in the same
network as `192.168.10.200/26`, despite sharing three octets.

**Writing an address without a mask.** It carries less information than it appears to.

> **Network+ note.** Objective 1.7 examines this constantly. Over-learn: **2^h − 2
> usable hosts**; **the network address is all-zeros in the host portion, the broadcast
> is all-ones**; **/30 gives 2 usable and is the point-to-point standard**; and **an
> address without a mask is ambiguous**. Expect to compute network, broadcast and usable
> range from an address and prefix repeatedly.
