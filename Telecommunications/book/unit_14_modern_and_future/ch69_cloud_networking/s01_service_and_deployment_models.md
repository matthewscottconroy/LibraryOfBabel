# 69.1 Service and Deployment Models

**The definitions are worth stating precisely once**, because **they are used loosely everywhere
and the distinction determines who is responsible for what** — which is the only part that
matters operationally.

## The three service models

**NIST's definitions, and the useful way to read them is as a division of responsibility.**

```
   ┌─────────────────┬──────────┬──────────┬──────────┐
   │                 │   IaaS   │   PaaS   │   SaaS   │
   ├─────────────────┼──────────┼──────────┼──────────┤
   │ Application     │   YOU    │   YOU    │ provider │
   │ Data            │   YOU    │   YOU    │   YOU*   │
   │ Runtime         │   YOU    │ provider │ provider │
   │ Middleware      │   YOU    │ provider │ provider │
   │ Operating system│   YOU    │ provider │ provider │
   │ Virtualisation  │ provider │ provider │ provider │
   │ Servers         │ provider │ provider │ provider │
   │ Storage         │ provider │ provider │ provider │
   │ NETWORKING      │  shared  │ provider │ provider │
   └─────────────────┴──────────┴──────────┴──────────┘
                                          * you own the data everywhere
```

| | You get | You manage | Network responsibility |
|---|---|---|---|
| **IaaS** | **virtual machines, storage, networks** | **everything above the hypervisor** | **substantial — §69.2 is yours** |
| **PaaS** | **a runtime for your code** | **the code and its configuration** | **almost none — and you cannot see it** |
| **SaaS** | **a working application** | **users, data and configuration** | **none, except reaching it** |

> **The network engineer's involvement falls sharply from left to right**, and **so does the
> visibility.** **A PaaS deployment's networking is a black box with a security group in front
> of it**, and **the diagnostic tools of Chapter 64 do not apply to it at all.**

**And the fourth model that is not in NIST's list:**

**Serverless / FaaS.** **You supply a function; the provider runs it on demand.** **The network
is entirely the provider's**, and **the operational consequences are real: cold starts, no
persistent connections, no fixed egress address, and no way to capture a packet.**

## The deployment models

| | |
|---|---|
| **Public** | **the provider's infrastructure, shared** |
| **Private** | **dedicated, whether on your premises or hosted** |
| **Hybrid** | **both, with something joining them** — §69.3 |
| **Community** | shared by organisations with common requirements |
| **Multicloud** | **more than one public provider** — and it is not the same as hybrid |

**Hybrid and multicloud are routinely confused and have different problems:**

| | **Hybrid** | **Multicloud** |
|---|---|---|
| Joins | **your infrastructure to a provider's** | **two providers' infrastructures** |
| Connectivity | **Chapter 51 §51.3's direct connect or VPN** | **usually over the Internet, or an exchange** |
| **Egress cost** | **charged once** | **charged by both** (Chapter 51 §51.3) |
| Identity | **one directory, federated** | **two, and they must be reconciled** |
| Skills | **your network plus one provider's model** | **two providers' models, which differ** |
| **Why** | **latency, data gravity, regulation, legacy** | **avoiding lock-in, or acquisition** |

> **Multicloud is frequently an outcome rather than a strategy** — **an acquisition, a team that
> chose differently, a SaaS provider that runs somewhere else** — **and the version that was
> deliberately chosen to avoid lock-in usually costs more than the lock-in it avoids.**

**Because the abstractions do not match.** **A VPC, a VNet and a Google VPC differ in ways that
matter** (§69.2), **and code that manages one does not manage another**, so **"portable across
clouds" means "using the least capable common subset."**

## The shared responsibility model

**The framing that matters most and is most often misunderstood.**

> **The provider is responsible for the security *of* the cloud. You are responsible for security
> *in* the cloud.**

