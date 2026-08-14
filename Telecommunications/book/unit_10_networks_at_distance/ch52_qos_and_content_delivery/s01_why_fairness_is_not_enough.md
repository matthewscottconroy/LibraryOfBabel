# 52.1 Why Fairness Is Not Enough

A router with no policy is not neutral. It has a policy, and the policy is "whoever sends
most, gets most."

## The mechanism of the bad outcome

A FIFO queue serves packets in arrival order. Under congestion, the share each flow
receives is proportional to the rate at which it offers packets.

Consider a 10 Mb/s link with three flows:

| Flow | Behaviour | What it needs |
|---|---|---|
| **Backup** | **sends continuously, as fast as TCP allows** | **finish sometime tonight** |
| **Voice call** | **one 200-byte packet every 20 ms — 50 pps** | **latency under 150 ms, jitter under 30** |
| Web page | bursts of a few hundred KB, then idle | responsiveness |

**The backup fills the queue.** TCP's congestion control (Chapter 38) drives it to keep the
bottleneck buffer occupied — that is what "fully utilising the link" means — and so at any
moment the queue holds a substantial number of the backup's 1,500-byte packets.

Now the voice packet arrives and joins the back of that queue.

$$\text{one 1500-byte packet at 10 Mb/s} = \frac{1500 \times 8}{10^7} = 1.2 \text{ ms}$$

| Packets already queued | Delay added to the voice packet |
|---|---|
| 10 | **12 ms** |
| **50** | **60 ms** |
| 100 | **120 ms** |
| 250 | **300 ms — the call is unusable** |

> **The voice call did nothing wrong and receives the worst service.** It sends least, so it
> has least influence over the queue, and it is the one application for which the queue is
> fatal.

And the jitter is worse than the delay. The queue depth varies with the backup's sawtooth,
so successive voice packets experience different delays — which is exactly what a jitter
buffer must absorb, and what makes speech sound broken (Chapter 3 §3.3).

## What each application actually needs

The requirements differ by orders of magnitude, and treating them identically cannot be
right.

| Application | Bandwidth | **Latency** | **Jitter** | **Loss** |
|---|---|---|---|---|
| **Voice** | **~90 kb/s** | **< 150 ms one way** | **< 30 ms** | **< 1%** |
| **Video call** | 1.5–4 Mb/s | **< 150 ms** | **< 30 ms** | **< 0.5%** |
| Streaming video | 3–25 Mb/s | **irrelevant** (buffered) | irrelevant | low, retransmitted |
| Interactive SSH | **trivial** | **< 100 ms perceptible** | some | low |
| Web | bursty | **< 200 ms feels instant** | tolerant | tolerant |
| **File transfer / backup** | **all of it** | **irrelevant** | **irrelevant** | **tolerant** |
| Industrial control | trivial | **< 10 ms, sometimes** | **very tight** | **near zero** |

**Two observations from that table.**

**Bandwidth and sensitivity are inversely related.** The applications that need the least
bandwidth need the most careful treatment, and the ones that will consume everything you give
them care least about how it arrives.

**Which is why prioritisation is cheap.** Giving voice absolute priority costs the backup
almost nothing — 90 kb/s out of 10 Mb/s is under 1% — and it transforms the voice call.

> **QoS works because the traffic that needs protection is small.** If real-time traffic were
> 80% of the load, no scheduling policy would help.

## The sentence

> **QoS does not create bandwidth. It decides who suffers.**

This is the most important idea in the chapter, and it is routinely ignored.

If a link is persistently oversubscribed, QoS lets you choose which traffic degrades. That
is genuinely valuable. It is much better that the backup slows than that every telephone call
becomes unusable.

What it is not is a capacity tool.

| Situation | QoS |
|---|---|
| **Occasional congestion, mixed traffic** | **exactly right** |
| **Persistent congestion, everything important** | **buys you a choice, not a solution** |
| **Link too small for the offered load** | **a more sophisticated description of the problem** |
| No congestion at all | **pure configuration overhead** |

An engineer who deploys QoS to fix a link that is simply too small has not fixed anything.
They have arranged for the failure to be distributed according to policy, which is better
than arbitrary — and it is not more capacity.

## Where QoS belongs

