# Chapter 68 — Further Reading

## Read these first

**McKeown, N. et al. (2008). "OpenFlow: Enabling Innovation in Campus Networks."** *ACM SIGCOMM
CCR*.
**Six pages, and the pitch in its original form.** **Read it and note how modest the ask was**
compared with what the movement became.

**Jain, S. et al. (2013). "B4: Experience with a Globally-Deployed Software Defined WAN."**
SIGCOMM.
**F5 uses it.** **The honest success, and the paper is candid about the failures and the
preconditions.** **Read section 6 in particular**, where they describe what went wrong.

**Bosshart, P. et al. (2014). "P4: Programming Protocol-Independent Packet Processors."**
*SIGCOMM CCR*.
**Short, and it states plainly what OpenFlow got wrong** — from people who had built OpenFlow.

**Feamster, N., Rexford, J. & Zegura, E. (2014). "The Road to SDN: An Intellectual History of
Programmable Networks."** *SIGCOMM CCR*.
**The best single account of where the ideas came from**, including ForCES, RCP, 4D and the
active networking work of the 1990s. **Corrects the impression that OpenFlow was the
beginning.**

## The earlier work

**Caesar, M., Caldwell, D., Feamster, N., Rexford, J., Shaikh, A. & van der Merwe, J. (2005).
"Design and Implementation of a Routing Control Platform."** NSDI.
**Centralised BGP route computation for an AS, three years before OpenFlow.**

**Greenberg, A. et al. (2005). "A Clean Slate 4D Approach to Network Control and Management."**
*SIGCOMM CCR*.
**The four-plane architecture**, and it reads as a manifesto.

**Casado, M. et al. (2007). "Ethane: Taking Control of the Enterprise."** SIGCOMM.
**OpenFlow's parent**, and reading it explains OpenFlow's shape — **a security architecture
first.**

**RFC 3746 and the ForCES documents.**
**Worth knowing they exist**, and §68's researcher notes explain why nobody implemented them.

## Specifications

**The OpenFlow specifications 1.0 and 1.3** (ONF, now at the Open Networking Foundation
archives).
**F8 compares the table models.** **1.0 is short and readable; 1.3 is where the model became
complicated.**

**The P4-16 language specification** (p4.org).
**Read the parser and the control-flow sections.** **F4 uses them.**

**RFC 7426 — "SDN: Layers and Architecture Terminology."**
**Vendor-neutral vocabulary**, and it is useful for cutting through marketing that uses the same
words differently.

## Verification, which is the practically useful part

**Fogel, A. et al. (2015). "A General Approach to Network Configuration Analysis."** NSDI.
**The Batfish paper.** **F3 uses the tool.**

**Kazemian, P., Varghese, G. & McKeown, N. (2012). "Header Space Analysis: Static Checking for
Networks."** NSDI.
**The formal foundation of reachability verification**, and it is more approachable than it
sounds.

**Khurshid, A. et al. (2013). "VeriFlow: Verifying Network-Wide Invariants in Real Time."**
NSDI.
**Verification of a change before it is applied**, which is §68.4's claim in its strongest form.

**Foster, N. et al. — the NetKAT papers.**
**A formal language for network behaviour with a decision procedure.** **Mathematical, and the
reason the verification tools can be exhaustive rather than heuristic.**

## Books and long-form

**Nadeau, T. & Gray, K. — *SDN: Software Defined Networks*.**
**The practitioner's overview**, and it is now a historical document as much as a current one —
which is useful for seeing what was expected.

**Goransson, P., Black, C. & Culver, T. — *Software Defined Networks: A Comprehensive
Approach*.**
More thorough, and the OpenFlow chapters are the reference.

**Shenker, S. — "The Future of Networking, and the Past of Protocols"** (talk, 2011, widely
available).
**Forty minutes, and the clearest statement of the abstraction argument.** **Worth watching
even now that the predictions can be assessed.**

**`ipSpace.net`'s SDN material** (Ivan Pepelnjak) — **the sceptical contemporaneous record.**
**Reading his 2012–2015 posts alongside the vendor material of the same period is an education
in how to assess a hype cycle while inside one.**

## Hands-on

**Mininet** (mininet.org) — **F1 and F2.** **An entire OpenFlow network on a laptop in one
command**, and it is the standard teaching environment.

**Ryu, Faucet, ONOS** — **controllers.** **Ryu for learning** (F1), **Faucet for something you
could actually run**, **ONOS for the production-scale architecture.**

**Open vSwitch** — **`ovs-ofctl dump-flows` on any host running containers or OpenStack shows a
real OpenFlow table in production use.** **The most accessible demonstration that the model
survived.**

**`bmv2` and the P4 tutorials** (github.com/p4lang/tutorials) — **F4.** **The tutorials are
excellent and build from a basic forwarder to in-band telemetry in about eight exercises.**

**Batfish** (batfish.org) — **F3, and Chapter 55's reading.** **Free, and it will find things in
your configurations.**

**Containerlab with FRR or SONiC** (Chapter 67's reading) — **for the disaggregated stack that
§68.1 says is SDN's real legacy.**

## Assessing the current claims

**Vendor intent-based networking documentation** — **Cisco DNA/Catalyst Center, Juniper Apstra,
Arista CloudVision.**
**Read the API documentation and the troubleshooting guide rather than the architecture
overview.** **The first tells you what the model actually is; the second tells you what goes
wrong.**

**Gartner's and the analysts' coverage**, read as a market document. **Useful for knowing what is
being claimed and by whom.**

**And the honest test for any such product**, from §68.4: **what is centralised, what happens
when it is unavailable, what can it verify, and what will it change automatically?**

## Where to look next

**Chapter 69** is the network with an API that SDN promised, arrived by a different route;
**Chapter 70** is the programmability that actually won; **Chapter 71** takes up the current
claims — AI-driven operations and autonomous networks — with this chapter's scepticism applied;
and **Chapter 67** is the fabric these ideas were mostly deployed in.
