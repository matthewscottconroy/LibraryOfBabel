# Chapter 19 — Important Concepts

Why loops are catastrophic *(§19.1)* — Three properties, all required: an
Ethernet frame has no TTL, so a looping frame circulates forever; flooding
multiplies, so the population grows exponentially rather than merely persisting; and
a switch has no memory of frames seen, so it cannot detect the repetition. The
result is collapse in under a second from a single frame.

A frame has no hop count *(§19.1)* — Chapter 15's header has destination, source
and type, and nothing else. IP added a TTL for precisely this reason. Ethernet was
designed for a single cable where a loop was physically impossible.

**The three failures** *(§19.1)* — (1) **broadcast storm**, exponential replication to
saturation; (2) **MAC flapping**, the table rewritten thousands of times per second as
the same source arrives on multiple ports, which breaks even unicast forwarding;
(3) **multiple frame delivery**, which higher layers do not expect.

`MACFLAP_NOTIF` means a loop *(§19.1)* — Until proven otherwise. The most
diagnostic log line in this chapter.

**The operations-desk signature** *(§19.1)* — Everything down at once across the whole
broadcast domain, switch CPU at 100%, **management unreachable**, all LEDs solid, and
no recovery without physical intervention. A loop has no self-limiting mechanism,
which distinguishes it in kind from congestion. It is also the argument for a console
cable and out-of-band management.

How loops actually happen *(§19.1)* — Overwhelmingly, a user plugging both ends
of a patch cable into the wall. No malice, no expertise. Which is why loop
protection goes on every access port rather than where loops seem plausible.

The two structural answers *(§19.1)* — Never build loops (requires perfect
discipline forever, and forfeits redundancy), or build loops and disable them
logically. The second requires switches to compute a global property of a topology
none of them can see, from neighbour messages alone.

**Algorhyme** *(§19.2)* — Perlman's poem, written in 1984 because the specification
needed an abstract. It is a complete statement of the algorithm: elect a root by
ID, trace least-cost paths from it, place those in the tree, span every LAN.

**The formal problem** *(§19.2)* — Find a **spanning tree** of the switch graph and
block every link not in it. Hard because no switch sees the topology, there is no
coordinator, it must converge from any state, and it must need no configuration.

**Bridge ID** *(§19.2)* — 2-byte **priority** (default **32768**) + 6-byte MAC.
**Lowest wins.** Uniqueness of MAC addresses guarantees the election terminates with
exactly one winner.

The default root is the wrong switch *(§19.2)* — MAC addresses correlate with
manufacturing date, so the unconfigured election reliably elects the oldest, most
peripheral device. The most common STP misconfiguration is one of omission.

**Port roles** *(§19.2)* — **Root port**: lowest cost to root, one per non-root
switch. **Designated port**: one per segment, on the switch with lowest cost to root.
**Everything else blocks** — and those are exactly the links that would form cycles.

**Path cost** *(§19.2)* — Speed-derived and cumulative, **added on receipt**. Original
scale: 100/19/4/2 for 10M/100M/1G/10G. Revised 802.1t scale for higher rates.

Consensus by identical computation *(§19.2)* — Five deterministic tie-breakers
(root ID, path cost, sender bridge ID, sender port ID, receiver port ID) mean every
switch reaches the same conclusion independently. No agreement protocol is needed
because there is nothing to negotiate.

**BPDUs** *(§19.2)* — Sent to `01:80:c2:00:00:00` every **2 seconds**. A switch keeps
the better of its own best and each received BPDU; superior information propagates,
inferior dies where received. The address is in the reserved range that bridges
**never forward**, which is essential — BPDUs must be processed hop by hop.

**Classic timers** *(§19.2)* — Blocking → Listening (15 s) → Learning (15 s) →
Forwarding. 30 seconds for a port to come up; up to 50 for a topology change. The
delay is deliberate: a transient loop is catastrophic and a transient outage is
annoying, so the algorithm waits for stale information to drain.

**RSTP (802.1w)** *(§19.3)* — Same tree, sub-second convergence. Three changes:
every switch originates BPDUs (so three missed hellos = 6 s means failure, not
20); **proposal/agreement handshake** on point-to-point links; **link types matter**.

