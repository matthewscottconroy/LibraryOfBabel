# Chapter 68 — Important Concepts

Distribution was a constraint, not a goal *(§68.1)* — Every router runs OSPF and computes
its own tree because there was no practical alternative in 1989. No controller could have
been reliable, fast or reachable enough. And distribution has real costs: every device
configured, consistency by convergence rather than construction, cross-device policy expressed
per device, local optimisation, and verification impossible in general.

The controller argument has not been refuted *(§68.1)* — A complete view permits globally
optimal paths, verification before deployment and atomic reconfiguration. What was wrong was
the estimate of how hard the other parts would be.

There are three architectures and the middle one won *(§68.1)* — **Distributed, hybrid,
centralised** — and hybrid is what SD-WAN, EVPN fabrics and wireless controllers all are.
It is the one nobody was selling in 2011.

Wireless controllers separated control from data in 2004, and nobody called it SDN
*(§68.1)* — Because RF is a shared medium, so channel and power decisions must be global and a
distributed algorithm cannot make them well. The architecture arrived where the problem
demanded it, without the name and without the movement.

Ask what happens when the controller is unreachable *(§68.1)* — The right answer is that
devices retain forwarding state and keep working while being unable to adapt. You have not
removed the distributed system; you have moved it and changed its failure modes — and
partition, with two controllers and two views, is the hard case.

Disaggregation is SDN's lasting consequence *(§68.1)* — A switch used to be one product:
hardware, forwarding software, control protocols and management from one vendor. It can now
be four purchases, and that is the most substantial and least-discussed achievement.

OpenFlow made "switch" and "router" configurations rather than product categories *(§68.2)*
— A switch matching a TCP port and a router matching an IP prefix are the same device
performing the same operation on different fields.

Reactive flow installation does not scale, by two orders of magnitude *(§68.2)* — 4,000 to
100,000 TCAM entries against a million concurrent flows, and at 10,000 new flows per second
every one incurs a controller round trip. The reactive model made the demonstrations
compelling and is why the model did not survive production.

The hardware was not what the protocol assumed *(§68.2)* — Real ASICs have fixed pipelines
with specific tables of specific widths. A general match across twelve fields must go in the
ACL TCAM, which is the smallest and most expensive table on the chip — so OpenFlow's
expressiveness mapped onto a fraction of the switch's capacity.

"Supports OpenFlow" meant almost nothing *(§68.2)* — Six versions in six years, each
changing the model, with vendors implementing different subsets — fatal for a protocol whose
purpose was interoperability.

The vendors had no incentive, and that is not a conspiracy *(§68.2)* — OpenFlow's explicit
goal was to commoditise the switch. Every incumbent's business depended on the opposite,
and participation was enthusiastic in the standards body and less so in the roadmap.

Open vSwitch is OpenFlow's most consequential outcome *(§68.2)* — Written to be an OpenFlow
switch, it became the virtual switch of the cloud. The programmable-datapath model won in
software where it lost in hardware.

B4 worked because Google controlled everything *(§68.2)* — Both ends, its own switches, its
own applications, and a failure model it designed for — precisely the conditions most
organisations do not have, and the paper is honest about it.

A new protocol requires new silicon, and that is a three-to-five-year cycle *(§68.3)* —
VXLAN took years to appear in hardware; GENEVE is still not universal. P4's proposition is
to specify a language in which the pipeline is written and compile it onto the target.

In-band telemetry answers a question no monitoring system can *(§68.3)* — Not "what is the
average queue depth on switch 7?" but "what did this specific packet experience, at every hop, on
this specific journey?" — for every packet, without sampling. Its cost is packet size.

In-network aggregation is genuinely deployed *(§68.3)* — An AI training cluster's gradient
exchange is a many-to-one reduction, and doing the arithmetic in the switch removes a substantial
fraction of the traffic.

The most capable programmable ASIC was cancelled for commercial reasons *(§68.3)* — The
volume was not there against fixed-function merchant silicon, which is cheaper, faster and
adequate for what most networks do. A technology requiring a hardware vendor to undermine
their own margin will struggle regardless of merit, and P4's centre of gravity moved to the
host, where the buyer is a server vendor.

P4's likely legacy is that the pipeline-as-a-program became normal *(§68.3)* — In eBPF on
hosts, in DPUs, and in specification languages. The same shape as OpenFlow's legacy: the
architecture survived and the product did not.

Automation deploys what you told it to; intent-based networking checks afterwards that the
network is doing what you asked *(§68.4)* — The verification is the claim, and it is the
part least emphasised in the marketing.

"Is VLAN 240 reachable from every access switch?" is a question no configuration audit can
answer *(§68.4)* — The configuration may be correct and the network not — a failed uplink,
a spanning tree decision, an unrelated ACL. Checking state rather than configuration is the
whole point.

Verification is exhaustive where testing samples *(§68.4)* — "We tested it and it works"
and "we proved no packet can reach the finance network from the guest VLAN" are different
claims, and the second was not previously available.

"Make the finance network secure" has no determinate meaning *(§68.4)* — Secure against
whom, at what cost, with what availability trade (Chapter 57 §57.3). A system producing a
configuration from it would be guessing, and the failure is not technical.

A system that "determines how" is selecting from designs its authors anticipated *(§68.4)* —
A template with better ergonomics, which is a genuine improvement and is not network design.

Remediation should report by default and act on a short deliberately-chosen list *(§68.4)* —
A remediation system acting on a fault it has misdiagnosed is a system causing an outage.

Most of the value is available to an organisation that writes down what it intends and checks
periodically that it is true *(§68.4)* — Version control and generation, a free verification
tool, state checks built from `show` commands, and explicit written intent. The products
industrialise that; they do not create it — and the prerequisite is the hard part.

The idea was correct and the implementation route was wrong *(§68.4)* — Control/data
separation won in unpredicted forms; the central controller lost except at hyperscale;
disaggregation won; the programmable pipeline is right and commercially difficult; and
programmability won completely, as APIs and the cloud. A more common outcome in engineering
than either triumph or failure, and worth recognising while it is happening — because the
claims about intent, AI-driven operations and autonomous networks have the same shape, and the
same question applies: which part is the architecture, and which is the product?
