# 19.3 RSTP, MSTP and Modern Practice

Fifty seconds of outage was tolerable in 1985 and intolerable by 1998. This section
covers what replaced classic STP, what a competent configuration looks like today, and
the small set of features that actually prevent outages.

## RSTP — 802.1w (1998, folded into 802.1D-2004)

**Rapid Spanning Tree Protocol.** Same tree, same election, same tie-breakers.
Different convergence — **under a second** in the common cases, against 30–50.

Three changes account for nearly all of it.

**1. Every switch originates BPDUs.**

In classic STP, only the root generates BPDUs; others relay them. A switch that stops
hearing BPDUs must wait max age (20 s) to conclude anything, because silence might
just mean the relay was slow.

In RSTP, **every switch sends its own BPDUs every hello interval**, and **three
consecutive missed hellos** (6 seconds) means the neighbour is gone. A BPDU becomes a
keepalive, and failure detection drops from 20 seconds to 6 — or to microseconds when
the link itself goes down, which RSTP also acts on immediately.

**2. Proposal/agreement handshake.**

The big one. Rather than waiting through timers, two switches on a point-to-point link
**negotiate directly**:

```
   SW-1                              SW-2
     │  ──── BPDU with Proposal ───▶  │
     │                                │  blocks all non-edge
     │                                │  designated ports  ("sync")
     │  ◀─── BPDU with Agreement ───  │
     │                                │
   forwards immediately          then repeats the handshake
                                 with its own downstream
```

The receiving switch blocks its other designated ports before agreeing — which
**guarantees no loop can form** — then agrees, and the link forwards at once. The
handshake then propagates outward, one hop at a time, at the speed of message
exchange rather than the speed of a timer.

The safety property is preserved by *construction* rather than by *waiting*. That is
the insight: classic STP's timers were a proxy for "everyone has stopped believing the
old topology", and an explicit handshake establishes the same thing directly.

**3. Link types matter.**

| Link type | Detected by | Fast convergence? |
|---|---|---|
| **Point-to-point** | full duplex | **yes** — handshake applies |
| **Shared** | half duplex | no — falls back to timers |
| **Edge** | administratively configured | **yes** — forwards immediately |

Which produces a consequence worth remembering: **a duplex mismatch turns off rapid
convergence**. A port that negotiates to half duplex is classified as shared, and RSTP
silently reverts to 802.1D timing on it. This is one of several ways a duplex mismatch
(Chapter 16 §16.4) produces a symptom that looks nothing like a duplex problem.

**Port roles and states simplified:**

| Classic 802.1D states | RSTP states |
|---|---|
| Disabled, Blocking, Listening | **Discarding** |
| Learning | Learning |
| Forwarding | Forwarding |

| Role | Meaning |
|---|---|
| **Root** | best path to root |
| **Designated** | forwards onto this segment |
| **Alternate** | a backup path to root via *another* switch — **discarding** |
| **Backup** | a backup path to the same segment via the *same* switch — discarding |

**Alternate and Backup are RSTP's addition**, and they matter: the switch has already
identified its fallback port. On losing the root port it promotes the alternate
**immediately**, with no computation and no waiting. Precomputing the backup is why
failover is fast, and the same idea appears in routing as loop-free alternates
(Chapter 31 §31.4).

**RSTP is the default everywhere now.** If a device says "spanning tree", it means
RSTP or a vendor variant. Classic 802.1D exists only in old equipment and in exam
questions.

## MSTP — 802.1s

With VLANs (Chapter 20), a single spanning tree wastes capacity: one tree means one
set of blocked links regardless of how many VLANs exist. A redundant link blocked for
the tree is blocked for *everything*.

Two answers emerged.

**PVST+ (Cisco, proprietary).** One spanning tree per VLAN. Full flexibility — VLAN 10
can use one link and VLAN 20 the other, balancing load across redundant paths. The
cost is one instance per VLAN: with 500 VLANs, 500 trees, 500 sets of BPDUs, 500
computations. It does not scale.

**MSTP — 802.1s (standard).** **Map many VLANs onto a few instances.** Two or three
instances suffice for load balancing, and each carries hundreds of VLANs:

```
   Instance 1 : VLANs 1–500    → root SW-A → uses the left uplink
   Instance 2 : VLANs 501–1000 → root SW-B → uses the right uplink
```

Both uplinks carry traffic; each is the backup for the other. **The benefit of
per-VLAN trees at a fraction of the cost.**

MSTP's one operational trap: switches join the same **region** only if their
**region name, revision number and complete VLAN-to-instance mapping match exactly**.
A single mismatched VLAN puts a switch in its own region, where it appears as a single
boundary instance to the others and behaves in ways that take an afternoon to
diagnose. Configuration must be identical, character for character.

## What to actually configure

Six items. In practice they prevent nearly every STP-related outage.

**1. Set the root bridge deliberately.**

