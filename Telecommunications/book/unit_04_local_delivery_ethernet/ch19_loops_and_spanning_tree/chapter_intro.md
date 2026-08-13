# Chapter 19 — Loops and Spanning Tree

Redundancy is obviously good. If a single cable failure can isolate a building,
run two cables. This is such elementary engineering sense that it needs no
argument.

On a switched Ethernet network, doing it naively destroys the network within
seconds, and the manner of the destruction is worth understanding in detail
because it reveals something fundamental about the difference between Layer 2 and
Layer 3.

## The catastrophe

Three switches, connected in a triangle for redundancy. A host sends one broadcast
frame — an ARP request, say, of the kind Chapter 18 just introduced.

Switch A receives it and, being a broadcast, floods it out every other port: to B
and to C. Switch B receives it and floods it to C. Switch C receives it and floods
it to B. Switch B receives *that* copy and floods it to A and C. And so on.

Nothing stops. There is no mechanism that stops it, and this is the crucial point:
**an Ethernet frame has no hop count.** The IP header has a TTL field (Chapter 24
§24.4) precisely so that a routing loop causes a packet to die after a bounded
number of hops. The Ethernet header, designed in 1980 for a single shared cable
where loops were physically impossible, has nothing of the kind. There is no field
to decrement, no counter, no expiry.

So the frame circulates forever, and every circuit multiplies it. Within a second
there are millions of copies. The switches' CPUs saturate flooding them. The links
saturate carrying them. And because every copy has the same source address arriving
on different ports at different times, the MAC address tables thrash — each switch
repeatedly relearning the same address on a different port, so unicast forwarding
becomes unreliable too.

The network does not degrade. It stops, completely, in under a second, and it does
not recover on its own. Anyone who has caused a **broadcast storm** by plugging a
patch cable into two wall sockets remembers it.

## Perlman's algorithm

In 1985, Radia Perlman was working at Digital Equipment Corporation and was given
the problem: allow physical redundancy without logical loops. She solved it in
about a week, and — as she has recounted — wrote a poem about it the same day,
because the algorithm's name was *spanning tree* and she found that funny.

> *I think that I shall never see*
> *a graph more lovely than a tree.*
> *A tree whose crucial property*
> *is loop-free connectivity.*

*Algorhyme* is printed in the original paper, which is a thing that would not
survive modern peer review and is much the better for having survived this one.

The algorithm's insight is that a graph with cycles can be reduced to a **spanning
tree** — a subgraph that reaches every node with no cycles — by selectively
disabling links. The physical redundancy remains, unused; if an active link fails,
a previously blocked link is unblocked and connectivity is restored.

The mechanism, in outline:

1. Elect a **root bridge** — the switch with the lowest bridge ID, which is
   priority followed by MAC address.
2. Every other switch determines its lowest-cost path to the root; that port
   becomes its **root port**.
3. On every segment, one switch is elected to forward traffic toward the root; its
   port is the **designated port**.
4. Every remaining port is **blocked**. Blocked ports receive protocol messages but
   forward no data.

Elegant, correct, and — in its original 1990 form as IEEE 802.1D — extremely slow.
Convergence took 30 to 50 seconds, because the timers were set conservatively for
the network diameters and propagation delays of the late 1980s. Fifty seconds of
outage after every topology change was accepted for a decade because the
alternative was a broadcast storm.

## What the chapter is really about

Three things beyond the mechanism.

**Default configurations are usually wrong.** The root bridge is elected by lowest
MAC address if priorities are left at their defaults, and MAC addresses correlate
with manufacturing date, so **the oldest switch in the building becomes the root**
— which is likely to be the least capable device, quite possibly in a cupboard,
and now carrying all inter-switch traffic. Setting the root bridge explicitly is
one of the highest-value five-minute configuration tasks in enterprise networking,
and it is skipped constantly.

**Speed came from questioning an assumption.** Rapid Spanning Tree (802.1w, 2001)
converges in a few seconds rather than fifty, and it achieves this largely by
recognising which ports connect to other switches and which connect to hosts. A
port with a single host on it cannot create a loop, so it can transition to
forwarding immediately. The classic timers existed to handle a case that, in a
modern access network, almost never arises.

**Modern designs try to avoid it entirely.** Link aggregation (§19.4) makes several
physical links behave as one logical link, so there is no loop to block and *both*
links carry traffic. Multi-chassis aggregation extends this across two switches.
Data-centre fabrics (Chapter 67) abandon Layer 2 loop prevention altogether in
favour of routed underlays with equal-cost multipath. The trajectory of the field
is away from spanning tree — but it is still running, by default, on essentially
every enterprise switch shipped today, and it will still be there when you arrive.

## What this chapter does

§19.1 works the broadcast storm through in detail, including MAC table thrashing
and the absence of a TTL.

§19.2 covers the classic algorithm: bridge IDs, root election, path cost, port
roles and states, BPDUs and the timers.

§19.3 covers RSTP and MSTP, the protection features (PortFast/edge ports, BPDU
Guard, Root Guard, Loop Guard), and what to configure on a real access switch.

§19.4 covers link aggregation: LACP, hashing and why a two-link bundle does not
double a single flow's throughput, and multi-chassis aggregation.

## By the end you will be able to

- Explain why a Layer 2 loop is catastrophic and a Layer 3 loop is merely wasteful.
- Determine the root bridge, root ports, designated ports and blocked ports for a
  described topology.
- Explain the consequence of leaving bridge priorities at their defaults, and set
  them correctly.
- Choose and justify the protection features for an access port.
- Explain what link aggregation does and does not improve, including why a single
  TCP flow does not benefit.
