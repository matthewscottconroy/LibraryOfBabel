# 25.3 The Subnet Mask

The mask is where the split of §25.2 comes from. It is a bit pattern, it is used with a
single bitwise operation, and understanding it as **an operator rather than a number**
is what makes subnetting straightforward instead of memorised.

## What a mask is

**32 bits: a run of ones, then a run of zeros. Nothing else.**

```
   255.255.255.0  =  11111111 11111111 11111111 00000000
                     └────── 24 ones ─────────┘└─ 8 zeros ┘
```

- **Ones mark the network portion.**
- **Zeros mark the host portion.**

**The ones must be contiguous and must come first.** RFC 4632 requires it, all modern
equipment enforces it, and the reason is that longest-prefix match (Chapter 29 §29.3)
depends on it: a prefix is meaningful only if it is a leading run.

Non-contiguous masks — `255.0.255.0` — were briefly legal in early implementations,
produced routing that could not be reasoned about, and are now simply invalid.

## The two notations

| Notation | Example | Also called |
|---|---|---|
| **Dotted decimal** | `255.255.255.0` | the subnet mask |
| **CIDR / prefix length** | `/24` | slash notation |

**They are the same information.** The prefix length counts the ones.

| Prefix | Mask | Prefix | Mask |
|---|---|---|---|
| /8 | 255.0.0.0 | /25 | 255.255.255.128 |
| /16 | 255.255.0.0 | /26 | 255.255.255.192 |
| /24 | 255.255.255.0 | /27 | 255.255.255.224 |
| /20 | 255.255.240.0 | /28 | 255.255.255.240 |
| /22 | 255.255.252.0 | /29 | 255.255.255.248 |
| /23 | 255.255.254.0 | /30 | 255.255.255.252 |

CIDR notation won for practical reasons: it is shorter, unambiguous, and it makes the
comparison that matters — *which prefix is longer?* — a simple numeric comparison rather
than a bit count.

**Converting between them** is just §25.1's mask octet table read in either direction.
A `/27` is 27 ones: three full octets (24) plus 3 more, and 3 ones is `11100000` = 224.
So `255.255.255.224`.

## The AND operation

The mask's entire purpose:

$$\text{network} = \text{address} \operatorname{AND} \text{mask}$$

AND is 1 only when both inputs are 1:

| A | B | A AND B |
|---|---|---|
| 0 | 0 | 0 |
| 0 | 1 | 0 |
| 1 | 0 | 0 |
| 1 | 1 | **1** |

**Applied:**

```
   address  192.168.10.70   11000000 10101000 00001010 01000110
   mask     255.255.255.192 11111111 11111111 11111111 11000000
            AND             ─────────────────────────────────────
   network  192.168.10.64   11000000 10101000 00001010 01000000
```

**What the AND does:** wherever the mask is 1, the address bit is preserved. Wherever
the mask is 0, the bit is zeroed.

> **The mask is a stencil. It keeps the network bits and erases the host bits.**

That sentence is worth carrying. Once the mask is understood as a stencil rather than as
a mysterious number, everything in Chapter 26 becomes mechanical.

## The local-or-remote decision

**The operation every host performs before sending every packet.** Chapter 18 §18.1
introduced it; here is the arithmetic.

Host `192.168.10.70/24` wants to send to `192.168.10.200`:

```
   my address    192.168.10.70   AND  /24  →  192.168.10.0
   destination   192.168.10.200  AND  /24  →  192.168.10.0
                                              ─────────────
                                              EQUAL → local
```

**Local.** ARP for `192.168.10.200` and send directly.

Same host, sending to `192.168.11.5`:

```
   my address    192.168.10.70  AND  /24  →  192.168.10.0
   destination   192.168.11.5   AND  /24  →  192.168.11.0
                                             ─────────────
                                             DIFFERENT → remote
```

**Remote.** ARP for the **default gateway** and send the frame there — with the IP
destination still `192.168.11.5`.

**Note that only the sender's mask is used.** The host does not know the destination's
mask, cannot know it, and does not need it. It is answering *"is this in my network?"*,
and only its own mask defines its own network.

**This asymmetry is the source of the failure below.**

## The mask mismatch — networking's most confusing fault

Two hosts on the same wire with different masks.

```
   Host A:  192.168.10.70/24    (255.255.255.0)
   Host B:  192.168.10.200/25   (255.255.255.128)
```

