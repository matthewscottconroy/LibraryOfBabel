# Chapter 69 — Exercises

## A. Recall

**A1.** Give the three service models and, for each, state where the boundary of your
responsibility falls.

**A2.** What is serverless's network consequence? List four.

**A3.** Distinguish hybrid from multicloud, and give three ways they differ operationally.

**A4.** State the shared responsibility model in one sentence, and give the network-specific
consequence.

**A5.** List six things a network engineer loses moving to cloud and five things gained.

**A6.** Name the three network costs that surprise, and say what design decisions each changes.

**A7.** Translate each into its pre-cloud equivalent: VPC, security group, NACL, internet
gateway, NAT gateway, availability zone, transit gateway.

**A8.** Why can the `local` route not be removed, and what follows for isolation within a VPC?

**A9.** Give four differences between a security group and a network ACL.

**A10.** What does referencing a security group from another achieve, and which chapter's
argument is it?

**A11.** What defines a "public" subnet?

**A12.** Why is VPC peering not transitive, and what exists because of it?

**A13.** State the peering count for $n$ VPCs and name the three earlier places in this book the
same arithmetic appeared.

**A14.** What does a gateway endpoint remove besides cost?

**A15.** Distinguish liveness from readiness, and say which the load balancer uses.

**A16.** What is connection draining, and what determines the correct timeout?

**A17.** What does a service mesh provide, and what are its four costs?

**A18.** Give the added latency for one, five and twelve sequential service calls through a
sidecar mesh.

## B. Apply

**B1.** For each, state the service model and who is responsible for the network fault:

(a) A virtual machine cannot reach the Internet
(b) A managed database is slow
(c) A SaaS application is unreachable from one office
(d) A serverless function times out calling an external API
(e) A container platform's ingress returns 503

**B2.** Design the VPC addressing for an organisation with a 10.0.0.0/8 allocation, three
regions, three availability zones each, and four tiers per zone.

(a) Choose the VPC size and justify it.
(b) Give the subnet plan for one region.
(c) State how much space remains for growth.
(d) State the check you would perform before allocating anything.

**B3.** For each requirement, state whether a security group or a NACL is correct, and write the
rule in words:

(a) Web instances may receive HTTPS from anywhere
(b) Application instances may receive 8443 from web instances only
(c) A subnet must never accept traffic from 198.51.100.0/24
(d) Database instances may receive 5432 from application instances only
(e) All instances may make outbound connections to anywhere

**B4.** An organisation has 14 VPCs that must all communicate, plus an on-premises network.

(a) How many peering connections would a full mesh require?
(b) How many transit gateway attachments?
(c) Design the transit gateway route tables to separate production from development while both
reach shared services and on-premises.
(d) State one circumstance in which you would use direct peering anyway.

**B5.** An instance in a private subnet reads 8 TB per month from object storage in the same
region.

(a) Describe the default path and what is charged.
(b) Compute the saving from a gateway endpoint, assuming \$0.045/GB of NAT processing.
(c) State the security benefit, which is larger.
(d) State when an interface endpoint would be required instead.

**B6.** Design the health check configuration for an application that takes 40 seconds to warm
up, has a maximum legitimate request duration of 3 minutes, and depends on a database and on a
non-critical recommendation service.

(a) Specify the liveness and readiness endpoints and what each checks.
(b) Specify the intervals, thresholds and drain timeout.
(c) State what would go wrong with each of two plausible wrong settings.

**B7.** A request path traverses eight services, each call adding two sidecar hops at 0.8 ms.

(a) Compute the added latency.
(b) The end-to-end budget is 200 ms and the services take 140 ms in total. Assess.
(c) Give two ways to reduce the mesh's contribution and their trade-offs.

**B8.** For each, decide whether a service mesh is warranted, with a reason:

(a) 9 services, all in Go, on a trusted network
(b) 200 services in five languages, with a regulatory mTLS requirement
(c) 40 services, one platform team, needing canary deployments
(d) 15 services and no platform engineering capability

## C. Analyse

**C1.** The chapter says the skills that transfer are the concepts and the skills that do not are
the ones touching hardware. Analyse what this means for a network engineer's career, and what
should actually be learned.

**C2.** Analyse the claim that most publicised cloud breaches are customer misconfigurations.
What does the shared responsibility model achieve, what does it obscure, and is the division
fair?

**C3.** Analyse routing-based segmentation (transit gateway route tables) against filter-based
segmentation (security groups). Which is stronger, why, and when would you use each?

