# 68.2 OpenFlow and the Controller

The protocol that named the movement, and understanding why a well-argued idea did not
arrive as promised is more instructive than either the original pitch or a dismissal.

## What OpenFlow specified

A protocol by which a controller programs a switch's forwarding table directly.

```
   ┌──────────────────────────────────────┐
   │            Controller                │
   └──────────────────┬───────────────────┘
                      │ OpenFlow (TCP 6653)
   ┌──────────────────┴───────────────────┐
   │  Switch                              │
   │   ┌────────────────────────────────┐ │
   │   │ Flow table                     │ │
   │   │  match          action  counter│ │
   │   │  in_port=1,     output:3  4471 │ │
   │   │  dst=10.1.2.3                  │ │
   │   │  vlan=20        drop        12 │ │
   │   │  *              controller   3 │ │  ← table-miss
   │   └────────────────────────────────┘ │
   └──────────────────────────────────────┘
```

A flow entry is match, action, counters — and the match may cover any header field the
switch supports, across layers.

| Match on | |
|---|---|
| Ingress port, source and destination MAC | Layer 2 |
| VLAN, priority | |
| Source and destination IP, protocol, DSCP | Layer 3 |
| Source and destination port, TCP flags | Layer 4 |

> **Which is the genuinely radical part.** A switch matching on a TCP port and a router matching
> on an IP prefix are the same device performing the same operation on different fields —
> **and OpenFlow made that explicit.** "Switch" and "router" become configurations rather than
> product categories.

**Actions:** forward to a port, flood, drop, send to controller, modify a field, push or pop a
tag, enqueue.

And the table-miss entry is where the two operating modes diverge.

## Reactive and proactive

| | **Reactive** | **Proactive** |
|---|---|---|
| On a new flow | **the first packet goes to the controller, which installs a rule** | **the rule is already there** |
| Latency | **a controller round trip on the first packet** | **none** |
| Controller load | **proportional to new flows per second** | **proportional to policy changes** |
| Table entries | **one per active flow** | **one per policy rule** |
| **Scales?** | **no** | **yes** |

The reactive model was the demonstration and the proactive model is the deployment, and the
arithmetic explains why:

> A data centre switch may have 4,000 to 100,000 TCAM entries (§68.3). **A busy server sees a
> million concurrent flows.** One entry per flow does not fit, by two orders of magnitude —
> and at 10,000 new flows per second, every one incurs a controller round trip.

The reactive model is what made the early demonstrations compelling — the controller sees
every flow and can make an arbitrary decision about it — and it is the reason the model did
not survive contact with production.

## Why it did not sweep the field

Six reasons, and each is worth stating because the failure modes recur.

### The hardware was not ready

OpenFlow assumed a switch could match on arbitrary combinations of fields.

> Real ASICs have fixed pipelines with specific tables of specific widths — a MAC table, a
> route table, an ACL TCAM — **each optimised for its purpose.** A general match across
> twelve fields must go in the ACL TCAM, which is the smallest and most expensive table on the
> chip.

So OpenFlow's expressiveness mapped onto a fraction of the switch's capacity, and
implementations were partial, inconsistent between vendors, and slow — which is exactly what
the specification was meant to prevent.

§68.3's P4 is the response: make the pipeline programmable rather than assuming it already
is.

### The controller was a new failure domain

§68.1's argument. And the early controllers were not good enough — single instances,
immature clustering, and a failure mode nobody had operated before.

### The protocol churned

OpenFlow 1.0 (2009), 1.1, 1.2, 1.3 (2012), 1.4, 1.5.

> **Each version changed the model substantially** — 1.1 introduced multiple tables, 1.3
> changed the meter and group abstractions — **and vendors implemented different subsets of
> different versions.** **"Supports OpenFlow" meant almost nothing**, which is fatal for a
> protocol whose purpose was interoperability.

And 1.3 became the de facto standard largely because implementations stopped following.

### The gap between the demonstration and the network

A campus network runs spanning tree, VLANs, DHCP snooping, 802.1X, multicast, QoS, and thirty
years of accumulated behaviour (Chapter 55 §55.1).