| Provider | You |
|---|---|
| **Physical facilities** | **your data** |
| **The hypervisor and host** | **your operating systems and patches** (IaaS) |
| **The network fabric** | **your security groups, routes and NACLs** |
| **The service's availability** | **your architecture's use of availability zones** |
| **Isolation between tenants** | **your isolation between your own workloads** |

**And the network-specific consequence:**

> **A misconfigured security group is your fault.** **A storage bucket exposed to the Internet is
> your fault.** **The overwhelming majority of publicised "cloud breaches" are customer
> misconfigurations**, not provider compromises — **and the distinction is not a provider's
> excuse; it is a statement about where to spend effort.**

## What the network engineer actually loses and gains

**Stated honestly, because the transition is disorienting.**

**What is lost:**

| | |
|---|---|
| **Packet capture** | **there is no port to span** — some providers offer mirroring, at cost |
| **Physical layer** | **no cables, no optics, no duplex** (Chapters 65, 66) |
| **`traceroute` inside** | **the fabric is invisible** |
| **Broadcast and multicast** | **generally not supported** — which breaks a class of application |
| **Direct device access** | **there is no device** |
| **MTU control** | **fixed by the provider, and it differs between them** |

**What is gained:**

| | |
|---|---|
| **The network is an API** | **Chapter 68's promise, arrived** |
| **Provisioning in seconds** | rather than weeks |
| **Configuration is code, by construction** | Chapter 70 |
| **The physical layer is someone else's** | **and its failures are too** |
| **Global reach without building it** | Chapter 50, rented |

> **The skills that transfer are the ones this book has spent seventy chapters on** —
> **addressing, routing, segmentation, load balancing, TLS, DNS, availability arithmetic** —
> **and the skills that do not are the ones that touch hardware.** **Which is why a network
> engineer moving to cloud finds the concepts familiar and the interface alien**, and the
> familiarity is the more important half.

## The costs that surprise

**Three, and they are network costs specifically.**

**Egress.** Chapter 51 §51.3. **Ingress is free and egress is charged, and it is the line that
makes multicloud expensive and makes exit difficult.**

**Cross-zone traffic.** **Traffic between availability zones is charged in most providers** —
**a few cents per gigabyte, in both directions** — **which makes a chatty application spread
across three zones surprisingly expensive**, and it is a design consideration rather than an
accounting one.

**NAT gateways and managed endpoints.** **A managed NAT gateway is charged per hour and per
gigabyte processed**, and **an application making many small outbound calls can spend more on
the NAT gateway than on the compute.**

> **The pattern is that the network is metered in ways an on-premises network is not**, and
> **an architecture that would be free in a data centre has a per-gigabyte cost in a cloud.**
> **Which changes design decisions** — placement, caching, protocol chattiness — **in ways that
> have nothing to do with performance.**

## What breaks here

**A PaaS application's networking cannot be diagnosed.** **Correct — it is not yours.** The
diagnostic boundary is the service's own logs and metrics.

**A multicloud architecture that costs more than the lock-in it avoided.** **Egress charged
twice, two skill sets, and the least capable common subset.**

**"The cloud is down" for a customer misconfiguration.** **Shared responsibility.** Check your
own configuration first; it is the more likely cause by a wide margin.

**An application requiring broadcast or multicast.** **Generally unsupported.** It must be
re-architected, and this is discovered during migration.

**A bill dominated by NAT gateway processing.** **Many small outbound calls.** A gateway endpoint
or a redesign.

**A chatty application spread across three availability zones.** **Cross-zone charges**, and it
is an architecture decision with a monthly invoice.

**No way to capture a packet.** **Provider mirroring where it exists, and endpoint-based
observability everywhere else** (Chapter 51 §51.4's argument, again).

> **Network+ note.** Objective 1.8 covers cloud concepts directly. Over-learn: **IaaS, PaaS and
> SaaS by what the provider manages**; **public, private, hybrid and community deployment
> models**; **the shared responsibility model**; **elasticity, scalability and multitenancy**;
> and **that cloud connectivity uses VPN or direct connection.** The service-model
> responsibility split is examined in almost every form.
