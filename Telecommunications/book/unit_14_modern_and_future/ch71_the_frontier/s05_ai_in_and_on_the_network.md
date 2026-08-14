# 71.5 AI in and on the Network

Two entirely different subjects share a name, and separating them is the section's first
job.

| | **AI *on* the network** | **AI *in* the network** |
|---|---|---|
| Means | **machine learning applied to operating the network** | **the network carrying AI workloads** |
| Question | **does it help?** | **what do those workloads require?** |
| Status | **modest and real** (Chapter 70 §70.4) | **the largest single force on network design today** |

The second is the one changing what networks are built, and it is discussed far less.

## AI on the network

Chapter 70 §70.4 covered this and the conclusions stand.

**What works:** anomaly detection on time series, alert correlation and deduplication, log
clustering, capacity forecasting, and — recently and genuinely — natural-language interfaces to
configuration and documentation.

**What does not:** automated root cause analysis, self-healing beyond anticipated faults,
predictive failure for anything without wear characteristics, and autonomous operation.

And the structural reasons (Chapter 70 §70.4): the training data does not exist because a
well-run network produces few labelled failures; every network is different so models transfer
poorly; the incidents that matter are novel; and the cost of a wrong action is asymmetric.

One addition belongs here, because it is recent:

> Large language models have made a specific class of task substantially easier: searching,
> summarising and explaining text. Configuration analysis, documentation, incident write-ups,
> generating a first draft of a script or a template (Chapter 70 §70.3) — **all genuinely
> faster.**

**And the honest limits of that:**

It is confident when wrong, which is the same failure mode as Chapter 58's cryptography
warning — plausible output indistinguishable from correct output — and it requires the same
response: verify rather than trust.

**It does not know your network.** It knows what networks are like, which is useful for
"explain what this configuration does" and unreliable for "what is wrong with our network."

And the verification burden shifts rather than disappears. A generated template must be
reviewed and tested (Chapter 70 §70.4) as thoroughly as a written one — and the temptation is
to review it less carefully because it looks finished.

> The correct framing is that it is a productivity tool for the text-handling parts of the
> job, which is a substantial fraction of the job, and it is not an operator.

## AI in the network: the requirement

The part that is reshaping infrastructure, and the arithmetic is unlike anything else in this
book.

A large model's training run is a distributed computation across thousands of accelerators,
and the communication pattern is the problem.

### The collective operation

Training proceeds in steps: every accelerator computes gradients on its own data, and then
*all* of them must be combined and redistributed before the next step.

> **This is an all-reduce**, and **it is a synchronous barrier.** **Every accelerator waits for
> every other.** The step time is the slowest path, not the average one.

**Which inverts every assumption in Chapter 52:**

| | **Conventional traffic** | **AI training traffic** |
|---|---|---|
| Pattern | **many small flows, statistically independent** | **a few enormous synchronised flows** |
| Statistical multiplexing | **works** (Chapter 9) | **does not — everything transmits at once** |
| Oversubscription | **3:1 is fine** (Chapter 67 §67.4) | **1:1, and sometimes better** |
| **Tail latency** | **affects some users** | **stalls every accelerator** |
| **Packet loss** | **TCP recovers** (Chapter 38) | **a retransmission stalls the barrier** |
| Congestion | **degrades throughput** | **degrades utilisation of very expensive hardware** |

> The economic driver is that the accelerators cost more than the network by a large factor,
> so a network that leaves them idle 20% of the time has wasted more money than the network
> cost. Which justifies engineering that would be absurd for any other workload.

### What that produces

| Requirement | Consequence |
|---|---|
| **No oversubscription** | **1:1 fabrics, or dedicated back-end networks** (Chapter 67 §67.4) |
| **Lossless** | **RoCE with priority flow control**, or InfiniBand |
| **Predictable, low tail latency** | **careful congestion control and load balancing** |
| **Very high bandwidth per node** | **400G or 800G per accelerator, multiple ports** |
| **Deterministic completion** | **which is §71.4's argument, in a data centre** |

And the lossless requirement is the interesting one, because it reintroduces flow control
(Chapter 66 §66.2):

> RoCE — RDMA over Converged Ethernet — requires a lossless fabric, provided by priority flow
> control (802.1Qbb). Which is the mechanism Chapter 66 §66.2 said was usually wrong —
> and it is correct here, because the traffic class is known, the fabric is dedicated, and the
> alternative is unacceptable.

