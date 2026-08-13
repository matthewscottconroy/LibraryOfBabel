# 19.2 Perlman's Algorhyme

In 1984 Radia Perlman was working at Digital Equipment Corporation. DEC wanted to sell
bridges, and bridges had the problem of §19.1. Her manager asked her to solve it. She
was given, by her own account, **about a week**.

She produced the algorithm in a day, spent the rest of the week writing the
specification, and — because the specification needed an abstract and she found
abstracts tedious — wrote a poem instead.

> I think that I shall never see
> A graph more lovely than a tree.
>
> A tree whose crucial property
> Is loop-free connectivity.
>
> A tree which must be sure to span
> So packets can reach every LAN.
>
> First the Root must be selected.
> By ID it is elected.
>
> Least cost paths from Root are traced.
> In the tree these paths are placed.
>
> A mesh is made by folks like me
> Then bridges find a spanning tree.
>
> — Radia Perlman, *Algorhyme*

It parodies Joyce Kilmer's *Trees*, and it is a **complete specification of the
algorithm**. Every operative step is in it: elect a root by identifier, trace
least-cost paths from that root, place those paths in the tree, and the result spans
every LAN without loops.

Reading the poem before the mechanism is the best possible orientation, because the
mechanism is only these six couplets made precise.

## The problem, stated formally

A network of switches and links is a **graph**. Loops are **cycles**. A connected
graph with no cycles is a **tree**, and a tree that includes every vertex is a
**spanning tree**.

So: **find a spanning tree of the switch topology, and block every link not in it.**

The constraints make it hard:

- **No switch can see the topology.** Each knows only its own ports and what arrives
  on them.
- **There is no central coordinator.** Nothing has the global view.
- **It must converge from any starting state**, and re-converge after any change.
- **It must be automatic.** Perlman's design goal, as with transparent bridging
  (Chapter 17), was that it work with no configuration at all.

This is a distributed graph algorithm running on devices that individually understand
almost nothing. That it fits in a page is the achievement.

## The algorithm

**Step 1 — Elect a root bridge.**

Every switch has a **bridge ID**: a 2-byte configurable **priority** followed by its
6-byte MAC address.

```
   ┌─────────────┬───────────────────────────┐
   │  priority   │       MAC address         │
   │  (2 bytes)  │        (6 bytes)          │
   └─────────────┴───────────────────────────┘
       32768                 unique
      default
```

**Lowest bridge ID wins.** Priority is compared first; the MAC address breaks ties.
Since MAC addresses are unique, **the election always terminates with exactly one
winner** — no negotiation, no deadlock. Every switch begins by claiming to be root and
concedes on hearing a better claim.

The default priority of 32768 means that, unconfigured, the switch with the lowest MAC
address wins. **This is almost always the wrong switch** — MAC addresses correlate
with manufacturing date, so the oldest, slowest, most peripheral device tends to win.
§19.3 returns to this; it is the most common STP misconfiguration in existence, and it
is a misconfiguration by omission.

**Step 2 — Each non-root switch picks a root port.**

The one port with the **lowest cost to the root**. Cost accumulates along the path,
with per-link costs set by speed:

| Speed | Cost (802.1D-1998) | Cost (802.1t / RSTP) |
|---|---|---|
| 10 Mb/s | 100 | 2,000,000 |
| 100 Mb/s | 19 | 200,000 |
| 1 Gb/s | 4 | 20,000 |
| 10 Gb/s | 2 | 2,000 |
| 100 Gb/s | — | 200 |

The original values were chosen so 10 Mb/s cost 100 and everything else scaled
downward — which ran out of room above 10 Gb/s, hence the revised 32-bit scale. **Cost
is added on receipt**, so a switch's cost to root is the advertised cost of the sender
plus its own port's cost.

**Step 3 — Each segment picks a designated port.**

For every link, exactly one port is the **designated port** — the one on the switch
with the lowest cost to root on that segment. That port forwards traffic onto the
segment. All root ports and designated ports **forward**.

**Step 4 — Everything else blocks.**

Any port that is neither a root port nor a designated port is **blocked**: it receives
BPDUs but forwards no data frames.

The blocked ports are exactly the links that would create cycles. Remove them and what
remains is a tree.

## The tie-breakers, in order

When two paths have equal cost — which is common in symmetric designs — the comparison
proceeds down a fixed list:

1. **Lowest root bridge ID** (who is root at all)
2. **Lowest path cost to root**
3. **Lowest sender bridge ID**
4. **Lowest sender port ID** (priority, then port number)
5. **Lowest receiver port ID**

