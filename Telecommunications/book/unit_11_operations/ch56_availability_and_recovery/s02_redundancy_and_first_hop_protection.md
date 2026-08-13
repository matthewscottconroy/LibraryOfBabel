# 56.2 Redundancy and First-Hop Protection

**The naive model says two components at 99% give 99.99%.** **The model is wrong in practice**,
and understanding why is the most important idea in this chapter.

## Shared fate

> **Components that share a fate are not independent, and the parallel availability formula
> assumes independence.**

**A partial inventory of shared fate, all of which have caused real outages:**

| Shared | Example |
|---|---|
| **Power circuit** | **two power supplies fed from the same board** |
| **Rack** | two switches in one rack, one UPS, one cooling unit |
| **Duct** | **two "diverse" fibres in the same trench** — **and the duct is invariably found by the same excavator** |
| **Building entry** | two circuits, two carriers, one hole through one wall |
| **Firmware** | **two routers running the same version, sharing the same defect** |
| **Configuration** | **an error applied to both members of a pair** |
| **Substation** | two data centres, one electricity supply |
| **Carrier** | two providers, one reselling the other's circuit |
| **Control plane** | **two cloud availability zones sharing a management service** |
| **Person** | **one engineer who configured both** (Chapter 53 §53.4) |

**The last two deserve emphasis because they are modern and invisible.**

> **"Two providers" frequently means one physical circuit.** **Wholesale is normal in
> telecommunications**, and a customer buying from two retailers may be buying the same
> underlying tail twice. **Ask for the physical path, not the provider's name.**

**And a shared control plane is the cloud-era version of a shared power feed.** **Two
availability zones that fail independently at the compute layer may share an authentication
service, a metadata service or a management API** — **and the outage takes both out while every
component in each remains healthy.** **Several large cloud incidents have this shape.**

## The discipline

**For each redundant pair, ask: what could take out both?**

**The answer is usually something, and often something addressable at low cost:**

| Finding | Mitigation | Cost |
|---|---|---|
| Same power circuit | **separate circuits, separate boards** | **low** |
| Same duct | **a second entry, or accept and document** | **high** |
| Same firmware | **stagger versions across the pair** | **low, and it complicates support** |
| Same rack | different racks | **low if planned, high if retrofitted** |
| Same carrier | **verify the physical path; change one** | moderate |
| Same engineer | **a second person reviews** | **low** |

**Staggering firmware deserves its own note**, because it is contested.

> **Running different versions on the two members of a pair protects against a version-specific
> defect and complicates every support conversation.** **The defensible compromise is to
> stagger the upgrade in time — upgrade one, wait a fortnight, upgrade the other** — which
> gives most of the protection and ends with the pair matched.

**And document what you decided to accept.** **"The two fibres share a duct for 400 m between
the building and the manhole; a second entry costs £180,000; accepted, reviewed annually"** is a
good record. **The failure is not accepting a risk; it is accepting one without knowing.**

## The failover that does not work

**The failure redundancy itself introduces**, and it is more common than the failure it was
meant to protect against.

> **A backup component that has never been exercised is of unknown status.**

**The catalogue, all observed:**

| | |
|---|---|
| **A standby firewall whose configuration drifted** | **fails over into a broken state** |
| **A generator that has never been load-tested** | **starts, and stops under load** |
| **An LTE backup with a deactivated SIM** | **deactivated for non-use** |
| **A backup circuit that was never provisioned end to end** | the order completed; the routing did not |
| **A standby that has been down for months** | **nothing monitors the thing that is not carrying traffic** |
| **A failover that works and takes 40 minutes** | **which is not what "redundant" meant** |
| **A UPS whose batteries expired** | four years old, never load-tested |

**The last item on that list is the general case:**

> **Nothing monitors the component that is not carrying traffic**, because it looks identical
> to a healthy idle component. **The standby's health must be checked explicitly**, and
> "checked" means exercised rather than pinged.

