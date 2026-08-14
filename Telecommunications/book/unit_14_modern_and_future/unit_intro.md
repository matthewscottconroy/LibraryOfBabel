# Unit XIV — Modern and Future Networks

Everything in the previous thirteen units describes networks made of boxes. A switch
is a box; a router is a box; a firewall is a box. You buy them, rack them, cable them,
and configure them one at a time through an interface designed for a human typing.

That model is not obsolete and will not be for a long time — the access switch in a
branch office in 2035 will still be a box, and it will still need someone who
understands Chapter 17. But it is no longer the *whole* model, and this unit is about
what has been built alongside it.

## The three changes

**The functions moved off the boxes.** A virtual switch inside a hypervisor forwards
more traffic in some data centres than the physical switches do. A firewall is a
software instance. A load balancer is a container. The forwarding logic of Chapter 17
and the routing logic of Chapter 29 are unchanged; what changed is that they now run
on general-purpose hardware, are created and destroyed by API call, and are managed by
people who have never seen a rack. Chapter 67.

**The control plane moved off the boxes.** Chapter 29 §29.1 distinguished control
plane from data plane, and in a traditional network both live in every device — each
router runs its own routing protocol and reaches its own conclusions. Software-defined
networking separates them physically: a central controller computes, and devices
forward according to what they are told. Chapter 68 covers what this bought, what the
first decade of it got wrong, and where the idea genuinely won.

**The configuration moved off the boxes.** The command line does not scale past a few
dozen devices, and more importantly it produces no record, no review, no testing and no
reproducibility. Chapter 70 covers the shift to declarative configuration in version
control, applied by automation — infrastructure as code — which is the operational
practices of Chapter 55 taken to their conclusion.

Plus **the destination moved**: Chapter 69's cloud networking, where the network is
something you describe in a template and someone else operates.

## The recurring pattern

One pattern runs through this unit, and it is worth naming because it will recur in
whatever comes after the technologies described here.

A specialised, expensive, physical thing is replaced by a general-purpose,
programmable, software thing — which is initially worse in every technical respect and
wins anyway, because it can be changed faster.

This has already happened several times in this book. Packet switching replaced circuit
switching (Chapter 13) despite offering no guarantees. Ethernet replaced Token Ring
(Chapter 16) despite being non-deterministic. IP replaced purpose-built networks
(Chapter 14) despite promising nothing. In each case the general substrate was
technically inferior at the moment of competition and improved faster than the
specialist could.

The unit's frontier chapter (71) asks where this goes next, and the answer is not
uniform — some things resist virtualisation because physics forbids it, and knowing
which is a useful instinct.

## What the unit contains

**Chapter 67 — Virtualization and Overlays.** Virtual switches and container
networking; VXLAN and GENEVE and the 24-bit segment identifier that replaced 802.1Q's
twelve; EVPN as the control plane that made overlays operable; and leaf–spine fabrics,
driven by the shift from north–south to east–west traffic.

**Chapter 68 — SDN and Programmable Networks.** Control and data plane separation;
OpenFlow and the honest account of why the centralised-controller vision did not
arrive as promised; P4 and programmable pipelines; and intent-based networking with
its limits stated.

**Chapter 69 — Cloud Networking.** Service and deployment models; the VPC and its
subnets, route tables, gateways and security groups — which are Chapters 26, 29 and 60
under different names, and recognising that is the fastest way to learn them; hybrid
connectivity; and cloud-native load balancing and service mesh.

Chapter 70 — Automation and Infrastructure as Code. Why the CLI does not scale;
APIs, NETCONF, RESTCONF, YANG and gNMI; Ansible and Terraform and declarative state;
CI/CD for network configuration; and an honest assessment of AIOps.

**Chapter 71 — The Frontier.** 6G research directions; coherent optics and the
shrinking margin at 800G and beyond; quantum networking and QKD, with the hype
separated from the physics; deterministic networking and TSN; and machine learning in
and on the network.

**Chapter 72 — Network Design: The Synthesis.** Where the whole book converges.
Requirements, topology, addressing, services, security and operations designed
together, and — the actual skill — defending every choice against the alternatives.

## A caution about this unit's shelf life

Units I through XIII age slowly. Shannon's limit will hold. Subnetting arithmetic will
not change. The seven-step troubleshooting method will still work.

This unit ages fast. Some technology described here as current will be superseded
within a few years, and the specific products certainly will be.

It is written accordingly: the emphasis throughout is on the problem each technology
solves and the tradeoff it makes, because those persist even when the implementation
does not. VXLAN may be displaced; the problem of extending a Layer 2 segment across a
routed network will not be. OpenFlow largely was displaced; the argument about where
the control plane belongs continues.

Read this unit for the arguments, not for the product names.