Its failure modes are real and are being learned expensively: PFC storms, head-of-line
blocking, and deadlock in a fabric with cyclic buffer dependencies — all of which the
industry has rediscovered at scale over the last five years.

And the alternative is to make loss acceptable: DCQCN, HPCC and the newer congestion
control work aim to run RoCE without PFC, and the Ultra Ethernet Consortium's work is an
attempt to build an Ethernet transport designed for this pattern rather than adapted to it.

### The topology consequence

Chapter 67 §67.4's leaf–spine is being modified rather than replaced.

| | |
|---|---|
| **Rail-optimised topologies** | **accelerator $n$ in every server connects to the same switch** — so an all-reduce among the $n$th accelerators is one hop |
| **Dragonfly and other high-radix designs** | **fewer hops at very large scale** |
| **Separate front-end and back-end fabrics** | **storage and management on one, collectives on the other** |
| **Optical circuit switching** | **reconfiguring the topology to match the job** — Google's TPU fabrics do this |

> **The last is the most interesting.** A training job's communication pattern is known in
> advance, so the topology can be configured for it rather than being general-purpose —
> **which is a circuit-switched argument** (Chapter 13 §13.1) arriving in a place nobody
> expected it, for the same reason it arrives everywhere: the traffic is predictable and the
> guarantee is worth more than the flexibility.

### And in-network computation

Chapter 68 §68.3's in-network aggregation, which is the clearest current application of a
programmable pipeline.

> **An all-reduce is a many-to-one reduction followed by a one-to-many broadcast.** **Performing
> the reduction in the switches** — **each switch summing what it receives before forwarding** —
> reduces the traffic substantially and shortens the critical path.

**Which is deployed** (NVIDIA's SHARP, and equivalents), and it is the strongest available
argument that P4-style programmability has a commercial future even though the general-purpose
programmable switch did not (Chapter 68 §68.3).

## Inference, which is a different problem

Training is the pattern above. Inference is closer to ordinary application traffic — and
its network requirements are more familiar.

| | Training | Inference |
|---|---|---|
| Pattern | **synchronous collectives** | **request-response** |
| Scale | **thousands of accelerators, one job** | **many independent requests** |
| Latency | **matters as a barrier** | **matters as user experience** (Chapter 3 §3.3) |
| Location | **centralised, where the power is** | **increasingly at the edge** (Chapter 46 §46.4) |

And the inference-at-the-edge argument is the one with network consequences: placing model
serving near users (Chapter 52 §52.4's argument, applied to computation) because the latency
is user-visible and the traffic volume to a central location would be substantial.

## The infrastructure consequence

Worth stating because it is where the constraint has moved.

> The binding constraint on large AI deployments is power, not network and not compute.

A rack of accelerators dissipates 40–130 kW against a conventional rack's 5–15
(Chapter 56 §56.3) — which requires liquid cooling, different power distribution and, at scale,
a site with grid capacity that may not exist.

And the network consequence is that facilities are placed where the power is, which
increases the distance between the training infrastructure and the users — making
Chapter 50's long-haul capacity and Chapter 52's content delivery arguments relevant to a
workload that did not previously need them.

## What breaks here

Statistical multiplexing assumptions applied to a training fabric. Everything transmits at
once. 1:1, and dedicated.

PFC deployed on a general-purpose network because a training cluster needed it. **It spreads
congestion** (Chapter 66 §66.2). Separate fabrics, or per-class.

A PFC deadlock in a fabric with cyclic dependencies. A real and expensive failure mode,
and it is why the industry is trying to run RoCE without PFC.

A training job whose completion time is dominated by one slow path. **The barrier.** The tail
is the metric.

**An AI-generated configuration deployed without review.** **Confident when wrong** — Chapter 58's
warning, in a new place.

An AIOps product asked what is wrong with your network. It knows what networks are like.

A data centre site chosen for connectivity and rejected for power. **The constraint moved.**

> **Network+ note.** Beyond the syllabus. The transferable content is Chapter 9's and Chapter
> 52's: **statistical multiplexing works because flows are independent**, and a workload whose
> flows are synchronised breaks every assumption built on that. Which is the clearest
> demonstration in this book that an architecture is only as good as the traffic model it
> assumed.
