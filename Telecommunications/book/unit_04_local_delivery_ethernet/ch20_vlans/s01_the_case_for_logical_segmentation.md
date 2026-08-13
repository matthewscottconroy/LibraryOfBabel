# 20.1 The Case for Logical Segmentation

By the end of Chapter 19 we can build a switched network of arbitrary size that
survives cable failures. It has one remaining structural problem, and it is the
problem VLANs exist to solve.

## Everything is one broadcast domain

A switch does not break up broadcast domains (Chapter 17 §17.3). Neither does a
bridge, a hub, or a repeater. **Connect a thousand devices to a switched network and
you have one broadcast domain containing a thousand devices**, no matter how many
switches or how carefully spanning tree has arranged them.

Three consequences follow, and they are of very different weights.

**Broadcast volume.** Every host processes every broadcast (§18.2's *n*² problem).
Real, measurable, and — with modern CPUs and modern NIC filtering — the *least*
important of the three. A thousand-host broadcast domain is unpleasant; it is not the
reason to segment.

**Failure blast radius.** One misbehaving device — a looping cable, a chattering NIC,
a storm — degrades every host that shares the domain. The domain *is* the failure
unit.

**Security.** This is the one that matters. Chapter 18 §18.3 established it precisely:
**anything on a broadcast domain can ARP-spoof anything else on that broadcast
domain**. It can also scan it, capture broadcast traffic on it, exhaust its DHCP pool,
and advertise itself as its router.

Which means a flat network has a single, uncomfortable property:

> **A visitor's laptop in the lobby is on the same trust boundary as the finance
> department's file server.**

Not "can route to". **On the same segment as** — able to attack it with the tools of
§18.3, no routing, no firewall traversal, nothing to bypass.

## The physical answer, and why it failed

The obvious remedy in 1990 was physical: **one switch per group**, connected by
routers.

```
   Finance switch ─┐
                   ├── Router ── (each switch its own broadcast domain)
   Sales switch ───┘
```

This works. It also fails on contact with buildings, for reasons that have nothing to
do with networking:

**People are not sorted by department.** Finance is on floors 2 and 4 because that is
where the offices were free. Sales is scattered across three floors. A departmental
switch means running cable from a person's desk to whichever comms room holds their
department's switch — which is not the comms room on their floor.

**People move.** Reorganisations, new hires, a manager who wants a window. Under
physical segmentation, moving a person between departments means **re-patching, and
sometimes re-cabling**. Every organisational change becomes a cabling project.

**Utilisation is terrible.** Each department needs a switch on each floor where it has
anyone at all. Finance has four people on floor 3, so floor 3 needs a finance switch —
a 48-port device with four ports used, and the same for every other department.

**One person breaks the model.** The contractor who needs access to two environments,
the machine that belongs to no department, the meeting room used by everyone. There is
no port to give them.

The physical approach was correct in principle and unaffordable in practice. What was
needed was **the segmentation without the geography**.

## The insight

Segmentation is a property of the **forwarding decision**, not of the wiring.

A switch already decides where each frame goes (Chapter 17 §17.2). Add a single
constraint to that decision:

> **A frame may only be forwarded to ports in the same VLAN as the port it arrived
> on.**

That is the whole idea. Ports are labelled with a VLAN number; the forwarding
algorithm is restricted to ports sharing the label; and the switch behaves exactly as
though it were several independent switches that happen to share a chassis.

```
   One physical switch, three VLANs:

   ┌───────────────────────────────────────────────┐
   │  [10][10][20][20][30][10][20][30][30][10]     │
   └───────────────────────────────────────────────┘
            ≡  behaves identically to  ≡

   ┌─────────────┐  ┌─────────────┐  ┌─────────────┐
   │  VLAN 10    │  │  VLAN 20    │  │  VLAN 30    │
   │  (4 ports)  │  │  (3 ports)  │  │  (3 ports)  │
   └─────────────┘  └─────────────┘  └─────────────┘
```

Everything already established still holds, **per VLAN**:

- Each VLAN is its own **broadcast domain** — a broadcast on VLAN 10 reaches VLAN 10
  ports only.
- Each VLAN has its own **MAC address table** (or its own entries in a shared one).
- Each VLAN has its own **spanning tree**, or shares an MSTP instance (§19.3).
- Each VLAN is its own **IP subnet** — and Chapter 25 will show that this is not a
  coincidence but a necessity.

**And the assignment is configuration, not cabling.** Moving a person from Finance to
Sales is one command on one port.

## What this bought

| Physical segmentation | VLANs |
|---|---|
| Group = location | **Group = policy** |
| Move a person → re-patch or re-cable | **Move a person → one command** |
| One switch per group per floor | **One switch per floor** |
| Terrible port utilisation | Full utilisation |
| Exceptions impossible | Exceptions trivial |

The economic argument was decisive and is why VLANs were adopted almost universally
within a few years of becoming available. But the durable argument is the security
one: **VLANs make the trust boundary something you decide rather than something the
building decided for you.**

## What VLANs do not do

Three limits, each of which is a common misunderstanding.

**A VLAN is not encryption.** Traffic within a VLAN is as visible to other members of
that VLAN as it ever was. VLANs partition; they do not protect within a partition.

**A VLAN is not a firewall.** Two VLANs cannot talk *without a router* — and once you
add a router to let them talk (§20.4), they can talk **freely** unless you also apply
policy. The VLAN boundary forces traffic through a point where policy *can* be
applied; it does not apply any.

This distinction matters enormously in practice. People say "we put them in separate
VLANs" as though that were the security control. It is the *precondition* for the
security control.

**VLANs do not scale indefinitely.** The 802.1Q tag has 12 bits, giving **4,094
usable** VLANs (§20.2). Adequate for any campus; inadequate for a cloud provider with
tens of thousands of tenants, which is why VXLAN exists (Chapter 67 §67.2).

## How VLANs are assigned

Four mechanisms, in decreasing order of how common they are:

| Method | Basis | Where used |
|---|---|---|
| **Static / port-based** | switch port configuration | overwhelmingly the norm |
| **Dynamic (802.1X)** | authenticated identity | modern enterprise, growing fast |
| MAC-based (VMPS and successors) | device MAC address | legacy, rare |
| Protocol-based | EtherType | historical, essentially extinct |

**Static** is what §20.3 configures and what you will meet everywhere.

**Dynamic assignment via 802.1X** (Chapter 59 §59.2) is worth flagging now because it
completes the argument. Under 802.1X the port has **no VLAN until the device
authenticates**; the RADIUS server returns the VLAN as an attribute of the
authenticated identity. The consequence:

> **The VLAN follows the user, not the cable.**

A finance employee plugging into any port in the building lands on the finance VLAN.
An unknown device lands on a guest VLAN or nothing at all. The 1990 goal —
segmentation by policy rather than geography — is fully achieved only here, and this is
where enterprise practice has moved.

## What breaks here

**"They're in different VLANs so they're isolated."** Only until inter-VLAN routing is
configured, which it invariably is. Isolation requires an ACL or a firewall at the
routing point.

**A device on the wrong VLAN gets no address.** DHCP is per-VLAN (Chapter 40 §40.3). A
port on the wrong VLAN sees the wrong DHCP server, or none. **"No IP address" is very
often a VLAN problem, not a DHCP problem.**

**Two devices on the same switch cannot reach each other.** Check whether they are on
the same VLAN before anything else. Physical adjacency means nothing.

**A VLAN that exists on one switch and not another.** The VLAN must be defined on
every switch that carries it and permitted on every trunk between them (§20.3).

> **Network+ note.** Objective 2.3 expects VLANs; objective 1.6 expects them as
> broadcast-domain boundaries. Two sentences to over-learn: **each VLAN is a separate
> broadcast domain and normally a separate IP subnet**, and **a VLAN is not a security
> control by itself — it is what makes a security control possible.**
