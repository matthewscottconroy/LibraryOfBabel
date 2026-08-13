# 19.4 Link Aggregation

Spanning tree's answer to two cables between two switches is to block one. That is
correct and it is wasteful: half the purchased capacity sits idle waiting for a
failure.

Link aggregation is the other answer. **Make the two cables look like one link**, so
there is no loop for STP to find and both carry traffic.

## The idea

```
   Without aggregation              With aggregation
   ───────────────────              ────────────────

   SW-1 ═══ fwd ═══ SW-2            SW-1 ═══════════ SW-2
        ═══ BLK ═══                      ═══════════
                                    (one logical 2 Gb/s link,
   1 Gb/s usable, 1 Gb/s idle        both members forwarding)
```

Two or more physical links are bound into a single logical interface. STP sees **one**
port and computes one tree; the aggregate has the combined bandwidth; and if a member
fails the aggregate continues at reduced capacity with **no STP recomputation at
all**, because as far as STP is concerned nothing changed.

The names proliferate — **EtherChannel** (Cisco), **port channel**, **bond** (Linux),
**team** (Windows), **trunk** (HP, Juniper) — and all refer to IEEE 802.3ad, now
**802.1AX**.

**"Trunk" is a genuine trap.** For HP and Juniper it means aggregation; for Cisco it
means a VLAN-tagged port (Chapter 20 §20.3). Two entirely different things in the same
comms room. Ask which is meant.

## LACP

**Link Aggregation Control Protocol.** The standard way to form a bundle: both ends
exchange LACPDUs and agree on membership.

| Mode | Behaviour |
|---|---|
| **Active** | sends LACPDUs, actively tries to form the bundle |
| **Passive** | responds to LACPDUs, does not initiate |
| **On / static** | no protocol; bundle assumed |

Active–active and active–passive both form a bundle. **Passive–passive does not** —
neither side speaks first, and the ports remain independent. This is a standard exam
item and a standard real-world mistake.

**Use LACP, not static.** The reason is a failure mode: with static configuration, if
one end is bundled and the other is not, the "bundle" becomes a set of parallel links
with no STP blocking — **a loop**. LACP detects the disagreement and refuses to form
the bundle, leaving STP to block the extras. **The protocol exists to catch the
mistake**, which is worth more than the configuration it saves.

LACP also detects a member that is up but not passing traffic — a unidirectional
failure that static bundling would happily keep using.

## What must match

All members of a bundle must agree on:

- **Speed** and **duplex**
- **VLAN membership** (access VLAN, or the trunk's allowed list and native VLAN)
- **MTU**
- **Trunk/access mode**

A mismatch on any of these causes the member to be suspended — visible as a bundle
with fewer active members than cables. `show etherchannel summary` or
`show lacp neighbor` reports it, and the cause is nearly always a VLAN list that was
edited on one interface and not the other.

## How traffic is distributed — the important part

This is where expectations go wrong, and the reason is a design decision that is
correct and unintuitive.

**Frames are not distributed round-robin.** They are assigned to a member by a **hash
of header fields**:

```
   member = hash(selected fields) mod (number of members)
```

Typical hash inputs, depending on platform and configuration:

| Hash basis | Granularity |
|---|---|
| Source MAC | coarse |
| Destination MAC | coarse |
| Source + destination MAC | moderate |
| Source + destination IP | good |
| Source + destination IP + L4 ports | **best** |

**Why hash rather than round-robin?** Because round-robin would deliver frames of the
same conversation out of order. A 2 Gb/s bundle sending alternate frames down two
1 Gb/s links would routinely deliver frame 2 before frame 1, and TCP interprets
out-of-order delivery as loss — triggering fast retransmit and collapsing throughput
(Chapter 37 §37.3). **Ethernet's ordering guarantee within a conversation is depended
upon by everything above it**, and hashing preserves it by pinning each conversation
to one member.

**The consequence that surprises everyone:**

> **A single conversation never exceeds the speed of one member link.**

Four 1 Gb/s links bundled do **not** give one file transfer 4 Gb/s. They give it
1 Gb/s, and give *aggregate* capacity of 4 Gb/s across many conversations.

This is the single most common misunderstanding of aggregation, and it produces real
disappointment: a backup job between two servers over a 4 Gb/s bundle runs at 1 Gb/s,
and everything is working exactly as designed.

**Mitigations**, when a single flow needs more:

- Use multiple TCP connections (most backup and storage software supports this)
- Hash on L4 ports, so parallel connections between the same hosts spread
- Buy a faster single link — **usually the right answer**

**Polarisation** is the related failure: a hash that produces uneven distribution
because the traffic is uneven. A bundle carrying traffic dominated by one server pair
may drive 90% down one member while the others idle. The fix is a hash with more
inputs; the diagnosis is per-member interface counters, which is the only place the
imbalance is visible.

## Multi-chassis aggregation

A plain bundle terminates on **one** switch at each end, so that switch is a single
point of failure. Multi-chassis techniques let a bundle terminate on **two** physical
switches that present themselves as one:

| Technology | Vendor |
|---|---|
| **vPC** (virtual Port Channel) | Cisco Nexus |
| **MLAG** | Arista, Cumulus, others |
| **VSS** / **StackWise** | Cisco, by merging control planes |
| **MC-LAG** | Juniper |

The server or downstream switch sees one bundle and needs no special configuration —
it runs ordinary LACP. The two upstream switches coordinate over a peer link.

**Result: a server survives the loss of an entire switch**, which is what makes
non-stop maintenance possible in a data centre. It is also the reason a redundant
design can be upgraded during business hours, which matters more to operations than
the availability arithmetic suggests.

## Where it fits against STP

| | Spanning Tree | Link Aggregation |
|---|---|---|
| Redundant links | one active, rest **blocked** | **all active** |
| Convergence on failure | seconds (RSTP: sub-second) | **milliseconds** |
| Loop prevention | by blocking | by presenting one logical link |
| Requires matched configuration | no | **yes** |
| Works between any two devices | yes | only where both support it |

They are complementary. Aggregation handles the parallel-links-between-two-switches
case efficiently; STP handles the general topology, including everything aggregation
cannot cover. **Run both**: bundle where you have parallel links, and leave STP
enabled to catch what you did not anticipate — which, given §19.1's table of causes,
is where the outages come from.

## What breaks here

**A bundle with fewer active members than cables.** A configuration mismatch —
usually VLAN membership. `show etherchannel summary`.

**No bundle at all.** Both ends passive. One must be active.

**A loop after configuring a bundle.** Static mode with one end unbundled. Use LACP.

**Throughput of one link on a four-link bundle.** Working correctly. One conversation,
one member. Explain the hash.

**Very uneven member utilisation.** Hash polarisation. More hash inputs, or accept it.

**A bundle that flaps.** Often a member with a marginal cable or transceiver; LACP
detects the failure and re-forms repeatedly. Per-member error counters will show it.

> **Network+ note.** Objective 2.3 expects link aggregation and LACP; objective 1.2
> may ask about it as a device feature. Over-learn: **LACP is 802.3ad/802.1AX**;
> **active–passive forms a bundle, passive–passive does not**; **all members must
> match in speed, duplex, VLAN and MTU**; and above all **a single flow is limited to
> one member link's speed**, which is the most-missed practical consequence in this
> chapter.
