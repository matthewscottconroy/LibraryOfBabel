# Chapter 68 — SDN and Programmable Networks

Around 2011, software-defined networking was going to change everything.

The pitch was genuinely compelling. Take the control plane out of every device
(Chapter 29 §29.1), put it in a central controller with a complete view of the
network, and let it compute optimal forwarding decisions and push them down. No more
distributed protocols reaching approximate consensus. No more configuring devices one
at a time. A network with an API, programmable like anything else in the data centre.

The academic work was serious — Ethane at Stanford, then OpenFlow, then the Open
Networking Foundation in 2011 with substantial industry backing. Predictions were made
about the imminent commoditisation of network hardware. A great deal of venture funding
followed.

Fifteen years later, the enterprise network in your building is almost certainly not
running an OpenFlow controller, and most network engineers have never configured one.

This chapter takes that seriously, because **understanding why a well-argued idea did
not arrive as promised is more instructive than either the original pitch or a
dismissal**, and because the idea's genuine successes are substantial and are not the
ones that were predicted.

## What was actually right

The core observation was correct and remains so.

A traditional network's control plane is **distributed by necessity rather than by
design**. Every router runs OSPF and computes its own shortest-path tree because there
was no practical alternative in 1989 — no controller could have been reliable enough or
fast enough. Distribution was the engineering constraint, not the goal.

But distribution has real costs. Every device must be configured. Consistency is
achieved by protocol convergence rather than by construction, so transient
inconsistency is normal and loops are possible during reconvergence. Policy that spans
devices — "traffic from this group takes that path" — is expressed by configuring each
device to do its part, which is error-prone and hard to verify. And optimisation is
local: each router does the best it can with what it knows, which is not the same as
what is globally best.

A controller with a complete view can compute globally optimal paths, verify policy
before applying it, and reconfigure the whole network atomically. That argument has not
been refuted.

## Why it did not sweep the field

§68.2 gives the honest account, and the reasons are worth having because they recur
whenever a centralised architecture is proposed.

**The controller is a single point of failure of an unusually consequential kind.**
Distributed protocols degrade; if a router loses contact with its neighbours, the rest
of the network continues. A network whose devices cannot forward without a controller
has a new and severe failure mode. Controller high availability is solvable and it is
not free, and it adds exactly the complexity that the pitch promised to remove.

**Scale is hard.** A controller managing thousands of devices and millions of flow
entries, with sub-second reaction times, is a demanding distributed system in its own
right — and the industry substituted one hard distributed-systems problem for another.

**The installed base is enormous and works.** Replacing functioning equipment requires
a benefit large enough to justify the risk and the capital, and "more elegant control
architecture" was not that benefit for most enterprises.

**Vendors adapted.** Rather than being commoditised, incumbents added APIs,
programmability and controller-based management to their existing platforms, offering
most of the operational benefit without the architectural rupture. Whether this was
absorption of a good idea or defence of a business model is a matter of interpretation
and both readings have merit.

**OpenFlow specifically was too low-level.** Programming a network by installing
match-action entries in flow tables turned out to be an awkward abstraction — too
detailed for expressing intent, too coarse for expressing everything a real network
does. Later versions accumulated complexity trying to fix this, which is its own kind
of evidence.

## Where it genuinely won

And it did win, decisively, in three places — none of which was the enterprise LAN.

**Data centre fabrics.** Chapter 67's overlays are centrally orchestrated. VMware NSX,
Cisco ACI and the cloud providers' internal fabrics are software-defined networking by
any reasonable definition, deployed at enormous scale, and they succeeded because the
data centre is a greenfield environment under one administration where the operational
benefit is largest.

**Wide-area traffic engineering.** Google's B4, described in a 2013 SIGCOMM paper, uses
centralised control to run its inter-data-centre WAN at close to 100% utilisation —
against the 30–40% typical of traditionally engineered networks, which must reserve
headroom for failure. The saving on a network of that scale is enormous, and the
environment is ideal: one owner, known demands, and the ability to defer traffic.

**SD-WAN** (Chapter 51 §51.2), which is the most commercially successful application
by a wide margin — central policy, distributed enforcement, and an obvious cost
justification.

The pattern in all three: **SDN succeeded where one organisation controls everything
and the optimisation benefit is measurable.** It struggled where the environment is
heterogeneous, incrementally grown and multi-vendor — which describes most enterprise
networks.

## Programmable data planes

§68.3 covers what may prove more consequential than controllers.

**P4** is a language for describing *how a switch's forwarding pipeline works* — not
which entries are in the table, but what the table matches on and what happens next.
A conventional switch's ASIC understands Ethernet, IP, TCP and a fixed set of others,
decided at silicon design time. A P4-programmable switch is told what headers exist
and how to process them.

This matters because it decouples protocol support from hardware refresh. A new
encapsulation format — the VXLAN-to-GENEVE transition of Chapter 67, say — historically
required new silicon and a three-year cycle. On a programmable pipeline it is a
software update.

It also enables in-network computation that was not previously possible: in-band
network telemetry recording per-packet path and queueing data, load balancing
implemented at line rate, and aggregation for distributed machine learning performed in
the switch. Whether these become mainstream or remain specialised is genuinely open.

## Intent-based networking

§68.4 covers the current framing, with its limits stated.

The pitch: describe *what you want* — "these groups may communicate, that one is
isolated, this application gets priority" — and let the system determine the
configuration, verify it continuously, and correct drift.

The genuinely valuable part is **verification**: continuously checking that the network
does what the policy says, and alerting when it does not. That addresses configuration
drift (Chapter 55 §55.1) directly, and it is a real capability.

The overstated part is the automatic translation from natural-language business intent
to correct network configuration. Current systems work within constrained domains with
well-defined primitives. "Make the network fast" is not an input any system accepts,
and the marketing occasionally implies otherwise.

## By the end you will be able to

- Explain control and data plane separation and what centralisation offers.
- Give four specific reasons the OpenFlow vision did not displace traditional
  networking.
- Identify the three environments where SDN succeeded and state the common property.
- Explain what P4 programs and why that decouples protocols from silicon.
- Distinguish the verifiable claims of intent-based networking from the aspirational
  ones.
