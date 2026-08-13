# 34.3 Traceroute and the TTL Trick

Chapter 24 §24.4 introduced the mechanism. This section is the full treatment: the two
implementation families, how to read the output correctly, and the several ways it lies.

## The trick

**Van Jacobson, 1987.** Deliberately cause errors at a controlled distance and read the
replies.

```
   TTL=1 ──▶ dies at hop 1 ──▶ ICMP Time Exceeded ──▶ you learn hop 1's address
   TTL=2 ──▶ dies at hop 2 ──▶ ICMP Time Exceeded ──▶ hop 2
   TTL=3 ──▶ dies at hop 3 ──▶ ...
   ...
   TTL=n ──▶ reaches the destination, which replies differently ──▶ done
```

**Each hop is discovered by making a packet fail there and reading the source address of
the complaint.**

**The insight is that TTL was a safety mechanism** (Chapter 24 §24.4) and nobody had
thought to use it as a probe. It is the best example in this book of a diagnostic built
entirely from a property that already existed.

**Three probes per hop** by convention, so variation is visible.

## Knowing when to stop

The probes are identical except for TTL, so how does the tool know the destination was
reached rather than another intermediate hop?

**By the reply being a different kind of message:**

| Implementation | Probe | Destination replies |
|---|---|---|
| **Unix `traceroute`** | **UDP** to high ports (33434+) | **Port Unreachable** (type 3 code 3) |
| **Windows `tracert`** | **ICMP Echo** | **Echo Reply** (type 0) |
| **`traceroute -I`** | ICMP Echo | Echo Reply |
| **`traceroute -T`** | **TCP SYN** | **SYN-ACK or RST** |

**The Unix design is clever:** send UDP to a port nothing could plausibly be listening on,
so the destination is obliged to answer *port unreachable* — which is both an
acknowledgement of arrival and a distinct message type.

**And it is the design that fails most often**, because firewalls block unsolicited UDP to
high ports as a matter of course. **Which is why traceroute frequently shows nothing while
the network is perfect.**

## The four implementations, and when to use each

```bash
traceroute host              # Unix default: UDP. Often filtered.
traceroute -I host           # ICMP, like Windows. Often permitted.
traceroute -T -p 443 host    # TCP SYN to 443. Gets through almost anything.
tracert host                 # Windows: ICMP.
mtr host                     # Continuous, both loss and latency.
```

> **When `traceroute` shows nothing and the service works, use `-T` to the port you
> actually care about.**

**This is the single most useful thing in this section.** A firewall permitting TCP/443
and dropping UDP is entirely normal, and `traceroute -T -p 443` traces **the path your
actual traffic takes**, through the filters your actual traffic passes.

## Reading it correctly

```
$ traceroute -T -p 443 example.com
 1  192.168.1.1        1.2 ms   1.1 ms   1.1 ms
 2  10.0.0.1           8.4 ms   8.2 ms   8.9 ms
 3  * * *
 4  203.0.113.1       45.1 ms  44.8 ms  45.2 ms
 5  198.51.100.7      12.1 ms  11.9 ms  12.4 ms
 6  93.184.216.34     88.2 ms  88.1 ms  88.4 ms
```

**Five things people get wrong**, and hop 4 above demonstrates two of them.

### 1. `* * *` does not mean the packet was dropped

**It means that router did not reply.** Almost always ICMP rate limiting (§34.1) or a
policy of not generating Time Exceeded.

**Traffic passes through it perfectly.** Hop 3 above is forwarding fine — hops 4, 5 and 6
prove it, because their replies had to traverse hop 3 to reach you.

> **Stars followed by working hops mean that router is silent, not broken.**

**Only stars all the way to the destination** suggest an actual problem — and even then it
may be that the probe type is filtered.

### 2. Intermediate latency is not path latency

**Hop 4 shows 45 ms and hop 5 shows 12 ms.** That looks impossible — how can the path get
faster?

**It is not the path.** The figure is the round trip to **that router's control plane**
(Chapter 29 §29.1) — a CPU that treats generating ICMP as its lowest-priority work. Hop 4
is busy; hop 5 is not.