```
spanning-tree vlan 1-100 root primary      # priority 24576
spanning-tree vlan 1-100 root secondary    # priority 28672
```

The root should be a core switch — one that is central to the traffic pattern, has the
capacity, and does not get power-cycled. **Never leave it to the MAC address
lottery**, which reliably elects the oldest device in the building.

Priorities are configurable in steps of 4096 (the low 12 bits hold the VLAN ID in
PVST+, so only the top 4 bits are free). Common practice: root 4096, backup 8192,
access switches 61440 so they can never win.

**2. PortFast / edge ports on every access port.**

```
spanning-tree portfast                     # per port
spanning-tree portfast default             # all access ports
```

Skips listening and learning; the port forwards immediately. Without it, every
workstation waits 30 seconds before it can send DHCP — the classic "my machine can't
get on the network until I've made coffee" complaint.

**Only on ports where no switch will ever be attached.** Which is why the next item is
mandatory.

**3. BPDU Guard on every PortFast port.**

```
spanning-tree portfast bpduguard default
```

**If a BPDU arrives on an edge port, shut the port down.** An edge port should never
receive one, so a BPDU means a switch has been attached — either an unauthorised
device or a miscable — and the port is disabled before a loop can form.

**PortFast without BPDU Guard is a loop waiting to happen**, because PortFast is
exactly the removal of the delay that would otherwise have caught the mistake. The two
are a pair; configure them together, always.

**4. Root Guard on ports facing downstream.**

```
spanning-tree guard root
```

**If a superior BPDU arrives, block the port.** Protects the root election from a
downstream switch — often one somebody plugged in without asking — that claims a
better bridge ID and drags the root into a cupboard.

BPDU Guard: *no BPDUs at all here.* Root Guard: *BPDUs are fine, but you may not
become root.*

**5. Loop Guard on blocking and root ports.**

```
spanning-tree guard loop default
```

Handles the **unidirectional link** failure: a fibre pair with one strand broken, or a
transceiver failing in one direction. The port can send but not receive, so BPDUs stop
arriving — and a blocked port that stops hearing BPDUs concludes the loop is gone and
**starts forwarding**. Which creates exactly the loop STP was preventing.

Loop Guard puts a port that stops receiving BPDUs into *loop-inconsistent* (blocking)
state rather than forwarding. **The safe interpretation of silence is "something is
broken", not "the path is clear".**

**UDLD** solves the same problem by explicit bidirectional confirmation; on fibre,
configure both.

**6. BPDU Filter — know what it does, and be careful.**

Suppresses BPDUs on a port. **In its global form it is genuinely dangerous**: a port
that neither sends nor receives BPDUs is a port with no loop protection whatsoever.

The per-interface form and the global form behave differently, and the difference
matters during an incident. If you find yourself reaching for BPDU Filter, the answer
is usually Root Guard or BPDU Guard instead.

## The design that avoids the question

Modern data-centre practice largely sidesteps STP: **route the fabric instead of
bridging it**.

A leaf-spine network (Chapter 67 §67.4) runs Layer 3 between every switch. IP has a
TTL, ECMP uses every path simultaneously rather than blocking all but one, and there
is no spanning tree because there is no bridged topology to loop.

This is exactly what Perlman argued for decades, and it is worth noting that the
industry's eventual answer to her algorithm's limitations was to adopt her other
recommendation. She also proposed **TRILL** — routing *inside* the Layer 2 domain,
with a hop count added — which was standardised, saw modest deployment, and was
largely overtaken by VXLAN (Chapter 67 §67.2), which achieves a similar result by
tunnelling over an ordinary routed network.

Campus access networks still bridge, still run STP, and still need the six items
above.

## What breaks here

**A workstation waits 30 seconds for an address.** No PortFast.

**An entire building down after someone plugged in a switch.** PortFast without BPDU
Guard.

**The root bridge is an access switch.** Priorities never set.

**A blocked port started forwarding and caused a storm.** Unidirectional link. Loop
Guard or UDLD.

**MSTP switches not forming one region.** Name, revision or VLAN mapping mismatch.
Compare the configurations character by character.

**Convergence is slow despite RSTP.** Check duplex. A half-duplex port is treated as a
shared link and reverts to timers.

**err-disabled ports after a change.** BPDU Guard did its job. Find what was plugged
in before re-enabling; `errdisable recovery` will simply re-enable it into the same
loop.

> **Network+ note.** Objective 2.3 expects STP and its enhancements; objective 4.3
> expects BPDU Guard and Root Guard as hardening. Over-learn: **RSTP converges in
> under a second, classic STP in 30–50**; **PortFast + BPDU Guard are a pair**;
> **BPDU Guard = no BPDUs allowed, Root Guard = no becoming root**; **Loop Guard
> handles unidirectional links**; **MSTP maps many VLANs to few instances**.
