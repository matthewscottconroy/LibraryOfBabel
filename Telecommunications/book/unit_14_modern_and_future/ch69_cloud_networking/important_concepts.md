# Chapter 69 — Important Concepts

**Cloud networking is the networking you already know, with different names and an API instead
of a console cable** *(intro)* — **A VPC is a routed network with an address plan; a security
group is a stateful firewall; an availability zone is a failure domain.** **Someone who
understands Chapters 26, 29 and 60 learns a provider's networking in a week; someone who learned
which buttons to click is lost the moment something does not work.**

**The three genuine differences** *(intro, §69.2)* — **the network is described in a document
rather than cabled; the failure domains are explicit and named; and you cannot capture packets.**

**Read the service models as a division of responsibility** *(§69.1)* — **The network engineer's
involvement falls sharply from IaaS to SaaS, and so does the visibility.** **A PaaS deployment's
networking is a black box with a security group in front of it**, and Chapter 64's tools do not
apply to it at all.

**Hybrid and multicloud are routinely confused and have different problems** *(§69.1)* — Hybrid
joins your infrastructure to a provider's; multicloud joins two providers'. **Egress is charged
once in the first case and by both in the second.** **And multicloud is frequently an outcome
rather than a strategy** — an acquisition, a team that chose differently — **and the version
chosen deliberately to avoid lock-in usually costs more than the lock-in it avoids**, because
"portable across clouds" means using the least capable common subset.

**Security *of* the cloud is the provider's; security *in* the cloud is yours** *(§69.1)* —
**The overwhelming majority of publicised cloud breaches are customer misconfigurations**, and
the distinction is not a provider's excuse but a statement about where to spend effort.

**The skills that transfer are the concepts; the ones that do not touch hardware** *(§69.1)* —
**Addressing, routing, segmentation, load balancing, TLS, DNS and availability arithmetic all
transfer.** **Packet capture, the physical layer, `traceroute` inside, broadcast and MTU control
do not.** Which is why the transition feels familiar and alien simultaneously.

**The network is metered in ways an on-premises network is not** *(§69.1)* — **Egress,
cross-zone traffic, and NAT gateway processing.** **An architecture that would be free in a data
centre has a per-gigabyte cost in a cloud**, which changes placement, caching and protocol
chattiness for reasons unrelated to performance.

**The VPC CIDR is chosen once and is difficult to change** *(§69.2)* — **The commonest and most
expensive addressing mistake is a VPC that overlaps the corporate network**: discovered when the
direct connect is established, unfixable without rebuilding, and caused by accepting a default.

**A subnet belongs to exactly one availability zone** *(§69.2)* — Which forces the address plan's
shape, and **a subnet cannot be resized in most providers**, so growth means adding subnets and
leaving half the space free.

**Referencing a security group from another means no addresses appear in the policy** *(§69.2)*
— **Instances are added and removed from tiers and the policy does not change.** **Chapter 60
§60.4's label-based microsegmentation, built into the platform**, and it is why cloud
segmentation is frequently better than the on-premises equivalent.

**NACLs are stateless, which is a trap** *(§69.2)* — **Permitting inbound 443 without permitting
the ephemeral return produces a connection that establishes and returns nothing.** Use security
groups; leave NACLs at defaults unless a subnet-level deny is genuinely required.

**The `local` route cannot be removed** *(§69.2)* — **Everything in a VPC can reach everything
else at the routing layer**, so isolation within a VPC is entirely the security groups' job.

**"Public" and "private" are defined solely by the default route** *(§69.2)* — Internet gateway
or NAT gateway. There is no other distinction.

**Peering is not transitive, deliberately** *(§69.2, §69.3)* — And $n$ VPCs needing full
connectivity require $n(n-1)/2$ peerings: **45 at ten VPCs, 190 at twenty.** **Chapter 11's full
mesh, Chapter 51's leased lines and Chapter 58's key distribution — the same arithmetic for the
fourth time, and the same answer: a hub.**

**Zones fail independently at compute and storage and may share a control plane** *(§69.2)* —
**Several large cloud incidents have exactly that shape: every resource healthy and nothing able
to be created, modified or authenticated.** **Which is why genuine resilience is multi-region**
and why most designs are not.

**The failure domains are named, and you are expected to use them** *(§69.2)* — **A single-zone
deployment that fails when that zone fails is your architecture's fault** — the shared
responsibility model applied to availability.

**Destroying and rebuilding the network in minutes changes disaster recovery** *(§69.2)* —
**The recovery is running the code in another region**, and Chapter 55 §55.4's inversion of
authority is the default here rather than a destination.

**Transit gateway route tables are segmentation by routing** *(§69.3)* — Production reaches
shared services and on-premises; development reaches shared services only; **and production and
development cannot reach each other because no route exists.** **Stronger than filter-based
segmentation** — no rule to misorder, no state to exhaust, no path permitted by accident.

**Traffic to a service in the same region leaves the provider's network and comes back**
*(§69.3)* — **Paying NAT processing and egress, and traversing the Internet.** **An endpoint
fixes it, and the security benefit is larger than the cost one**: the private subnet then needs
no route to the Internet at all, **which removes the exfiltration route** (Chapter 57 §57.4's
"the strongest control is absence").

**Multicloud for resilience rarely delivers what is claimed** *(§69.3)* — **The failure modes
that take out a provider are frequently the control plane, and the application cannot fail over
faster than a human can decide.** **Most of the value attributed to it is available from
multi-region within one provider**, at a fraction of the cost.

**A global load balancer is anycast sold as a product** *(§69.4)* — One address from every edge,
BGP delivers the user to the nearest, TLS terminates there, and the request crosses the
provider's backbone. **Which is a CDN's architecture applied to dynamic traffic**, and why the
distinction has dissolved.

**A health endpoint that returns 200 unconditionally never fails; one that checks every
dependency fails when any does** *(§69.4)* — **Two endpoints: liveness for the orchestrator,
readiness for the load balancer** — **and readiness should check what the instance needs, not
what it merely uses.**

**Readiness passing before the application is warm is the commonest cause of deployment errors**
*(§69.4)* — And **a drain timeout shorter than the longest legitimate request is the second.**

**One ingress controller collapses fifty per-service load balancers into one** *(§69.4)* — And
**the Gateway API's contribution is role separation**: infrastructure defines the Gateway,
applications define the Routes — Chapter 59 §59.3's separation of duties, expressed as an API.

**A mesh's strongest argument is uniformity** *(§69.4)* — **Six languages means six retry
implementations, six TLS configurations and six metrics conventions; a mesh has one**, operated
by a platform team rather than reimplemented by each application team.

**Two sidecar hops per call, and twelve sequential calls is 18 ms** *(§69.4)* — Negligible for
one, not for twelve. **Plus a sidecar per pod's resources, a new control plane, and a debugging
boundary with five candidates for any failure** — **where the mesh's own telemetry is the only
discriminator, which is a dependency on the thing that may be broken.**

**A mesh deployed without a team to operate it will be blamed for every application fault and
understood by nobody** *(§69.4)* — **"We have twelve services and we installed Istio" is a
recognisable and expensive mistake**, and a library gives most of the benefit at the cost of the
uniformity the mesh exists to provide.
