# 56.4 Disaster Recovery, RPO and RTO

Two numbers make disaster recovery discussions concrete, and stating them per system forces
the conversation to be specific rather than aspirational.

## The two numbers

```
   ◀──────── RPO ────────┤ DISASTER ├──────── RTO ────────▶
   
   last good state       the event      service restored
   
   "how much work        "how long until we are
    may we lose?"         working again?"
```

| | **RPO — recovery point objective** | **RTO — recovery time objective** |
|---|---|---|
| Measures | **data loss, in time** | **outage duration** |
| Determined by | **backup or replication frequency** | **standby capacity and rehearsal** |
| An RPO of 1 hour means | **at most one hour of work is lost** | |
| An RTO of 4 hours means | | **service is back within four hours** |
| Cost driver | **replication bandwidth and storage** | **idle capacity and practice** |

**They are independent**, and this is the point most often missed.

> A system can have an RPO of zero and an RTO of a week — perfectly replicated data, and no
> hardware to run it on. Or an RPO of a day and an RTO of ten minutes — a nightly backup and
> a warm standby that restores fast. **Neither is wrong; they are different requirements.**

## Deriving the requirement from the cost

Both numbers are business decisions, and both should be derived rather than chosen.

RPO follows from the cost of re-doing lost work:

| Question | Implies |
|---|---|
| **Can the work be reconstructed?** | **If yes, RPO can be long.** An hour of re-keyed orders is recoverable. |
| **Is it recorded elsewhere?** | Email, paper, a partner's system |
| **Is it regulated?** | **Financial transactions frequently have a zero-loss requirement** |

