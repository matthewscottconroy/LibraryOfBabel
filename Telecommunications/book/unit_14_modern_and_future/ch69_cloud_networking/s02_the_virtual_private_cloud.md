# 69.2 The Virtual Private Cloud

**Every concept here is from Units VI, VII and XII with a different name.** **This section is
the translation table plus the four things that genuinely differ.**

## The translation

| Cloud term | Is | Chapter |
|---|---|---|
| **VPC / VNet** | **a routed network with an address plan** | 27 |
| **Subnet** | **a subnet** — and it is bound to one availability zone | 26 |
| **Route table** | **a route table** | 29 |
| **Internet gateway** | **a default route plus NAT for public addresses** | 33 |
| **NAT gateway** | **PAT for private instances** | 33 §33.2 |
| **Security group** | **a stateful firewall, per instance** | 60 §60.2 |
| **Network ACL** | **a stateless filter, per subnet** | 60 §60.1 |
| **Availability zone** | **a failure domain** | 56 §56.2 |
| **Region** | **a geography** | 50 §50.5 |
| **Peering** | **a route between two VPCs** | 48 §48.2's name, different mechanism |
| **Transit gateway** | **a hub router** | 11 §11.2 |
| **Endpoint / PrivateLink** | **a private path to a service, bypassing the Internet** | |
| **Load balancer** | **a load balancer** | 52 §52.4 |

> **Someone who understands Chapters 26, 29 and 60 can learn a provider's networking in a
> week.** **Someone who learned which buttons to click will be lost the moment something does
> not work.**

## Addressing, and the decisions that are permanent

**The VPC's CIDR block is chosen once and is difficult to change**, which makes it Chapter 27's
address planning with higher stakes.

**The rules that matter:**

| | |
|---|---|
| **It must not overlap anything you will ever connect to** | **on-premises, other VPCs, partners, other providers** |
| **It must be large enough** | **and cloud address consumption is higher than you expect** |
| **Some addresses per subnet are reserved** | **AWS reserves 5; others reserve fewer** |
| **A subnet belongs to exactly one availability zone** | **which forces the plan's shape** |

**A worked plan for a `/16` across three zones and three tiers:**

```
   VPC          10.20.0.0/16          65,536 addresses

   AZ-a  public   10.20.0.0/20        4,096  (4,091 usable)
         app      10.20.16.0/20
         data     10.20.32.0/20
   AZ-b  public   10.20.48.0/20
         app      10.20.64.0/20
         data     10.20.80.0/20
   AZ-c  public   10.20.96.0/20
         app      10.20.112.0/20
         data     10.20.128.0/20

   Used: 36,864 of 65,536.  Remainder: growth, and a fourth zone.
```

**Which is Chapter 27 §27.2's structured plan, with the zone dimension added** — **and the
reason to leave half the space free is that a subnet cannot be resized in most providers, so
growth means adding subnets.**

> **The commonest and most expensive addressing mistake is a VPC that overlaps the corporate
> network** (Chapter 67 §67.1 made the same observation about Kubernetes). **It is discovered
> when the direct connect is established, it cannot be fixed without rebuilding, and it happens
> because someone accepted a default.**

## Security groups and NACLs

**Two filtering mechanisms with different semantics**, and the difference is Chapter 60's.

| | **Security group** | **Network ACL** |
|---|---|---|
| Applies to | **an instance's interface** | **a subnet** |
| **Stateful** | **yes** — return traffic implicit | **no** — both directions required |
| Rules | **allow only** | **allow and deny** |
| Evaluation | **all rules, any match permits** | **in order, first match wins** |
| Default | **deny inbound, allow outbound** | **allow all** (default NACL) |
| **Reference other groups** | **yes — and this is the important feature** | **no, addresses only** |

**The group-referencing feature is what makes security groups genuinely better than a firewall
rule set:**

```
   Security group "app-tier":
     inbound  tcp 8443 from security group "web-tier"
   Security group "data-tier":
     inbound  tcp 5432 from security group "app-tier"
```

> **No addresses appear.** **Instances are added and removed from the tiers and the policy does
> not change** — **which is Chapter 60 §60.4's label-based microsegmentation, built into the
> platform**, and it is why cloud segmentation is frequently better than the on-premises
> equivalent.

**And the practical guidance is to use security groups and leave NACLs at their defaults**,
except **where a stateless subnet-level deny is genuinely needed** — blocking a specific address
range, or a compliance requirement for a second enforcement layer.

**Because NACLs' statelessness is a trap:** **permitting inbound 443 without permitting outbound
on the ephemeral range produces a connection that establishes and returns nothing**, which is
Chapter 60 §60.1's directionality problem in a place people do not expect it.

## Routing, and what a route table can and cannot do

**Each subnet has one route table; a route table may serve many subnets.**

```
   Public subnet route table:
     10.20.0.0/16    local
     0.0.0.0/0       internet-gateway

   Private subnet route table:
     10.20.0.0/16    local
     0.0.0.0/0       nat-gateway
     192.168.0.0/16  transit-gateway     ← on-premises
```