> Replacing it with a controller means reimplementing all of that in the controller, and
> the demonstration that computed shortest paths did not.

### The distributed protocols already worked

§68.1's honest point. The improvement was not large enough for most networks.

### And the vendors had no incentive

OpenFlow's explicit goal was to commoditise the switch — to make the hardware a
commodity and move the value to the controller.

> **Every incumbent's business depended on the opposite**, and their participation was
> enthusiastic in the standards body and considerably less so in the product roadmap.
> "Supports OpenFlow" appeared on data sheets; the implementations were partial and
> deprioritised.

Which is not a conspiracy; it is an incentive, and Chapter 57 §57.4's economics argument in a
different domain.

## What the controller ecosystem produced

Even without the predicted outcome, the work had substantial results.

| | Became |
|---|---|
| **NOX, POX, Beacon, Floodlight** | research and teaching platforms |
| **OpenDaylight** | **a large industry project; used in specific products rather than as a general controller** |
| **ONOS** | **service provider deployments; genuinely in production** |
| **Ryu** | **the teaching controller** — and F-exercises use it |
| **Open vSwitch** | **the most consequential outcome by a wide margin** |

**Open vSwitch deserves the emphasis:**

> OVS was written to be an OpenFlow switch and became the virtual switch of the cloud
> (Chapter 67 §67.1). It runs in essentially every OpenStack deployment, in a great deal of
> Kubernetes networking, and in NSX. The OpenFlow protocol it speaks is largely used by a
> local controller rather than by a central one — but the programmable-datapath model is
> exactly what SDN proposed, and it won in software where it lost in hardware.

## Where it genuinely runs

**Three places, honestly.**

**Google's B4 wide-area network.** **The best-documented success** — a centralised traffic
engineering system driving OpenFlow switches across Google's inter-data-centre WAN, achieving
utilisation near 100% where a conventionally-engineered WAN runs at 30–40%.

> B4 worked because Google controlled both ends, wrote its own switches, ran its own
> applications, and could tolerate a failure model it designed for. Which is precisely the
> set of conditions most organisations do not have, and the paper is honest about it.

**Service provider transport.** ONOS and similar, for optical and packet transport control —
where the number of devices is modest, the changes are infrequent and central optimisation is
valuable (Chapter 50 §50.3's wavelength assignment is an optimisation problem).

And research and education networks, which have both the appetite and the tolerance.

## What replaced the pitch

The idea's descendants are more successful than the idea.

| SDN promised | What arrived |
|---|---|
| **A central controller programming switches** | **a controller setting policy, devices running protocols** (§68.1's hybrid) |
| **Commodity hardware** | **merchant silicon and open network operating systems** — this one arrived |
| **Network programmability** | **APIs and automation** (Chapter 70) — **arrived, differently** |
| **Vendor independence** | **partially, via disaggregation** |
| **A network with an API** | **the cloud** (Chapter 69) — **completely, and by a different route** |

> **The last row is the honest summary.** The most SDN-like networks in existence are the
> public clouds, where there is no device to configure and the entire network is an API —
> and they were built by companies that were not participating in the OpenFlow standards
> process.

## What breaks here

**"Supports OpenFlow" taken at face value.** Ask which version and which tables. The
implementations were partial.

**A general match consuming the ACL TCAM.** The smallest table on the chip, and the fault is
"the switch supports 2,000 rules" when the design assumed 100,000.

**Reactive flow setup at scale.** A controller round trip on every new flow. Proactive.

**A controller failure that stopped forwarding.** The devices retained no state. §68.1.

A campus SDN pilot that reimplemented shortest-path routing and stopped. The remaining 95%
of the network's behaviour was the hard part.

A vendor's SDN product that is a management platform. Which may be useful and is not
control-plane separation. Ask what is actually centralised.

> **Network+ note.** Objective 1.8. Over-learn: OpenFlow is the southbound protocol between an
> SDN controller and network devices; the controller has a global view and programs
> forwarding; **northbound APIs expose the network to applications**; and **SDN enables
> programmability and automation.** OpenFlow's specific limitations are not examined and are the
> reason you will probably never configure one.