RTO follows from the cost of the outage (§56.1's cost-per-hour), and from how long the
business can operate without the system — which is not the same thing. Many
organisations can operate manually for a day and not for three.

**And the derived requirement is per system:**

| System | RPO | RTO | Why |
|---|---|---|---|
| **Finance database** | **5 min** | **1 hour** | transactions cannot be reconstructed |
| Customer-facing web | 1 hour | **15 min** | **revenue stops immediately; content is reproducible** |
| Email | 15 min | 4 hours | |
| **File shares** | 4 hours | 8 hours | staff work locally in the meantime |
| **Internal wiki** | **24 hours** | **24 hours** | nobody dies |
| **Network configuration** | **on change** | **1 hour** | **Chapter 55 §55.4 — and everything else depends on it** |

> Uniform targets across all systems are a sign that nobody did the analysis. "RPO 1 hour,
> RTO 4 hours, for everything" means the wiki is over-protected and the finance database is
> under-protected, and both cost money.

## What the numbers buy, and what they cost

**RPO's ladder:**

| RPO | Mechanism | Relative cost |
|---|---|---|
| **24 h** | **nightly backup** | **low** |
| 4 h | backup every 4 hours | low |
| **1 h** | **hourly incremental / log shipping** | moderate |
| **Minutes** | **continuous replication, asynchronous** | **high** |
| **Zero** | **synchronous replication** | **very high — and it costs latency** |

**Synchronous replication deserves its warning:**

> A synchronous write must be acknowledged by both sites before it completes, so every
> write pays the round-trip latency to the second site. At 100 km that is about 1 ms; at
> 1,000 km it is 10 ms (Chapter 50 §50.5), and an application performing thousands of
> sequential writes becomes unusable.

Which is why synchronous replication is a metropolitan-distance technology, and why a
"disaster recovery site 20 km away" is a real design rather than a compromise — far enough
for most events, close enough for synchronous writes. The trade against a genuinely distant
site is regional events: flood, grid failure, civil disruption.

**RTO's ladder:**

| RTO | Mechanism | Relative cost |
|---|---|---|
| **Days** | **rebuild from backup on procured hardware** | **very low** |
| **24 h** | **cold site — space and power, no equipment** | low |
| **4 h** | **warm site — equipment present, data restored on demand** | moderate |
| **< 1 h** | **hot site — running, data current, needs a switch** | **high** |
| **Seconds** | **active–active — both sites serving** | **highest, and it changes the application** |

Active–active is not simply "hot site, but better." It requires the application to
tolerate being run in two places at once, which is an application property rather than an
infrastructure one — and retrofitting it is frequently impossible.

## The network's part

Three things the network must supply and which are frequently forgotten until the test.

**Addressing.** Does the recovered service keep its IP address?

| Approach | Consequence |
|---|---|
| **Stretched Layer 2 between sites** | **addresses move; nothing reconfigures; and you now have one failure domain across two sites** |
| **Different subnets, DNS change** | **clean, and bounded by DNS TTL** — **set it low in advance** |
| **Route the prefix from either site** | **clean, needs your own address space and BGP** (Chapter 48 §48.3) |
| **Global load balancer / anycast** | Chapter 52 §52.4 |

> Stretched Layer 2 for disaster recovery is the option that looks easiest and is worst.
> It couples the two sites into one broadcast domain — one spanning tree, one set of
> broadcast storms, one failure — **which defeats the purpose of having two sites.** The
> industry has learned this repeatedly and it is still proposed.

**DNS TTL.** A failover that depends on DNS is bounded by the TTL, and by resolvers that
ignore it. Lower the TTL to 60 seconds days before a planned migration, and keep DR
records at a low TTL permanently — the extra query load is negligible and the alternative is
waiting an hour.

**Capacity.** The DR site's network must carry the load, and it frequently is sized for
the equipment rather than for the traffic. A warm site with a 100 Mb/s circuit cannot serve
what the primary served over 10 Gb/s, and this is discovered during the test — if there is
a test.

## Testing the plan

> A DR plan that has never been executed is a document, not a capability.

Organisations that discover this during an actual disaster discover it in the worst possible
way, and the list of them is long and public.

The ladder of test types, and each finds different things:

| Type | What it is | Finds |
|---|---|---|
| **Walkthrough** | read the plan aloud together | **missing steps, wrong contacts, ambiguity** |
| **Tabletop** | **simulate an incident, discuss decisions** | **decision-making gaps, unclear authority** |
| **Component test** | **fail one thing for real** | **that one thing** |
| **Parallel test** | **stand up DR alongside production** | **capacity, data currency, configuration drift** |
| **Full failover** | **run production from DR for a period** | **everything, including what nobody thought of** |

Most organisations do the first two and call it testing. The first two are worth doing and
they do not establish that the recovery works.

**What makes a full failover test useful:**

- Run on DR for a full working day, at least. Problems that appear under real load do not
  appear in an hour.
- **Fail back deliberately**, and time it. Failback is harder than failover and is almost
  never rehearsed.
- Exclude the people who built it (Chapter 53 §53.4). If the runbook does not suffice, the
  finding is the runbook.
- Do not pre-warm anything. A test in which someone "just checks" the standby the day
  before is a test of a system that has been maintained for the test.
- Record what went wrong, and fix it. A test that surfaces twelve problems is a successful
  test. A test that surfaces none was not a test.

And the cultural point, which determines whether any of this happens:

> A failover test that reveals problems must be treated as a success, or it will be the last
> one. If the outcome of finding twelve issues is criticism of the team, the next test will
> be arranged so as not to find any.

## The plan itself

What a usable DR plan contains, beyond the RPO/RTO table:

| | |
|---|---|
| **Declaration criteria and authority** | **who decides this is a disaster, and on what basis** |
| **The order of restoration** | **dependencies first — network, DNS, authentication, then applications** |
| **Contact details**, in text | **not in a system that may be down** (Chapter 53 §53.4) |
| **What is not covered** | **stated explicitly** |
| **The manual workaround** | **what the business does in the meantime** |
| **Communication plan** | staff, customers, regulators |
| **The plan's own location** | **offline, and at both sites** |

"Who decides" is the item most often missing and most costly. An hour spent establishing
whether to invoke DR is an hour of the RTO, and the decision is frequently made by someone
who is asleep and unreachable. Name a role, name a deputy, and state the criteria.

And the order of restoration is a network engineer's contribution. Nothing works before
the network, DNS and authentication do — and an organisation that starts by restoring its
finance database into a site with no DNS has spent its RTO learning about dependencies.

## What breaks here

RPO met and RTO missed by a factor of ten. The data was replicated and nothing was
rehearsed. They are independent, and both must be designed.

**Synchronous replication making the application unusable.** Distance latency on every
write. Asynchronous, or a closer site.

**A DNS-based failover that takes an hour.** **The TTL.** Lower it permanently for DR records.

A DR site that cannot carry the load. Sized for equipment, not for traffic.

A stretched Layer 2 DR design that took out both sites. **One broadcast domain.** This is a
known outcome and it is still proposed.

Nobody invoked DR because nobody was authorised to. Declaration criteria and authority
were not in the plan.

A recovery that restored the applications and not DNS. **Dependency order.**

**A test that found nothing.** It was not a test. Something was pre-warmed, or the scope
excluded the hard part.

A DR plan stored on the file share that is down. Offline copies, at both sites.
Chapter 53 §53.4's point, at its sharpest.

> **Network+ note.** Objective 3.3 covers business continuity. Over-learn: RPO is the maximum
> acceptable data loss and RTO the maximum acceptable downtime; cold, warm and hot sites
> differ in readiness and cost; **MTTR and MTBF relate to availability**; and **plans must be
> tested.** The RPO/RTO distinction is examined constantly and the two are frequently confused —
> **RPO points backwards, RTO points forwards.**
