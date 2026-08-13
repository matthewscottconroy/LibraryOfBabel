# 38.3 CUBIC, BBR and ECN

§38.2 ended with a number: to fill a 10 Gb/s path at 100 ms RTT, classic TCP needs a loss
rate below **2 × 10⁻¹⁰** — one loss in five billion packets — and would take **over an
hour** to recover from a single one.

**Reno cannot fill a modern long fat path.** This section is what replaced it, and the
three approaches are genuinely different in kind.

## CUBIC — a better curve

**Sangtae Ha, Injong Rhee and Lisong Xu, 2008.** **The default in Linux since 2006 and in
Windows since 2019**, so it carries the majority of the world's TCP traffic.

**The idea: make window growth a function of *time since the last loss*, not of round
trips.**

$$W(t) = C(t - K)^3 + W_{\max}$$

where *W*ₘₐₓ is the window when loss occurred, *K* is a scaling constant, and *t* is the
time since.

**A cubic function**, and its shape is the whole design:

```
   cwnd
     │            ╱
     │          ╱      ← probing upward, fast
     │        ╱
  Wmax ─ ─ ──────── ─ ─ ─ ─ ─ ─ ─ ─
     │  ╱  ▔▔▔▔▔        ← flat near the previous maximum
     │ ╱                   (cautious where loss last happened)
     │╱  ← fast recovery back toward Wmax
     └────────────────────────────▶ time since loss
```

**Three regions:**

**Below *W*ₘₐₓ — grow fast.** The path carried this much before, so return to it quickly.

**Near *W*ₘₐₓ — grow slowly.** This is where loss happened last time; probe gently and stay
here as long as it works.

**Above *W*ₘₐₓ — grow fast again.** If the flat region held without loss, the path has more
capacity than before; find it.

**Why this fixes the long-fat-pipe problem:**

**Growth is independent of RTT.** Reno adds one segment per *round trip*, so a 200 ms path
grows ten times more slowly than a 20 ms one. **CUBIC's growth depends on elapsed time**, so
a long path recovers at the same wall-clock rate as a short one.

> **CUBIC removed TCP's structural bias against long paths**, which is the single most
> important thing about it.

**It is still loss-based.** CUBIC infers congestion from loss exactly as Reno does — it
merely responds with a better curve. So it inherits §38.1's wrong inference on wireless,
**and it inherits bufferbloat**: if the bottleneck queue is large, CUBIC fills it before
seeing loss, and everyone's latency rises.

**And it is more aggressive than Reno**, deliberately. On a shared link, CUBIC takes a
larger share than Reno would — which was necessary to make it useful and which means the
two do not compete fairly.

## BBR — a different question

**Neal Cardwell, Yuchung Cheng, Van Jacobson and others at Google, 2016.** **The first
widely-deployed algorithm that does not use loss as its primary signal.**

**The observation that motivates it:**

> **Loss is not congestion. Loss is what happens *after* congestion has already filled a
> queue.**

**By the time a packet is dropped, the bottleneck buffer is full — and every packet in it
is being delayed.** A loss-based algorithm therefore operates, by construction, at the
point of maximum queueing delay.

