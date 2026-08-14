# 60.4 Segmentation, DMZs and Microsegmentation

Chapter 57 §57.1 identified lateral movement as the step at which a foothold becomes a
catastrophe. This section is the countermeasure.

## The arithmetic

Segmentation's value is quantifiable, which is unusual for a security control.

**A flat network of 2,000 hosts:**

| Design | **Reachable from one compromised host** | Reduction |
|---|---|---|
| **Flat** | **1,999** | — |
| 4 segments | 499 | **75%** |
| **20 segments** | **99** | **95%** |
| 100 segments | 19 | **99%** |

> **Every additional segment reduces what a compromise reaches.** It does not prevent the
> compromise, and it changes the outcome from "the organisation" to "one department's
> workstations."

**And the second effect matters as much:** crossing a segment boundary is an event that can be
logged, alerted on, and blocked. Lateral movement within a flat network is invisible;
lateral movement across a policy enforcement point is a detection opportunity.

## The traditional design

```
                          Internet
                             │
                    ┌────────┴────────┐
                    │    Firewall     │
                    └──┬──────┬───────┘
                       │      │
              ┌────────┴──┐  ┌┴──────────────┐
              │    DMZ    │  │   Internal    │
              │ web, mail │  │ users, servers│
              │ DNS, VPN  │  │               │
              └───────────┘  └───────────────┘

   Internet → DMZ:      permitted, specific services
   Internet → Internal: DENIED
   DMZ → Internal:      DENIED, or narrowly permitted
   Internal → DMZ:      permitted
   Internal → Internet: permitted, filtered
```

The DMZ's purpose is precisely the "DMZ → Internal: DENIED" line.

> A DMZ is not "where the public servers go". It is a zone from which a compromise cannot
> reach the internal network — and if a compromised web server can reach the internal
> database with full credentials, the DMZ is decorative.

Which is the most common DMZ failure: the web server needs the database, so a rule is added
permitting it, and the rule is broad because narrowing it took effort. The correct answer is
a specific rule to a specific host on a specific port, or — better — an application tier in
between.

**The three-tier version** — web in the DMZ, application in a second zone, database in a
third, each permitted only to speak to its neighbour — is the design this argument produces,
and its cost is the additional zones and rules.

## Segmentation approaches

Five, in increasing order of granularity and cost.

| Approach | Boundary | Enforcement |
|---|---|---|
| **Physical separation** | **separate networks entirely** | **air gap; the strongest and least flexible** |
| **VLANs** | **broadcast domains** (Chapter 20) | **a router or firewall between them** |
| **VRFs** | **separate routing tables** | **no route means no reachability** |
| **Firewall zones** | policy zones | the firewall |
| **Microsegmentation** | **per workload** | **a distributed enforcement point** |

**Two clarifications that matter.**

> **A VLAN is not a security control. It is a broadcast domain.** Two VLANs on the same switch
> are separated only until something routes between them, and **in almost every network,
> something does.** The security control is the policy applied at the routing point, and a
> VLAN without one is organisational, not protective.

**VRFs are stronger than VLANs and under-used.** Two VRFs on the same router have separate
routing tables, so there is no path between them at all unless one is deliberately created
— which is a much better default than "routed by default, filtered by policy."

## Microsegmentation

Policy per workload rather than per subnet, and the arithmetic explains both its power and
its difficulty.

**Enforcement happens at the workload:** a hypervisor's virtual switch, a host firewall, a
container network policy, or an agent — so traffic between two virtual machines on the same
host is filtered without leaving the host.

```
   Traditional:                        Microsegmented:

   VM ─┐                               VM ─[policy]─┐
   VM ─┼─ same VLAN, unfiltered        VM ─[policy]─┼─ every flow evaluated
   VM ─┘                               VM ─[policy]─┘
        │                                    │
     firewall (only for                 the enforcement point
     traffic leaving)                   is at every workload
```

**And the difficulty is the policy count:**

| Workloads | **Possible pairwise flows** |
|---|---|
| 50 | 1,225 |
| **500** | **124,750** |
| 5,000 | **12,497,500** |

> **Nobody writes twelve million rules.** Microsegmentation is only tractable with
> label-based policy — "web tier may reach application tier on 8443" — where the labels come
> from the orchestration system and the rules are generated. A microsegmentation project
> that begins by enumerating flows will not finish.

Which is why it succeeded in cloud and container environments first: the orchestrator
already knows what every workload is, and the labels exist. In a traditional data centre
with unlabelled servers, the labelling is the project.