**The practice that fixes this is deliberate, scheduled failover testing** — **which is
uncomfortable, is the reason it is skipped, and is the only way to know.**

**What makes a failover test useful:**

- **Fail the primary, do not merely fail over.** **A graceful, administrator-initiated
  switchover exercises a different path from a power cut.**
- **Run in production, in a window.** **A lab test proves the mechanism; it does not prove your
  configuration.**
- **Stay on the standby for a working day.** **Capacity, licensing and slow-appearing problems
  do not show in ten minutes.**
- **Fail back deliberately**, and time it. **Failback is frequently harder than failover and is
  almost never tested.**
- **Exclude the person who built it** (Chapter 53 §53.4), **and see whether the runbook
  suffices.**

## First-hop redundancy

**The specific mechanism worth knowing in detail**, because it addresses a single point of
failure that Chapter 25 §25.3 built into every host.

**A host has one default gateway address.** **If the router holding it fails, the host cannot
reach anything remote** — **and it will not discover an alternative**, because it has one static
entry and no mechanism to change it.

### The virtual address

```
                    ┌──────────────────┐
   Hosts ──────────▶│ 10.20.0.1        │ ← the virtual IP; what hosts use
   default gw       │ MAC 0000.5e00.0101│ ← the virtual MAC; also moves
   10.20.0.1        └────────┬─────────┘
                    ┌────────┴─────────┐
              ┌─────┴─────┐      ┌─────┴─────┐
              │ Router A  │      │ Router B  │
              │ 10.20.0.2 │      │ 10.20.0.3 │
              │ MASTER    │◀────▶│ BACKUP    │
              │ pri 110   │ hello│ pri 100   │
              └───────────┘      └───────────┘
```

**Two or more routers share a virtual IP address, with one active at a time.**

**And — this is the part that makes it invisible to hosts — the virtual MAC moves too.** **A
host's ARP cache does not need to change, and it receives no gratuitous ARP it must act on.**
**From the host's position, nothing happened.**

| | **VRRP** | **HSRP** | **GLBP** |
|---|---|---|---|
| Standard | **RFC 5798, open** | Cisco | Cisco |
| Terms | **master / backup** | **active / standby** | AVG / AVF |
| Virtual MAC | `0000.5e00.01xx` | `0000.0c07.acxx` | `0007.b4xx.xxxx` |
| Default timers | **1 s hello, 3 s hold** | 3 s hello, 10 s hold | 3 s / 10 s |
| **Load sharing** | **by VLAN** | by VLAN | **within one VLAN** |
| Highest priority | **wins** | wins | |

**GLBP's distinction is worth a sentence:** **it answers ARP requests with different virtual MAC
addresses for different hosts**, so **several routers forward simultaneously for one subnet**.
**Elegant, and Cisco-only, and largely superseded by designs where the pair is a single logical
device** (MLAG, stacking, or a routed access layer).

### Timers, and the tuning trap

| | Hello | Hold | **Failover** |
|---|---|---|---|
| VRRP default | 1 s | 3 s | **~3 s** |
| HSRP default | **3 s** | **10 s** | **~10 s** |
| Tuned | **250 ms** | **750 ms** | **< 1 s** |
| **BFD-assisted** | — | — | **< 100 ms** |

**Ten seconds is a long time.** **TCP sessions survive it; voice calls frequently do not**, and
**HSRP's defaults are a common and quiet cause of "the failover works and calls drop."**

**Tuning down has a cost:** **more control traffic, and more sensitivity to transient loss.**
**A pair with 250 ms timers across a congested link will flap**, and a flapping FHRP is worse
than a slow one.

> **BFD is the better answer where it is supported** (Chapter 31 §31.4): **a lightweight
> sub-second liveness protocol that the FHRP subscribes to**, giving fast detection without
> aggressive protocol timers.

### Tracking

**The mechanism that prevents the commonest FHRP failure.**

