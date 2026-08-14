# 51.3 Direct Cloud Interconnect

A private circuit from your network into a cloud provider's, bypassing the public Internet
entirely — and the deciding factor is usually the invoice rather than the latency.

## The products

| Provider | Product | Speeds |
|---|---|---|
| **AWS** | **Direct Connect** | 50 Mb/s – 100 Gb/s |
| **Microsoft Azure** | **ExpressRoute** | 50 Mb/s – 100 Gb/s |
| **Google Cloud** | **Cloud Interconnect** (Dedicated / Partner) | 50 Mb/s – 100 Gb/s |
| Oracle, IBM, others | equivalents | |

All work the same way, and the shape is worth having:

```
   ┌──────────────┐        ┌───────────────────┐        ┌─────────────┐
   │ Your network │────────│  Colocation /     │────────│   Cloud     │
   │ (DC or edge) │ cross- │  interconnect     │ provider│  provider   │
   └──────────────┘ connect│  facility         │  port  │   region    │
                           └───────────────────┘        └─────────────┘
                            Equinix, Digital Realty,
                            Telehouse, Interxion …
```

You need a presence where the provider does. Either your own equipment in a colocation
facility, or — far more commonly — a partner carrier who already has a port there and
resells you a virtual circuit into it.

| | **Dedicated** | **Partner / hosted** |
|---|---|---|
| You need | **a port in the facility** | **a circuit from a partner** |
| Speeds | **1, 10, 100 Gb/s** | **50 Mb/s and up, granular** |
| Lead time | **weeks to months** | **days** |
| Cost | port + cross-connect + transport | **one bill** |
| Suits | **large, steady volume** | **most enterprises** |

BGP runs over the circuit (Chapter 32), and you exchange routes with the provider: your
prefixes to them, their service prefixes to you. It is an ordinary peering relationship
with an unusual counterparty.

## The four motivations, honestly weighted

### Egress cost — usually the real reason

Cloud providers charge for data leaving their network, and the rate over a direct connection
is substantially lower than over the Internet.

Representative figures — check current pricing, which changes:

| Path | Typical rate |
|---|---|
| **Internet egress** | **~$0.09 per GB** (first tiers) |
| **Direct interconnect egress** | **~$0.02 per GB** |
| Ingress | **free, both ways** |

**Work an example:**

| Monthly egress | Internet | Direct | **Saving** |
|---|---|---|---|
| 10 TB | $922 | $205 | **$717** |
| 50 TB | $4,608 | $1,024 | **$3,584** |
| **100 TB** | **$9,216** | **$2,048** | **$7,168** |

Against a 1 Gb/s hosted connection costing perhaps $500–1,500 a month all-in, the crossover
sits somewhere around 10–20 TB of monthly egress — and above that the circuit pays for
itself and then keeps paying.

> **Model the egress before designing the connectivity.** It is frequently the entire business
> case, and it is the number most often left out of the architecture discussion because it
> appears on a different budget line.

**And note the asymmetry:** **ingress is free.** The cloud is cheap to enter and expensive
to leave, which is a commercial fact with architectural consequences — it is why data
gravity is real and why "we can move to another provider" is easier to say than to price.

### Consistent latency

A direct circuit has a known path, known capacity and no Internet in the middle.

For most applications this matters less than expected, because the Internet path to a
nearby cloud region is usually good (Chapter 48 §48.1's flattening put the region close to
you). For latency-sensitive applications it matters a great deal, and the value is
consistency rather than absolute figure: the direct path's latency does not change when
someone else's traffic surges.

### Throughput

A 10 Gb/s direct circuit delivers 10 Gb/s. An Internet path may or may not, and a single
TCP stream across a long Internet path frequently does not (Chapter 3 §3.4). For bulk
data movement — backup, replication, migration — this is often the deciding factor.

### Compliance

Some regulatory regimes and some internal policies prohibit particular data from traversing
the public Internet. Whether that prohibition is technically meaningful is arguable —
the traffic is encrypted either way — and it is frequently non-negotiable, so it decides
the design.

