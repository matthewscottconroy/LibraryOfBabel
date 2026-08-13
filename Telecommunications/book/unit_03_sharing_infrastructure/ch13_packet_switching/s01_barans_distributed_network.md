# 13.1 Baran's Distributed Network

Between 1960 and 1964, at the RAND Corporation in Santa Monica, Paul Baran produced
a series of reports that described packet switching in essentially finished form,
several years before anyone built one and for reasons that had nothing to do with
computers.

## The problem he was given

The United States' strategic command-and-control communications in the late 1950s
ran over the commercial telephone network, which was **centralised**: a hierarchy of
exchanges, with a small number of high-level switching centres through which
long-distance traffic passed.

Baran's assignment was to determine whether such a network could survive a nuclear
first strike, and the answer was clearly no. Destroying a few dozen switching
centres would sever the country's communications, and the locations of those centres
were not secret.

The requirement was therefore: **a communications network that continues to function
after losing a substantial fraction of its nodes**, with no node whose loss is
catastrophic.

## The three topologies

Baran's most reproduced contribution is a diagram distinguishing three network
structures, and the analysis attached to it.

```
   CENTRALISED          DECENTRALISED              DISTRIBUTED
                                                  ●─●─●─●─●
        ●              ●───●───●                  │ │ │ │ │
      ╱ │ ╲           ╱│╲ ╱│╲ ╱│╲                 ●─●─●─●─●
     ●  ●  ●         ● ● ● ● ● ● ●                │ │ │ │ │
    ╱│╲ │ ╱│╲                                     ●─●─●─●─●
   ● ● ●●● ● ● ●                                  │ │ │ │ │
                                                  ●─●─●─●─●
```

- **Centralised** — one hub. Destroy it and everything stops. Maximum
  vulnerability.
- **Decentralised** — several hubs. Destroy them and it fragments. This is what the
  telephone network was, and it was the target.
- **Distributed** — a mesh in which every node connects to several neighbours. No
  node is essential.

Baran's quantitative result, from the simulations in the reports, was the
significant part. He defined **redundancy level** as the average number of links per
node relative to the minimum needed for connectivity, and showed that:

> At a redundancy level of only **three or four**, a distributed network survives
> the destruction of a large fraction of its nodes with most surviving nodes still
> able to reach one another.

This was the counterintuitive finding. Extreme survivability did not require extreme
redundancy — it required *modest* redundancy in the right topology. A network with
three or four links per node, which is affordable, behaves qualitatively differently
from one with a hierarchy.

Chapter 11 §11.1's cost/resilience curve is this result, generalised.

## What survivability implies mechanically

The topology is necessary and not sufficient. Baran then derived what such a network
must do, and the derivation is the interesting part because each step follows from
the last.

**If any node may be destroyed, no path can be pre-established.** A circuit is
built at setup and held; if a node on it is destroyed, the circuit is gone and must
be rebuilt. In a network under attack, paths must be established continuously, per
message, or the network spends its time rebuilding.

**Therefore each unit of data must carry its own destination address**, because
there is no reserved path to identify it and no state at intermediate nodes saying
where it belongs.

**Therefore each node must make an independent forwarding decision** for each unit,
based on the address and on its current knowledge of which neighbours are alive.

**Therefore the units must be small and standardised.** Small, so that a lost unit
costs little to retransmit and so that a node's buffers are bounded; standardised,
so that any node can handle any unit without prior arrangement.

Baran called these **message blocks** and specified 1,024 bits. He described the
forwarding rule as **hot-potato routing**: a node holds a block for as short a time
as possible and passes it to whichever neighbour currently offers the best path,
updating its estimate of "best" continuously from what its neighbours report.

That is a distance-vector routing protocol, described in 1962, and Chapter 31 §31.1
is its descendant.

## The rest of the design

Two further elements are worth noting because they are so far ahead of their time.

**Digital transmission throughout.** Baran argued for an all-digital network with
regeneration at every node, for the reasons Chapter 5 §5.1 gives, at a time when the
telephone network was overwhelmingly analog and the T1 had just been deployed.

**Cryptography end to end.** The reports specify encryption of message blocks, on
the grounds that a distributed network necessarily routes traffic through nodes the
sender does not control. This is the end-to-end argument applied to confidentiality,
seventeen years before Saltzer, Reed and Clark named the principle.

## The rejection

Baran presented the design to AT&T, which operated essentially the entire American
telephone network and would have had to build it.

The response was that it would not work. Baran's later accounts describe being told,
in substance, that the people proposing it did not understand telephony — and there
is a well-known anecdote in which an AT&T engineer, after a lengthy explanation of
how a distributed network would carry a call, said: *"Son, here's how a telephone
works."*

Two things are worth separating here, because the story is usually told as pure
institutional obstruction and it is more interesting than that.

**The objection had technical merit at the time.** The switching Baran proposed
required each node to receive a block, examine an address, consult a routing table,
and forward — all in a few milliseconds, for thousands of blocks per second. In 1964
that meant a computer at every node, and computers were expensive, large, unreliable
and slow. AT&T's assessment that this was impractical *with 1964 technology* was
correct.

**And the objection was also institutional.** Accepting the design meant accepting
that eighty years of investment in circuit switching pointed in the wrong direction,
by a company whose entire operational, financial and regulatory structure was built
around the circuit. Organisations do not evaluate such proposals neutrally, and it
would be remarkable if AT&T had.

The Air Force did fund a demonstration programme, which was then transferred to the
Defense Communications Agency — an organisation with no relevant expertise — and
Baran, judging that it would be built badly and discredit the idea, recommended that
it be cancelled. It was.

**The design was never built.** It was published, and Larry Roberts read it while
designing the ARPANET.

## Why this matters beyond the history

Three things transfer.

**Requirements determine architecture.** Baran was not trying to build an efficient
network; he was trying to build a survivable one. Every property of packet switching
followed from that single requirement, and the efficiency that later made it
dominant was a side effect nobody was optimising for. Chapter 72 §72.1's insistence
on requirements before design is this, in miniature.

**Modest redundancy in the right topology beats extensive redundancy in the wrong
one.** Three or four links per node, distributed, beats a heavily protected
hierarchy. This is the single most useful result in the reports and it is directly
applicable to designing any network today.

**A technically sound proposal that requires an incumbent to obsolete itself will be
rejected**, and the rejection will be argued on technical grounds that are partly
valid. Chapter 71's warning about predicting the future — watch the economics, not
the specifications — is the same observation from the other direction.

## What this section does not claim

Two corrections to the popular account, because both are common and both are wrong.

**The ARPANET was not built to survive nuclear war.** Baran's *motivation* was
survivability; the ARPANET's motivation was resource sharing between expensive
computers at research institutions. Larry Roberts and Bob Taylor have both said this
explicitly and repeatedly. The mechanism was borrowed; the requirement was not.

**Baran did not invent packet switching alone.** Donald Davies at the National
Physical Laboratory arrived at the same architecture independently, from an entirely
different requirement, and supplied the word. §13.2 covers him, and the convergence
of two independent designs on one mechanism is a stronger argument for the mechanism
than either alone.

> **Network+ note.** Not examined. The transferable content is §13.1's derivation:
> **no reserved path → each unit carries its own address → independent forwarding
> per hop → small standardised units.** That chain is why packets have headers, why
> routers are stateless, and why the network is what it is.
