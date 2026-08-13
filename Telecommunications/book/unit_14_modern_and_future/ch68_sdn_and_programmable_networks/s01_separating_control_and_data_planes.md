# 68.1 Separating Control and Data Planes

**The distinction Chapter 29 §29.1 introduced, taken seriously as an architectural principle** —
and **it is the one idea in this chapter that has unambiguously won**, in forms that were not
predicted.

## The two planes

| | **Control plane** | **Data plane** (forwarding plane) |
|---|---|---|
| Decides | **where traffic should go** | **nothing** |
| Does | **runs protocols, computes tables** | **matches, and forwards** |
| Rate | **events per second** | **packets per second** |
| Implemented in | **software, on a general CPU** | **ASIC, FPGA, or optimised software** |
| Failure means | **the network stops adapting** | **the network stops carrying traffic** |

**And a third is worth naming because it matters operationally:**

**The management plane** — **configuration, monitoring and administration** — **which is what
Chapter 60 §60.4 argued should be segmented separately**, and which is a different thing from
either.

> **The distinction is not new and it is not an SDN invention.** **A router in 1990 had a
> control plane running OSPF and a data plane forwarding packets**, and the separation was
> internal to the box. **What SDN proposed was to move the control plane out of the box.**

## Why distribution was a constraint, not a goal

**The chapter's central historical point.**

> **Every router runs OSPF and computes its own shortest-path tree because there was no
> practical alternative in 1989.** **No controller could have been reliable enough, fast enough,
> or reachable enough.** **Distribution was the engineering constraint.**

**And it has real costs:**

| Cost | |
|---|---|
| **Every device must be configured** | **and the configurations must be consistent** (Chapter 55 §55.1) |
| **Consistency is achieved by convergence, not construction** | **so transient inconsistency is normal, and loops are possible during it** (Chapter 31 §31.4) |
| **Cross-device policy is expressed per device** | **"traffic from this group takes that path" becomes forty configurations** |
| **Optimisation is local** | **each router does the best it can with what it knows**, which is not what is globally best |
| **Verification is impossible in general** | **you cannot ask a distributed system "what will happen to this packet?"** |

**A controller with a complete view can compute globally optimal paths, verify policy before
applying it, and reconfigure the whole network atomically.**

> **That argument has not been refuted.** **What was wrong was the estimate of how hard the
> other parts would be.**

## The three architectures

**Not two, and the middle one is where almost everything actually is.**

```
   DISTRIBUTED                 HYBRID                    CENTRALISED
   
   ┌──────┐                 ┌────────────┐             ┌────────────┐
   │ CP+DP│                 │ Controller │             │ Controller │
   ├──────┤                 └─────┬──────┘             └─────┬──────┘
   │ CP+DP│  each device            │ policy                   │ every
   ├──────┤  runs both       ┌──────┴──────┐            forwarding
   │ CP+DP│                  │ CP+DP  CP+DP│            decision
   └──────┘                  └─────────────┘             ┌────┴───┐
                             devices still run           │ DP only│
   OSPF, BGP, STP            protocols; the              └────────┘
                             controller sets policy      OpenFlow
```

| | **Distributed** | **Hybrid** | **Centralised** |
|---|---|---|---|
| Control | **in every device** | **in devices, directed centrally** | **in the controller** |
| Failure of the controller | — | **the network keeps forwarding and stops changing** | **catastrophic, unless mitigated** |
| Scale | **proven** | **proven** | **the hard part** |
| **Where it is** | **the Internet, every campus** | **SD-WAN, EVPN fabrics, wireless controllers** | **some data centres, some WANs** |

> **The hybrid model is what won**, and **it is the one nobody was selling in 2011.**

## Where the separation has actually succeeded

**Six places, and only one of them is what SDN predicted.**

| | Where | The controller does |
|---|---|---|
| **1** | **Wireless controllers** (Chapter 45 §45.2) | **channel and power assignment, roaming, policy** — and it arrived first, in about 2004 |
| **2** | **SD-WAN** (Chapter 51 §51.2) | **path policy, per application, from central policy** |
| **3** | **EVPN fabrics** (Chapter 67 §67.3) | **the control plane is BGP — distributed, and separated from the data plane** |
| **4** | **Cloud networks** (Chapter 69) | **the entire network is an API; there is no device to configure** |
| **5** | **Segment routing** (Chapter 50 §50.4) | **the path is computed centrally and encoded in the packet** |
| **6** | **Data centre fabrics at scale** | **Google's, Microsoft's, Meta's — genuinely centralised** |

