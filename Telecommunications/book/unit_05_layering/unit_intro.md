# Unit V — The Idea of Layers

Almost every networking textbook opens with the OSI model. This one has waited
twenty chapters, and the delay was the point.

Consider what you have now done. You have taken a physical medium and worked out
how to put bits on it (Chapter 7). You have marked the boundaries of a message
(Chapter 15). You have given machines addresses and arbitrated who may transmit
(Chapters 15 and 16). You have built a device that learns where things are and
forwards accordingly (Chapter 17). You have reconciled two incompatible addressing
schemes (Chapter 18), prevented a topology from destroying itself (Chapter 19),
and partitioned one physical network into several logical ones (Chapter 20).

Seven problems. Seven mechanisms. And here is the observation this unit is built
on: **you could replace any one of them without touching the others.**

Swap the copper for fibre and the framing, addressing, switching and VLAN
machinery are all unaffected. Swap CSMA/CD for full duplex — which the industry
actually did — and the frame format is unchanged. Change the spanning tree
algorithm from 802.1D to 802.1w and nothing above notices.

That independence did not happen by accident, and it is not free. It is the
product of a deliberate design discipline, and this unit names it, justifies it,
and then examines the two models the field uses to describe it.

## The argument, in one paragraph

Suppose there are *m* applications and *n* kinds of network. Without a shared
abstraction, each application must be written for each network: *m* × *n* pieces of
software. Email over Ethernet, email over Wi-Fi, email over satellite; file
transfer over each; and every new network technology requires all *m* applications
to be rewritten before anyone can use it.

With a layer in between — an abstraction that every network implements and every
application targets — the cost becomes *m* + *n*. Write each application once
against the abstraction; write each network once to provide it. Adding a new
network technology costs one piece of work and every existing application
immediately runs over it.

The multiplication becomes an addition. That is the whole argument, and it is
worth noticing that it is not a networking argument at all — it is the same
argument that justifies device drivers, instruction set architectures, database
query languages, and the shipping container. Chapter 21 develops it and, more
usefully, examines what it costs, because layering is not free and pretending
otherwise produces engineers who cannot explain why their VPN is slow.

## Two models, unequal in fate

**The OSI model** (Chapter 22) has seven layers and was produced by the
International Organization for Standardization between 1977 and 1984. It is a
careful, thorough, committee-designed reference model, accompanied by a full suite
of protocols intended to implement it.

Those protocols are all dead. Every one. The model, however, is not merely alive
but universal: engineers say "that's a Layer 2 problem" and "we need a Layer 7
firewall" and "it's a Layer 1 issue" constantly, in shops that have never run a
single OSI protocol.

**The TCP/IP model** (Chapter 23) has four layers, was described by Cerf and Kahn
in 1974, and was never really designed as a reference model at all — it is a
description of what a working system did. Its protocols run everything.

So the field ended up with a vocabulary from one model and an implementation from
another, which is untidy, and which is why students are asked to learn both and are
rarely told why. The honest answer, developed in §22.4: **the OSI model survived
because it is a superb diagnostic instrument.** Its value is not that it describes
the protocol stack accurately — it does not, particularly — but that it gives you a
disciplined way to cut a problem in half. Is the link up? Then it is not Layer 1.
Can you ping by address but not by name? Then Layers 1 through 4 are fine and the
problem is in name resolution. That procedure, formalised in Chapter 63, is what
seven layers are actually *for*, and it is why this book teaches the model here,
after you have things to locate on it, rather than in Chapter 1 as an alphabet to
be recited.

## What the unit contains

**Chapter 21 — Why Layering Exists.** The *m* × *n* argument; interfaces and
service primitives; the genuine costs of abstraction (overhead, information hiding,
duplicated function); and layer violations that shipped anyway and why.

**Chapter 22 — The OSI Model.** The seven layers, mapped onto the mechanisms you
have already built; what each layer actually does; and OSI as a diagnostic tool.

**Chapter 23 — TCP/IP and Encapsulation.** Cerf and Kahn's paper; the four-layer
model; encapsulation traced through one real HTTP request from application byte to
electrical signal and back; and the end-to-end argument — the paper that
explains more about why the Internet is shaped the way it is than any other single
document.

## One caution before we start

Layering is a model. Reality is messier, and the messiness is not a failure of your
understanding.

ARP sits between Layers 2 and 3 and belongs to neither. MPLS is routinely called
"Layer 2.5." TLS occupies a position that has been argued about for twenty-five
years. A modern firewall inspects Layer 7 content to make Layer 3 forwarding
decisions. NAT rewrites Layer 3 and Layer 4 headers in flight, in flat defiance of
the end-to-end principle, and is deployed universally.

None of this means layering is wrong. It means layering is a *useful
simplification*, and the mark of understanding it well is knowing where it stops
being accurate — not being confused when it does.
