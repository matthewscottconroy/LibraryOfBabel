# Chapter 56 — Availability and Recovery

"We need five nines."

It is said in meetings constantly, usually by someone who has not computed what it
means, and the arithmetic is worth having at hand because it converts an aspiration
into a budget.

| Availability | Downtime per year | Per month | Per week |
|---|---|---|---|
| 99% | 3.65 days | 7.3 hours | 1.7 hours |
| 99.9% | 8.77 hours | 43.8 min | 10.1 min |
| 99.99% | 52.6 min | 4.4 min | 1.0 min |
| 99.999% | 5.26 min | 26 sec | 6 sec |
| 99.9999% | 31.6 sec | 2.6 sec | 0.6 sec |

Five nines is **5.26 minutes of downtime per year, including all planned
maintenance**. That single clause defeats most claims immediately: an organisation
that reboots its firewalls quarterly for patching has already spent its entire annual
budget, several times over, on planned work.

Achieving it requires redundancy in every component including power and cooling,
automated failover measured in seconds, no single maintenance window that takes
anything fully offline, and staff available continuously. Each additional nine costs
roughly an order of magnitude more than the last, and the honest engineering
conversation is not "how many nines can we have" but "what does an hour of downtime
cost us, and at what point does prevention cost more than the outage."

Frequently the correct answer is three nines with a fast, rehearsed recovery. §56.1
makes that argument with numbers.

## Redundancy's failure mode

The naive model of redundancy is that two components fail independently, so if each has
availability *A*, the pair has 1 − (1 − *A*)². Two devices at 99% give 99.99%.

The model is wrong in practice, and the reason is the most important idea in this
chapter: **components that share a fate are not independent.**

Two power supplies fed from the same circuit. Two switches in the same rack, on the
same UPS, cooled by the same unit. Two fibres in the same duct — a common failure, and
the duct is invariably found by the same excavator. Two routers running the same
firmware version, sharing the same defect. Two data centres served by the same
electricity substation. Two cloud availability zones that turn out to share a control
plane.

The discipline is to enumerate **shared fate** explicitly: for each redundant pair,
what could take out both? The answer is usually something, and often something
addressable at low cost — different circuits, different ducts, staggered firmware
versions, different vendors for a critical pair.

§56.2 also covers the failure that redundancy itself introduces: **the failover that
does not work.** A backup component that has never been exercised is of unknown
status. A standby firewall whose configuration drifted from the primary will fail over
into a broken state. A generator that has not been load-tested will not start. The
practice that fixes this is deliberate, scheduled failover testing — which is
uncomfortable, is the reason it is skipped, and is the only way to know.

## First-hop redundancy

The specific mechanism worth knowing in detail, because it addresses a single point of
failure that Chapter 25 §25.3 built into every host.

A host has one default gateway address. If the router holding that address fails, the
host cannot reach anything remote, and it will not discover an alternative — it has
one static entry and no mechanism to change it.

**FHRP** — first hop redundancy protocols — solve this by having two or more routers
share a **virtual IP address**, with one active at a time. VRRP is the open standard
(RFC 5798); HSRP and GLBP are Cisco's. Hosts point at the virtual address; if the
active router fails, a standby takes over the virtual address and the virtual MAC, and
hosts notice nothing at all — not even an ARP change, because the MAC moves too.

§56.2 covers configuration, failover timing, preemption and its hazards, and the
common design error of putting the FHRP active router and the spanning tree root
(Chapter 19 §19.3) on different devices, which produces traffic that crosses the
inter-switch link twice for no reason.

## The physical plant

§56.3 covers what network engineers usually treat as somebody else's problem until
the day it is not.

**Power.** UPS for ride-through, generator for duration, and the arithmetic of runtime
against load. Dual feeds from separate circuits — and the check that the "separate"
circuits are not the same board. Devices with a single power supply, which are a
single point of failure regardless of everything else in the rack.

**Cooling.** Heat is the primary cause of electronic ageing, and Chapter 4 §4.3 noted
that thermal noise rises with temperature — a hot cabinet performs measurably worse
before it fails. Hot aisle/cold aisle containment, and the fact that a cooling failure
becomes an outage in minutes, not hours.

**Environmental monitoring**, which is cheap and routinely absent: temperature,
humidity, water detection, door contacts. The cost of a sensor is trivial against the
cost of discovering a cooling failure the following morning.

## Recovery

§56.4 defines the two numbers that make disaster recovery discussions concrete.

**RPO — recovery point objective.** How much data may be lost, measured in time. An
RPO of one hour means losing at most one hour of work, which means backing up at least
hourly.

**RTO — recovery time objective.** How long restoration may take.

The two are independent and both cost money, but not in the same way — RPO is largely
a replication cost, RTO largely a standby-capacity and rehearsal cost. Stating them
separately, per system, forces the conversation to be specific: the finance database
may need RPO of minutes and RTO of an hour; the internal wiki may be fine with a day
of each. Uniform targets across all systems are a sign that nobody did the analysis.

And the practice that determines whether any of it works: **test the plan.** A DR plan
that has never been executed is a document, not a capability. Organisations that
discover this during an actual disaster discover it in the worst possible way, and the
list of them is long and public.

## By the end you will be able to

- Convert an availability target into downtime and assess whether it is achievable
  given planned maintenance.
- Identify shared fate in a redundant design and propose mitigations.
- Explain and configure an FHRP, and align it correctly with spanning tree.
- Specify power, cooling and environmental monitoring for a network room.
- Define RPO and RTO for a set of systems and derive the backup and standby
  requirements from them.
- Design a failover test that would actually reveal a problem.
