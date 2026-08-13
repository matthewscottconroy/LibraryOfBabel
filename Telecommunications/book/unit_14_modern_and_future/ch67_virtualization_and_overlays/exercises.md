# Chapter 67 — Exercises

## A. Recall

**A1.** Distinguish north–south from east–west traffic and say why the shift broke three-tier
designs.

**A2.** Why does a routed leaf–spine fabric use all its links when a three-tier design does not?

**A3.** State the visibility problem that virtual switches create, and why it matters
operationally.

**A4.** What does SR-IOV buy and what does it cost?

**A5.** What is a network namespace, and what does a `veth` pair do?

**A6.** State Kubernetes' four networking requirements, and the design decision the fourth
embodies.

**A7.** Describe Calico's approach and say why it is frequently rejected.

**A8.** How is a Kubernetes Service load balanced, and why is it invisible to network tooling?

**A9.** Give VXLAN's overhead in bytes and the required fabric MTU for a 1,500-byte inner frame.

**A10.** Why is the VXLAN UDP source port set to a hash of the inner frame?

**A11.** Compare the two mechanisms for VXLAN BUM traffic and say which is commonly deployed.

**A12.** What does GENEVE add over VXLAN, and what is the argument for it?

**A13.** Name the three EVPN route types that do the work and what each solves.

**A14.** What is ARP suppression and why does it matter?

**A15.** Explain the distributed anycast gateway, and why it would be a fault in any other
context.

**A16.** Distinguish symmetric from asymmetric IRB and say which scales.

**A17.** What does EVPN multihoming replace, and what three things does it remove?

**A18.** State the oversubscription formula and give an appropriate ratio for general
virtualisation and for storage.

**A19.** Why is BGP used as a data centre underlay protocol? State the reason precisely.

**A20.** What bounds a two-tier fabric's size?

## B. Apply

**B1.** Compute the oversubscription ratio for each leaf configuration and state a workload for
which each is appropriate:

(a) 48 × 10G down, 4 × 40G up
(b) 48 × 25G down, 6 × 100G up
(c) 48 × 25G down, 8 × 100G up
(d) 32 × 100G down, 8 × 400G up
(e) 24 × 100G down, 2 × 400G up

**B2.** Design a fabric for 2,000 servers each requiring one 25G port.

(a) How many leaves are needed at 48 ports each?
(b) What spine port count is required?
(c) How many spines for a 2:1 oversubscription, given 6 × 100G uplinks per leaf?
(d) How many fabric cables in total?
(e) What happens to capacity if one spine fails?

**B3.** A tenant network carries 1,500-byte frames over VXLAN.

(a) What fabric MTU is the minimum?
(b) The fabric is configured at 1,500. Predict the symptoms precisely.
(c) Give the diagnostic command and the value you would test with.
(d) State the standard remedy and why it is chosen over the alternative.

**B4.** For each requirement, state whether an overlay is warranted and why:

(a) A campus with 40 VLANs and static servers
(b) A cloud provider with 30,000 tenants
(c) A data centre where VMs must migrate between racks without renumbering
(d) A Kubernetes cluster where the network team will assign routable addresses
(e) A Kubernetes cluster where the network team will not

**B5.** An EVPN fabric is deployed. For each symptom, give the diagnosis:

(a) Two hosts in the same VNI on different leaves cannot communicate; the underlay pings fine
(b) ARP traffic is high across the fabric despite EVPN
(c) A VM moved to another rack and its traffic returns to the original leaf
(d) Every leaf must be configured for every subnet
(e) A duplicate MAC is reported across two VTEPs after a migration

**B6.** Compare, in a table, the three ways a Kubernetes cluster's pod traffic can reach the
physical network — overlay (VXLAN), routed (BGP), and host port mapping — on: address
consumption, MTU impact, visibility to network tooling, and organisational prerequisite.

**B7.** Two virtual machines on the same host exchange 4 Gb/s of traffic.

(a) What does the physical leaf switch see?
(b) What does a SPAN-based IDS see?
(c) What does NetFlow from the leaf see?
(d) Design the instrumentation that would give visibility, and state its cost.

**B8.** A five-stage Clos fabric has two pods.

(a) How many hops between two servers in the same pod?
(b) Between two servers in different pods?
(c) State the consequence for workload placement and who should be aware of it.

## C. Analyse

**C1.** The chapter argues that the traffic pattern changed and the topology followed. Analyse
what drove the traffic change, and predict what would have to change for the topology to shift
again.

**C2.** Analyse the SR-IOV trade rigorously: what exactly is lost, for which workloads is the
trade correct, and what compensating controls would you require before permitting it?

**C3.** Kubernetes deliberately rejected NAT inside the cluster. Analyse this decision against
Chapter 33's arguments — what does it buy, what does it cost in addresses, and would you make
the same choice?