**Only the final hop's latency describes the path.** Intermediate figures describe how busy
each router's CPU is, which is almost never what you wanted to know.

**A high figure at one hop, with lower figures after it, is normal and means nothing.**

### 3. The reverse path is invisible

**Every measurement is a round trip**, and the return half takes its own route — which may
be entirely different (Chapter 32 §32.2's hot potato).

**So a high latency at a hop may be caused by the return path from that hop**, not by the
forward path to it. **You cannot tell from this output**, and this is why asymmetric
routing problems are so hard to diagnose from traceroute alone.

**Where possible, get a traceroute from the other end too.** Public looking glasses and
RIPE Atlas make this feasible for Internet destinations.

### 4. Load balancing scatters the path

**ECMP** (Chapter 29 §29.3) means successive probes may take different paths, so the three
probes at one hop may show **three different routers** — and the path as a whole may be a
composite that no packet ever took.

**`paris-traceroute` and `dublin-traceroute`** fix this by keeping the flow identifiers
constant across probes, so all probes hash to the same path. **Use them when the output
looks incoherent.**

### 5. MPLS hides hops

Inside an MPLS network (Chapter 51) the intermediate routers may not decrement the IP TTL
at all, so **a provider's entire backbone can appear as one hop** — or the hops appear but
report addresses from a private range that means nothing to you.

**Not a fault, and not something you can see through.**

## `mtr` — usually the better tool

```
$ mtr --report --report-cycles 100 example.com
HOST                     Loss%   Snt   Last   Avg  Best  Wrst StDev
1. 192.168.1.1            0.0%   100    1.1   1.2   1.0   3.2   0.3
2. 10.0.0.1               0.0%   100    8.4   8.5   8.1  12.1   0.6
3. 203.0.113.1           12.0%   100   45.1  44.9  43.2  61.0   2.1
4. 198.51.100.7           0.0%   100   12.1  12.3  11.8  15.2   0.4
5. 93.184.216.34          0.0%   100   88.2  88.3  87.9  91.1   0.5
```

**Continuous probing with per-hop loss and variance**, and it answers a question a single
traceroute cannot.

**Hop 3 shows 12% loss and hops 4 and 5 show none.**

> **This is not a lossy link. It is a router rate-limiting its own ICMP responses.**

**If hop 3 were genuinely dropping 12% of traffic, hops 4 and 5 would show at least 12%
too** — every packet reaching them passed through hop 3. **Loss that does not propagate
downstream is not real loss.**

**That single rule is the most valuable thing `mtr` gives you**, and it resolves the
majority of "traceroute shows packet loss" reports, which are nearly always rate limiting.

**Real loss appears at a hop and at every hop after it.**

## What traceroute is genuinely good for

Given all the caveats:

**Finding where a path stops.** The last responding hop localises the fault to that
router's onward path.

**Identifying the providers in the path.** Reverse DNS names on backbone routers usually
encode the operator and often the city — `ae-1.r02.londen12.uk.bb.example.net` is
readable once you know the convention.

**Detecting routing loops.** An alternating pair of addresses repeating is unmistakable.

**Confirming a path changed.** Compare against a baseline taken when things worked.
**Keeping such a baseline is worth the trouble** and almost nobody does.

**Measuring end-to-end latency** — from the **final** hop only.

## What breaks here

**`* * *` mid-path.** Rate limiting. **Not a fault.**

**High latency at a middle hop.** Control-plane priority. **Not a fault.**

**Traceroute showing nothing while the application works.** UDP probes filtered. Use `-T`.

**Loss at one hop and not after it.** ICMP rate limiting, not real loss.

**Three different addresses at one hop.** ECMP. Use `paris-traceroute`.

**A provider's whole network as one hop.** MPLS.

**A path that looks wrong and works.** You may be reading a composite of several ECMP
paths.

> **Network+ note.** Objective 5.5 expects `traceroute`/`tracert`. Over-learn: **it works
> by incrementing TTL and reading ICMP Time Exceeded**; **Windows `tracert` uses ICMP and
> Unix `traceroute` uses UDP by default**; **`* * *` means no reply, not no path**; and
> **intermediate latency is not path latency.** The Windows/Unix probe difference is
> examined and explains a real operational difference.
