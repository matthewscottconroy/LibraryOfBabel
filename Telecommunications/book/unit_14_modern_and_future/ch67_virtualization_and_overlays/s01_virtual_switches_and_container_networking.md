# 67.1 Virtual Switches and Container Networking

The network moved inside the server, and most of the packets in a modern data centre are
forwarded by software before they reach any physical switch.

## The virtual switch

A hypervisor runs many virtual machines on one physical host, each with a virtual NIC, and
something must forward frames between them and to the physical network.

```
   ┌──────────────────────────────────────────────────┐
   │  Host                                            │
   │   ┌─────┐  ┌─────┐  ┌─────┐  ┌─────┐            │
   │   │ VM1 │  │ VM2 │  │ VM3 │  │ VM4 │            │
   │   └──┬──┘  └──┬──┘  └──┬──┘  └──┬──┘            │
   │      │        │        │        │               │
   │   ┌──┴────────┴────────┴────────┴──┐            │
   │   │       VIRTUAL SWITCH           │            │
   │   └──────────────┬─────────────────┘            │
   └──────────────────┼──────────────────────────────┘
                   ┌──┴──┐
                   │ NIC │──── to the physical leaf switch
                   └─────┘
```

And it is a switch in the full sense (Chapter 17): it learns MAC addresses, it has VLANs,
it has ACLs, it counts traffic, and it is configured.

| Implementation | Notes |
|---|---|
| **Linux bridge** | **simple, in-kernel, adequate** |
| **Open vSwitch (OVS)** | **OpenFlow-programmable** (Chapter 68), VXLAN, QoS, mirroring |
| **VMware vSphere Distributed Switch** | managed across a cluster from vCenter |
| **Hyper-V virtual switch** | |
| **SR-IOV** | **bypasses the vSwitch entirely** — see below |

> Two virtual machines on the same host communicating with each other generate traffic that
> never reaches a physical switch, a physical cable, or any monitoring you have deployed
> (Chapter 54 §54.4). The visibility gap this creates is the single most important operational
> consequence of virtualisation for a network engineer.

And it is why microsegmentation (Chapter 60 §60.4) is enforced at the virtual switch:
it is the only place that sees that traffic.

## The cost of software forwarding

Every packet costs CPU, and at 25 or 100 Gb/s that becomes the constraint.

| Approach | Mechanism | Trade |
|---|---|---|
| **Kernel bridge / OVS** | **the host CPU forwards** | **flexible, and it consumes cores** |
| **OVS with DPDK** | **poll-mode drivers in userspace, dedicated cores** | **much faster; the cores are gone permanently** |
| **SR-IOV** | **the NIC presents virtual functions directly to VMs** | **near line rate, and the vSwitch is bypassed** |
| **Smart NIC / DPU** | **the forwarding runs on the NIC's own processor** | **the fastest, and it is a computer on your NIC** |

**SR-IOV's trade is the one worth understanding:**

> **A VM with an SR-IOV virtual function talks to the NIC directly.** **The hypervisor's switch
> does not see the traffic**, so no vSwitch ACLs, no microsegmentation, no mirroring, and no
> live migration on many platforms. You have bought line rate and lost the policy
> enforcement point.

Which is a legitimate choice for a specific workload — a network function, a storage node,
a high-frequency trading system — and a poor default.

And the DPU is the direction of travel. Offloading the virtual switch, the encryption, the
storage protocol and the firewall to a processor on the NIC returns the host's cores to
applications, and it makes the NIC a device with its own operating system, its own management
plane and its own patching requirement (Chapter 55 §55.3) — which is a new class of thing to
own.

## Containers: a different model

A container is not a small virtual machine. It shares the host's kernel, and its network
isolation comes from **namespaces** rather than from virtualisation.

```
   Host network namespace
   ┌──────────────────────────────────────────────┐
   │  eth0 ── bridge ──┬── veth0 ─┐               │
   │                   │           │  ┌─────────┐ │
   │                   │           └──┤ veth1   │ │  container netns
   │                   │              │ eth0    │ │  (its own routes,
   │                   │              │ 10.1.2.3│ │   ARP table, ports)
   │                   │              └─────────┘ │
   │                   └── veth2 ─── … container 2 │
   └──────────────────────────────────────────────┘
```

A network namespace has its own interfaces, routing table, ARP cache, iptables rules and port
space — so two containers can both listen on port 80 without conflict, and each behaves as
though it has a machine to itself.

**And a `veth` pair is the mechanism:** two virtual interfaces joined back to back, one in
each namespace, so a frame sent into one emerges from the other.

The primitives are all Linux, all available from the shell, and worth trying once:

```
   ip netns add red
   ip link add veth0 type veth peer name veth1
   ip link set veth1 netns red
   ip netns exec red ip addr add 10.1.2.3/24 dev veth1
```

> **Four commands and you have built a container's network by hand.** Every container runtime
> does exactly this, with more automation.

## CNI, and the Kubernetes model

Kubernetes imposes a networking model and does not implement it — which is the design
decision that produced an ecosystem.

**The model's four requirements:**

