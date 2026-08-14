# 72.3 Addressing, Services and Security Together

**The section's title is its argument.** These three are designed together or they are bolted
on, and the bolted-on version is what most networks have.

## Why together

Because each constrains the others, and designing them in sequence means revisiting.

| | Constrains |
|---|---|
| **Addressing** | **what can be summarised, what can be filtered as a block, what a firewall rule can express** |
| **Segmentation** | **how many subnets are needed, and therefore the address plan's shape** |
| **Services** | **DHCP scopes, DNS zones and NTP sources per segment** |
| **Security** | **where the enforcement points are, which determines where the routing boundaries must be** |

> A security policy that says "the warehouse must not reach finance" is trivial if the two are
> in summarisable address blocks and painful if they are interleaved — and the address plan
> was made before anyone asked.

Which is Chapter 27 §27.2's argument, and this section's contribution is to state the other
two dependencies explicitly.

## The address plan

Structured, hierarchical, and with the security boundaries visible in it.

A worked plan for a 12-site organisation with 10.0.0.0/8:

```
   10.0.0.0/8
   │
   ├── 10.0.0.0/16    reserved — loopbacks, point-to-point, management
   ├── 10.10.0.0/16   Site A   (headquarters)
   ├── 10.20.0.0/16   Site B   (warehouse)
   ├── 10.30.0.0/16   Site C
   │   :                                        256 sites available; 12 used
   └── 10.250.0.0/16  reserved — cloud, DMZ, partners

   Within a site, 10.20.0.0/16:
   ├── 10.20.0.0/20    infrastructure   management, point-to-point, loopbacks
   ├── 10.20.16.0/20   staff            user VLANs
   ├── 10.20.32.0/20   voice
   ├── 10.20.48.0/20   servers
   ├── 10.20.64.0/20   operational tech scanners, building systems, cameras
   ├── 10.20.80.0/20   guest and BYOD
   ├── 10.20.96.0/20   reserved
   │   :                                        16 functional blocks; 6 used
   └── 10.20.240.0/20  reserved
```

Four properties, and each is a design decision:

**The site is summarisable.** `10.20.0.0/16` is one route and one firewall object, which
matters at every WAN boundary (Chapter 32 §32.2's aggregation argument, at enterprise scale).

**The function is summarisable within the site.** "All operational technology at site B" is
`10.20.64.0/20` — **one ACL entry**, and it will still be one entry when the number of
OT VLANs doubles.

The same function is at the same offset at every site. `10.x.64.0/20` is OT everywhere,
so a policy written once applies everywhere and an engineer reading an address knows what
it is.

**And half of everything is reserved.** 6 of 16 functional blocks, 12 of 256 sites —
Chapter 72 §72.2's headroom argument, in its most valuable application, because renumbering is
a project and address space costs nothing.

**And IPv6 in parallel, not later:**

```
   2001:db8::/32 allocated
   2001:db8:0020::/48   Site B            65,536 sites available
       2001:db8:0020:0064::/64            OT      — the same offsets, in hex
       2001:db8:0020:0016::/64            staff
```

> Design the IPv6 plan when you design the IPv4 one, even if deployment is later
> (Chapter 28 §28.2). The cost is an afternoon; retrofitting a coherent IPv6 plan onto a
> deployed network is the same project as renumbering.

## Segmentation, from the requirements

§72.1's first question produced "who talks to whom." The segments are its complement:
who does not.

| Segment | May reach | Rationale |
|---|---|---|
| **Staff** | **Internet, servers (specified ports), voice for signalling** | |
| **Voice** | **the call platform, and each other** | Chapter 20 §20.4 |
| **Servers** | **each other as required, and out** | |
| **OT / scanners** | **the WMS only** | **cannot be patched** (Chapter 57 §57.1) |
| **Building systems** | **their controller only** | the same |
| **Guest** | **Internet only** | |
| **Management** | **nothing reaches it; it reaches devices** | **Chapter 60 §60.4** |
| **Backup** | **nothing reaches it** | **Chapter 57 §57.1 step 5** |

And the ordering is Chapter 60 §60.4's priority list: management, backups, servers from
workstations first — which delivers most of the benefit before anything sophisticated is
attempted.

The enforcement point for each boundary is a design decision, and it should be stated:

| Boundary | Enforced by |
|---|---|
| Between site VLANs | **the collapsed core's ACLs, or a firewall** |
| **Into the management network** | **a firewall, and out-of-band access** |
| Site to site | **the WAN edge, or the SD-WAN policy** (Chapter 51 §51.2) |
| To the Internet | the perimeter firewall |
| **Within the server segment** | **microsegmentation, or nothing — and say which** |

> **The last row is the honest one.** "We will microsegment the servers" is frequently written
> and rarely done (Chapter 60 §60.4), and a design that says "server-to-server traffic is
> unrestricted" is better than one that claims a control that will not exist.

## Services, per segment

The four that every segment needs, and the decisions are not obvious.

| | Decision |
|---|---|
| **DHCP** | **scopes per VLAN, relays configured, pool sized with growth** (Chapter 65 §65.4) |
| **DNS** | **internal zones, forwarders, split-horizon or not** (Chapter 39 §39.3) |
| **NTP** | **two sources, authenticated, from a stratum you trust** (Chapter 54 §54.3) |
| **Default gateway** | **FHRP, aligned with spanning tree** (Chapter 56 §56.2) |

**And three that are frequently forgotten:**

Where does the guest network's DNS come from? Not the internal servers — that is a
reconnaissance path.

What does the OT segment use for time? If it cannot reach the internal NTP servers, it
needs its own — and industrial protocols frequently depend on time.

And what happens to each segment when the WAN fails? A branch whose DHCP relay points to
HQ has no addressing when the circuit drops — which is a design decision, and "the branch
survives with local DHCP and DNS caching" or "the branch stops" are both defensible and only one
of them should be a surprise.

## Designing the three together: a worked boundary

**The warehouse example, showing the interaction.**

**Requirement (§72.1):** *the scanners talk to the WMS and nothing else; they must never lose
connectivity while a picker is walking; the devices cannot be patched.*

| Layer | Decision | Because |
|---|---|---|
| **Addressing** | **`10.20.64.0/22` — one summarisable block** | **one ACL entry, at every boundary** |
| **Segmentation** | **its own VLAN and VRF; no route to anything but the WMS** | **routing-based, not filter-based** (Chapter 69 §69.3) |
| **Enforcement** | **the VRF has one route: to the WMS's address** | **no rule to misorder** |
| **Services** | **local DHCP, local DNS, local NTP** | **survives a WAN failure** |
| **Wireless** | **802.1X where the devices support it; MAB with tight authorisation where not** | Chapter 59 §59.2 |
| **Roaming** | **802.11r and k; one VLAN across the roaming domain** | Chapter 45 §45.2 |
| **Availability** | **two APs covering every aisle; controller failure does not disconnect** | Chapter 45 §45.3 |
| **Monitoring** | **alert on any traffic from this block to anywhere but the WMS** | **the control, instrumented** (Chapter 57 §57.4) |

> Eight decisions, from one requirement, spanning addressing, security, services, wireless and
> monitoring — and each is derivable from the requirement rather than chosen from a
> catalogue. **That derivability is what makes it defensible** (§72.4).

## What must be designed and is not

Five items that are consistently absent from designs and consistently needed.

| | |
|---|---|
| **Out-of-band management** | **Chapter 60 §60.4** — and it is cheap at build time |
| **The monitoring** | **what is collected, from where, and what alerts** (Chapter 54) |
| **The address management** | **where the record lives** (Chapter 53 §53.3) |
| **The change process** | **Chapter 55 §55.2 — before the first change, not after the first incident** |
| **The documentation** | **the three diagrams, produced as part of the build** (Chapter 53 §53.1) |

> **All five are operational rather than architectural**, which is why they are omitted — and
> Chapter 56 §56.1's argument is that MTTR is where the availability leverage is, so omitting
> them is omitting the cheapest availability improvement in the design.

## What breaks here

A security policy that requires forty ACL entries because the addresses are interleaved.
The address plan was made before anyone asked about the policy.

A site added and the WAN's routing table doubling. **No summarisation** (Chapter 32 §32.2).

An IPv6 deployment that cannot be structured coherently. The IPv4 plan was made alone, and
retrofitting is renumbering.

A branch with no addressing when the WAN fails. The DHCP relay points to HQ — a design
decision that nobody made deliberately.

A design claiming microsegmentation that was never implemented. Say what will exist.

**Guest devices resolving internal names.** The guest DNS points at the internal servers.

An OT segment that cannot be reached to be monitored, and is therefore not monitored.
Design the monitoring path with the segment.

No out-of-band access, discovered during the first outage. Cheap at build time
(Chapter 60 §60.4).

A network in production with no address record, no monitoring and no change process. All
five operational items omitted, and the first year will be spent building them under pressure.

> **Network+ note.** Objective 1.7, 3.1 and 4.3. Over-learn: **address plans should be
> hierarchical and summarisable**; VLANs segment networks and require a routed boundary for
> enforcement; DHCP, DNS and NTP must be planned per segment; **management networks should
> be separate**; and **security requirements should be identified during design.** The
> designed-together argument is the professional content and is not examined.
