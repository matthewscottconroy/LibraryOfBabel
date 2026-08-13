# Chapter 67 — Further Reading

## Read these first

**Clos, C. (1953). "A Study of Non-Blocking Switching Networks." *Bell System Technical
Journal*.**
**The topology, from a telephone problem.** **The mathematics is followable and the recurrence
seventy years later is the pleasure of reading it.**

**Lapukhov, P., Premji, A. & Mitchell, J. (2016). RFC 7938, "Use of BGP for Routing in
Large-Scale Data Centers."**
**F7 uses it.** **Short, and it is the argument for the underlay design in §67.4** — written by
people operating fabrics at scale rather than by a standards committee in the abstract.

**Sajassi, A. et al. (2015). RFC 7432, "BGP MPLS-Based Ethernet VPN."**
**F8 uses sections 1–8.** **Dense, and the route type definitions are the reference you will
return to.** **RFC 8365** covers the VXLAN data plane specifically and is the one to read
alongside it.

**Mahalingam, M. et al. (2014). RFC 7348, "VXLAN."**
**Short.** **Read section 4 for the frame format** and note how much the specification leaves
undefined — which is §67.2's point about flood-and-learn.

## Data centre architecture

**Al-Fares, M., Loukissas, A. & Vahdat, A. (2008). "A Scalable, Commodity Data Center Network
Architecture."** SIGCOMM.
**The paper that brought Clos into the data centre literature**, and it argues the economics —
commodity switches in a fat tree against expensive modular chassis — **which is the reason the
industry moved.**

**Singh, A. et al. (2015). "Jupiter Rising: A Decade of Clos Topologies and Centralized Control
in Google's Datacenter Network."** SIGCOMM.
**Google's own account of five generations of fabric**, and it is unusually candid about what
did not work. **The best single paper on data centre networking at scale.**

**Greenberg, A. et al. (2009). "VL2: A Scalable and Flexible Data Center Network."** SIGCOMM.
Microsoft's parallel work, **and the load-balancing and address-virtualisation arguments are
distinct from Google's.**

**Barroso, L., Hölzle, U. & Ranganathan, P. — *The Datacenter as a Computer*.**
**Free.** Recommended in Chapter 56 and it belongs here — **the chapters on networking and on
cost are the context for §67.4's optics argument.**

## Overlays and virtualisation

**Koponen, T. et al. (2014). "Network Virtualization in Multi-tenant Datacenters."** NSDI.
**The NVP/NSX paper**, by the Nicira team. **The clearest statement of the network
virtualisation argument** and of what a distributed control plane must actually do.

**Casado, M. et al. — the Ethane and NOX papers** (Chapter 68's reading) — **the intellectual
lineage.**

**GENEVE: RFC 8926.** **Read the introduction for the "we kept needing another
encapsulation" argument** that motivated it.

**VMware NSX, Cisco ACI and Arista CloudVision documentation** — **read one thoroughly and one
superficially.** **The design documents explain the model; the troubleshooting guides explain
what actually goes wrong**, and the second is more useful.

**`ipSpace.net`** (Ivan Pepelnjak) — **recommended in Chapters 51 and 60 and most relevant here.**
**Consistently the most rigorous and most sceptical available material on overlays, EVPN and
data centre design**, and the "do you actually need this?" material is the necessary counterweight
to vendor architecture documents.

## Containers and Kubernetes

**The Kubernetes networking documentation** (kubernetes.io) — **the four requirements, stated by
the people who chose them.** **Short.**

**The CNI specification** (github.com/containernetworking/cni) — **twenty pages, and reading it
makes the plugin ecosystem comprehensible.**

**Cilium's documentation and Thomas Graf's talks** — **the best available introduction to eBPF
in a networking context**, and the "why not iptables" material is quantitative.

**Calico's documentation on BGP peering with the fabric** — **directly relevant to a network
engineer**, and it is the design that requires a conversation between teams.

**Gregg, B. — *BPF Performance Tools*, and the eBPF documentation.**
**For eBPF itself rather than for its networking use**, and Chapter 64's BPF entry is the
history.

## Hands-on

**containerlab** (containerlab.dev) — **F3 uses it.** **Builds multi-vendor labs from a YAML
file, including FRR, SONiC, Arista cEOS and Nokia SR Linux**, and **an EVPN/VXLAN fabric on a
laptop is genuinely achievable in an afternoon.**

**FRRouting** (frrouting.org) — **free, and it implements BGP EVPN.** **The EVPN documentation is
the practical reference for F3.**

**`kind`, `k3s` or `minikube`** — **F4.** **A cluster on a laptop, and examining its iptables
rules teaches more about service implementation than any diagram.**

**`ip netns`, `ip link add … type veth`, `ip link add … type vxlan`** — **F1 and F2.** **Every
mechanism in §67.1 and §67.2 is available from a Linux shell with no special software**, and
building one by hand is the fastest route to understanding it.

**Open vSwitch** — **`ovs-vsctl`, `ovs-ofctl`, `ovs-appctl`.** **The programmable virtual switch,
and Chapter 68's OpenFlow material runs on it.**

**Wireshark's VXLAN and GENEVE dissectors** — **F2**, and worth checking whether your own flow
tooling decodes them, because many do not.

## Following the field

**The IETF `bess` and `nvo3` working groups** — where EVPN and the encapsulations are developed.

**NANOG and the data centre track at RIPE** — **operators describing fabrics they run**, and the
scaling war stories are where the honest constraints appear.

**The SONiC and Open Compute Project material** — **open switch operating systems and open
hardware designs**, and the direction that decouples the switch from its vendor.

**The AI-cluster networking literature**, which is where the current pressure is: **RoCE,
lossless Ethernet, and whether the fabric should be Ethernet or InfiniBand.** **The
oversubscription arithmetic of §67.4 is being pushed to 1:1 and beyond**, and the requirements
are unlike anything else in this book.

## Where to look next

**Chapter 68** covers the control-plane separation this chapter's overlays depend on and the
programmability argument in general; **Chapter 69** is the same architecture as a service you
rent rather than build; **Chapter 70** is how a fabric of two hundred switches is actually
configured; and **Chapter 72** takes up the organisational question this chapter's history
raises.
