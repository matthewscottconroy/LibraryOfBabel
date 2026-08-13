# 20.4 Inter-VLAN Routing and Voice VLANs

VLANs isolate. Almost nothing useful stays isolated. This section covers how traffic
crosses a VLAN boundary — which requires a router, by definition — and the one special
case that appears on nearly every enterprise access port.

## Why a router is required

Not a design preference. A definitional consequence.

A switch forwards within a VLAN. A frame arriving on a VLAN 10 port can only be sent
out VLAN 10 ports (§20.1). There is no forwarding path between VLANs at Layer 2,
because that is precisely what a VLAN is.

And the hosts themselves will not attempt it. A host on VLAN 10 with address
`192.168.10.50/24`, sending to `192.168.20.50`, performs the local-or-remote decision
(§18.1) and concludes the destination is remote. It sends the frame **to its default
gateway**, with the gateway's MAC address and the final destination's IP address.

**So the traffic goes to a router because the host sends it to a router**, and the
router — having interfaces in both VLANs — forwards between them.

Which reveals why the VLAN-to-subnet relationship is not a convention:

> **One VLAN = one broadcast domain = one IP subnet.**

A VLAN with two subnets in it means hosts that consider each other remote and send to
a gateway, which sends the traffic straight back into the same VLAN — traffic taking a
pointless detour through a router that never had to be involved. A subnet spanning two
VLANs means hosts that consider each other local, ARP for each other, and receive no
reply — because ARP does not cross a VLAN boundary.

Both are configurable and both are broken. The correspondence is enforced by the
mechanisms, not by policy.

## Three ways to do it

### 1. Router with one interface per VLAN

```
   VLAN 10 ────────── Gi0/0 ┐
                            ├── Router
   VLAN 20 ────────── Gi0/1 ┘
```

Conceptually cleanest, and impractical. Twenty VLANs need twenty router interfaces and
twenty switch ports. Routers have few interfaces and they are expensive.

Historically real, and the reason the next approach was invented.

### 2. Router-on-a-stick

**One** physical link between switch and router, configured as a trunk, with a
**subinterface** per VLAN on the router.

```
   ┌────────────────┐        trunk         ┌──────────┐
   │     Switch     │═════════════════════ │  Router  │
   │ [10][10][20]   │   VLANs 10, 20       │ Gi0/0.10 │
   └────────────────┘                      │ Gi0/0.20 │
                                           └──────────┘
```

```
interface GigabitEthernet0/0.10
 encapsulation dot1Q 10
 ip address 192.168.10.1 255.255.255.0
!
interface GigabitEthernet0/0.20
 encapsulation dot1Q 20
 ip address 192.168.20.1 255.255.255.0
```

Each subinterface has an IP address and is the default gateway for its VLAN. The
router receives a tagged frame, associates it with the subinterface, routes it,
re-tags for the destination VLAN, and sends it back down the same link.

**The bottleneck is visible in the diagram.** Every inter-VLAN packet crosses the
trunk **twice** — in and out. A 1 Gb/s link supports at most 500 Mb/s of inter-VLAN
traffic, and the router is doing the forwarding in software.

Fine for a small site. Inadequate for a campus, which is why the third approach exists
and is what you will meet.

### 3. Layer 3 switch with SVIs — what is actually used

A **Switched Virtual Interface** is a virtual routed interface on the switch itself,
one per VLAN:

```
interface Vlan10
 ip address 192.168.10.1 255.255.255.0
!
interface Vlan20
 ip address 192.168.20.1 255.255.255.0
!
ip routing
```

The `ip routing` line is essential and is the single most commonly forgotten line in
this entire configuration. **Without it the switch has interfaces in both VLANs and
will not route between them**, and the symptom — everything works within each VLAN,
nothing works between them, and the configuration looks correct — is confusing out of
proportion to the cause.

**Why this won:**

| | Router-on-a-stick | SVIs on a Layer 3 switch |
|---|---|---|
| Forwarding | software, in a router | **hardware ASIC** |
| Throughput | limited by one trunk, halved | **line rate on every port** |
| Latency | router hop over a shared link | microseconds, internal |
| Cost per port | high | low |
| Extra devices | a router | **none** |

The switch already has the silicon to make forwarding decisions at line rate
(Chapter 17 §17.4). Making those decisions on IP headers rather than MAC headers is a
different lookup, not a different class of device. **"Layer 3 switch" means a router
that was built as a switch**, and for inter-VLAN routing it is simply the correct
answer.

## Applying policy — the point of the exercise

Here is where §20.1's argument completes. Once inter-VLAN routing exists, the VLANs can
talk **freely**. Segmentation without policy has achieved almost nothing security-wise.

The routing point is where policy goes:

```
ip access-list extended GUEST-POLICY
 permit udp any any eq 53
 permit tcp any any eq 443
 deny   ip any 192.168.0.0 0.0.255.255
 permit ip any any
!
interface Vlan50
 ip access-group GUEST-POLICY in
```

Guests may resolve names and reach HTTPS, may not reach any internal network, and may
otherwise reach the Internet.

