# Chapter 69 — The People

**This chapter's history is corporate and recent**, and **most of the significant work was done
inside companies and published afterwards, if at all.** **What follows is the part that is
documented.**

**Chris Pinkham and Benjamin Black.** **The 2003 document that became AWS.**

**Pinkham and Black wrote a paper for Amazon's leadership describing what the company's
infrastructure would look like if every component were exposed as a service with an API** —
**storage, compute, and networking.**

> **The observation was that Amazon had built, for its own retail operation, an internal
> platform that other organisations would pay for**, and **that the discipline of exposing
> everything through APIs — which Bezos had mandated internally — had incidentally produced a
> product.**

**EC2 launched in 2006**, and **the network was the part that took longest**, because **the
others could be built on existing abstractions and the network could not:** **giving each
customer an isolated, routable, configurable network on shared physical infrastructure had no
precedent at that scale.**

**Werner Vogels** — **Amazon's CTO from 2005 — supplied the architectural framing that shaped
what followed.**

**Two of his positions are directly this chapter's:**

> **"Everything fails, all the time."** **Which is Chapter 56's assume-breach argument, stated as
> a design premise rather than as a caution** — **and it is why availability zones are named,
> documented and expected to be used.**

**And the shared responsibility model**, which **Vogels and his contemporaries articulated
because the alternative — the provider being responsible for everything a customer configures —
was neither possible nor desirable.**

**Vogels's earlier academic work on distributed systems and eventual consistency** — **and the
Dynamo paper (2007), which he co-authored** — **is the intellectual foundation of a great deal
of cloud architecture**, and **its arguments about the trade between consistency and
availability are the ones underneath every multi-region design decision.**

**Eric Brewer (b. 1967).** **The CAP theorem, 2000 — and the correction, 2012.**

**Brewer's conjecture** — **that a distributed system cannot simultaneously guarantee
consistency, availability and partition tolerance** — **was formalised by Gilbert and Lynch in
2002 and became the most-cited and most-misused result in distributed systems.**

**His 2012 essay "CAP Twelve Years Later" is the more useful document:**

> **"The 2 of 3 formulation was always misleading."** **Partitions are rare; the choice between
> consistency and availability is made only during a partition; and the interesting engineering
> is what the system does before, during and after one** — **not which two letters were
> selected.**

**And the network relevance is direct:** **CAP's "partition" is a network partition** (Chapter 56
§56.2, Chapter 68 §68.1), **so every distributed system's guarantees are conditional on the
network's behaviour** — **which makes the network's failure modes an application architecture
concern rather than a separate discipline.**

**Brewer is also worth knowing for Google's own cloud work and for the observation that
"partition tolerance" is not optional** — **you do not choose whether partitions occur.**

**The OpenStack contributors, and a cautionary history.**

**OpenStack (2010, from NASA and Rackspace) was the open-source answer to AWS**, and **its
networking component — Quantum, then Neutron — is instructive.**

> **Neutron was ambitious and difficult.** **It attempted a plugin architecture spanning every
> vendor's approach, and the result was a component with a reputation for complexity and
> fragility that the project never fully shed.**

**OpenStack persists** — **in telecommunications, in some private clouds, in China** — **and it
did not become what it intended.** **The reasons are worth knowing:**

**Operating a cloud is harder than using one.** **The abstraction is the product**, and an
organisation that installs OpenStack has acquired the abstraction and all of the operational
burden it was meant to hide.

**And the public providers moved faster.** **A project coordinating dozens of vendors could not
match the release cadence of a company shipping to its own infrastructure.**

**Brendan Burns, Joe Beda and Craig McLuckie.** **Kubernetes, 2014 — and Google's Borg before
it.**

**Chapter 67 §67.1 covered the networking model. What belongs here is the decision to open
it.**

**Google had run Borg internally for a decade** — **the cluster manager from which Kubernetes
descends** — **and the choice to release an open reimplementation rather than a service was
strategic:**

> **A proprietary orchestration layer would have been a barrier to using Google's cloud.** **An
> open one that ran everywhere made the orchestration a commodity and the infrastructure the
> competition** — **which is Chapter 68's commoditisation argument, executed deliberately by a
> company that benefited from it.**

**And it worked.** **Kubernetes is the abstraction, every provider offers it, and workloads are
substantially more portable than they were** — **which is the closest thing to the multicloud
promise that has actually arrived, and it arrived at the orchestration layer rather than at the
network layer.**

**The Envoy and Istio teams — Matt Klein, and the Google/IBM/Lyft collaboration.**

**Envoy (Lyft, 2016)** is **the proxy underneath most service meshes**, and **Klein has been
notably candid about what a mesh costs.**

> **His public position — that a service mesh is a substantial operational commitment and that
> many organisations adopting one do not need it** — **is unusual from someone whose work
> depends on its adoption**, and it is §69.4's argument stated by the person best placed to
> make it.

**And Envoy's success independent of Istio is worth noticing:** **it is the data plane of
several meshes, of API gateways, of ingress controllers and of edge proxies** — **because a
well-designed, well-documented proxy with a dynamic configuration API turned out to be more
generally useful than the mesh it was built for.**

## What this chapter's history shows

**Three observations.**

**The network was the hard part.** **Compute and storage virtualisation had precedents;
multi-tenant isolated networking at scale did not**, and it is why EC2's network took longest and
why the abstractions still differ most between providers.

**Operating the abstraction is harder than using it.** **OpenStack's history is the evidence**,
and **the same lesson applies to every "we will build our own" proposal** — the abstraction is
the product, and the operational burden is what the provider is charging for.

**And commoditisation succeeded where it was executed by a beneficiary.** **Kubernetes
commoditised orchestration because Google gained from it; OpenFlow failed to commoditise
switching because no vendor gained from it** (Chapter 68 §68.2). **Which is the same economics
argument, twice, with opposite outcomes** — and it is the most reliable predictor available of
whether an open standard will be implemented well.
