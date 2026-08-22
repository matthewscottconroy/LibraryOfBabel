# Chapter 67 — The People

**Charles Clos.** Bell Labs, 1953 — and the topology this chapter is built on.

Clos's paper "A Study of Non-Blocking Switching Networks" answered a telephone problem: how
to connect $N$ inputs to $N$ outputs without a full $N \times N$ crossbar, whose cost grows as
$N^2$ and becomes impossible at exchange scale.

His answer was three stages — **ingress, middle, egress** — with the middle stage providing
alternative paths. The mathematics gives the condition under which the network is
strictly non-blocking, and the crosspoint count grows far more slowly than $N^2$.

> **A leaf–spine fabric is a folded three-stage Clos network.** **Leaf, spine, leaf** — and a
> five-stage Clos is the same construction applied again for a larger fabric. The design is
> seventy-two years old and the arithmetic is unchanged, which is a satisfying thing to know
> while looking at a rack of 400G optics.

Clos also established the trade this chapter's §67.4 uses: that a network can be made
*rearrangeably* non-blocking with fewer resources than strictly non-blocking — which is
exactly the oversubscription decision, expressed for telephone circuits rather than for
packets.

Martin Casado, Nick McKeown and Scott Shenker, again — and here for Nicira rather than for
OpenFlow (Chapter 68 covers that).

Nicira's argument, from about 2007, was that the network was the last thing in the data centre
that had not been virtualised.

> **Compute had been virtualised. Storage had been virtualised.** The network had not, and it
> was therefore the constraint — a virtual machine could be created in seconds and its
> network took a change request and a week.

Their product — network virtualisation with a distributed control plane, overlays and policy
attached to workloads rather than to ports — became VMware NSX, and the acquisition price in
2012 (about \$1.26 billion for a company with modest revenue) is the clearest available statement
of what the industry thought the idea was worth.

The argument has aged well and its framing is the useful part:

**Casado's formulation** — that the network should present the same abstraction to a workload
regardless of where it runs — is the requirement from which §67.2's overlays, §67.3's control
plane and §67.1's policy-follows-workload all follow.

**Ivan Pepelnjak**, for the counterweight, and it is a necessary one.

Pepelnjak has spent fifteen years arguing, in detail and with evidence, that most organisations
deploying overlays do not need them — and that the complexity is frequently adopted because
it is what large operators do rather than because the problem is present.

> His recurring question is the useful one: "what problem are you solving?" — and the
> honest answer for a two-rack environment is usually "none", which is §67.4's closing
> argument.

He is also the most reliable available source on what these technologies actually do as
opposed to what their marketing says, and his material is recommended in this chapter's further
reading for that reason.

Aldrin Isaac, Ali Sajassi, and the EVPN authors. **RFC 7432, 2015.**

EVPN's design decision — to reuse MP-BGP rather than to invent a control protocol — was
contested and correct.

> **The alternative proposals used purpose-built protocols.** The argument for BGP was that
> the fabric already ran it, the scaling was demonstrated, route reflectors and route targets
> already existed, and a new protocol would need its own implementations, its own operational
> tooling and its own decade of bug-finding.

Sajassi in particular carried the work from a service provider context — EVPN was originally
for provider Ethernet services over MPLS — into the data centre with VXLAN encapsulation, and
the fact that one control plane serves both is why the RFC's route types look slightly odd for
data centre use.

The Kubernetes networking authors — Tim Hockin and the SIG-Network group.

The four requirements in §67.1 are a small document and a large decision.

> "Every pod gets its own routable IP address and NAT does not exist inside the cluster" was
> chosen over the obvious alternative — port mapping onto host addresses — because port mapping
> makes every application's configuration depend on where it happens to be running.

Hockin has been explicit that the model was chosen for developer experience rather than for
network elegance, and the address consumption it implies was accepted as a cost.

And the design's second decision was to specify the model and not the implementation.
CNI is a plugin interface, which produced Flannel, Calico, Cilium and a dozen others — and
that competition produced better outcomes than a single implementation would have, at the cost
of every cluster's networking being a choice a platform team must make and frequently makes
badly.

Thomas Graf and the Cilium team, for eBPF's arrival in networking.

Cilium's argument is that iptables was the wrong tool: a linear list of rules, evaluated
per packet, whose length grows with the number of services — and at ten thousand services the
per-packet cost is measurable.

**eBPF** (Chapter 64's BPF entry) replaces it with programmable hooks in the kernel's
datapath, compiled and verified, so policy, load balancing and observability run at
near-native speed and can be changed without a kernel module.

> The interesting part is what it enables rather than what it speeds up. Observability at
> the syscall and socket level, policy expressed in identities rather than addresses, and load
> balancing without NAT state — each of which is difficult or impossible with iptables, and
> Chapter 68's programmability argument arriving in the host rather than in the switch.

## What this chapter's history shows

Two observations, and they point the same way.

The topology is seventy years old and the control plane is twenty. Clos solved the
switching problem in 1953; BGP was specified in 1989 and EVPN reused it in 2015. Almost
nothing in this chapter is a new idea — what is new is the combination, and the economics
that made it worth building.

**And the pressure came from outside networking.** Virtualisation, then containers, then
microservices — each changed the traffic pattern and the network followed. Nicira's
observation that the network was the last unvirtualised layer was correct and it was made by
people who came to networking from systems.

> Which is the chapter's uncomfortable summary: the data centre network was redesigned in
> response to requirements set by people who did not consider themselves network engineers, using
> arguments about developer experience and deployment velocity. The engineering was sound and
> the initiative was elsewhere, and Chapter 72 takes that up as a question about the
> profession.
