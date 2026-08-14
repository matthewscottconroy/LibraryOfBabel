# Chapter 11 — Topologies

The word comes from the Greek *topos*, place, and it entered mathematics in the
nineteenth century to describe the properties of a shape that survive stretching
and bending. In networking it means something more modest: **the arrangement of
nodes and the links between them**, considered without regard to physical distance
or geography.

That last clause is doing more work than it appears to. Two switches in the same
rack and two switches on opposite sides of an ocean have the same topology if they
are connected the same way. Topology is about *connectivity structure*, and the
reason it matters is that connectivity structure determines the answers to the
questions that actually keep networks running: how many failures can this survive,
how far must traffic travel, how much does it cost, and how does it grow.

## The arithmetic that starts everything

Unit III's introduction gave the number; this chapter takes it seriously. To
connect *n* nodes in a full mesh requires *n*(*n*−1)/2 links, and each node needs
*n*−1 interfaces.

| Nodes | Links | Interfaces per node |
|---|---|---|
| 4 | 6 | 3 |
| 10 | 45 | 9 |
| 50 | 1,225 | 49 |
| 500 | 124,750 | 499 |

The growth is quadratic in links and linear in per-node cost, and both become
absurd quickly. But notice what full mesh *buys*: any single link failure is
irrelevant, every path is one hop, and there is no shared resource to contend for.
Full mesh is not a bad design; it is an unaffordable one, and every other topology
in this chapter is a way of purchasing some fraction of its properties at a
fraction of its cost.

That framing — **topologies as points on a cost/resilience curve** — is more
useful than memorising which shape is which, and it is how the chapter proceeds.

## Physical and logical, and why the distinction matters

Here is a question that catches people out.

A 10BASE-T Ethernet network in 1993: twelve computers, each with a cable running
to a central hub. What is its topology?

*Physically*, it is a **star**: every cable radiates from one central point, and if
you look at the cable tray that is unambiguously what you see. *Logically*, it is a
**bus**: the hub is an electrical repeater that copies every incoming signal to
every other port, so every station hears every transmission, exactly as if they
were all tapped onto one length of coaxial cable. Collisions occur. CSMA/CD is
required. The bandwidth is shared.

Both descriptions are correct. They describe different things, and nearly every
confusing statement about topologies comes from someone applying a fact about one
to the other.

Replace the hub with a switch and the physical topology is unchanged — same
cables, same star — while the logical topology becomes a set of point-to-point
links. Collisions vanish. The bandwidth is no longer shared. **Nothing about the
cabling changed and everything about the network's behaviour did.** This is the
most important thing in the chapter, and Chapter 17 explains the mechanism.

The same distinction recurs throughout the book. A VLAN (Chapter 20) creates a
logical topology unrelated to the physical one. A VXLAN overlay (Chapter 67)
creates a logical Ethernet segment spanning three data centres that share no
cables at all. An MPLS label-switched path (Chapter 50) creates a logical circuit
across a packet network. The whole modern practice of network virtualisation is
the systematic exploitation of the gap between physical and logical.

## What this chapter does

§11.1 develops the combinatorics of connection and frames topologies as a
cost/resilience tradeoff, including the concept of a **single point of failure**
and how to count them.

§11.2 covers the classical shapes — bus, ring, star, mesh, tree, hybrid — with
their real historical instances (10BASE5 coax, Token Ring and FDDI, modern
switched Ethernet, ISP backbones), their failure modes, and their honest
advantages. Ring topologies in particular deserve better than the dismissal they
usually get; SONET's dual counter-rotating rings achieved 50 ms protection
switching, which Ethernet took another two decades to approach.

§11.3 works the physical/logical distinction through several examples and
introduces the vocabulary for describing each.

§11.4 covers hierarchical design: the access/distribution/core model, why
hierarchy is the general answer to scale, what "collapsed core" means and when it
is appropriate, and the beginnings of the leaf–spine argument that Chapter 67
completes.

## By the end you will be able to

- Compute the link count and per-node interface count for any topology, and use
  that to argue about cost.
- Identify single points of failure in a described or diagrammed network.
- State, for any given topology, what happens when one link fails and when one
  node fails.
- Distinguish physical from logical topology in a real network and explain which
  properties follow from which.
- Explain why hierarchical designs scale and flat ones do not, in terms of
  broadcast domains, table sizes and failure blast radius.
- Read and produce a network diagram at the three standard levels of abstraction
  (a skill Chapter 53 formalises).

## Where this sits in the argument

This chapter poses the sharing problem in structural terms. Chapters 12 and 13
then give the two great answers to *how the shared infrastructure is allocated* —
by reservation, and by labelling — and Chapter 14 assembles the result into the
familiar categories of LAN, WAN, and Internet.
