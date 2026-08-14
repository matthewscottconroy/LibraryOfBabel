# Chapter 67 — Important Concepts

The traffic changed direction and the topology followed *(intro, §67.4)* — A single request
arriving at a web tier triggers twenty or fifty internal exchanges before a byte returns to the
user. Three-tier optimised for north–south; east–west now dwarfs it, frequently by an order
of magnitude — and two servers in adjacent racks traversed six hops through the most
contended devices while spanning tree blocked half the links.

Most packets in a modern data centre are forwarded by software before reaching any physical
switch *(§67.1)* — Two VMs on one host generate traffic that never reaches a cable, a switch
or any monitoring you have deployed. This visibility gap is the most important
operational consequence of virtualisation for a network engineer, and it is why
microsegmentation is enforced at the virtual switch — the only place that sees it.

SR-IOV buys line rate and loses the policy enforcement point *(§67.1)* — The hypervisor's
switch does not see the traffic, so no ACLs, no microsegmentation, no mirroring and frequently
no live migration. A legitimate choice for a specific workload and a poor default.

A DPU is a computer on your NIC *(§67.1)* — Its own operating system, management plane,
versions and vulnerabilities. A new class of thing to own, and it belongs in Chapter 53's
inventory.

A container's network is four Linux commands *(§67.1)* — A namespace with its own routes,
ARP table and port space, joined to the host by a `veth` pair. Two containers can both
listen on port 80 without conflict, and every runtime does exactly this with more automation.

"Every pod gets a real IP and NAT does not exist inside the cluster" *(§67.1)* — A
deliberate rejection of port mapping, and Chapter 33's argument applied inside a data centre.
Its cost is addresses: 500 nodes at 100 pods each is 50,000, which is why cluster ranges are
large and why IPv6 is attractive here.

Calico routes; it does not encapsulate *(§67.1)* — BGP on every node advertising pod
prefixes into the fabric, which the leaves learn as ordinary routes. Elegant, and it requires
the network team and the platform team to have a conversation — and that requirement is
frequently why an overlay is chosen instead.

A Kubernetes Service is distributed load balancing implemented as NAT on every node
*(§67.1)* — There is no load balancer. Which means a capture shows traffic to a pod address
that appears in no DNS record and no configuration, and tracing it needs cluster tooling rather
than network tooling.

The IP address is no longer a stable identifier *(§67.1)* — A pod's address is reused
within minutes, so a firewall rule or monitoring configuration keyed on one is wrong almost
immediately. Policy is expressed in labels.

A substantial and growing fraction of the network is operated by people who do not call
themselves network engineers *(§67.1)* — using tools a network engineer does not have. The
practical response is to be in the conversation early.

VXLAN: encapsulate the frame in UDP, route the datagram, decapsulate at the far end
*(§67.2)* — The fabric routes; the endpoints see a switch. 24-bit VNI gives 16.7 million
segments against 802.1Q's 4,094.

The VXLAN UDP source port is a hash of the inner frame, and it has no other purpose
*(§67.2)* — So the fabric's ECMP, which sees only the outer five-tuple, spreads different inner
flows across different spines. A field with no meaning, used to communicate with a mechanism
that cannot see the encapsulated content.

A VTEP's forwarding table is a MAC address table whose ports are IP addresses *(§67.2)* —
Chapter 17's switch, with the far side of each entry being a tunnel.

VXLAN as specified was flood-and-learn over a routed fabric *(§67.2)* — Which works, floods
a great deal and scales badly. Multicast requires PIM in the fabric and most operators
declined; head-end replication sends $n-1$ copies of every broadcast. EVPN is what made it
operationally sensible, and it is why VXLAN succeeded where earlier overlays did not.

50 bytes of overhead, and jumbo frames on the fabric are the standard answer *(§67.2)* —
The failure without them is Chapter 66 §66.3's: small packets work, large ones vanish,
diagnosed in one `ping -M do`.

Where the underlay can simply route to the endpoints, it should *(§67.2)* — An overlay
adds encapsulation, an MTU constraint, a control plane, a troubleshooting boundary and a class
of failure the tools do not see.

Separate the underlay from the overlay when troubleshooting *(§67.2)* — VTEP-to-VTEP ping,
then MTU, then VTEP peering, then the inner MAC table, then the tenant network. Steps one and
two resolve most overlay faults, and `traceroute` inside an overlay shows one hop because
the fabric is invisible to the encapsulated packet.

EVPN distributes what flood-and-learn discovers *(§67.3)* — "MAC aa:bb:…, IP 10.1.2.3, VNI
10010, next-hop 192.0.2.11", advertised by BGP. No flooding, no learning — the information
was told.

BGP was chosen for reasons rather than inertia *(§67.3)* — MP-BGP already distributed
arbitrary address families; route reflectors avoid a full mesh; route targets express policy;
and it demonstrably scales. EVPN is MPLS L3VPN's mechanism applied to MAC addresses, twenty
years later.

ARP suppression removes the largest single source of BUM traffic *(§67.3)* — A type 2 route
carries MAC and IP, so the leaf the requester is plugged into answers the ARP locally — a
broadcast that would have crossed every rack.

The distributed anycast gateway puts the same IP and MAC on forty switches *(§67.3)* —
Which would be a duplicate-address catastrophe anywhere else and is correct here, because the
leaves never speak to each other in that VNI at Layer 2. Moving a VM changes nothing, not even
its ARP cache, and traffic is routed at the first hop, always.

Symmetric IRB is why a large fabric is manageable *(§67.3)* — A leaf configures only the
VNIs it has workloads in; asymmetric requires every leaf configured for every subnet.

EVPN multihoming removes a proprietary mechanism, a physical peer link and a documented class
of failure *(§67.3)* — The least-discussed and most practically valuable part of EVPN for
anyone who has operated an MLAG pair through a peer-link failure.

Every server is exactly two hops from every other *(§67.4)* — Up to a spine, down to a leaf.
Latency is uniform, all links forward, and a spine failure costs $1/n$ of the capacity rather
than being a failover — which is the underrated property.

Routed to the leaf makes the rest possible and breaks VM mobility *(§67.4)* — No spanning
tree, ECMP everywhere, failure domains of one rack, sub-second convergence — and §67.2 and
§67.3 exist to restore the mobility it removed.

BGP as an underlay was chosen because it is explicitly configured *(§67.4)* — An IGP floods
and every router reacts to every change; BGP advertises what it is told, to whom it is told.
In a fabric of hundreds of switches, "nothing happens unless configured" is a feature.

A 3:1 oversubscription is not "a third as fast" *(§67.4)* — It means that if every server
transmitted at line rate simultaneously to another rack, a third would fit — and they do
not. The question is what your traffic actually does, which requires measurement.

A telephone exchange design from 1953 is the standard data centre topology *(§67.4)* —
Clos's mathematics: a three-stage network can be non-blocking with far fewer crosspoints than a
full crossbar.

The optics are frequently the budget *(§67.4)* — At 400G a transceiver can cost more than
the port it occupies, which makes the reach choice a genuine engineering decision. And a
64-leaf, 8-spine fabric has 512 identical long cables — Chapter 53 §53.2's labelling is not
optional at this scale.

A two-rack, forty-server environment does not need any of this *(§67.4)* — Two switches, a
few VLANs and a router is correct, and building this chapter's architecture for it produces
complexity with no benefit. The most common design error in the field.