```
   Router A is MASTER. Its uplink to the core fails.
   Router A is still alive on the LAN, so it remains MASTER.
   Every host sends its traffic to a router that cannot forward it.
```

> **A black hole, produced by redundancy working exactly as specified.**

**Interface or object tracking fixes it:** **the router monitors its uplink (or a route, or a
reachability test) and decrements its own priority when the tracked object fails**, causing the
standby to take over.

```
   track 1 interface GigabitEthernet0/1 line-protocol
   vrrp 1 priority 110
   vrrp 1 track 1 decrement 20      ! 110 → 90, below B's 100
```

**Track something that reflects actual forwarding capability**, not merely link state. **A link
that is up to a switch that has lost its own uplink is still "up".** **Tracking a route, or an
IP SLA probe to a real destination, is better.**

### Preemption

**Whether a recovered higher-priority router takes back the active role.**

| | For | Against |
|---|---|---|
| **Preempt on** | **predictable, matches the design** | **a second outage when it recovers**; **a flapping device flaps the gateway** |
| **Preempt off** | **one outage per failure** | the active role ends up wherever chance left it |

**The defensible position: preemption on, with a delay.**

```
   vrrp 1 preempt delay minimum 180
```

**Three minutes lets the recovered router finish booting, re-establish its routing adjacencies
and populate its forwarding table before it accepts traffic.** **Without the delay, a router
that has just booted takes the active role and black-holes traffic for thirty seconds while
OSPF converges** — **which is a self-inflicted outage caused by recovering.**

### Aligning FHRP with spanning tree

**The design error worth naming**, because it is common and it is invisible.

```
   Wrong:                              Right:

   STP root: Switch A                  STP root: Switch A
   VRRP master: Router B               VRRP master: Router A

   Host ──▶ A ──(inter-switch)──▶ B    Host ──▶ A ──▶ done
   and back again                      
   
   Every packet crosses the link       Traffic takes the direct path
   between the switches, twice
```

**In a collapsed core where the switches are also the routers, the spanning tree root and the
FHRP master must be the same device** (Chapter 19 §19.3). **When they are not, all inter-VLAN
traffic traverses the inter-switch link unnecessarily**, and the symptom is **a peer link at
much higher utilisation than the design predicts, with no other explanation.**

**And with multiple VLANs, align them per VLAN** — **VLAN 10's root and VRRP master on switch A,
VLAN 20's on switch B** — **which both balances the load and keeps each VLAN's path direct.**

## What breaks here

**A failover that has never been tested.** **Status unknown.** Test it, in production, in a
window.

**A "redundant" pair that failed together.** **Shared fate.** Enumerate it after the incident
and — better — before.

**Failover works and voice calls drop.** **HSRP's 10-second defaults.** Tune, or use BFD.

**The gateway is up and nothing can reach the Internet.** **No uplink tracking.** The classic
FHRP black hole.

**A brief outage every time a router recovers.** **Preemption with no delay.** Add one.

**The inter-switch link unexpectedly busy.** **Spanning tree root and FHRP master on different
devices.** Align them.

**Both routers claiming master.** **They cannot see each other's hellos** — a VLAN problem, an
ACL, or a trunk not carrying the VLAN. **Two masters means two devices answering for one IP**,
and the symptom is intermittent and confusing.

**A standby that was down for three months.** **Nothing monitors the idle component.** Monitor
it explicitly, and exercise it.

**"Two carriers" that turned out to be one circuit.** **Ask for the physical path**, and do not
accept the provider's name as an answer.

> **Network+ note.** Objective 3.3 covers redundancy and FHRP. Over-learn: **VRRP is the open
> standard and HSRP is Cisco's**; **an FHRP provides a virtual IP shared by two or more
> routers**; **the active router forwards and the standby takes over on failure**; **NIC
> teaming and link aggregation provide redundancy at other layers**; and **redundancy requires
> genuine diversity to be effective.** The VRRP/HSRP distinction is examined; the shared-fate
> point is the one that matters.
