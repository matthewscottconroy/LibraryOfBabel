# 67.2 VXLAN and GENEVE

§67.4's routed fabric solves the traffic problem and breaks something applications depend on:
virtual machines expect to move between hosts without changing IP address.

VXLAN is the reconciliation — a Layer 2 network built on top of a Layer 3 one.

## The idea

> Encapsulate the Ethernet frame in a UDP datagram, route the datagram normally, and
> decapsulate at the far end. **The fabric routes; the endpoints see a switch.**

```
   ┌──────────┬─────────┬──────────┬────────┬─────────────────────────────┐
   │ Outer    │ Outer   │ UDP      │ VXLAN  │  ORIGINAL ETHERNET FRAME    │
   │ Ethernet │ IP      │ dst 4789 │ hdr    │  (dst MAC, src MAC, payload)│
   │  14 B    │  20 B   │   8 B    │  8 B   │                             │
   └──────────┴─────────┴──────────┴────────┴─────────────────────────────┘
    └──────────────── 50 bytes of overhead ────────────────┘
```

| Field | |
|---|---|
| **Outer IP** | **the source and destination VTEPs** — routable addresses in the fabric |
| **UDP destination port** | **4789** |
| **UDP source port** | **a hash of the inner frame** — **so ECMP spreads flows across spines** |
| **VNI** — VXLAN Network Identifier | **24 bits — 16.7 million segments** |

**Two of those deserve expansion.**

The 24-bit VNI is the answer to 802.1Q's 12 bits (Chapter 20 §20.2): 4,094 usable VLANs
against 16.7 million. A campus never approaches the first; a cloud provider with 50,000
tenants exhausts it immediately.

And the UDP source port is a deliberate trick:

> **Nothing in a VXLAN packet needs a UDP source port.** It is set to a hash of the inner
> frame's headers so that the fabric's ECMP hashing, which looks at the outer five-tuple,
> spreads different inner flows across different spines (Chapter 29 §29.3). Without it,
> every packet between two hosts would take one path and the fabric's parallelism would be
> wasted.

Which is a good piece of design: a field with no meaning, used to communicate with a
mechanism that cannot see the encapsulated content.

## The VTEP

A VXLAN Tunnel End Point encapsulates and decapsulates, and it may be:

| Where | Notes |
|---|---|
| **The hypervisor's virtual switch** | **software; the VM never knows** |
| **The leaf switch** | **hardware; for bare-metal servers and for performance** |
| **A gateway** | **bridging VXLAN to a physical VLAN** — the migration path |
| **A NIC or DPU** | §67.1 |

And the VTEP maintains the mapping that makes it work:

$$\text{inner MAC} \rightarrow \text{remote VTEP address}$$

> **Which is a MAC address table whose "ports" are IP addresses.** Chapter 17's switch,
> with the far side of each entry being a tunnel rather than an interface.

## The problem VXLAN alone does not solve

Ethernet needs broadcast, unknown-unicast and multicast — BUM traffic (Chapter 17 §17.2) —
and a routed fabric does not do broadcast.

**The original answer was IP multicast:** each VNI maps to a multicast group; a VTEP joins it;
BUM traffic is sent to the group.

> Which works and requires multicast routing in the fabric — PIM, rendezvous points, the whole
> apparatus (Chapter 27 §27.3). **Many operators declined**, because multicast is
> operationally demanding and rarely otherwise needed.

**The alternative is head-end replication (ingress replication):** the VTEP sends a separate
unicast copy to every other VTEP in that VNI.

| | **Multicast** | **Head-end replication** |
|---|---|---|
| Fabric requirement | **PIM and multicast routing** | **none** |
| Traffic for one broadcast | **one copy** | **$n-1$ copies** |
| Scaling | **good** | **poor at large $n$** |
| Operational complexity | **high** | **low** |
| **Deployed** | **rarely** | **commonly** |

And both require the VTEP to know which other VTEPs are in each VNI, which the original
specification did not say how to learn.

> **VXLAN as originally specified was flood-and-learn** (Chapter 17 §17.1) **over a routed
> fabric** — which works, floods a great deal, and scales badly. **§67.3's EVPN is what made
> it operationally sensible**, and it is the reason VXLAN succeeded where earlier overlay
> attempts did not.

## The MTU consequence

50 bytes of overhead, and it is Chapter 66 §66.3's fault waiting to happen.

| | |
|---|---|
| **Inner frame** | 1,500 bytes |
| **VXLAN overhead** | **50 bytes** |
| **Required fabric MTU** | **1,550 minimum** |

> The standard answer is to configure jumbo frames — 9,000 bytes — on the entire fabric,
> so that the overhead never matters and the inner MTU can be anything reasonable.

