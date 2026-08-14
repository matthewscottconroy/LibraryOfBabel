# 38.2 Slow Start and AIMD

Two phases and one rule. A sender must discover a capacity it cannot measure directly, and
must share it with other senders it cannot see or talk to.

## Slow start — finding the capacity

**The name is misleading.** Slow start is **exponential**, and it is the fastest phase of a
TCP connection.

**It is "slow" only relative to the alternative it replaced: starting at the receiver's
full advertised window**, which is what pre-1988 TCP did and what caused §38.1's collapse.

**The rule:**

$$\text{cwnd} \leftarrow \text{cwnd} + \text{MSS} \quad \text{for every ACK received}$$

Which doubles the window every round trip, because a window's worth of segments
produces a window's worth of ACKs:

| RTT | `cwnd` (segments) | Bytes in flight |
|---|---|---|
| 0 | **10** (RFC 6928 initial window) | 14 KB |
| 1 | 20 | 29 KB |
| 2 | 40 | 58 KB |
| 3 | 80 | 116 KB |
| 4 | 160 | 232 KB |
| 5 | 320 | 464 KB |
| 10 | **10,240** | **14.3 MB** |

**Ten round trips to reach 14 MB in flight.** On a 100 ms path that is one second.

**The initial window matters more than it looks.** It was 1 segment originally, then 2–4,
and RFC 6928 (2013) raised it to 10 after Google measured that most web transfers were
short enough that slow start dominated their entire lifetime. **A 10-segment initial window
delivers ~14 KB in the first round trip** — enough for many complete responses — and it
measurably improved page-load times across the Internet.

**Slow start ends when:**

- **`cwnd` reaches `ssthresh`** (the slow-start threshold), or
- **loss is detected**

## Congestion avoidance — the careful phase

**Once near capacity, stop doubling.**

$$\text{cwnd} \leftarrow \text{cwnd} + \frac{\text{MSS}^2}{\text{cwnd}} \quad \text{per ACK}$$

Which works out to roughly one MSS per round trip — linear growth instead of
exponential.

```
   cwnd
     │                              ╱ (linear: +1 MSS/RTT)
     │                          ╱
     │                      ╱
     │   ssthresh ─ ─ ─ ╱─ ─ ─ ─ ─ ─ ─ ─ ─
     │              ╱
     │           ╱ (exponential: ×2/RTT)
     │        ╱
     │     ╱
     └──────────────────────────────────▶ time
       slow start │ congestion avoidance
```

**The two phases answer two different questions:** *how big is this path?* (exponential, get
there fast) and *has it got bigger?* (linear, probe gently).

## AIMD

**Additive Increase, Multiplicative Decrease.** The rule that makes it work, and the reason
is mathematical rather than intuitive.

$$\begin{aligned}
\text{no loss:} \quad &\text{cwnd} \leftarrow \text{cwnd} + 1 \ \text{(per RTT)} \\
\text{loss:} \quad &\text{cwnd} \leftarrow \text{cwnd} / 2
\end{aligned}$$

**Increase slowly, decrease sharply.**

### Why this converges to fairness

Chiu and Jain proved it in 1989, and the argument is worth seeing because the result is
not obvious.

**Consider two flows sharing a link.** Plot their windows against each other:

```
   Flow B's
   window
     │╲
     │  ╲  ← the "efficiency line": x + y = capacity
     │    ╲
     │  ╱   ╲
     │ ╱      ╲     ← the "fairness line": x = y
     │╱         ╲
     └────────────╲──────▶  Flow A's window
```

Additive increase moves the point along a 45° line — both flows gain the same amount,
so the *difference* between them is unchanged.

Multiplicative decrease moves the point toward the origin along a line through it —
both flows halve, so the *ratio* is unchanged and **the absolute difference shrinks.**

**Repeating: the difference shrinks on every decrease and never grows.** The point spirals
in toward the fairness line.

> **Additive increase preserves the difference; multiplicative decrease shrinks it.
> Iterated, the flows converge to equal shares.**

**And the converse fails.** Multiplicative increase with additive decrease **diverges** —
the flow that is ahead stays ahead. **AIMD is the only one of the four combinations that
converges**, which is why every congestion-control algorithm since uses it or an
approximation of it.

## The sawtooth

The resulting behaviour, and it is what a TCP connection looks like:

```
   cwnd
     │      ╱│    ╱│    ╱│    ╱│
     │    ╱  │  ╱  │  ╱  │  ╱  │
     │  ╱    │╱    │╱    │╱    │
     │╱      ┴     ┴     ┴     ┴     ← loss: halve
     └──────────────────────────────▶ time
```

**Grow linearly until loss, halve, repeat.**

**Two consequences worth stating:**

**Average utilisation is about 75%.** The window oscillates between *W* and *W*/2, averaging
0.75*W*. A single TCP flow cannot fully use a link — and this is by design, because the
headroom is what allows other flows to start.

**TCP needs loss.** The sawtooth requires a loss to turn the corner. **A path with zero loss
gives TCP no signal to stop growing, so it grows until it causes loss. This is why a
network with no packet loss is a network that is not being used**, and it is Chapter 24
§24.1's argument in mechanical form.