**The VLAN created a place to stand. The ACL is the control.** This is worth stating as
a principle because it recurs at every scale:

> **Segmentation creates enforcement points. It does not enforce anything.**

The same relationship holds for firewall zones, for cloud security groups, and for the
microsegmentation of Chapter 60 §60.4.

Modern practice puts the enforcement on a firewall rather than a switch ACL when the
policy is complex, because firewalls are stateful (Chapter 60 §60.2) and switch ACLs
generally are not. The design pattern — VLAN boundary as enforcement point — is
identical.

## First-hop redundancy

An SVI is the default gateway for every host in its VLAN. If the switch holding it
fails, **every host in that VLAN loses all off-VLAN connectivity**, even though the
network around them is fine.

**VRRP** (standard), **HSRP** and **GLBP** (Cisco) solve this: two switches share a
**virtual IP and virtual MAC address**, one active and one standby, and the hosts point
at the virtual address and never know which physical device is serving them.

Failover works by **gratuitous ARP** (§18.3) — the newly active router announces the
virtual MAC, switches update their tables, and traffic follows within a second.
Chapter 56 §56.2 covers this properly; it is noted here because it is the standard
companion to SVIs and because the mechanism is one already met.

## Voice VLANs

The one special case worth knowing in detail, because it appears on most enterprise
access ports and it looks like a contradiction.

**The situation.** An IP telephone sits on the desk. The workstation plugs into the
back of the telephone; the telephone plugs into the wall. One cable to the desk.

**The requirement.** Voice and data should be in **different VLANs**: voice needs
priority (Chapter 52) and different security policy; data should not be able to
trivially reach the telephony infrastructure.

**The problem.** That is one switch port, and an access port carries one VLAN.

**The solution** — and it is a deliberate special case:

```
interface GigabitEthernet0/5
 switchport mode access
 switchport access vlan 10          ! data,  untagged
 switchport voice vlan 20           ! voice, tagged
 spanning-tree portfast
 spanning-tree bpduguard enable
 mls qos trust device cisco-phone
```

The port carries **two** VLANs: the data VLAN untagged and the voice VLAN tagged. It is
sometimes called a "multi-VLAN access port", and it is neither a normal access port nor
a trunk.

**How it works:**

1. The telephone boots and learns its voice VLAN ID via **CDP or LLDP-MED** from the
   switch — no configuration on the telephone.
2. The telephone tags its own traffic with VLAN 20 and marks it with a high PCP.
3. The workstation's traffic passes through the telephone's internal switch
   **untagged**, and lands in VLAN 10.
4. The switch separates the two by the presence of the tag.

**Why this is not simply a trunk.** A trunk would work, and would also expose the port
to VLAN hopping and would make DTP negotiation relevant. The voice VLAN construct
restricts the port to exactly two VLANs, keeps PortFast and BPDU Guard applicable, and
allows the switch to apply QoS trust only to the telephone. It is a narrowly-scoped
special case rather than a general capability, and that narrowness is the security
property.

**`mls qos trust device cisco-phone`** is worth noting: the switch **accepts the QoS
markings from the telephone and rewrites everything else to zero**. Otherwise any
workstation could mark its traffic as voice and take priority over actual calls. This
is a general principle in QoS design — **trust markings only at a boundary you
control** — and Chapter 52 §52.2 develops it.

**Power over Ethernet** almost always accompanies this. The telephone draws power from
the same cable (Chapter 16 §16.4), which is why a switch's PoE budget is a real design
constraint: 48 ports × 15.4 W exceeds what many switches supply, and the failure mode
is telephones that fail to boot in an order nobody predicted.

## What breaks here

**Everything works within VLANs, nothing between them, configuration looks right.**
`ip routing` is missing. Check it first, every time.

**Inter-VLAN traffic is slow.** Router-on-a-stick, and the trunk is carrying it twice.
Move to SVIs.

**A host reaches other VLANs but not the Internet.** The SVI is fine; the router's
default route or NAT is not. Chapter 33.

**A whole VLAN loses off-VLAN connectivity while everything else works.** The SVI's
switch failed, or the SVI went down because every port in that VLAN went down — an SVI
is only up while at least one port in the VLAN is up, which surprises people.

**A telephone works and the workstation behind it does not.** Data VLAN not configured
on the port, or the telephone's pass-through disabled.

**A telephone gets no address.** Voice VLAN not configured, CDP/LLDP disabled, or DHCP
not configured for the voice VLAN. The telephone needs option 150 or its equivalent to
find its call manager as well.

**Voice quality poor despite QoS configuration.** Markings not trusted at the boundary,
or trusted from everything. Check what the switch does with the PCP value it receives.

> **Network+ note.** Objective 2.3 expects inter-VLAN routing and voice VLANs;
> objective 1.2 expects the Layer 3 switch. Over-learn: **one VLAN = one subnet**;
> **inter-VLAN traffic requires a router or a Layer 3 switch**; **router-on-a-stick
> uses subinterfaces with `encapsulation dot1Q`**; **an SVI is `interface VlanN`**; and
> **a voice VLAN carries voice tagged and data untagged on one access port**.
