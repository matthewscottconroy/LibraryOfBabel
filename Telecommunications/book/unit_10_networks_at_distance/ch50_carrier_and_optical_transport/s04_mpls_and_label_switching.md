# 50.4 MPLS and Label Switching

**MPLS was built to solve a problem that stopped existing within five years, and it became one
of the most widely deployed technologies in carrier networking anyway.** The gap between those
two facts is the interesting part.

## The original argument

**In 1997, a router's forwarding decision was expensive.** Longest-prefix match (Chapter 29
§29.3) against a table of 50,000 routes, in software, on a general CPU — **and ATM switches,
which matched a fixed-length label in hardware, were dramatically faster.**

**The proposals — Ipsilon's IP Switching, Cisco's Tag Switching, IBM's ARIS — all made the same
move:** **attach a short fixed-length label to a packet at the network edge, and forward on
the label instead of the address.**

**A label lookup is an array index.** No longest match, no tree walk, no variable-length
comparison. **In 1997 hardware that was worth an order of magnitude.**

**The IETF standardised the idea as MPLS in 1999** — **Multi-Protocol** because it was meant to
carry anything over anything.

> **And by 2002 the argument was dead.** **TCAM-based hardware made longest-prefix match a
> single-cycle operation**, and the performance case for labels evaporated entirely.

**MPLS should have disappeared. Instead its deployment accelerated**, because two capabilities
nobody had emphasised turned out to be what carriers actually wanted.

## How it works

**Labels are 20 bits, carried in a 32-bit shim between the link header and the IP header:**

```
   ┌──────────┬──────────────────────────┬─────────┬──────────┐
   │ Ethernet │  MPLS shim (4 bytes)     │   IP    │ payload  │
   │  header  │                          │ header  │          │
   └──────────┴──────────────────────────┴─────────┴──────────┘
                        │
        ┌───────────────┴────────────────┐
        │ Label (20) │ TC (3) │ S (1) │ TTL (8) │
        └────────────────────────────────┘
          ▲            ▲        ▲        ▲
       the index   traffic   bottom   copied from IP,
                   class    of stack  so traceroute works
```

**The forwarding operations are three:**

| Operation | Where | What |
|---|---|---|
| **Push** | **ingress LER** | classify the packet, attach a label |
| **Swap** | **transit LSR** | look up the label, replace it, forward |
| **Pop** | **egress** | remove the label, forward normally |

```
   ┌─────┐  push 17  ┌─────┐ swap 17→22 ┌─────┐ swap 22→9 ┌─────┐  pop
   │ LER │ ────────▶ │ LSR │ ─────────▶ │ LSR │ ────────▶ │ LER │ ────▶ IP
   └─────┘           └─────┘            └─────┘           └─────┘
     the label is locally significant — it means something only on one link
```

**A label has meaning only on the link it traverses.** Each router allocates its own labels and
tells its upstream neighbour which label to use, **so the "path" is a chain of independent
local agreements** rather than a global identifier. **This is exactly ATM's VPI/VCI model
(Chapter 13 §13.2), and exactly Frame Relay's DLCI** — the third appearance of the same idea.

