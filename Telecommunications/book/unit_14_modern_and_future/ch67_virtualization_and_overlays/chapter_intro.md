# Chapter 67 — Virtualization and Overlays

Around 2010, data centre operators noticed that their traffic had changed direction.

The traditional pattern was **north–south**: a user outside the data centre requests
something, a server inside responds, and traffic flows in and out through the top of a
hierarchical design. The three-tier access/distribution/core model of Chapter 11 §11.4
is optimised for exactly this, with capacity concentrated toward the core.

What replaced it was **east–west**: a single user request arriving at a web tier
triggers calls to a dozen microservices, each of which queries a database, a cache and
an authentication service, generating twenty or fifty internal exchanges before a
single byte returns to the user. The traffic between servers now dwarfs the traffic to
users, frequently by an order of magnitude.

A three-tier design serves this badly. Two servers in adjacent racks may have to
traverse access, distribution and core switches to reach each other — three hops up,
three down — while spanning tree (Chapter 19) blocks half the available links to
prevent loops. The topology was optimised for a traffic pattern that no longer existed.

Everything in this chapter descends from that observation.

## Leaf–spine

§67.4 covers the replacement, and its logic is straightforward once the problem is
stated.

Every **leaf** switch (top of rack) connects to every **spine** switch. Nothing
connects leaf to leaf or spine to spine. The consequence: **every server is exactly
two hops from every other server** — up to a spine, down to a leaf — so latency is
uniform and predictable regardless of which racks are involved.

The critical design change is that the fabric is **routed rather than switched**.
Layer 3 all the way to the leaf, with **equal-cost multipath** distributing flows
across every spine. Spanning tree does not run, no links are blocked, and all of the
purchased capacity is in use — which is the direct answer to the three-tier design's
worst property.

Scaling is by addition rather than by replacement: more capacity means more spines,
more servers means more leaves, and the fabric grows without redesign. §67.4 covers the
oversubscription arithmetic that determines how many uplinks a leaf needs.

## The problem this creates

Routing to the leaf solves the traffic problem and breaks something applications
depend on: **virtual machines expect to move between hosts without changing IP
address.** Live migration, high availability and clustering all assume the machine
stays in the same subnet. But if every rack is its own routed subnet, moving a VM to
another rack means renumbering it, which defeats the purpose.

So we need a Layer 2 segment that spans racks which are, at the physical level,
separated by routing. That is what an **overlay** provides.

## VXLAN

§67.2 covers the mechanism, and it is the tunnelling idea of Chapter 61 applied to
Ethernet: take the frame, wrap it in UDP, send it across the routed fabric, unwrap at
the far end.

```
[ Outer Ethernet ][ Outer IP ][ UDP :4789 ][ VXLAN hdr ][ Original Ethernet frame ]
```

The endpoints that wrap and unwrap are **VTEPs**, typically the leaf switches or the
hypervisors' virtual switches. From the fabric's point of view this is ordinary routed
UDP traffic, ECMP-hashed across the spines like anything else. From the virtual
machine's point of view it is a flat Ethernet segment.

The field that matters most is the **VNI** — the VXLAN Network Identifier — which is
**24 bits**, giving 16,777,216 segments.

Compare 802.1Q's 12 bits and 4,094 usable VLANs (Chapter 20 §20.2). For a single
enterprise, 4,094 is generous. For a cloud provider hosting tens of thousands of
customers, each wanting several isolated networks, it is a hard wall that was reached
in practice, and the 24-bit VNI is the direct response. This is a clean example of a
design decision that was correct for its era being invalidated purely by a change in
scale.

**GENEVE** (RFC 8926) is the successor, differing chiefly in having extensible
type-length-value options rather than VXLAN's fixed header — the recognition, learned
from VXLAN and its several competitors, that the encapsulation should be able to carry
metadata nobody has thought of yet.

## EVPN, and why it mattered

§67.3 covers the part that turned overlays from a technology into a deployable
architecture, and the story is instructive.

Original VXLAN used **flood-and-learn**: to find where a MAC address lived, flood to
all VTEPs (via IP multicast) and learn from the responses. This is Chapter 17's switch
algorithm, transposed onto a fabric — and it inherits every one of its scaling
problems, plus a dependency on multicast in the underlay, which many operators did not
want to run.

**EVPN** replaces this with a control plane: BGP (Chapter 32) carries MAC and IP
reachability information between VTEPs, so each one *knows* where every address lives
rather than discovering it by flooding.

The pattern is exactly Chapter 31's distance-vector-versus-link-state argument. Learn
by observation and flooding, or distribute knowledge explicitly? The same answer wins
for the same reasons: explicit distribution scales, converges faster, and does not
depend on flooding a network that is expensive to flood.

Using BGP for this surprises people who learned it as an inter-domain protocol
(Chapter 32). The justification is that BGP is the industry's most operationally
mature mechanism for distributing arbitrary reachability information with policy, at
scale, and reusing it was cheaper than inventing something. It is now used inside data
centres far more than between autonomous systems.

## Where the virtual switches are

§67.1 covers the layer below all of this. Every hypervisor contains a **virtual
switch** — Open vSwitch, VMware's vDS, Hyper-V's — forwarding between virtual machines
on the same host without any frame reaching a physical wire. In a well-consolidated
data centre, a substantial fraction of all switching happens here, in software, on
devices that the network team may not administer and may not monitor.

Container networking adds another layer: Linux network namespaces, veth pairs, bridges,
and the **CNI** plugin model, with Kubernetes' requirement that every pod get a
routable address and reach every other pod without NAT. §67.1 covers how the common
plugins satisfy that — overlay, routed, or eBPF-based — and the point worth carrying is
that these are the same mechanisms from Units IV through VII, implemented in software
and configured by a scheduler rather than a person.

## By the end you will be able to

- Explain the east–west traffic shift and why three-tier designs serve it badly.
- Explain leaf–spine and compute oversubscription for a stated design.
- Explain why the fabric is routed and what that eliminates.
- Explain VXLAN encapsulation field by field and compute the overhead.
- Explain why the VNI is 24 bits, in terms of a specific scaling failure.
- Explain what EVPN replaced and connect it to the distance-vector/link-state argument.
- Describe how container networking satisfies the pod-addressing model.
