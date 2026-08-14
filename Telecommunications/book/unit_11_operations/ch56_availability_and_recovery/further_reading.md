# Chapter 56 — Further Reading

## The papers

Gray, J. (1985). "Why Do Computers Stop and What Can Be Done About It?" Tandem Technical
Report 85.7.
**Read this one.** Freely available, and it is the measurement that should change what you
do: administration and software dominate, hardware does not. Forty years old and it has been
replicated repeatedly since.

Gray, J. & Siewiorek, D. (1991). "High-Availability Computer Systems." *IEEE Computer*.
The follow-up, with the taxonomy of fault tolerance techniques. Good on the distinction
between fault masking and fast recovery.

Weibull, W. (1951). "A Statistical Distribution Function of Wide Applicability." *Journal
of Applied Mechanics*.
Historical, and worth knowing exists. The bathtub curve and its consequence for
same-batch redundant pairs is the transferable idea, and any reliability engineering text
covers it more accessibly.

Oppenheimer, D., Ganapathi, A. & Patterson, D. (2003). "Why Do Internet Services Fail, and
What Can Be Done About It?" USENIX.
Gray's study repeated for Internet services, with the same conclusion: operator error
dominates, and the most effective remedy is faster detection.

Barroso, L., Hölzle, U. & Ranganathan, P. — *The Datacenter as a Computer*.
**Free.** The chapters on failure rates at scale are the best available data on what actually
breaks, and they make the reliability arithmetic concrete with real numbers.

## Books

Beyer, B. et al. — *Site Reliability Engineering*, particularly **"Embracing Risk"** and
**"Service Level Objectives"** (free at sre.google).
Where error budgets come from, argued properly. Read these two chapters even if you read
nothing else from the book.

**Blank-Edelman, D. (ed.) — *Seeking SRE*.**
Essays, uneven, and several are excellent on the organisational side of availability — which
is where most of the difficulty actually is.

Limoncelli, T. et al. — *The Practice of System and Network Administration*, and *The
Practice of Cloud System Administration*.
The second volume's chapters on designing for failure and on disaster recovery are directly
this chapter, and are practical rather than theoretical.

**Nygard, M. — *Release It!***
About software, and the stability patterns — circuit breakers, bulkheads, timeouts — are
exactly §56.2's shared-fate argument applied inside an application. The failure case studies
are excellent.

Dooley, K. — *Designing Large-Scale LANs*, and Oppenheimer, P. — *Top-Down Network Design*.
The traditional network availability design references. Older, and the redundancy and
FHRP material is sound.

## Standards and references

RFC 5798 — "Virtual Router Redundancy Protocol Version 3."
Short, and the state machine and timer discussion are worth reading before tuning anything.

**RFC 5880 — Bidirectional Forwarding Detection.**
The better answer to fast failure detection than aggressive protocol timers, and Chapter 31
§31.4 covers it.

**ASHRAE TC 9.9 thermal guidelines.**
The source of the 18–27 °C recommended range. A summary is sufficient; the full document
is for data centre designers.

**Uptime Institute tier standards.**
**Read a summary.** The tier definitions are a useful vocabulary for what "redundant" means
concretely — concurrently maintainable, fault tolerant — and they are frequently
misquoted, so knowing the actual definitions is worth ten minutes.

ISO 22301 — business continuity management, and NIST SP 800-34 — contingency planning.
Consult rather than read. SP 800-34's structure is a reasonable checklist for D5's plan,
and it is free.

IEC 62443 / NERC CIP, if you work in industrial or utility networks — where availability
requirements are regulatory rather than commercial.

## Practice

Netflix's Chaos Engineering material, and the *Chaos Engineering* book (Rosenthal &
Jones).
The principles document at principlesofchaos.org is one page and worth it. The idea
transfers to enterprise networks at a much smaller scale than the Netflix material assumes.

Google's DiRT (Disaster Recovery Testing) programme write-ups, and AWS's and Azure's
well-architected reliability pillars.
The cloud providers' documents are unusually specific about what is and is not shared between
availability zones, which is F8.

Allspaw, J. — writing on game days, and the **Learning From Incidents** community
(Chapter 53's reading).

**Your organisation's actual DR plan.**
**F6 uses it.** Reading one carefully is consistently more instructive than reading about
them, and the exercise of counting its ambiguities takes an afternoon.

## Tools

**`keepalived`** (Linux VRRP) — F3 can be done on two virtual machines with no network
hardware at all, including the tracking behaviour.

containerlab / GNS3 / EVE-NG — for the FHRP and failover labs.

`tcpdump` with timestamps, or Wireshark's I/O graph — F4 measures a real failover from
packets rather than from the protocol's own claims, and the two frequently differ.

Metered PDUs with SNMP — **per-outlet power measurement.** The most useful
environmental instrument after a temperature sensor, and it makes B8's arithmetic real.

**Inexpensive environmental sensors** — temperature, humidity and water detection with SNMP or
MQTT output cost very little. §56.3's argument is that the cost is trivial against one
overnight cooling failure, and it is easy to demonstrate.

A UPS with SNMP — and configure the "on battery" trap (§56.3). Most sites have the
capability and have not enabled it.

## Post-mortems worth reading

Any major cloud provider's availability zone incident report.
The valuable ones are those where a shared component failed — a control plane, a metadata
service, a network fabric — while every "independent" resource remained healthy. F7 and F8
use one.

**The 2021 OVHcloud data centre fire.**
A physical-plant incident with the full range of consequences, including customers who
discovered their backups were in the same building.

**Published accounts of failed disaster recovery invocations.**
Rarer, because organisations do not publish them willingly — but the regulated sectors
(finance, healthcare) produce public reports, and they read like a checklist of §56.4's
failure modes.

## Where to look next

**Chapter 62** covers the deliberate failures this chapter treats as accidental; **Chapter 63**
is where the runbooks and the RTO meet reality; **Chapter 65** covers the layer-by-layer
diagnosis that determines how much of the RTO is spent finding the problem; and **Chapter 69**
takes up availability in cloud architectures, where the shared-fate question becomes harder
rather than easier.