Why proposal/agreement is safe *(§19.3)* — The receiver blocks its other
designated ports before agreeing, so no loop can form. The safety property is
established by construction rather than by waiting — classic STP's timers were only
a proxy for the same condition.

Alternate and Backup roles *(§19.3)* — RSTP's addition: the switch has already
identified its fallback and promotes it **immediately** on failure. Precomputing the
backup is why failover is fast; the same idea appears as loop-free alternates in
routing.

Duplex mismatch disables rapid convergence *(§19.3)* — Half duplex → classified as
a shared link → RSTP silently reverts to 802.1D timers. One of several ways a duplex
mismatch produces a symptom that looks nothing like a duplex problem.

**PVST+ vs MSTP** *(§19.3)* — One tree per VLAN gives full flexibility and does not
scale (500 VLANs, 500 instances). MSTP maps many VLANs onto a few instances,
giving the load-balancing benefit at a fraction of the cost. MSTP's trap: switches
share a region only if name, revision and the complete VLAN mapping match exactly.

The six things to configure *(§19.3)* —
1. Set the root deliberately (and a secondary). Never the MAC lottery.
2. **PortFast** on access ports — removes the 30-second wait.
3. **BPDU Guard** on every PortFast port — a BPDU on an edge port shuts it down.
   PortFast without BPDU Guard is a loop waiting to happen; they are a pair.
4. **Root Guard** downstream — a superior BPDU blocks the port.
5. **Loop Guard** (and UDLD on fibre) — handles unidirectional links.
6. **BPDU Filter** — know it, and be careful; the global form removes all protection.

BPDU Guard vs Root Guard *(§19.3)* — *No BPDUs at all here* versus *BPDUs are
fine, but you may not become root.*

**The unidirectional-link failure** *(§19.3)* — A blocked port stops hearing BPDUs and
concludes the loop is gone, so it forwards — creating the loop. The safe
interpretation of silence is "something is broken", not "the path is clear."

**Route the fabric** *(§19.3)* — Leaf-spine runs Layer 3 between switches: IP has a
TTL, ECMP uses **all** paths rather than blocking all but one, and there is no tree
because there is no bridged topology. Perlman's own long-standing recommendation.

**Link aggregation** *(§19.4)* — Bind several physical links into one logical
interface. STP sees one port, so there is no loop to block; all members carry traffic;
a member failure causes no STP recomputation at all. IEEE 802.3ad / 802.1AX.

**"Trunk" is ambiguous** *(§19.4)* — Aggregation for HP and Juniper; a VLAN-tagged
port for Cisco. Ask which is meant.

**LACP modes** *(§19.4)* — Active–active and active–passive form a bundle;
**passive–passive does not**. Use LACP rather than static: if one end is bundled
and the other is not, static configuration produces parallel unblocked links — a loop
— and LACP refuses to form instead. The protocol exists to catch the mistake.

**Hashing, not round-robin** *(§19.4)* — A member is chosen by hashing header fields,
which pins each conversation to one link. Round-robin would deliver frames out of
order, and TCP reads reordering as loss and collapses throughput. Ethernet's
per-conversation ordering is depended on by everything above it.

A single flow cannot exceed one member's speed *(§19.4)* — Four bundled 1 Gb/s
links give one transfer **1 Gb/s**, not 4. The most common disappointment in
aggregation, and it is the design working correctly. Mitigations: parallel connections,
an L4-port hash, or a faster single link.

**Polarisation** *(§19.4)* — Uneven traffic through a narrow hash drives most traffic
down one member. Visible only in per-member counters; fixed by more hash inputs.

**Multi-chassis aggregation** *(§19.4)* — vPC / MLAG / VSS / MC-LAG let a bundle
terminate on two physical switches presenting as one, so a server survives the loss
of an entire switch — and maintenance can happen during business hours, which matters
more operationally than the availability arithmetic suggests.

**They are complementary** *(§19.4)* — Aggregate where there are parallel links between
two devices; leave STP enabled to catch what you did not anticipate, which is where
the outages come from.
