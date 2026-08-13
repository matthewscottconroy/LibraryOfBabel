# Chapter 69 — Cloud Networking

Here is the most useful thing to know before starting this chapter, and it will save
you a great deal of time:

> **Cloud networking is the networking you already know, with different names and an
> API instead of a console cable.**

A VPC is a routed network with an address plan. A subnet is a subnet. A route table is
a route table. A security group is a stateful firewall rule set. An internet gateway is
a default route plus NAT. An availability zone is a failure domain. A load balancer is
a load balancer.

Every one of those is a concept from Units VI, VII and XII, and someone who understands
Chapters 26, 29 and 60 can learn a cloud provider's networking in a week. Someone who
learned "how to click the buttons in the console" without those chapters will be lost
the moment something does not work — which is the situation a great many people are
actually in, and it is why this book put those chapters where it did.

What is genuinely different is not the concepts. It is three things: **the network is
described in a document rather than cabled**, **the failure domains are explicit and
named**, and **you cannot capture packets in the way you are used to**. Those three
differences shape everything in this chapter.

## The models, stated precisely

§69.1 covers the taxonomy properly, because the terms are used loosely and the
distinctions determine who is responsible when something breaks.

**IaaS** — you get virtual machines, storage and networks; you manage the operating
system upward. Networking is very visible and largely your problem.

**PaaS** — you get a platform; you deploy code. Networking is mostly abstracted, and
what remains is connectivity and access control.

**SaaS** — you get an application. Networking is somebody else's problem entirely, and
your concern is reaching it reliably (Chapter 51's local breakout question).

The **shared responsibility model** is the operational point: the provider secures the
infrastructure *of* the cloud; you secure what you put *in* it. The overwhelming
majority of publicly reported cloud data breaches are customer misconfiguration —
storage left public, security groups permitting the world, credentials committed to a
repository — not provider compromise. That distribution should shape where you spend
effort.

## The VPC

§69.2 works through the construct, and it is worth mapping the vocabulary explicitly
because the mapping is the fastest route to competence:

| Cloud term | What it actually is | Chapter |
|---|---|---|
| VPC / VNet | A routed network with a CIDR block you choose | 26, 27 |
| Subnet | A subnet, confined to one availability zone | 26 |
| Route table | A routing table, attached per subnet | 29 |
| Internet gateway | A default route to the Internet, with 1:1 NAT | 33 |
| NAT gateway | PAT for outbound-only traffic from private subnets | 33 |
| Security group | A **stateful** firewall, applied per instance | 60 |
| Network ACL | A **stateless** filter, applied per subnet | 60 |
| Availability zone | An independent failure domain within a region | 56 |
| Peering | A route between two VPCs, non-transitive | 32 |
| Transit gateway | A hub that makes peering transitive | 11 |

Two of those repay attention.

**Security groups are stateful; network ACLs are not.** A security group permitting
inbound port 443 automatically permits the return traffic (Chapter 60 §60.2). A network
ACL does not — you must permit the ephemeral port range in the other direction, and
forgetting this is one of the most common cloud networking faults. The two mechanisms
exist because they operate at different scopes and one is a defence-in-depth layer for
the other.

**"Public subnet" is not a property of the subnet.** It means a subnet whose route
table has a default route pointing at an internet gateway. There is no flag; it is
routing, exactly as Chapter 29 describes. Understanding that removes most of the
mystery from cloud network design.

The design work that matters most is the one this book has emphasised throughout:
**the address plan** (Chapter 27). A VPC's CIDR block is difficult to change after
deployment, it must not overlap with on-premises space or with any VPC it will ever
peer with, and it must leave room for growth. Organisations that allocated
`10.0.0.0/16` to their first VPC without a plan discover the problem at their fifth
VPC or their first acquisition.

## Connecting to it

§69.3 covers hybrid connectivity: site-to-site VPN over the Internet (cheap, quick,
variable), and dedicated interconnect (Chapter 51 §51.3) for consistent performance and
lower egress costs.

**Egress charges** deserve their own mention because they are the cost that surprises
people. Traffic into a cloud provider is typically free; traffic out is charged per
gigabyte, and traffic between availability zones is often charged too. An architecture
that moves data across zones casually can produce a bill substantially larger than the
compute it runs, and this is a **network design decision with a direct financial
consequence** — which is unusual and worth knowing, because it is one of the few places
where a network engineer's choice appears directly on a finance report.

The non-transitivity of peering is the other structural fact: if A peers with B and B
peers with C, A cannot reach C. This is deliberate (it prevents accidental transit,
exactly as in Chapter 32's peering-versus-transit distinction), and it is why transit
gateways exist and why any design with more than a handful of VPCs needs a hub.

## What you cannot do

§69.4's honest section, because these are the differences that catch experienced
engineers.

**You cannot capture packets the way you are used to.** There is no SPAN port on a
switch you do not own. Providers offer traffic mirroring services and flow logs, and
they are more limited and more expensive than `tcpdump` on a mirror port. Chapter 64's
capture-based diagnosis needs adaptation.

**You cannot traceroute meaningfully through much of it.** The underlying network is
abstracted; intermediate hops are frequently invisible.

**Some things are simply not exposed.** Multicast, broadcast, and arbitrary Layer 2
adjacency generally are not available, because the underlying implementation is a
routed overlay (Chapter 67). Applications that assume Layer 2 behaviour — some
clustering software, some legacy licence servers — do not lift and shift.

**The abstractions leak under load** in ways that are documented poorly, and instance
network performance is frequently tied to instance size in ways that are not obvious
until measured.

## By the end you will be able to

- Distinguish IaaS, PaaS and SaaS and state where responsibility falls.
- Map every major VPC construct to the traditional networking concept it implements.
- Explain the security group / network ACL distinction and predict the return-traffic
  failure.
- Explain what makes a subnet "public."
- Design a multi-VPC address plan that avoids overlap and permits peering.
- Explain egress and cross-zone charging and identify an architecture that will be
  expensive.
- State three things you cannot do in cloud networking that you can do on-premises.