Deterministic at every stage, so every switch reaches the same conclusion
independently. **No agreement protocol is needed because there is nothing to agree
about** — given the same inputs, the comparison yields the same answer everywhere.
That is the elegance: consensus by identical computation rather than by negotiation.

## BPDUs

Switches exchange **Bridge Protocol Data Units** — small frames sent to the multicast
address `01:80:c2:00:00:00` every **2 seconds** by default.

A configuration BPDU carries:

| Field | Meaning |
|---|---|
| Root bridge ID | who I think the root is |
| Root path cost | what it costs *me* to reach it |
| Sender bridge ID | who I am |
| Port ID | which of my ports this left by |
| Message age, Max age, Hello time, Forward delay | the timers |
| Flags | topology change, and RSTP's proposal/agreement |

**A switch compares every received BPDU with its own best and keeps the better.** That
is the entire protocol logic. Superior information propagates outward from the root;
inferior information dies where it is received.

Note that BPDUs go to a multicast address in the reserved `01:80:c2:00:00:0x` range,
which conforming bridges **never forward**. This is essential — BPDUs must be
processed hop by hop, not flooded, or the cost arithmetic would be meaningless.

## A worked example

Three switches, fully meshed with 1 Gb/s links (cost 4 each), all at default priority
32768:

```
              SW-A  (MAC …:11)
              /            \
         4  /                \  4
           /                  \
       SW-B ────────4───────── SW-C
    (MAC …:22)              (MAC …:33)
```

**Root election.** All priorities equal at 32768, so the lowest MAC address wins:
**SW-A is root**.

**SW-A's ports.** Both are designated (the root's ports always are). Both forward.

**SW-B.** Two paths to root: directly to A (cost 4), or via C (cost 4 + 4 = 8). The
direct port is the **root port**. Forwards.

**SW-C.** Same: the direct port to A is its **root port**. Forwards.

**The B–C link.** Neither end is a root port, so one end must be designated and one
must block. Both B and C have cost 4 to root — a tie — so tie-breaker 3 applies:
lowest sender bridge ID. B (…:22) beats C (…:33), so **B's port is designated and C's
port blocks**.

**Result:**

```
              SW-A  (root)
              /            \
        fwd /                \  fwd
           /                  \
       SW-B ─────╳ BLOCKED ──── SW-C
```

A tree. B reaches C via A — a longer path than the direct link, which is the price of
loop freedom. If the A–C link fails, C's blocked port becomes its root port and the
direct link carries the traffic.

## The timers, and why convergence was slow

Classic 802.1D moves a port through states, and it takes its time:

| State | Duration | Learns MACs? | Forwards? |
|---|---|---|---|
| Blocking | — | no | no |
| Listening | 15 s (forward delay) | no | no |
| Learning | 15 s (forward delay) | **yes** | no |
| Forwarding | — | yes | yes |

**A port coming up takes 30 seconds to forward.** From a topology change requiring
re-election, up to **50 seconds** (max age 20 + two forward delays).

The delays are not arbitrary. Perlman's constraint was that **a transient loop is
catastrophic and a transient outage is merely annoying** (§19.1 explains why), so the
algorithm waits long enough to be certain that stale information has drained from the
network before committing a port to forwarding. Max age of 20 seconds allows a BPDU to
traverse a network of diameter 7 with margin.

In 1985, with terminal sessions and file shares, 30–50 seconds was acceptable. By 1998
it was not, and §19.3 covers what replaced it.

## What breaks here

**Everything works, but a workstation takes 30 seconds to get a DHCP address.** The
access port is running the full listening/learning cycle. **PortFast**/edge-port
configuration is the fix, and it is what §19.3 covers.

**The root bridge is a switch in a cupboard.** Nobody configured priority, so the
lowest MAC address won. Traffic between two core switches may now traverse an access
switch.

**Blocked ports on links you expected to use.** That is the algorithm working. If the
*wrong* links are blocked, adjust cost or priority rather than disabling STP.

**Topology recalculating repeatedly.** A flapping link, or a duplex mismatch causing
BPDU loss. Persistent recalculation is a fault, not normal behaviour.

> **Network+ note.** Objective 2.3 expects STP's purpose, root bridge election, port
> roles and states. Over-learn: **lowest bridge ID wins, priority then MAC**;
> **priority default 32768**; **root port = lowest cost to root**; **one designated
> port per segment**; **everything else blocks**; **BPDUs every 2 seconds**;
> **30 seconds from blocking to forwarding**. And be able to work an election from a
> diagram — it is a standard exam item.