**Label distribution** happens by **LDP** (which simply follows the IGP's shortest paths) or
**RSVP-TE / SR** (which can place a path deliberately). **The distinction is the whole of the
next subsection.**

## Traffic engineering

**The first reason MPLS survived**, and it is Chapter 31's routing model's most glaring
limitation.

**An IGP sends everything along the shortest path.** If two paths exist and one is shorter,
**the longer one carries nothing — even when the shorter one is congested and the longer one is
empty.**

```
        ┌──── cost 10 ────┐          congested
   A ───┤                 ├─── D
        └──── cost 30 ────┘          idle

   IGP: everything takes the cost-10 path. Always.
   The cost-30 path is a standby that never carries anything.
```

**The classical workaround is to adjust link costs**, and it does not work: **changing a cost
to move one flow moves every flow that used that link**, and the operator ends up playing
whack-a-mole across a national network.

**MPLS traffic engineering places paths explicitly.**

$$\text{"This LSP goes A → B → C → D, and reserves 2 Gb/s along it."}$$

**With constrained shortest path first (CSPF)**, the head-end router computes a path that
satisfies bandwidth, latency, affinity ("avoid links marked as satellite") and diversity
constraints, **and RSVP-TE signals the reservation hop by hop.**

> **This is a virtual circuit** (Chapter 13 §13.2), **built on packet infrastructure, with
> admission control.** The industry spent the 1980s arguing that circuits were the wrong model
> and the 1990s reinventing them where they were needed. **Both positions were right about
> different things: statistical multiplexing for the general case, reservation for the
> engineered core.**

**Fast reroute** is traffic engineering's other product. **A pre-computed backup path is
installed in the forwarding hardware in advance**, so when a link fails **the node adjacent to
the failure switches locally in under 50 ms** — no convergence, no IGP recomputation, no
waiting for anyone else.

**Which closes the gap with SONET** (§50.2), by the same mechanism SONET used: **pre-computation
and local action.**

## L3VPN — the reason it is everywhere

**The second reason MPLS survived, and commercially the larger one.**

**Labels stack.** A packet may carry several, and only the outermost is examined at each hop.

```
   ┌──────────┬─────────┬─────────┬────────┬─────────┐
   │ Ethernet │ outer   │  inner  │   IP   │ payload │
   │          │ label   │  label  │        │         │
   └──────────┴─────────┴─────────┴────────┴─────────┘
                  │          │
          "get to PE-B"  "customer Acme,
           — the core     VRF red" — only
           swaps this     PE-B looks at this
```

**And that two-label structure solves the problem carriers had:**

**Customer A uses 10.0.0.0/8. Customer B uses 10.0.0.0/8. Customer C uses 10.0.0.0/8.**
(RFC 1918, Chapter 27 §27.1 — everybody does.) **All three want a WAN across the same
provider.**

**The mechanism:**

| Element | Role |
|---|---|
| **VRF** | **a separate routing table per customer, per PE router** |
| **CE** | customer edge — an ordinary router, knows nothing of MPLS |
| **PE** | **provider edge — holds a VRF per attached customer** |
| **P** | **provider core — knows only the outer label; has never heard of the customers** |
| **RD** — route distinguisher | **prepended to a prefix to make 10.0.0.0/8 unique per customer** |
| **RT** — route target | **controls which VRFs import which routes** — the policy knob |
| **MP-BGP** | carries the VPN routes between PEs |

> **The P routers in the core carry no customer routes at all.** They see an outer label and
> swap it. **A carrier can serve ten thousand customers with overlapping address space, and its
> core routers hold one routing table — its own.**

**That scaling property is why MPLS L3VPN became the standard enterprise WAN product for
twenty years**, and Chapter 51 §51.1 covers what displaced it.

**Route targets deserve a second look**, because they are more flexible than the usual "one VPN
per customer" picture. **A hub-and-spoke VPN, an extranet shared between two customers, or a
shared services VRF reachable from all of them** are all expressed as import/export policy on
route targets. **It is a small, well-designed mechanism**, and its expressiveness is why it
outlasted the alternatives.

**L2VPN** (VPLS, and EVPN) does the same for Ethernet — **delivering a single broadcast domain
across a carrier network** — and EVPN has since become the standard control plane for data
centre overlays too (Chapter 67 §67.3).

## Segment routing — the simplification

**MPLS's control plane accumulated: LDP, RSVP-TE, and the state each maintained on every
router along every path.**

**A network with 10,000 LSPs has 10,000 pieces of state on every transit router**, refreshed
periodically, **and RSVP-TE's soft state was a genuine operational burden at scale.**

**Segment routing removes it.** **The ingress router encodes the entire path as a stack of
labels in the packet**, and the transit routers hold no per-path state at all.

| | **RSVP-TE** | **Segment routing** |
|---|---|---|
| Per-path state in the core | **yes, on every router** | **none** |
| Signalling protocol | **RSVP** | **none — the IGP carries segment IDs** |
| Path encoded | in the network | **in the packet** |
| Scaling | limited by state | **limited by label stack depth** |

> **This is the end-to-end argument again** (Chapter 23 §23.4): **move the state to the edge and
> keep the core simple.** Segment routing is MPLS's control plane rewritten by people who
> found Chapter 23 persuasive, **and SRv6 does the same thing using IPv6 addresses as segment
> identifiers, dispensing with MPLS labels entirely.**

## What breaks here

**Traceroute showing an MPLS path as fewer hops than it is.** **Label TTL propagation is
disabled** — the carrier's choice, to hide its topology. `no mpls ip propagate-ttl`. **Normal,
and it makes customer troubleshooting harder by design.**

**An LSP that will not establish.** **The RSVP reservation cannot be satisfied** — insufficient
bandwidth, an affinity constraint excluding every path, or a link down. **The head end's CSPF
log says which.**

**Traffic taking an unexpected path.** **The TE constraints changed, or a preferred path came
back and the LSP re-optimised.** Check for a recent re-optimisation event.

**MTU problems that appear only over MPLS.** **Each label adds 4 bytes**, and a two-label VPN
packet is 8 bytes larger than the IP packet inside it. **The carrier's core MTU must exceed
the customer's, and when it does not the symptom is Chapter 24 §24.3's classic: small packets
work, large ones vanish.**

**A VPN route not appearing at a remote site.** **Route target import/export mismatch**, or the
VRF is not exporting. **This is the commonest L3VPN misconfiguration by a wide margin.**

**Two customers seeing each other's routes.** **A route target configured on the wrong VRF.**
Serious, and it is a configuration error rather than a protocol failure.

**Fast reroute not protecting.** **The backup path shares a link, a node or an optical span
with the primary** — §50.3's shared risk again. **Node protection and link protection are
different things and must both be configured.**

> **Network+ note.** Objective 2.1 covers MPLS. Over-learn: **MPLS forwards on labels rather
> than IP addresses**; **labels are pushed at the edge, swapped in the core and popped at
> egress**; **it is called multi-protocol because it can carry any payload**; and **MPLS is
> commonly used to provide carrier VPN services.** The label-versus-address distinction is the
> examinable idea.