**Wireless is the instructive one:**

> **Wireless controllers separated control from data in about 2004, seven years before SDN was
> named**, and **for a reason SDN's proponents would have recognised: RF is a shared medium, so
> channel and power decisions must be global.** **A distributed algorithm cannot make them
> well.**

**And nobody called it SDN**, which is worth noticing — **the architecture arrived where the
problem demanded it, without the name and without the movement.**

## What centralisation costs

**Four costs, each of which was underestimated in 2011.**

### The controller becomes the network

> **A control plane in one place is a single point of failure for the entire network's ability
> to adapt.**

**Which is addressable — clustering, consensus protocols, distributed state — and each of those
is a distributed systems problem of exactly the kind that distributed routing protocols were
solving in the first place.** **You have not removed the distributed system; you have moved it
and changed its failure modes.**

**And the failure mode matters:**

| Controller state | Network |
|---|---|
| **Healthy** | works |
| **Unreachable, devices retain state** | **forwards correctly; cannot adapt or be changed** |
| **Unreachable, devices have no state** | **stops** |
| **Partitioned** | **two controllers, two views** — and this is the hard one |

**Every serious implementation chooses the second row** — **retain the forwarding state and keep
working** — **which is exactly Chapter 51 §51.2's question about SD-WAN controllers**, and it is
the question to ask of any centralised design.

### Latency to the controller

**A forwarding decision that requires a round trip to a controller is a forwarding decision that
takes milliseconds rather than nanoseconds.**

**Which is why pure reactive OpenFlow — "ask the controller about every new flow" — did not
scale** (§68.2), **and why every practical design is proactive: the controller installs rules in
advance and the data plane never asks.**

### Scale of state

**A controller must hold the state of every device and every flow it manages.**

**Which is tractable for a data centre fabric and was not for the Internet-scale ambitions of
the early pitch.**

### And the protocols already worked

**The honest one.**

> **OSPF and BGP converge, scale to the Internet, interoperate between vendors, and have been
> debugged for thirty years.** **Replacing something that works with something that might work
> better requires the improvement to be large**, and for most networks it was not.

## The principle beyond SDN

**The separation's real value is broader than any product.**

**Once control and data are distinct, several things become possible that are not obviously
about SDN:**

| | |
|---|---|
| **Configuration can be generated** | **from a model, by a tool** (Chapter 70) |
| **Policy can be verified before deployment** | **Batfish, and the formal verification work** (Chapter 55's reading) |
| **The data plane can be replaced independently** | **merchant silicon, SONiC, open hardware** |
| **The control plane can be replaced independently** | **FRR on a switch that used to run a vendor's stack** |
| **Telemetry can be streamed from the data plane** | **without the control plane's involvement** (Chapter 54 §54.4) |

> **Disaggregation is the lasting consequence.** **A switch used to be one product: hardware,
> forwarding software, control protocols and management, from one vendor.** **It can now be four
> purchases** — **merchant ASIC, network operating system, routing stack, management platform** —
> **and that is SDN's most substantial and least-discussed achievement.**

## What breaks here

**A controller outage taking the network down.** **The devices had no retained state.** Ask this
question before adopting any centralised design.

**A controller partition producing two views.** **The hard distributed systems problem**, moved
rather than removed.

**Forwarding latency measured in milliseconds.** **Reactive flow installation.** Proactive
programming is the answer.

**A centralised design that cannot verify what it deployed.** **Verification was the argument for
centralising**; a controller that does not verify has taken the cost without the benefit.

**"SDN" used to mean "a controller product".** **The term was used for so many things that it
stopped distinguishing anything** — **ask what is centralised, what is distributed, and what
happens when the controller is unreachable.**

**A distributed protocol replaced with a controller for no measured benefit.** **The protocols
work.** The improvement must be large enough to justify the new failure modes.

> **Network+ note.** Objective 1.8 covers SDN. Over-learn: **SDN separates the control plane from
> the data plane**; **a controller provides centralised management and programmability**; **the
> application layer, control layer and infrastructure layer are the three SDN planes**; and
> **APIs (northbound to applications, southbound to devices) connect them.** The three-layer
> model and the northbound/southbound distinction are examined.
