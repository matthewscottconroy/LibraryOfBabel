# 14.4 Convergence

Voice, video, television, industrial control and data now run over one packet
fabric. This was neither obvious nor welcome, and understanding why it happened —
and what it cost — is the closing argument of Unit III.

## What convergence replaced

Within living memory, an organisation ran **separate networks**, and they shared
nothing:

| Traffic | Network | Cabling |
|---|---|---|
| Voice | PBX, TDM trunks | Telephone pairs |
| Data | Ethernet | Cat3/Cat5 |
| Video | Coax, or dedicated codecs on ISDN | Coax |
| Building control | Proprietary buses | Its own |
| CCTV | Analog coax to a matrix | Coax |

Separate cabling, separate equipment, separate suppliers, separate staff, separate
budgets, separate faults. A building might contain four independent distribution
systems, each with its own risers and its own comms rooms.

Today it is one Ethernet plant carrying all of it.

## The telephone engineers' objection

They objected, and the objection deserves to be stated properly rather than
caricatured, because it was correct about the requirements.

A voice call needs:

- **Bounded delay** — one-way mouth-to-ear under 150 ms (ITU-T G.114, Chapter 3
  §3.3).
- **Bounded jitter**, or a buffer that converts it to delay and eats the budget.
- **Very low loss**, and bursty loss is far worse than the same rate spread evenly.
- **Availability at the five-nines level**, which the PSTN routinely met.
- **Admission control**, so that an overloaded network refuses new calls rather than
  degrading existing ones (Chapter 13 §13.4).

A best-effort packet network in 1995 provided **none** of these. Not "provided them
imperfectly" — provided none. Delay was unbounded, jitter was uncontrolled, loss was
the congestion signal, availability was nowhere near five nines, and admission
control did not exist as a concept.

The engineers who said "voice needs a purpose-built network" were describing the
network as it then was, accurately.

## Why it happened anyway

Four reasons, and they compound.

**Cost.** One cabling plant instead of four. One set of switches. One team. One
supplier relationship. For an organisation cabling a new building, the saving is
immediate and large, and it does not depend on the packet network being *better* —
only on it being adequate.

**The packet network improved faster.** Switched Ethernet removed collisions.
Bandwidth rose by three orders of magnitude. QoS mechanisms (Chapter 52) provided
prioritisation. Codecs improved. Jitter buffers got smarter. None of these made a
packet network as good as a circuit for voice; together they made it good enough.

**Features became software.** A PBX feature required a hardware module and a vendor
visit. A softswitch feature is a configuration change. Once voice was an
application, it inherited software's development economics, and the feature gap
reversed within a few years.

**Integration.** Voice as an application can integrate with other applications —
presence, directories, calendars, CRM systems, screen sharing. A TDM PBX cannot, and
this turned out to matter more to buyers than call quality did.

**The general shape**, and it is worth naming because it recurs:

> A general-purpose substrate absorbs a specialised one because its economics and
> its rate of improvement are better, and it does so before it is technically
> superior. The specialists are right about the deficiencies and wrong about the
> trajectory.

Chapter 13 §13.4 identified this pattern for packet versus circuit; Chapter 67
identifies it again for hardware versus software networking; Chapter 71 §71.1 warns
that it will recur.

## What convergence costs

The honest half, and it is not small.

**Everything now shares a failure domain.** A network fault used to take out data
and leave the telephones working, because they were separate systems. Now a switch
failure, a broadcast storm, a routing loop or a power problem takes out voice,
video, CCTV, door access and data **simultaneously** — including, notably, the
telephones you would use to report it.

The mitigations — separate VLANs, redundant paths, UPS on the access switches,
PoE budgeting so telephones survive a mains failure — are real work, and they are
frequently under-specified because the organisation remembers when the telephones
were somebody else's problem.

**Voice becomes the network team's responsibility**, whether or not they wanted it,
and voice users have expectations formed by a system that worked flawlessly for
decades.

**QoS becomes necessary**, with all of Chapter 52's complexity — classification,
marking, trust boundaries, queue design — where previously the separation was
physical and free.

**Security surface expands.** Voice, CCTV and building control on the same network
as everything else means a compromised camera can reach the finance server unless
segmentation prevents it. Chapter 47's IoT discussion and Chapter 60 §60.4's
microsegmentation both exist substantially because of convergence.

**Regulatory obligations transfer.** Emergency calling, lawful intercept, and
service availability requirements that used to belong to a telephone company now
belong, in part, to the enterprise operating the VoIP system.

## The pendulum

And the closing observation for the unit.

Convergence took specialised networks and replaced them with one general one. The
industry then spent two decades **partially recreating the specialisation** inside
the general network:

| Mechanism | What it recreates |
|---|---|
| VLANs | Separate broadcast domains — logically separate networks |
| QoS | Per-traffic-class treatment, as separate networks gave physically |
| MPLS traffic engineering | Placed paths with reserved bandwidth |
| Network slicing (5G) | Isolated virtual networks with guarantees |
| TSN | Bounded latency, as an industrial bus provided |
| Lossless data-centre Ethernet | The loss-free behaviour storage networks had |

Every row is a **logical** recreation of something that used to be **physical**, and
each is more flexible and more complicated than what it replaced.

That is the trade convergence made: **physical separation, which was simple and
expensive, exchanged for logical separation, which is cheap and requires
configuration you can get wrong.** Chapter 20's VLAN misconfigurations, Chapter 52's
trust boundaries and Chapter 60's segmentation projects are all consequences of that
exchange.

Which is the right trade — but it should be made knowingly, and a design that
converges everything onto one fabric without specifying the logical separation has
taken the cost and not bought the benefit.

## What breaks here

**A converged network with no VLAN separation.** Everything shares a broadcast
domain, a security domain and a failure domain, and the CCTV system can reach the
payroll server.

**No QoS on a converged link.** Voice degrades whenever anyone transfers a file, and
the complaint arrives as "the phone system is broken".

**Telephones that lose power with the switch.** PoE means the switch's power is the
telephone's power. A UPS on the access switch is now part of the *voice* system's
availability design, and it is routinely forgotten.

**Emergency calling from a VoIP handset that has moved.** Location is no longer
implied by the physical line, which is a regulatory problem as much as a technical
one.

**A design that lists convergence as a benefit and does not specify the separation.**
Common in proposals, and it is the failure of Deliverable 5 that Chapter 72's rubric
penalises.

> **Network+ note.** Objective 1.2 expects VoIP and its infrastructure; objective
> 2.1 expects QoS; objective 1.6 expects the converged network as an architecture.
> The transferable point is §14.4's closing trade: **physical separation exchanged
> for logical separation** — cheaper, more flexible, and dependent on configuration
> that must be got right.
