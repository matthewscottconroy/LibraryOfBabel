# Chapter 14 — Important Concepts

**The scope categories** *(§14.1)* — PAN (metres), LAN (building), CAN (site), MAN
(city), WAN (continental). Conventional, taught as a taxonomy, and more useful read
as a set of continuous parameters.

**What has eroded** *(§14.1)* — **Speed** (a 100 Gb/s wide-area circuit is
unremarkable, and many organisations have a faster path to their cloud provider than
between two floors); **cost per bit**; **ownership** (cloud and colocation blur it);
and **error rate** (which is why X.25's per-hop correction was removed in Frame
Relay).

**What has not** *(§14.1)* — **Latency.** Propagation delay is distance over the
speed of light and is not an engineering parameter. Chicago to Frankfurt was ~35 ms
one way in 1980 and will be in 2075.

The modern LAN/WAN distinction *(§14.1)* — Not ownership, speed or cost, but
latency, and therefore round trips. An application performing twenty round trips
per operation costs 4 ms on a LAN and 700 ms over an ocean, and no amount of
bandwidth changes it.

Read off which parameter binds *(§14.1)* — The productive question is not "LAN
or WAN?" but which of latency, bandwidth, loss, power, RF contention, provider
diversity or cost is the constraint. Naming the scope tells you little; naming the
constraint tells you what to do.

**Internetwork** *(§14.2)* — What you get when you connect **networks**, not hosts,
treating each constituent as a black box that delivers packets internally by means
you need not know. Genuinely different from "a big network", which extends one
technology until its limits stop it.

The construction is recursive *(§14.2)* — Connect internetworks and you get an
internetwork. The global Internet is this applied at every scale, and no level needs
to understand any other's internals.

Requirement 1: a universal address space *(§14.2)* — Meaningful across every
constituent network. MAC addresses cannot serve, being flat and unaggregatable, so
the internetwork needs a **hierarchical** space — which is IP, and which is what
makes longest-prefix forwarding possible. Note it is a *second* address space layered
above the local ones, which is why ARP exists.

Requirement 2: a device that joins networks *(§14.2)* — The **router**
(originally *gateway*, surviving in "default gateway"), making one stateless
decision repeatedly.

Requirement 3: a lowest-common-denominator service *(§14.2)* — The internetwork
can promise only what its **weakest** constituent delivers. A 20%-loss radio link
cannot promise delivery at any price, so IP promises nothing. Best-effort is not a
limitation better engineering would remove; it is the only universally satisfiable
contract, and it is what let IP run over Ethernet, Wi-Fi, LTE and everything since
without amendment.

Requirement 4: inter-domain reachability exchange *(§14.2)* — Independent
networks must tell each other what they can reach, with no central authority and
without trusting each other. That is BGP, and it is why the Internet has no owner,
no off switch and no complete inventory.

**The hourglass, derived** *(§14.2)* — Universal addressing plus a
lowest-common-denominator service **necessarily** produce one minimal protocol at
the waist and free multiplication above and below. RFC 1958: *"everything over IP,
and IP over everything."*

**Innovation without permission** *(§14.2)* — Neither end of the hourglass needs the
other's agreement, which is why the web could be invented without consulting a
single network operator. The property Chapter 13 §13.4 identified as mattering most.

Why the waist is hardest to change *(§14.2)* — Everything above and below
depends on it and there is no incremental path: an IPv6-only host cannot reach an
IPv4-only one, so adopting it alone reaches nothing new. Hence thirty years.

**Client–server** *(§14.3)* — Asymmetric: the server must be reachable at a stable
known address (hence DNS, hence static addressing), the client need not be reachable
at all (hence NAT works for clients), and traffic is asymmetric (hence asymmetric
access technologies).

**Peer-to-peer** *(§14.3)* — Every participant both client and server. Capacity
scales with participants; every participant must be reachable inbound, which NAT
breaks — hence STUN, TURN, ICE and hole punching in every video-calling application.

**The three-stage re-centralisation** *(§14.3)* — **NAT** converted a network of
peers into a network of clients as a side effect of an addressing workaround;
**the cloud** moved applications off organisations' own machines; **CDNs**
concentrated a majority of consumer bytes in a few sources.

**Consequences of re-centralisation** *(§14.3)* — Traffic goes outward not inward
(hence local breakout); the perimeter model stops making sense (hence zero trust);
latency to a handful of destinations dominates; and **resilience concentrates**, so
one provider's outage takes down many apparently unrelated services.

The opposite movement inside the data centre *(§14.3)* — Microservices
decentralised the application layer internally at the same time as it centralised
externally, producing **east–west dominance** and leaf-spine fabrics.

The network follows the application architecture, with a lag *(§14.3)* — A
network designed for the previous era works and is wrong in ways that surface as
performance problems attributed to capacity. The design question is where does the
traffic actually go — measured, not assumed.

**Convergence** *(§14.4)* — Voice, video, CCTV, building control and data on one
packet fabric, replacing four separate cabling plants, equipment sets, suppliers and
teams.

The telephone engineers' objection was correct *(§14.4)* — Voice needs bounded
delay (150 ms one way, G.114), bounded jitter, very low and non-bursty loss,
five-nines availability, and **admission control**. A 1995 packet network provided
none of these.

Why convergence happened anyway *(§14.4)* — Cost (one plant instead of four);
the packet network improved faster; features became software with software's
economics; and integration with other applications mattered more to buyers than call
quality.

**The recurring pattern** *(§14.4)* — A general-purpose substrate absorbs a
specialised one because its economics and rate of improvement are better, **before**
it is technically superior. The specialists are right about the deficiencies and
wrong about the trajectory.

**What convergence costs** *(§14.4)* — Everything shares a failure domain (a switch
failure now takes out voice, video, CCTV, door access **and** the telephones you
would use to report it); voice becomes the network team's problem; QoS becomes
necessary where separation was previously physical and free; the security surface
expands; and regulatory obligations transfer.

**The pendulum** *(§14.4)* — VLANs, QoS, MPLS-TE, network slicing, TSN and lossless
Ethernet each **logically recreate** something convergence removed **physically**.
The trade is physical separation, simple and expensive, exchanged for logical
separation, cheap and dependent on configuration you can get wrong — which is the
right trade, made knowingly.