**A's view.** Its network is `192.168.10.0` – `192.168.10.255`. B at `.200` is **inside**
it. So A ARPs for B directly.

**B's view.** Its network is `192.168.10.128` – `192.168.10.255`. A at `.70` is
**outside** it. So B sends replies to its **default gateway**.

**The result:**

| Direction | What happens |
|---|---|
| A → B | Direct frame. **Arrives.** |
| B → A | Sent to the gateway. Gateway may route it back onto the same wire, may send an ICMP redirect, may drop it, or may have no route |

**Communication works in one direction and not the other**, or works erratically
depending on what the gateway does with a packet whose destination is on the interface it
arrived on.

**The symptom set:**

- Some hosts reach each other, others do not, with no obvious pattern
- `ping` works from one end and not the other
- ARP requests appear for addresses outside the sender's subnet (Chapter 18 §18.2)
- Traffic between two hosts on the same switch traverses the router
- Everything looks correct in configuration, individually

**This is the most confusing common fault in IP networking**, and the diagnosis is
always the same: **compare masks on both hosts.** It takes ten seconds and it is skipped
constantly, because a wrong mask is not a wrong-looking configuration.

**And a capture makes it obvious:** what a host ARPs for reveals what it believes its
subnet is. An ARP request for an off-subnet address proves a wrong mask, immediately.

## Wildcard masks

The mask's inverse, and it appears in access lists and OSPF.

```
   Subnet mask     255.255.255.0    11111111 11111111 11111111 00000000
   Wildcard mask     0.0.0.255      00000000 00000000 00000000 11111111
```

**Wildcard: 0 means "must match", 1 means "don't care"** — exactly inverted.

**Converting:** subtract each octet from 255.

$$255.255.255.192 \longrightarrow 0.0.0.63$$

| Prefix | Subnet mask | Wildcard |
|---|---|---|
| /24 | 255.255.255.0 | 0.0.0.255 |
| /25 | 255.255.255.128 | 0.0.0.127 |
| /26 | 255.255.255.192 | 0.0.0.63 |
| /27 | 255.255.255.224 | 0.0.0.31 |
| /30 | 255.255.255.252 | 0.0.0.3 |
| /32 | 255.255.255.255 | **0.0.0.0** (`host`) |
| /0 | 0.0.0.0 | **255.255.255.255** (`any`) |

Used in Cisco ACLs and OSPF `network` statements:

```
access-list 10 permit 192.168.10.0 0.0.0.255
network 192.168.10.0 0.0.0.255 area 0
```

**Why they exist:** unlike a subnet mask, a wildcard mask **may be non-contiguous**,
allowing matches that a prefix cannot express — for example every odd-numbered subnet.
This is rarely used and it is the reason for the separate concept.

Newer syntax accepts prefix notation and much modern equipment does. Cisco IOS ACLs and
OSPF still expect wildcards, and getting them backwards is a rite of passage.

## Reading a mask quickly

Three habits worth developing:

**Count the ones for the prefix.** `255.255.255.240` → 8+8+8+4 = **/28**.

**Subtract from 256 for the block size.** 256 − 240 = **16**, so networks start at .0,
.16, .32, .48, … This is the "magic number" method of Chapter 26 §26.2.

**Check it is valid.** Every octet must be from {0, 128, 192, 224, 240, 248, 252, 254,
255}, and once a 0 appears everything after it must be 0. `255.255.0.255` is invalid.

## What breaks here

**Mask mismatch between hosts.** The confusing one. Always compare both.

**Wildcard and subnet masks confused.** `0.0.0.255` in a place expecting
`255.255.255.0` matches nothing or everything. Cisco ACLs, OSPF.

**Non-contiguous masks.** Invalid. Modern equipment rejects them.

**A wrong mask on the gateway rather than the host.** Same symptom set, harder to find,
because you check the complaining host first.

**Assuming both ends must have the same mask on a point-to-point link.** They must, and
this is violated more often than you would expect when one end is /30 and the other /31.

> **Network+ note.** Objective 1.7 expects subnet masks in both notations and the
> conversion between them. Objective 5.3 expects **incorrect subnet mask** as a named
> troubleshooting cause — and it is examined, because the symptom is so distinctive.
> Over-learn: **the mask defines what is local**; **AND the address with the mask to get
> the network**; **only the sender's own mask matters for its own decision**; and **a
> mask mismatch causes one-way or selective connectivity**.