## Tahoe, Reno, NewReno

**The evolution, and the differences are examinable:**

| Version | On 3 duplicate ACKs | On timeout |
|---|---|---|
| **Tahoe** (1988) | retransmit, **`cwnd` = 1**, slow start | same |
| **Reno** (1990) | retransmit, **`cwnd` = `cwnd`/2**, congestion avoidance | `cwnd` = 1, slow start |
| **NewReno** (1996) | as Reno, **plus correct handling of multiple losses in one window** | same |

**Reno's improvement is fast recovery** (Chapter 37 §37.3): duplicate ACKs prove packets
are still flowing, so the path is lossy rather than broken — **halve rather than restart.**

NewReno fixes Reno's failure with multiple losses, where Reno exits recovery on the
first partial ACK and takes a timeout for the second loss.

A timeout is always treated as severe — `cwnd` to 1 and slow start again — because a
timeout means *nothing* got through, which is a much worse signal than a single loss.

## The Mathis equation

The formula that quantifies how badly loss hurts, and it is one of the most useful in
this book.

$$\text{throughput} \approx \frac{\text{MSS}}{\text{RTT}} \times \frac{C}{\sqrt{p}}$$

where *p* is the loss probability and *C* ≈ 1.22 for the standard AIMD sawtooth.

**Two consequences, both counter-intuitive:**

**Throughput falls with the *square root* of loss.** Doubling the loss rate does not halve
throughput — it divides it by √2 ≈ 1.41. **Which sounds gentle until you look at the
absolute numbers.**

**Throughput is inversely proportional to RTT.** A long path is penalised twice — once
because each round trip takes longer, and again because the recovery from each loss takes
longer.

### Worked

**MSS 1460 bytes, RTT 100 ms:**

| Loss rate | Throughput |
|---|---|
| 0.0001% | 142 Mb/s |
| **0.001%** | **45 Mb/s** |
| **0.01%** | **14 Mb/s** |
| **0.1%** | **4.5 Mb/s** |
| **1%** | **1.4 Mb/s** |

> **One per cent loss caps a single TCP flow at about 1.4 Mb/s on a 100 ms path, regardless
> of whether the link is 100 Mb/s or 100 Gb/s.**

**Read the top row again.** Even at one loss in a million packets, a single standard
TCP flow on a 100 ms path is limited to **142 Mb/s** — a seventh of a gigabit link. **Loss
rates that sound negligible are not.**

This is why "the link is not full so the network is fine" is wrong. A link at 10%
utilisation with 0.1% loss will not carry a fast transfer, and the loss is the reason.

**And it explains Chapter 24 §24.3's fragmentation argument concretely:** three fragments
turn 1% link loss into 3% packet loss, which by the equation cuts throughput by **√3 ≈
1.7×**.

### The high-speed problem

**Run the equation backwards.** To sustain 10 Gb/s on a 100 ms path with 1460-byte
segments:

$$p \le \left(\frac{1.22 \times \text{MSS}}{\text{RTT} \times \text{throughput}}\right)^2 \approx 2 \times 10^{-10}$$

**One loss in five billion packets.**

That is far below the error rate of real hardware, and it means **standard Reno/NewReno
cannot fill a modern long fat path at all.** Recovering from a single loss would take
**over an hour** of linear growth — about 43,000 round trips.

This is the problem CUBIC was built to solve, and it is §38.3's subject.

## Reading it live

```bash
ss -tni
```

```
cubic wscale:7,7 rto:236 rtt:35.5/2.1 mss:1448
cwnd:24 ssthresh:18 bytes_acked:14200000 retrans:0/12
```

| Field | Meaning |
|---|---|
| **`cwnd:24`** | 24 segments ≈ 34 KB in flight |
| **`ssthresh:18`** | **below `cwnd`, so this connection is in congestion avoidance** |
| `retrans:0/12` | twelve retransmissions over its life |

**`ssthresh` present and below `cwnd` means the connection has experienced loss and is in
the linear phase.** A connection still in slow start shows `ssthresh` as unset or very
large.

**Watching it over time is the instructive part:**

```bash
watch -n0.5 "ss -tni 'dst 203.0.113.10' | grep -o 'cwnd:[0-9]*'"
```

**The sawtooth, live.**

## What breaks here

A transfer that never reaches full speed on a long path. Slow start takes many round
trips; on a 300 ms path, reaching a large window takes seconds.

**Throughput capped well below the link rate with low utilisation.** Loss. Apply the Mathis
equation before blaming bandwidth.

A 10 Gb/s path achieving 100 Mb/s with a standard algorithm. Reno/NewReno cannot fill
a long fat pipe. Use CUBIC or BBR (§38.3).

**Every flow getting an equal share except the long-distance one.** RTT bias, structural.

**Utilisation stuck around 75% with a single flow.** The sawtooth. Expected.

> **Network+ note.** Objective 2.2 touches congestion control. Over-learn: **slow start
> grows the window exponentially and congestion avoidance grows it linearly**; **AIMD —
> additive increase, multiplicative decrease**; **loss halves the window**; and **a timeout
> is treated more severely than duplicate ACKs.** The Mathis equation is not examined and
> is the most useful thing here for real work.