## Designing it properly

The mistakes here are expensive, so the design questions deserve stating.

**Where does it terminate?**

| Option | Consequence |
|---|---|
| **Your own data centre** | traffic to cloud goes DC-first; **branches trombone** (§51.1) |
| **A colocation facility** | **a cloud on-ramp separate from your DC**; better for a distributed WAN |
| **Both** | resilient and more expensive |

**How many, and where?**

> A single direct connection is a single point of failure with a very expensive outage.

Genuine redundancy requires two circuits in different facilities, and providers' SLAs
require exactly that: AWS, for instance, offers a meaningful availability commitment only for
multiple connections at multiple locations. One circuit gets you no SLA worth the name.

**And the fallback matters.** Design what happens when the direct connection fails:

- Fail over to an IPsec VPN over the Internet — the standard answer, and it must be
  configured, tested and its capacity understood, because the VPN will not carry what the
  10 Gb/s circuit did
- Ensure BGP local preference and AS-path settings actually cause the failover — this is
  the part that is usually wrong, and it is only discovered during the outage

**How does it interact with SD-WAN?**

**Badly, if nobody thinks about it.** The SD-WAN's policy must know that cloud-bound traffic
should take the direct circuit rather than breaking out locally to the Internet — and the
two systems are frequently procured by different teams.

**A common and correct pattern:** terminate the direct connection at a colocation facility
that is also an SD-WAN hub, so cloud traffic from any branch reaches it over the overlay and
then takes the private path.

## Cloud-to-cloud and the exchange model

**A development worth knowing about.**

**Interconnect exchanges** — Equinix Fabric, Megaport, Console Connect — sell software-defined
connectivity between many providers over one physical port.

```
   Your port ──┬──▶ AWS eu-west-1
               ├──▶ Azure UK South
               ├──▶ Google europe-west2
               ├──▶ a SaaS provider
               └──▶ your own colocation cage
```

Provision a new virtual circuit in minutes, by API, and pay by the month.

> This is the peering model of Chapter 48 §48.2 sold as a product to enterprises, and it is
> the right answer for a multi-cloud organisation: one port, many destinations, no separate
> procurement for each.

And it addresses the multi-cloud egress problem — traffic between two cloud providers over
the Internet is charged as egress by both — which is a cost few architectures anticipate
until the bill arrives.

## What breaks here

A direct connection down and traffic not failing over. **BGP policy.** The backup path
exists and is not preferred. Test the failover deliberately; it is the commonest finding.

**Failover working and performance collapsing.** The VPN backup is 200 Mb/s and the circuit
was 10 Gb/s. Expected, and it should be documented so nobody treats it as a fault.

**Asymmetric routing after failover.** Traffic leaves by one path and returns by another,
which breaks stateful firewalls (Chapter 60). A classic, and it appears only during
failover — which is to say, during an incident.

**Egress charges unchanged after installing the circuit.** The traffic is not using it.
Check the route advertisement and the local preference; advertising the prefix is not the
same as the cloud provider preferring it.

**Cloud provider not learning your on-premises routes.** **Route advertisement limits** — there
is a maximum prefix count per connection, and exceeding it drops the session. Summarise.

Unexpected charges for a connection that is barely used. Port charges are fixed. A
1 Gb/s port costs the same at 5% utilisation as at 95%, which makes under-utilised circuits
the most expensive bandwidth in the organisation.

**Multi-cloud traffic costing far more than expected.** **Egress charged twice**, once by each
provider. An interconnect exchange fixes it; nothing else does.

> **Network+ note.** Objective 1.8 covers cloud connectivity. Over-learn: direct connect /
> ExpressRoute provides a private connection to a cloud provider, bypassing the Internet;
> it offers more consistent performance and lower egress cost; and a VPN over the
> Internet is the common alternative and backup. The private-versus-Internet distinction is
> the examinable content.