**C4.** The chapter argues that multicloud for resilience rarely delivers what is claimed.
Construct the strongest case for multicloud resilience, then assess it against the failure modes
that actually take out a provider.

**C5.** Analyse the loss of packet capture. What questions can no longer be answered, what
replaces them, and is the net position better or worse than a typical on-premises environment?

**C6.** Analyse cloud network metering as a design force. Give three architectural decisions that
would differ between an on-premises and a cloud deployment purely because of charging, and
assess whether the resulting designs are better or worse.

**C7.** A service mesh's telemetry is the only thing that can attribute a failure among five
candidates, and it is a dependency on a component that may be broken. Analyse this circularity
and propose what would break it.

**C8.** The chapter says the distinction between a CDN and a global load balancer has dissolved.
Analyse what remains of the distinction and what it means for Chapter 52's architecture.

## D. Design

**D1.** Design the network for a three-tier application deployed across three availability zones
in one region: VPC addressing, subnets, route tables, gateways, security groups, load balancing
and health checks. Present it as a diagram plus a table, and state every design decision's
reason.

**D2.** Design the hybrid connectivity for an organisation with a data centre, 12 VPCs across two
regions, and 40 branch offices already on SD-WAN. Address topology, redundancy, routing,
addressing, segmentation and failover testing.

**D3.** An organisation's cloud bill is dominated by network charges. Design the investigation
and the remediation: what you would measure, in what order, and the five changes most likely to
help, with an estimate of each.

**D4.** Design the ingress architecture for a Kubernetes platform serving 60 applications:
load balancing, TLS termination, certificate management, routing, and the boundary between what
the platform team and the application teams control.

**D5.** Write the two-page assessment you would give an architect proposing a service mesh for a
fifteen-service platform with no dedicated platform team. Be fair to the technology, specific
about the costs, and offer a defensible alternative.

## E. Troubleshoot

**E1.** An instance in a private subnet cannot reach the Internet. Give your diagnostic sequence
in order.

**E2.** A connection to an instance establishes and returns nothing. Give the most likely cause
and the specific configuration to check.

**E3.** A direct connect is established and the on-premises network cannot reach the VPC.
Everything appears configured. Give three causes.

**E4.** Instances in VPC A can reach VPC B, and cannot reach VPC C, which B can reach. Explain.

**E5.** A VPN into the cloud works for interactive traffic and file transfers hang. Diagnose in
one command.

**E6.** During every deployment, users see errors for about twenty seconds. Give three causes and
the fix for each.

**E7.** A load balancer removes healthy instances intermittently. Give two causes.

**E8.** A request fails and the application logs on both services show nothing wrong. The mesh is
deployed. Describe your investigation.

**E9.** A newly built VPC cannot be connected to the corporate network. Give the most likely
cause and what should have been done.

**E10.** Every resource in a region is healthy and nothing can be created or modified. Explain,
and state which chapter's argument this is.

## F. Extend

**F1.** Build a VPC by hand in a provider's free tier or with LocalStack: two subnets in
different zones, a route table each, an internet gateway, a NAT gateway, and security groups
implementing a two-tier application. Then destroy it and rebuild it from a script.

**F2.** Enable flow logs on a VPC and analyse a day's data. Determine the top talkers, the
proportion of traffic that is cross-zone, and anything reaching the Internet that should not be.

**F3.** Compare the networking models of two cloud providers on: routing, security semantics,
what is regional versus zonal, MTU, and load balancer types. Present as a table and state which
differences would matter to a portable design.

**F4.** Measure the effect of a gateway endpoint: record the NAT gateway's processed bytes for a
workload reading from object storage, add the endpoint, and measure again. Report the change and
the cost saving.

**F5.** Deploy a service mesh in a test cluster and measure the latency added per hop, with and
without mTLS. Then measure the sidecars' memory and CPU as a fraction of the cluster's.

**F6.** Deliberately misconfigure a health check in three ways — too aggressive, checking too
much, and passing before warm-up — and record the observable symptom of each.

**F7.** Compute the network cost of a realistic architecture: egress, cross-zone, NAT processing,
load balancers, and endpoints. Then redesign it to halve the cost and state what you gave up.

**F8.** Read one cloud provider's networking documentation on MTU end to end and determine the
effective MTU for: within a subnet, across a peering connection, over a VPN, and over a direct
connection. Compare with the tunnel overheads in Chapter 61 §61.1.