And the failure when it is not done is the familiar one: small packets work, large packets
vanish (Chapter 66 §66.3), and it is diagnosed in one `ping -M do` command.

Note that the encapsulated frame's own DF bit is not visible to the fabric, so the
fabric's routers will fragment the outer packet if they can — which works and costs
performance — and if DF is set on the outer header, they will not, producing the black
hole.

## GENEVE

Generic Network Virtualization Encapsulation — the successor, and its argument is
extensibility.

| | **VXLAN** | **GENEVE** |
|---|---|---|
| Header | **fixed, 8 bytes** | **variable, with TLV options** |
| Identifier | **24-bit VNI** | **24-bit VNI** |
| UDP port | **4789** | **6081** |
| **Metadata** | **none** | **arbitrary, in options** |
| Hardware support | **universal** | **growing** |

The argument is that overlays kept needing to carry information the header had no room for —
a security group, a policy tag, a tenant identifier, an OAM marker — and every vendor
solved it by inventing another encapsulation (NVGRE, STT, and VXLAN-GPE among them).

> GENEVE's design principle is that the encapsulation should not need to be revised again.
> A TLV option space means new metadata is added without a new protocol, which is the lesson
> of IPv4's option field being unusable and IPv6's extension headers being the replacement
> (Chapter 28 §28.2).

**Its adoption is real and partial.** NSX uses it, OVN supports it, and hardware VTEP support
is less universal than VXLAN's — so VXLAN remains the default for anything that must be
terminated in a switch ASIC.

## Where overlays are and are not the answer

An honest assessment, because overlays are frequently proposed reflexively.

| Use an overlay when | Do not when |
|---|---|
| **Tenants need isolated Layer 2 across a routed fabric** | **the fabric can simply route to the workloads** (§67.1's Calico) |
| **More than 4,094 segments are needed** | **a campus with forty VLANs** |
| **Workloads must move without renumbering** | **workloads are static** |
| **Policy must follow the workload** | |
| **The physical network cannot be changed** | **you control it and could route instead** |

> An overlay adds a layer of encapsulation, an MTU constraint, a control plane, a
> troubleshooting boundary and a class of failure that the tools do not see. Where the
> underlay can simply route to the endpoints, it should — and "we built an overlay because
> the network team would not give us routable addresses" is a real and common reason, and it is
> an organisational failure expressed as an architecture.

## Troubleshooting an overlay

The specific difficulty: two networks, and a fault in either presents in the other.

**The discipline is to separate them:**

```
   1.  Is the UNDERLAY healthy?
       ping VTEP-to-VTEP, by outer address, with the fabric MTU
   2.  Is the MTU sufficient?
       ping -M do -s <inner+50> between VTEPs
   3.  Does the VTEP know the remote VTEP for this VNI?
       show vxlan / show nve peers
   4.  Does the VTEP have the inner MAC?
       show mac address-table vni <n>
   5.  Only then, the inner network
```

> **Steps 1 and 2 resolve most overlay faults**, and **step 2 in particular** — because the
> overlay's MTU problem is the single commonest overlay fault and it presents as an application
> problem three layers up.

**And the tooling gap is real:** `traceroute` inside an overlay shows one hop, because the
fabric's routers are invisible to the encapsulated packet. A fault in the underlay presents
as "the two hosts cannot talk" with nothing in between, which is why step 1 exists.

## What breaks here

Small packets work and large ones vanish, inside a tenant network. **Fabric MTU.** 50 bytes,
and jumbo frames are the answer.

Two hosts in the same VNI cannot communicate; the underlay is fine. The VTEPs do not know
about each other for that VNI — a control plane problem (§67.3).

**Broadcast-heavy applications performing badly.** Head-end replication at scale — $n-1$
copies of every ARP.

All traffic between two hosts taking one spine. The UDP source port is not being varied,
so ECMP cannot spread it. A VTEP implementation problem.

**`traceroute` inside the overlay showing one hop.** **Correct.** The fabric is invisible.

A capture in the fabric showing only UDP 4789. **Correct**, and Wireshark decodes it — but
your IDS and your flow analysis may not (Chapter 54 §54.4).

An overlay built because routable addresses were refused. An organisational failure
expressed as an architecture, and it will be maintained forever.

> **Network+ note.** Objective 1.8 touches VXLAN. Over-learn: VXLAN encapsulates Layer 2
> frames in UDP for transport over a Layer 3 network; the VNI is 24 bits, allowing 16
> million segments against VLAN's 4,094; **a VTEP performs encapsulation and
> decapsulation**; and **overlays add MTU overhead.** The VNI-versus-VLAN comparison is
> examined.
