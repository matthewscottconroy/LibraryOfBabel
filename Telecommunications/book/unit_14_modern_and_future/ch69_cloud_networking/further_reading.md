# Chapter 69 — Further Reading

## Read these first

**The provider's own networking documentation** — **AWS VPC, Azure Virtual Network, Google VPC.**
**Genuinely, and read one thoroughly rather than three superficially.** **They are well written,
current, free, and they are the authority** — every third-party summary is a lossy copy.

**The AWS Well-Architected Framework's Reliability and Performance pillars**, and the
**Azure and Google equivalents.**
**Vendor material and unusually good** — **the failure-mode discussions and the multi-zone
guidance are specific and testable**, and they state the availability commitments precisely
enough to design against.

**Brewer, E. (2012). "CAP Twelve Years Later: How the 'Rules' Have Changed." *IEEE Computer*.**
**Four pages, and it corrects the version everyone learned.** **The relevant point for a network
engineer is that CAP's partition is a network partition**, so the application's guarantees are
conditional on your network's behaviour.

**Vogels, W. — the "Everything fails, all the time" material**, and **the Dynamo paper
(DeCandia et al., 2007)**.
**The design premise underneath cloud architecture**, and Chapter 56's argument stated as a
starting assumption rather than as a caution.

## Cloud networking specifically

**The providers' networking deep-dive conference sessions** — **AWS re:Invent's "Advanced VPC
Design" and "Networking Deep Dive" tracks, and the Azure and Google equivalents.**
**Free, recorded, technical, and consistently the best available material on what the
abstractions actually do underneath.**

**Each provider's flow log documentation and format reference.**
**F2 uses it.** **Flow logs are your primary diagnostic instrument here** (Chapter 54 §54.4), and
knowing the field semantics — particularly what "ACCEPT" and "REJECT" mean and which device
recorded it — is the difference between reading them and guessing.

**The providers' MTU documentation.**
**F8 uses it.** **The effective MTU differs by path, by provider and by gateway type**, and it is
the cause of a specific and confusing class of fault (Chapter 66 §66.3).

**Each provider's networking limits and quotas page.**
**Route table entries, security group rules, peering connections, endpoint counts.** **Design
against them rather than discovering them**, and several are soft limits that must be requested
in advance.

## Architecture

**Barroso, L., Hölzle, U. & Ranganathan, P. — *The Datacenter as a Computer*.**
**Free**, recommended in Chapters 56 and 67, **and the context for what a provider is actually
operating.**

**Burns, B. — *Designing Distributed Systems*, and Burns & Beda & Hightower — *Kubernetes: Up
and Running*.**
**For the orchestration layer this chapter's §69.4 sits on.**

**Newman, S. — *Building Microservices* (2nd ed.).**
**Not a networking book**, and its chapters on service communication, on resilience patterns and
on when *not* to decompose are the necessary context for §69.4's mesh argument.

**Nygard, M. — *Release It!*** (Chapter 56's reading) — **circuit breakers, bulkheads and
timeouts**, which is what a mesh implements and what a library implements instead.

## Service mesh, read sceptically

**Klein, M. — his writing and talks on Envoy and on service mesh adoption.**
**Notably candid about the operational cost**, from the author of the proxy underneath most of
them.

**The Istio, Linkerd and Cilium Service Mesh documentation** — **read all three's "do you need
a service mesh?" sections**, which differ and are each honest about different things.

**The CNCF's service mesh landscape material**, used to know what exists rather than for
guidance.

**Buoyant's (Linkerd's) writing on mesh complexity** — **the argument for a smaller mesh**, and
it is the most rigorous available comparison of the resource costs.

## Cost, which is a network concern here

**The providers' pricing pages** — **and use them rather than any figure in this book.**
**Egress, cross-zone, NAT gateway processing, load balancer capacity units, endpoint hours.**

**The FinOps Foundation's material** — **for the discipline of attributing and reducing cloud
cost**, and the network sections are directly §69.1's argument.

**Each provider's Cost and Usage report schema** — **because "which architecture decision is
costing this?" is answerable and almost never asked.**

## Hands-on

**LocalStack, or a provider's free tier** — **F1.** **Build a VPC by hand, then destroy it and
rebuild it from Terraform** (Chapter 70), **which demonstrates the chapter's central point about
the network being a document.**

**`kind` or `k3s` with an ingress controller** — **F4's material at laptop scale.**

**A service mesh in a test cluster** — **F5.** **Linkerd is the lightest to install and the
easiest to measure**; **Istio is what you will meet.**

**The providers' network reachability analysers** — **AWS Reachability Analyzer, Azure Network
Watcher, Google Connectivity Tests.**
**These are Chapter 68 §68.4's verification, provided as a service**, and they will tell you why
a packet cannot reach a destination without sending one. **Under-used, and they are the closest
available replacement for the packet capture you no longer have.**

**Traffic mirroring, where the provider offers it** — **expensive, limited, and the only true
packet capture available.** **Know whether you can, and what it costs, before you need it.**

## Following the field

**The providers' networking release notes and "what's new" feeds.**
**The abstractions change**, and an architecture built on last year's limits will meet this
year's.

**The CNCF's networking projects** — **Cilium, Envoy, the Gateway API** — **where the container
networking model is being developed.**

**Cloud outage post-mortems** (Chapter 56's reading) — **and specifically the ones where the
control plane failed while every resource remained healthy**, which is the failure mode §69.2
describes and which the marketing does not.

**`ipSpace.net`'s cloud networking material** — **the sceptical treatment**, and Pepelnjak's
comparisons of the providers' models are more rigorous than anything the providers publish about
each other.

## Where to look next

**Chapter 70** is how the network in this chapter is actually built and changed — a document, in
version control, applied by a tool; **Chapter 67** is the same architecture built rather than
rented; **Chapter 51 §51.3** is the enterprise side of §69.3's connectivity; and **Chapter 56**
supplies the availability arithmetic that the zone and region decisions require.
