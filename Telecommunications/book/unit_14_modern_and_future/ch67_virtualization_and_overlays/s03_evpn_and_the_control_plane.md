# 67.3 EVPN and the Control Plane

**§67.2 left VXLAN flooding and learning over a routed fabric**, which works and scales badly.
**EVPN is the control plane that fixed it**, and its selection is worth understanding because it
is a good decision made for good reasons.

## The problem, stated precisely

**A VTEP needs to know three things and the data plane tells it none of them well:**

| It needs to know | Flood-and-learn supplies |
|---|---|
| **Which other VTEPs are in this VNI** | **nothing — it must be configured or multicast-discovered** |
| **Which MAC addresses are behind which VTEP** | **by flooding and observing replies** |
| **Which IP maps to which MAC** | **by flooding ARP everywhere** |

> **Flood-and-learn works because Ethernet was designed for a shared medium where flooding was
> free** (Chapter 17 §17.1). **On a routed fabric spanning a data centre, flooding is a unicast
> replication to every VTEP**, and **the cost is proportional to the square of the fabric's
> size.**

**Which produces the specific symptom:** **a data centre where ARP traffic is a measurable
fraction of the fabric's load**, and where **a VM's first packet to a new destination triggers a
flood to every rack.**

## The answer: distribute the information instead

**EVPN — Ethernet VPN — uses BGP to advertise MAC addresses.**

```
   VTEP-A learns MAC aa:bb:cc:dd:ee:ff on its local port
   VTEP-A advertises via BGP:
       "MAC aa:bb:cc:dd:ee:ff, IP 10.1.2.3, VNI 10010, next-hop 192.0.2.11"
   Every other VTEP installs it.
   
   No flooding. No learning. The information was told, not discovered.
```

**And BGP was chosen for reasons rather than by inertia** (Chapter 32):

| Property | Why it matters here |
|---|---|
| **It carries arbitrary address families** | **MP-BGP already distributed VPNv4 and IPv6; MACs are another AFI/SAFI** |
| **Route reflectors** | **the fabric does not need a full mesh of sessions** |
| **Policy** | **which VTEPs receive which routes, expressed as route targets** (Chapter 50 §50.4) |
| **It scales to hundreds of thousands of routes** | **demonstrably, on the Internet** |
| **It is already in the fabric** | **the underlay routes with BGP in most designs** |

> **EVPN is MPLS L3VPN's mechanism, applied to MAC addresses** (Chapter 50 §50.4). **Route
> distinguishers, route targets, MP-BGP and the separation of the control plane from the data
> plane** — **the same design, twenty years later, for a different encapsulation.**

## What EVPN advertises

**Five route types, and three of them do the work.**

| Type | Carries | Solves |
|---|---|---|
| **2** | **MAC and IP of a host** | **MAC learning, and ARP suppression** |
| **3** | **"I am a VTEP for this VNI"** | **VTEP discovery, and BUM replication lists** |
| **5** | **IP prefix** | **routing between VNIs, and to the outside** |
| 1, 4 | multihoming, Ethernet segments | **all-active dual-homing of a server** |

**Type 2 is the interesting one, because it carries both MAC and IP.**

> **A VTEP that knows the MAC-to-IP mapping for every host in the VNI can answer ARP requests
> locally.** **This is ARP suppression**, and it removes the largest single source of BUM traffic
> in a data centre fabric — **a broadcast that would have crossed every rack is answered by the
> leaf switch the requester is plugged into.**

**Type 3 supplies the BUM replication list** that §67.2 said the original specification did not
define: **each VTEP advertises its membership, so every other VTEP knows exactly whom to
replicate to** — **without multicast, and without configuration.**

**And type 5 is what makes an EVPN fabric a routed network rather than a set of stretched
VLANs**, which is the next subsection.

## Distributed anycast gateway

**The mechanism that makes workload mobility actually work**, and it is elegant.

**The problem:** **a VM's default gateway is one router.** **Move the VM to another rack and its
traffic must return to that router** — **which is Chapter 51 §51.1's tromboning, inside a data
centre, and it is called hairpinning here.**

**The answer: every leaf switch is the gateway, with the same address and the same MAC.**

```
   Leaf 1  ┐
   Leaf 2  ├── all configured with 10.1.2.1 / 00:00:5e:00:01:01 for VNI 10010
   Leaf 3  ┘

   A VM anywhere in VNI 10010 uses 10.1.2.1 as its gateway.
   Whichever leaf it is attached to answers.
   Moving the VM changes nothing — not even its ARP cache.
```