**The realistic sequence:**

1. **Discover** — observe actual flows for weeks (Chapter 54 §54.4's flow records)
2. **Label** — by application, tier and environment
3. Author policy in monitor mode — log what would be blocked
4. **Tune** — for weeks; the discovery will be incomplete
5. **Enforce**, one application at a time

Steps 1 and 3 are the ones that are skipped, and skipping them is what produces the outages
that give microsegmentation its reputation.

## The management plane

The segment that matters most and is most often absent.

> A compromised network management system is a position from which to reconfigure every device
> in the estate (Chapter 57 §57.3).

**The correct arrangement:**

| | |
|---|---|
| **A dedicated management network or VRF** | **not reachable from user networks at all** |
| **Out-of-band access** | **a separate physical path — console servers, dedicated management ports, a cellular modem** |
| **Jump hosts / bastions** | **the only path in, logged and monitored** (Chapter 59) |
| **No Internet access from it** | **management systems do not need to browse** |

The out-of-band point is the one that is under-funded until it is needed.

> **In-band management fails exactly when you need it: during the outage.** A misconfigured ACL
> that removes your own access, a routing failure, a switch that will not boot — all require
> a path that does not depend on the network working.

And a console server plus a cellular modem is inexpensive relative to a site visit, which is
the comparison that wins the argument.

## Segmentation as a compliance driver

Worth an honest paragraph, because it is frequently the actual funding source.

**PCI DSS is the clearest case:** the requirements apply to the cardholder data environment,
and everything that connects to it. A flat network means the entire network is in scope,
which is enormously expensive to assess and remediate.

> **Segmenting reduces scope**, and the saving in assessment cost frequently exceeds the cost
> of the segmentation — which is a rare instance of a security control funding itself in
> terms a finance director accepts without argument.

The same applies to other regimes — healthcare, industrial safety, government
classification — and using scope reduction as the business case is legitimate and effective
even where the actual motivation is Chapter 57's ransomware argument.

## What actually gets segmented, in practice

A defensible priority order for an organisation starting from flat.

| Priority | Segment | Why |
|---|---|---|
| **1** | **Management plane** | **the highest-value target; and it is cheap** |
| **2** | **Backups** | **Chapter 57 §57.1 step 5 — and this alone changes ransomware outcomes** |
| **3** | **Servers from workstations** | **the largest single reduction in lateral movement** |
| **4** | **OT / industrial / building systems** | **different lifecycle, different patching, frequently unpatchable** |
| **5** | **Guest and BYOD** | easy, visible, and expected |
| **6** | **IoT and cameras** | Chapter 47's devices, which are rarely patchable |
| **7** | **By business function** | the long tail |
| **8** | **Microsegmentation within tiers** | **the expensive part, last** |

> Items 1 to 3 deliver most of the benefit for a small fraction of the cost, and an
> organisation that does only those has substantially changed its outcomes. Starting with
> item 8 is how segmentation projects fail.

## What breaks here

A DMZ whose servers can reach the internal network with full credentials. **Decorative.**
The "DMZ → Internal: DENIED" line is the DMZ.

**VLANs treated as a security control.** A VLAN is a broadcast domain. The control is at the
routing point.

**A microsegmentation project that never finishes.** It began by enumerating flows rather than
by labelling.

**Microsegmentation enforcement causing outages.** Monitor mode was skipped, or the
discovery period was too short.

**Management systems reachable from the user network.** The highest-value target, undefended.

**No out-of-band access, discovered during an outage.** In-band management fails when you need
it.

**Segmentation everywhere and lateral movement still succeeded.** Check what the segments
actually permit — a rule permitting SMB between every segment defeats the design, and it was
added because file sharing broke.

A backup server in the same segment as the servers it backs up, with domain credentials.
Chapter 57 §57.1 step 5. This is the single most valuable segment to create.

**PCI scope including the entire network.** **Flat.** Segmentation reduces it, and the assessment
saving pays for the work.

> **Network+ note.** Objective 4.3 and 1.2 cover segmentation. Over-learn: a DMZ is a
> perimeter network hosting public-facing services, separated from the internal network;
> VLANs provide logical segmentation and require a routed boundary for enforcement;
> **microsegmentation applies policy per workload**; **screened subnet is the current term for
> DMZ**; and **segmentation limits lateral movement.** The DMZ concept and the
> lateral-movement rationale are both examined.