| | |
|---|---|
| **Every pod gets its own IP address** | **not a port on a shared address** |
| **Pods can reach each other without NAT** | **flat, routable, within the cluster** |
| **Nodes can reach pods without NAT** | |
| **The address a pod sees for itself is the one others use** | **which NAT would break** |

> "Every pod gets a real IP address and NAT does not exist inside the cluster" is a deliberate
> rejection of the port-mapping model that preceded it, and it is Chapter 33's argument
> applied inside a data centre: NAT breaks things, so do not use it.

**And the consequence is an addressing problem.** A cluster of 500 nodes with 100 pods each
needs 50,000 addresses (Chapter 27), which is why cluster networking uses large private
ranges and why IPv6 is increasingly attractive here.

CNI — the Container Network Interface — is the plugin API. Kubernetes calls a binary when
a pod starts, and the binary attaches it to a network however it likes.

| Plugin | Approach |
|---|---|
| **Flannel** | **VXLAN overlay** (§67.2) — simple, and it works |
| **Calico** | **routed, with BGP** (Chapter 32) — **no overlay; the fabric routes pod addresses** |
| **Cilium** | **eBPF-based** — policy and load balancing in the kernel, without iptables |
| **Weave, Antrea, others** | various |

Calico's approach deserves a note because it is the one a network engineer will recognise:

> Calico runs BGP on every node and advertises the pod addresses into the fabric. **There is
> no overlay and no encapsulation** — **the physical network routes to the pods directly**, and
> the leaf switches learn pod prefixes as ordinary routes. Which is elegant, and it requires
> the network team and the platform team to have a conversation, and that requirement is
> frequently the reason an overlay is chosen instead.

And Cilium's approach is the direction of travel. **eBPF** (Chapter 64's BPF entry) lets
policy, load balancing and observability run in the kernel's datapath without iptables' linear
rule evaluation — which matters at a scale where an iptables chain has ten thousand rules
and every packet traverses it.

## Services, and the load balancing nobody sees

**A pod's address is ephemeral.** Pods are created and destroyed constantly, so nothing can
be configured to talk to one.

A Kubernetes Service is a stable virtual address in front of a changing set of pods, and
the load balancing is implemented on every node — by iptables rules, by IPVS, or by eBPF.

> **There is no load balancer.** Each node rewrites the destination address of packets to the
> service's virtual IP, choosing a pod, using rules programmed locally. **It is distributed
> load balancing implemented as NAT**, and it is invisible to every network tool you own.

**Which has a specific diagnostic consequence:** a packet capture on the wire shows traffic to
a pod address that appears in no DNS record and no configuration, and tracing why requires
tooling that understands the cluster (`kubectl`, `cilium monitor`) rather than tooling that
understands the network.

## What this means for a network engineer

Four things, and they are the chapter's practical content.

**Your visibility ends at the physical port.** Intra-host and intra-cluster traffic is
invisible to SPAN, to NetFlow and to your IDS — and it is the majority of the traffic.
The instrumentation must be inside (Chapter 54's flow export from the vSwitch, or eBPF-based
observability).

The IP address is no longer a stable identifier. A pod's address is reused within minutes,
so a firewall rule or a monitoring configuration keyed on an address is wrong almost
immediately — which is why policy in these environments is expressed in labels (Chapter 60
§60.4).

**The addressing plan must accommodate it.** Cluster pod and service ranges are large, and they
must not collide with anything (Chapter 27) — and the commonest failure is a cluster
installed with a default range that overlaps the corporate network.

And there is a new device class to operate. Virtual switches, CNI plugins, DPUs and service
meshes are network devices, with configuration, versions, EOL dates and vulnerabilities
(Chapter 55) — owned by a team that may not consider itself a network team.

> The honest summary: a substantial and growing fraction of the network is operated by people
> who do not call themselves network engineers, using tools a network engineer does not have.
> Chapter 72 returns to this as an organisational question, and the practical response is to
> be in the conversation early.

## What breaks here

**Traffic between two VMs invisible to monitoring.** They are on the same host. Flow export
from the vSwitch.

**Microsegmentation not applying to some workloads.** SR-IOV bypasses the virtual switch.

**A VM that cannot be live-migrated.** **SR-IOV, usually.** The trade was made and not recorded.

A cluster whose pod range overlaps the corporate network. A default accepted at
installation. Chapter 27 §27.2's plan should have covered it.

A firewall rule keyed on a pod's address. The address was reused nine minutes later.
Label-based policy.

A capture showing traffic to an address in no configuration. A Kubernetes Service's
distributed NAT.

**Host CPU exhausted at high packet rates.** **Software forwarding.** DPDK, SR-IOV or a DPU,
each with its own trade.

**A NIC with an unpatched vulnerability.** A DPU is a computer. It belongs in the inventory
(Chapter 53 §53.2).

> **Network+ note.** Objective 1.8 and 1.2 touch virtualisation. Over-learn: a virtual switch
> connects virtual machines within a host; **VMs and containers share the host's physical
> NICs**; containers are lighter than virtual machines and share the host kernel; and
> **network functions can be virtualised (NFV).** The vSwitch's existence and the intra-host
> visibility gap are the examinable and the practical points respectively.