> **The same address and the same MAC, active on forty switches simultaneously.** **Which would
> be a duplicate-address catastrophe in any other context** (Chapter 53 §53.3) **and is correct
> here, because the leaves never speak to each other in that VNI at Layer 2.**

**And the consequence is that traffic is routed at the first hop, always.** **A VM in rack 3
talking to a VM in rack 17 on a different subnet is routed by leaf 3 and delivered by leaf 17
— two hops, the fabric's minimum** — **rather than travelling to a central gateway and back.**

## Symmetric and asymmetric IRB

**A design detail that is worth knowing because it appears in every vendor's documentation and
the difference matters.**

**Integrated Routing and Bridging — routing between VNIs at the leaf.**

| | **Asymmetric** | **Symmetric** |
|---|---|---|
| Ingress leaf | **routes into the destination VNI, then bridges** | **routes into a transit VNI** |
| Egress leaf | **bridges only** | **routes out of the transit VNI** |
| **Every leaf must know** | **every VNI it might route to** | **only its local VNIs, plus the transit** |
| Scaling | **poor — configuration everywhere** | **good** |
| **Deployed** | legacy | **the standard choice** |

> **Symmetric IRB is why a large fabric is manageable**: **a leaf configures only the VNIs it has
> workloads in**, and inter-subnet traffic crosses a shared transit VNI whose MAC and IP tables
> are small. **Asymmetric requires every leaf to be configured for every subnet, which does not
> scale past a modest fabric.**

## Multihoming, and what EVPN replaced

**A server dual-homed to two leaves for redundancy.**

**Before EVPN this required a vendor-specific mechanism** — **MLAG, vPC, VSS, stacking** — **in
which two switches pretend to be one**, with a proprietary control-plane link between them.

| | **MLAG / vPC** | **EVPN multihoming (types 1 and 4)** |
|---|---|---|
| Switches | **two, and exactly two** | **any number** |
| Mechanism | **proprietary; a peer link** | **standardised, in BGP** |
| Interoperability | **none between vendors** | **specified** |
| The peer link | **a failure domain of its own** | **not required** |
| Split brain | **a real and studied failure mode** | **handled by the designated forwarder election** |

> **EVPN multihoming removes a proprietary mechanism, a physical peer link and a documented
> class of failure**, and **it is the least-discussed and most practically valuable part of
> EVPN** for anyone who has operated an MLAG pair through a peer-link failure.

## What the whole thing produces

**A fabric with a coherent description, which is the point.**

| Layer | Protocol | Carries |
|---|---|---|
| **Underlay** | **BGP (or OSPF/IS-IS)** | **VTEP loopback addresses; ECMP across spines** |
| **Overlay control** | **MP-BGP EVPN** | **MAC, IP, VTEP membership, prefixes** |
| **Overlay data** | **VXLAN** | **the tenant frames** |

**And the properties that follow:**

- **No flooding for known destinations** — type 2 told everyone
- **No ARP flooding** — suppressed at the leaf
- **No spanning tree** — the fabric is routed
- **No configuration per tenant on the spines** — they route the underlay and nothing else
- **Workload mobility without renumbering** — the anycast gateway
- **All links forwarding** — ECMP, not STP

> **This is the architecture the whole chapter has been building towards**, and **its coherence
> is why it displaced every earlier data centre overlay attempt within about five years.**

## What breaks here

**MACs not being learned across the fabric.** **The EVPN session, or a route target mismatch**
(Chapter 50 §50.4). Check the BGP session and the received route count.

**ARP suppression not working.** **Type 2 routes carrying MAC only, not MAC and IP** — **which
happens when the leaf has not learned the IP**, and it resolves after the host's first ARP.

**Traffic hairpinning to a central gateway.** **The distributed anycast gateway is not configured
on that leaf.**

**A leaf that must be configured for every subnet.** **Asymmetric IRB.** Symmetric scales.

**A VM that cannot be moved.** **Its VNI is not configured on the destination leaf.**

**Duplicate MAC detected across two VTEPs.** **Either a genuine duplicate, or a VM that moved and
the old entry has not withdrawn** — **EVPN has a MAC mobility sequence number for exactly this,
and repeated moves increment it.**

**Everything working and no visibility into it.** **The control plane is BGP and the data plane
is UDP** — **your existing tooling sees neither.** Chapter 54 §54.4's telemetry gap, in a fabric.

> **Network+ note.** EVPN is beyond Network+'s scope and is what you will meet in a data centre.
> The transferable content is: **a control plane distributes reachability information so that the
> data plane does not have to discover it by flooding** — **which is Chapter 17's switch learning
> versus Chapter 31's routing protocols, and the same argument won again.**