**And "public" and "private" are defined entirely by that default route** — **there is no other
distinction.** **A subnet whose default route is an internet gateway is public; one whose
default route is a NAT gateway is private.**

**Three constraints that catch people:**

**The `local` route cannot be removed or overridden.** **Everything in the VPC can reach
everything else at the routing layer**, and **isolation within a VPC is entirely the security
groups' job** — which is why they matter so much.

**Route tables have limited entries**, and **more specific routes win** (Chapter 29 §29.3),
which behaves as expected.

**And peering is not transitive.** **If A peers with B and B peers with C, A cannot reach C** —
**which is deliberate, and it is why transit gateways exist.**

## Availability zones, and what they actually are

**Chapter 56 §56.2's failure domain, named explicitly** — **which is a genuine improvement over
an on-premises environment where the failure domains are implicit and undocumented.**

| | |
|---|---|
| **Separate power, cooling and network** | within a region |
| **Independent failure** | **that is the guarantee** |
| **Low latency between them** | **single-digit milliseconds** — synchronous replication is viable (Chapter 56 §56.4) |
| **A subnet is in exactly one** | which forces multi-zone designs to have multiple subnets |
| **Cross-zone traffic is charged** | §69.1 |

**And the shared-fate question of Chapter 56 §56.2 applies:**

> **Zones fail independently at the compute and storage layer.** **They may share a control
> plane, an authentication service, a metadata service or a regional API** — **and several large
> cloud incidents have exactly that shape: every resource in every zone healthy, and nothing
> able to be created, modified or authenticated.**

**Which is why a genuinely resilient design is multi-region**, and **why most designs are not,
because multi-region is substantially harder and more expensive.**

## What is genuinely different

**Four things, and they are the chapter's real content.**

### The network is a document

**Every element above is created by an API call**, and **the whole VPC can be described in a
file** (Chapter 70).

> **Which means the network can be version-controlled, reviewed, tested and recreated** — **the
> inversion of authority that Chapter 55 §55.4 described as a destination is the default
> here.**

**And it means the network can be destroyed and rebuilt in minutes**, which changes disaster
recovery (Chapter 56 §56.4) fundamentally: **the recovery is running the code in another
region.**

### The failure domains are named

**Zones, regions and services are explicit.** **On-premises, "what fails together?" requires
investigation** (Chapter 56 §56.2); **here it is documented.**

**And the corollary: you are expected to use them.** **A single-zone deployment that fails when
that zone fails is your architecture's fault, not the provider's** — the shared responsibility
model, applied to availability.

### You cannot capture packets

**Some providers offer traffic mirroring, at cost and with limits.** **Flow logs are universal
and are §54.4's flow records with the provider's metadata added** — **and they are the primary
diagnostic tool.**

> **Chapter 64 §64.3's method does not transfer.** **The replacement is flow logs, the load
> balancer's access logs, the application's own telemetry and — where it exists — mirroring.**
> **Which is less than a packet capture and more than most on-premises environments actually
> collect.**

### And MTU is fixed and differs

| Provider | Typical |
|---|---|
| AWS | **1500 within a VPC; 9001 jumbo where supported; 1500 over most gateways** |
| Azure | **1400 for some paths** |
| GCP | **1460 default, configurable** |

**And it interacts with tunnels** (Chapter 61 §61.1, Chapter 66 §66.3): **a VPN over a cloud
network has the cloud's MTU minus the tunnel's overhead**, and **the resulting value is
frequently the cause of "it works from the office and not from the cloud."**

## What breaks here

**A VPC overlapping the corporate network.** **Discovered when the direct connect is
established.** It cannot be fixed without rebuilding.

**Two peered VPCs unable to reach a third.** **Peering is not transitive.** A transit gateway.

**A connection that establishes and returns nothing.** **A NACL permitting inbound and not the
ephemeral return.** Stateless.

**Instances in a VPC reaching each other despite security groups.** **Check the groups** — the
`local` route cannot be removed, so the groups are the only isolation.

**A single-zone deployment that failed with the zone.** **The failure domain was named and not
used.**

**Every resource healthy and nothing able to be created.** **A regional control plane
incident**, and it is the shared-fate case Chapter 56 §56.2 describes.

**A VPN into the cloud where small packets work and large ones do not.** **The provider's MTU
minus the tunnel overhead.** Chapter 66 §66.3.

**No packet capture available.** **Flow logs, load balancer logs and application telemetry**, and
they are collected by default in a way on-premises equivalents rarely are.

> **Network+ note.** Objective 1.8 covers cloud networking. Over-learn: **a VPC is an isolated
> virtual network**; **subnets are placed in availability zones**; **security groups are
> stateful and apply to instances, NACLs are stateless and apply to subnets**; **an internet
> gateway provides public connectivity and a NAT gateway provides outbound-only access for
> private subnets**; and **regions contain multiple availability zones.** The
> security-group-versus-NACL comparison is examined heavily.