**And with modern deep buffers (Chapter 13's bufferbloat), that delay can be seconds.**

### What BBR measures instead

**Two quantities, continuously:**

| Measured | How |
|---|---|
| **BtlBw** — bottleneck bandwidth | the **maximum** delivery rate observed recently |
| **RTprop** — round-trip propagation delay | the **minimum** RTT observed recently |

**Then it sends at exactly:**

$$\text{rate} = \text{BtlBw}, \qquad \text{in flight} = \text{BtlBw} \times \text{RTprop}$$

**Which is the bandwidth-delay product** (Chapter 37 §37.4) — **exactly enough to keep the
pipe full and no more.**

> **BBR aims for the point where the pipe is full and the queue is empty.** Loss-based
> algorithms aim for the point where the queue is full and something has been dropped.

**The two estimates cannot be measured simultaneously** — measuring maximum bandwidth
requires filling the pipe, which creates queueing and inflates the RTT; measuring minimum
RTT requires draining the queue, which means not filling the pipe. **So BBR alternates**,
cycling its sending rate to probe each in turn.

### What it buys

| | Loss-based (CUBIC) | BBR |
|---|---|---|
| Operating point | queue full | **queue empty** |
| Latency under load | **high** — bufferbloat | **low** |
| Tolerance of random loss | poor | **good** — loss is not the signal |
| Long fat pipes | adequate (CUBIC) | **good** |
| Fairness with CUBIC | — | **contested** |

**The random-loss tolerance is dramatic.** On a path with 1% random loss, CUBIC collapses
(§38.2's Mathis arithmetic); **BBR is largely unaffected**, because it never treated loss as
the signal.

**This makes BBR very effective on wireless and on long international paths**, which is
precisely why Google deployed it — YouTube throughput improved substantially, and latency
improved more.

### The controversy

**BBRv1 was not fair to CUBIC.** In shared bottlenecks with shallow buffers it took more
than its share; in some configurations it took much more. **The measurements were public
and the criticism was substantial.**

**BBRv2 and BBRv3 respond to loss and ECN as *supplementary* signals**, which improves
coexistence considerably. **v3 is the current version and is in Linux.**

**The honest position:** BBR is a genuine advance and its fairness properties are still
being argued about. **An algorithm that measures the right thing but competes badly is not
obviously better than one that measures the wrong thing and competes fairly**, and that
tension is unresolved.

## ECN — asking the network

**Explicit Congestion Notification**, RFC 3168. **The obvious idea, and its deployment
history is the interesting part.**

**Instead of inferring congestion from loss, have the router say so.**

**The mechanism** (Chapter 24 §24.2):

```
   Two bits in the IP header's ECN field:

     00  Not ECN-Capable Transport
     10  ECT(0)  ← "I support ECN"
     01  ECT(1)
     11  CE      ← "Congestion Experienced" — set by the ROUTER
```

**And two flags in the TCP header** (Chapter 37 §37.2): **ECE** (echo) and **CWR**
(congestion window reduced).

**The sequence:**

1. Sender marks packets **ECT**
2. **A congested router sets CE instead of dropping the packet**
3. The receiver sees CE and sets **ECE** in its ACKs
4. **The sender reduces its window as if it had seen loss**, and sets **CWR** to confirm

> **Congestion is signalled without anything being lost or retransmitted.**

**The benefit is real:** no retransmission, no waiting for a duplicate-ACK threshold or a
timeout, and the congestion signal arrives **one RTT earlier** than loss would have.

### Why it took twenty years

**Specified in 2001. Deployment was blocked for over a decade, and the reasons are
instructive:**

**Middleboxes cleared or mangled the bits.** Some firewalls zeroed the ECN field; some
**dropped ECN-marked packets entirely**. **A host enabling ECN found some destinations
became unreachable**, which is an unacceptable trade for a performance improvement.

**Negotiation failures.** ECN is negotiated in the handshake, and a middlebox that stripped
the negotiation left one end believing it was enabled and the other not.

**The chicken-and-egg problem.** Routers had no reason to mark until hosts negotiated;
hosts had no reason to negotiate until routers marked.

**What broke the deadlock** was **RFC 8311's fallback behaviour** — try ECN, detect
failure, fall back silently — plus large operators enabling it on their own networks where
they controlled both ends.

**It is now widely enabled**, and **L4S** (RFC 9330, 2023) is the next step: a
higher-fidelity ECN signal designed to give **consistently low latency** rather than merely
avoiding loss.

**The deployment story is Chapter 21 §21.4's ossification measured precisely: twenty years,
for a two-bit field that already existed.**

## Data centres — a different regime

**Everything above assumes an Internet path.** Data centres invert the assumptions:

| | Internet | Data centre |
|---|---|---|
| RTT | 10–300 ms | **10–100 µs** |
| Loss | 0.01–1% | **~0** |
| Bandwidth | variable | uniform and known |
| Control | none | **one operator, both ends** |

**Which permits algorithms that would be impossible on the Internet:**

**DCTCP** (RFC 8257) — uses ECN, but responds **proportionally to the fraction of packets
marked** rather than halving on any mark. **Keeps queues very short**, which matters because
a data centre's problem is latency, not throughput.

**Because the operator controls every switch and every host**, the middlebox problem that
delayed ECN for twenty years simply does not exist.

**And the incast problem** (Chapter 17 §17.4) — many servers responding simultaneously and
overwhelming one switch buffer — is the specific pathology these algorithms address.

## Choosing, in practice

```bash
# What is available and what is in use
sysctl net.ipv4.tcp_available_congestion_control
sysctl net.ipv4.tcp_congestion_control

# Change it
sysctl -w net.ipv4.tcp_congestion_control=bbr

# ECN: 0=off, 1=on, 2=accept but do not request  (2 is a common default)
sysctl net.ipv4.tcp_ecn
```

**Per connection:**

```bash
ss -tni | grep -o '^\w*'    # the algorithm is the first token of the detail line
```

**Guidance:**

| Situation | Choose |
|---|---|
| **General purpose** | **CUBIC** — the default, well-understood, fair |
| **Long paths with random loss** (wireless, international) | **BBR** |
| **Serving content at scale** | BBR, measured against your own traffic |
| **Data centre, you control both ends** | **DCTCP** with ECN |
| Competing with unknown traffic on a shared link | **CUBIC** — its fairness is understood |

**Do not change it because a blog post recommended it.** Measure. **BBR is better for some
traffic on some paths and is not universally better**, and the fairness question is real.

## What breaks here

**A long fat path achieving a small fraction of its capacity with Reno.** Use CUBIC or BBR.

**High latency under load with plenty of bandwidth.** Bufferbloat — the queue is full
because the algorithm fills it. BBR, or fix the queue (fq_codel).

**BBR taking more than its share on a shared link.** A known property of v1; use v3.

**ECN enabled and some destinations unreachable.** A middlebox dropping marked packets.
Modern stacks fall back; older ones do not.

**A data-centre algorithm used across the Internet.** DCTCP assumes ECN everywhere and
near-zero loss. It will behave badly.

> **Network+ note.** Objective 2.2 touches congestion control lightly. Over-learn:
> **CUBIC is the modern default and grows as a function of time rather than round trips**;
> **BBR measures bandwidth and RTT instead of using loss**; and **ECN lets a router signal
> congestion by marking rather than dropping.** The mechanisms are not examined in depth;
> the concepts appear in performance-troubleshooting scenarios.
