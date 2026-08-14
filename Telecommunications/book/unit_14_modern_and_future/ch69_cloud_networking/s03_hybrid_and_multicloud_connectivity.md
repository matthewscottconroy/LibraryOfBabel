# 69.3 Hybrid and Multicloud Connectivity

Chapter 51 §51.3 covered the enterprise's side of this. This section is the cloud's side, and
the scaling problem that dominates it.

## The four ways in

| Mechanism | Path | Bandwidth | Use |
|---|---|---|---|
| **Site-to-site VPN** | **over the Internet** | **~1.25 Gb/s per tunnel, typically** | **quick, cheap, adequate** |
| **Direct connect / ExpressRoute** | **private circuit** | **50 Mb/s – 100 Gb/s** | **volume, consistency, compliance** |
| **SD-WAN into the cloud** | **the provider's overlay, terminated in a cloud instance** | varies | **policy consistency** (Chapter 51 §51.2) |
| **Client VPN** | **individual users** | | remote access to cloud resources |

The VPN's per-tunnel bandwidth limit is the constraint people meet first:

> A cloud VPN gateway's throughput is typically capped per tunnel — around 1.25 Gb/s on AWS —
> and the answer is multiple tunnels with ECMP, which requires the on-premises device to
> support it and requires the routing to distribute across them.

And the redundancy requirement is Chapter 51 §51.3's: a single direct connection carries no
meaningful availability commitment, and the providers' SLAs require multiple connections at
multiple locations — with a VPN as the tested fallback, sized for what it can actually
carry.

## The topology problem

The reason transit gateways exist, and it is Chapter 11's arithmetic arriving again.

VPC peering is point to point and not transitive (§69.2). So $n$ VPCs that must all
communicate require:

$$\frac{n(n-1)}{2} \text{ peering connections}$$

| VPCs | Peerings |
|---|---|
| 5 | 10 |
| 10 | **45** |
| **20** | **190** |
| 50 | **1,225** |

And each requires route table entries in both VPCs, so the configuration burden grows with
the square as well.

> This is Chapter 11 §11.2's full mesh, Chapter 51 §51.1's leased lines and Chapter 58 §58.1's
> key distribution — the same arithmetic, for the fourth time, and the same answer: a hub.

**A transit gateway is that hub.**

```
   Before:                        After:

   VPC-A ──── VPC-B               VPC-A ─┐
     │  ╲    ╱  │                 VPC-B ─┼── Transit ── on-premises
     │   ╲  ╱   │                 VPC-C ─┤   Gateway
   VPC-C ──── VPC-D               VPC-D ─┘
   
   6 peerings, 8 route entries    4 attachments, and routing policy
                                  in one place
```

**And it adds what peering cannot:**

| | |
|---|---|
| **Transitivity** | **any attachment can reach any other, subject to policy** |
| **Route tables per attachment** | **which is segmentation** — VPCs that must not communicate |
| **A single on-premises attachment** | **serving every VPC** |
| **Cross-region peering** | between transit gateways |

**Its costs are real:** an hourly charge per attachment, a per-gigabyte data processing charge,
and a bandwidth limit per attachment — so a transit gateway carrying substantial traffic is
a substantial line on the bill, and direct peering remains correct for a single
high-volume pair.

## Segmentation with a transit gateway

The feature that makes it more than a router.

Multiple route tables, and each attachment is associated with one and propagates into others:

```
   Route table "prod":     receives routes from prod VPCs and on-premises
   Route table "dev":      receives routes from dev VPCs only
   Route table "shared":   receives routes from everything

   prod VPCs      → associated with "prod",   propagate to "prod" and "shared"
   dev VPCs       → associated with "dev",    propagate to "dev" and "shared"
   shared services→ associated with "shared", propagate to "prod" and "dev"
```

> Which produces: production reaches shared services and on-premises; development reaches
> shared services only; and production and development cannot reach each other at all — not
> by a firewall rule, but because no route exists (Chapter 60 §60.4's VRF argument, in a
> cloud).

And routing-based segmentation is stronger than filter-based segmentation, because there is
no rule to be misordered, no state to exhaust, and no path to be permitted by accident.

## Private access to provider services

A subtlety that is easy to miss and matters for both cost and security.

A cloud provider's own services — object storage, databases, message queues — have public
endpoints. An instance in a private subnet reaching them by default goes:

```
   Instance → NAT gateway → Internet gateway → the Internet → the service
              (charged)      (charged egress)                 (in the same region)
```

> Traffic to a service in the same region leaves the provider's network and comes back,
> paying NAT processing and egress charges, and traversing the Internet.

**Endpoints fix it:**

| | |
|---|---|
| **Gateway endpoint** | **a route table entry** — traffic to the service goes directly. **Free, and limited to a few services** |
| **Interface endpoint / PrivateLink** | **an interface in your subnet with a private address**. **Charged hourly and per gigabyte, and works for most services** |

And the security consequence is larger than the cost one:

> With an endpoint, the private subnet needs no route to the Internet at all — no NAT
> gateway, no internet gateway, no egress path. Which removes the exfiltration route and
> substantially reduces the attack surface (Chapter 57 §57.4's "the strongest control is
> absence").

**PrivateLink also works in the other direction:** exposing your own service to another VPC or
another organisation, privately, without peering and without exposing anything else — which
is the correct mechanism for a partner integration and is under-used.

## Multicloud, honestly

§69.1 noted that multicloud is frequently an outcome rather than a strategy. This is what
it costs at the network layer.

| | |
|---|---|
| **Egress charged by both** | Chapter 51 §51.3 — **the largest single cost** |
| **Two networking models** | **VPC, VNet and GCP VPC differ in routing, in security semantics and in what is regional versus zonal** |
| **Two identity systems** | to be federated |
| **The least capable common subset** | **"portable" means avoiding what each does well** |
| **Two sets of skills** | and two on-call rotations |

**The mechanisms that make it tolerable:**

**An interconnect exchange** (Chapter 51 §51.3) — Megaport, Equinix Fabric, Console Connect —
one port reaching several providers, and it avoids the Internet path and reduces the egress
rate.

And a consistent overlay — an SD-WAN or a service mesh spanning both — which gives one
policy model at the cost of another layer.

> The honest position: multicloud for resilience is expensive and rarely delivers what is
> claimed, because the failure modes that take out a provider are frequently the control plane
> and the application cannot fail over faster than a human can decide. Multicloud because you
> acquired a company, or because a SaaS provider runs elsewhere, is a fact to be managed rather
> than a strategy to be defended — and most of the value attributed to multicloud is actually
> available from multi-region within one provider, at a fraction of the cost.

## Designing the connection

**Five decisions, in order.**

**How much traffic, in which direction?** Egress dominates the cost (Chapter 51 §51.3), and
the volume determines VPN versus direct connect.

**What availability?** Two direct connections at two locations, or one plus a tested VPN, or a
VPN pair — Chapter 56 §56.2's arithmetic and its shared-fate question.

**Where does it terminate?** A transit gateway, a single VPC, or a colocation facility —
and terminating at the VPC that happens to be first is the mistake that produces a redesign.

**What addresses?** §69.2 — and the overlap check must include every VPC, every cloud, every
partner and every future acquisition.

**And what routing?** BGP over the direct connect or VPN (Chapter 32), with the same
questions as any peering: what do you advertise, what do you accept, and what is the failover
behaviour — and the last one must be tested (Chapter 56 §56.2).

## What breaks here

A VPN throughput ceiling that is not the circuit's. **Per-tunnel limits.** Multiple tunnels
with ECMP.

VPC A cannot reach VPC C through B. Peering is not transitive.

Forty-five peering connections and a configuration nobody can reason about. **The full-mesh
arithmetic.** A transit gateway.

Traffic to object storage in the same region charged as egress. **No endpoint**, and it is
traversing the Internet.

A private subnet with a NAT gateway that exists only for provider services. Endpoints
would remove it entirely, along with the exfiltration path.

Production and development able to reach each other despite security groups. Both attached
to the same transit gateway route table. Segment by routing.

A failover to the VPN that halves throughput. Expected, and it should be documented
(Chapter 51 §51.3).

**A multicloud architecture justified by resilience.** Examine what failure it actually
survives, and compare with multi-region in one provider.

> **Network+ note.** Objective 1.8. Over-learn: cloud connectivity uses VPN over the Internet
> or a direct private connection; direct connections offer consistent performance and lower
> egress cost; **hybrid cloud combines on-premises and cloud**; and **multicloud uses more than
> one provider.** The VPN-versus-direct-connect trade is examined; the transit gateway
> arithmetic is what you will use.