The corollary, and it determines every deployment decision:

> **QoS matters where congestion happens. Nowhere else.**

And congestion happens where the rate changes downwards:

```
   ┌──────────┐  1 Gb/s  ┌────────┐  20 Mb/s  ┌─────────┐
   │  LAN     │─────────▶│ Router │──────────▶│  WAN    │
   └──────────┘          └────────┘           └─────────┘
                              ▲
                    THE bottleneck. Queue forms here.
                    This is where QoS does something.
```

| Location | Congestion? | QoS? |
|---|---|---|
| **WAN edge — LAN to a slower circuit** | **yes, constantly** | **essential** |
| **Access switch uplink, if oversubscribed** | sometimes | **worth configuring** |
| **Wireless — the medium is shared** | **yes** (Chapter 44 §44.2) | **yes, and WMM does it** |
| Data centre fabric, non-blocking | **rarely** | **almost never worth it** |
| **A 10 Gb/s link carrying 1 Gb/s** | **no** | **no. Configuring it achieves nothing.** |

The commonest mistake in QoS deployment is configuring it everywhere. It adds
configuration, it adds a thing to get wrong, and on an uncongested link it does exactly
nothing — because a queue that never has more than one packet in it cannot be scheduled.

The second commonest mistake is configuring it in only one direction. The bottleneck for
inbound traffic is at the far end of the circuit, in the carrier's equipment, where your
policy does not apply. §52.3's shaping discussion is the partial answer, and the full answer
is that you cannot control inbound congestion from the receiving end — you can only ask the
senders to slow down, which is what TCP does anyway.

## The three service models

Worth knowing, because the first two names appear in exams and the third describes reality.

| Model | Mechanism | Status |
|---|---|---|
| **Best effort** | **no differentiation** | **the Internet's default** |
| **IntServ** | **per-flow reservation via RSVP** | **does not scale; effectively dead** |
| **DiffServ** | **per-class marking, aggregate treatment** | **what everyone actually uses** |

**IntServ deserves its two sentences.** Every router along a path holds state for every
flow and reserves resources for it. It gives genuine guarantees, and a core router carrying
a million flows cannot hold a million reservations — the state does not scale, and this is
Chapter 23 §23.4's argument arriving as an engineering failure rather than a principle.

**DiffServ's insight is to stop tracking flows.** Mark the packet with a class; each router
treats classes according to a local policy; no per-flow state anywhere. It gives weaker
guarantees and it scales, and §52.2 is entirely about it.

> This is the same trade as Chapter 50 §50.4's segment routing versus RSVP-TE, and the
> same trade as stateless versus stateful firewalls: per-flow state gives precision and does
> not scale; aggregate treatment scales and is approximate. The Internet has chosen the
> second answer every time.

## What breaks here

A voice call breaking up while the link shows 60% utilisation. **Averages hide bursts.**
A five-minute average of 60% is entirely consistent with the queue being full for hundreds of
milliseconds at a time. Measure at one-second granularity, or measure the queue directly.

**QoS configured and nothing improved.** Either the link is not the bottleneck, or the
markings are not being honoured, or the congestion is inbound and your policy applies
outbound.

QoS configured on a 10 Gb/s uplink carrying 400 Mb/s. It is doing nothing. Remove it, or
accept it as documentation of intent.

**Everything marked as high priority.** **Then nothing is.** The most common outcome of a QoS
project without a trust boundary (§52.2), and it produces a network that is more complex and
behaves identically.

Voice fine on the LAN and poor over the WAN. The WAN is where the rate steps down.
Expected, and it tells you exactly where to configure.

**Adding bandwidth not fixing a latency problem.** The problem was queueing or propagation,
not capacity. Chapter 66 §66.1 separates these properly; the habit worth forming now is to
ask which of the four delay components (Chapter 3 §3.1) **is actually large.**

> **Network+ note.** Objective 2.1 and 3.2 cover QoS. Over-learn: **QoS prioritises traffic
> types**; **voice and video are latency-sensitive and need priority**; QoS is applied where
> congestion occurs, typically the WAN edge; and DiffServ marks packets by class while
> IntServ reserves per flow. The "QoS does not create bandwidth" point is not examinable and
> is the one worth carrying into a career.