**C4.** Analyse the claim that "we built an overlay because the network team would not give us
routable addresses" is an organisational failure expressed as an architecture. Is this fair?
What would the network team's counter-argument be, and how should it be resolved?

**C5.** EVPN reuses MPLS L3VPN's mechanism twenty years later for a different encapsulation.
Analyse what this says about the durability of the underlying idea, and identify one other
mechanism in this book that has been reused in the same way.

**C6.** The distributed anycast gateway configures the same IP and MAC on forty switches.
Analyse why this is safe here and catastrophic elsewhere, and state precisely what property
makes the difference.

**C7.** Analyse the choice of BGP as a fabric underlay protocol. The chapter says it was chosen
because it is explicitly configured rather than because it is a better IGP. Assess this
argument, and say when an IGP would nonetheless be the right choice.

**C8.** Analyse the visibility gap this chapter creates. A network engineer's tools see the
physical ports and a growing majority of traffic is elsewhere. What should a network team
actually do about this — technically and organisationally?

## D. Design

**D1.** Design a fabric for a 3,000-server data centre: topology, oversubscription, underlay
protocol, overlay decision, cabling approach, and the growth path to 6,000 servers. Justify each
choice and state the cost drivers.

**D2.** Design the network for a Kubernetes platform of 200 nodes: CNI choice, addressing
(pods, services, nodes), the interaction with the physical fabric, MTU, policy enforcement, and
observability. State what you would require from the platform team and what you would provide.

**D3.** An organisation runs a three-tier data centre and is planning a refresh. Design the
migration to a fabric: the sequence, how workloads move, what runs in parallel, how Layer 2
adjacency is preserved during the transition, and the rollback position at each stage.

**D4.** Design the observability for a virtualised environment where 70% of traffic never
reaches a physical switch. Specify what is collected, from where, at what cost, and what
questions it must be able to answer.

**D5.** Write the two-page argument you would make against building a leaf–spine fabric with an
EVPN/VXLAN overlay for a 40-server environment in two racks, addressed to an architect who has
proposed it. Be specific about the costs and offer a defensible alternative.

## E. Troubleshoot

**E1.** Two VMs in the same VNI on different hosts cannot communicate. The underlay VTEP-to-VTEP
ping succeeds. Give your next four checks.

**E2.** An application works within a rack and fails between racks, with large transfers only.
Diagnose in one command.

**E3.** All traffic between two heavily-communicating hosts traverses one spine while seven are
idle. Diagnose.

**E4.** After a Kubernetes upgrade, pods can reach each other and not the outside. Give three
possible causes.

**E5.** A newly installed cluster cannot reach a corporate service at 10.244.0.0/16. Diagnose.

**E6.** A firewall rule permitting a specific container's address stops working every few days.
Explain and give the correct approach.

**E7.** An IDS reports no east–west traffic in a virtualised environment where the application
team says there is a great deal. Explain.

**E8.** A VM cannot be live-migrated to another host, and the configuration appears correct.
Give the most likely cause.

**E9.** After a spine switch failure, the fabric's capacity falls by 40% rather than the
expected 12.5%. Give two explanations.

**E10.** A packet capture in the fabric shows only UDP port 4789 and your flow analysis reports
one application. Explain, and say what would be needed.

## F. Extend

**F1.** Build a container network by hand with `ip netns`, `veth` pairs and a bridge: two
namespaces, connectivity between them, and out to the host. Then add a third and route between
two bridges. Document what each command did.

**F2.** Build a VXLAN tunnel between two Linux hosts with `ip link add vxlan0 type vxlan`.
Capture the traffic and identify every field in §67.2's diagram. Then break the MTU deliberately
and reproduce the symptom.

**F3.** Build a small EVPN/VXLAN fabric in containerlab or GNS3 (FRR supports EVPN). Advertise a
MAC, observe the type 2 route in BGP, and demonstrate ARP suppression. Then move a host between
leaves and watch the MAC mobility sequence number.

**F4.** Install a Kubernetes cluster (kind, k3s or minikube) and examine its networking: the pod
CIDR, the service CIDR, the CNI in use, and the iptables or eBPF rules implementing services.
Trace one service request end to end.

**F5.** Measure the visibility gap: on a virtualised host, generate traffic between two VMs and
confirm what the physical switch's counters and flow export report. Then enable flow export from
the virtual switch and repeat.

**F6.** Compute the full cost of a fabric for 1,000 servers: switches, optics, cables, and
power. Determine what fraction is optics, and how it changes between 25G, 100G and 400G server
connectivity.

**F7.** Read RFC 7938 ("Use of BGP for Routing in Large-Scale Data Centers") and summarise its
argument in one page. Identify the two design recommendations you find least intuitive and
explain them.

**F8.** Read RFC 7432 (EVPN) sections 1 to 8 and map each route type onto a problem from §67.3.
Identify one capability the RFC provides that this chapter did not cover.
