# 35.1 The Final Hop Problem

Units VI and VII built a system that delivers a packet to any host on Earth. This section
is about what is still missing, and it is worth stating the gap before naming the
solution.

## The gap

**A packet arrives at `203.0.113.10`. Now what?**

That machine is running:

- a web server
- an SSH daemon
- a mail server
- a DNS resolver
- a monitoring agent
- a database
- forty other things

**IP delivered the packet to the machine. It did not say which program it was for.**

The IP header (Chapter 24 §24.2) contains a source address, a destination address, and a
**protocol** field — which says the payload is TCP, or UDP, or ICMP. **It does not say
which TCP conversation, or which program.**

**Without a solution, a machine could run exactly one network application.** Every packet
arriving would have to be for the only thing listening, because there would be no way to
tell otherwise.

## The requirement, derived

What would a solution need?

**1. An identifier in the packet.** The receiving host must be able to tell, from the
packet alone, which program to hand it to. Nothing external will help — the sender is
remote and the host cannot ask.

**2. Agreed in advance for servers.** A client contacting a web server must know what to
put in that field **before** it makes contact. If the identifier were negotiated, you
would need a prior conversation to negotiate it, and that conversation would need an
identifier.

**So some identifiers must be fixed by convention**, known to everyone, published.

**3. Arbitrary for clients.** A client's identifier need not be agreed with anyone. It must
only be **unique on that host**, so replies can be matched to the right program.

**4. Small.** It is in every packet. Every bit costs bandwidth on every transmission
(Chapter 21 §21.3).

**5. Enough of them.** A busy server holds many thousands of simultaneous conversations.

**The answer:** a **16-bit number**, present in every TCP and UDP header, called a
**port**.

$$2^{16} = 65{,}536 \text{ values, } 0 \text{ through } 65{,}535$$

## Why 16 bits

**Large enough** that a host can hold ~65,000 simultaneous conversations per address —
which was extravagant in 1981 and is occasionally binding now (Chapter 33 §33.2's NAT
capacity).

**Small enough** to cost four bytes per segment for both ports together, which is
tolerable overhead.

**And it is the smallest field that works.** Eight bits would give 256, exhausted by the
well-known services alone.

## The two-level address

**The result is a two-part address**, and stating it plainly resolves a great deal of
confusion:

$$\underbrace{\texttt{203.0.113.10}}_{\text{which machine}} : \underbrace{\texttt{443}}_{\text{which program}}$$

| Layer | Identifies | Chapter |
|---|---|---|
| **IP address** | **a host** | 25 |
| **Port** | **a process on that host** | this one |

> **The IP address gets the packet to the machine. The port gets it to the program.**

**This is the layer boundary made concrete.** Layer 3's job ends at the host; Layer 4's job
is the last step, and it is why Chapter 22 §22.3 said Layer 4 provides *process-to-process*
communication where Layer 3 provides host-to-host.

## The demultiplexing chain, completed

Chapter 23 §23.3 traced a packet through the stack and identified a chain of
demultiplexing keys. **The port is the last link:**

$$\underbrace{\texttt{EtherType}}_{0\text{x}0800} \rightarrow \underbrace{\texttt{Protocol}}_{6} \rightarrow \underbrace{\texttt{Port}}_{443} \rightarrow \text{the process}$$

| Layer | Reads | To choose |
|---|---|---|
| Link | **EtherType** `0x0800` | IPv4 |
| Internet | **Protocol** `6` | TCP |
| Transport | **Destination port** `443` | the web server |
| Application | — | (the end of the chain) |

**Every layer has exactly one field that says who gets the payload**, and the chain
terminates at a process. **That is the whole architecture of demultiplexing**, and once it
is clear, the placement of any protocol becomes derivable rather than memorised.

## Ports are per-protocol

**A detail that surprises people, and it follows from the chain above.**

**TCP port 53 and UDP port 53 are different ports.** They are not the same number used
twice — they are entries in two separate namespaces, because the **protocol field is
consulted before the port field.**

**A host may have one program listening on TCP/53 and a completely different one on
UDP/53**, and nothing prevents it.

**In practice they are assigned together** — DNS uses both (Chapter 39 §39.2), and IANA
reserves both when it assigns a service a number — but the separation is real and it shows
up in firewall rules, where `permit tcp any any eq 53` and `permit udp any any eq 53` are
two different rules and forgetting one is a common error.

## What a port is not

Three misconceptions worth clearing before §35.2.

**A port is not a physical thing.** The word is borrowed from hardware, and it is
unfortunate. **A port is a number in a header.** There is no socket on the back of the
machine, nothing is plugged in, and "port 443 is open" means "a program has asked the
operating system to deliver packets bearing that number".

**A port is not owned by a protocol.** HTTP conventionally uses 443, and nothing enforces
it. You can run a web server on port 8080, or 9999, or 22, and it will work — clients
simply need to be told. **The numbers are convention, and §35.3 develops how strong a
convention.**

**"Open" is a property of a host, not of a network.** A port is open if something is
listening. A firewall may permit or block traffic to it, and that is a different question.
**"The port is blocked" and "the port is closed" are different faults** with different
symptoms (Chapter 22 §22.4's RST-versus-silence), and conflating them wastes time.

## The problem it does not solve

**Ports identify a program. They do not identify a conversation.**

A busy web server has thousands of clients connected to port 443 **simultaneously**. The
destination port is 443 for every one of them, so it cannot distinguish them.

**Something more is needed**, and §35.2 is that: the **five-tuple**, of which the port is
one component. It is worth noticing the shape of the argument — each layer's identifier is
sufficient for that layer's job and insufficient for the next, which is why the chain has
the length it does.

## What breaks here

**Expecting the IP address alone to identify a service.** It identifies a machine.

**Assuming TCP/53 and UDP/53 are the same thing.** They are separate namespaces, and
firewall rules must cover both.

**Confusing "closed" with "blocked".** A closed port answers with a refusal; a blocked one
is silent.

**Assuming a service must use its conventional port.** Nothing enforces it.

> **Network+ note.** Objective 1.4 expects ports and protocols. Over-learn: **an IP
> address identifies a host and a port identifies a process on it**; **ports are 16 bits,
> 0–65535**; and **TCP and UDP port spaces are separate.** The two-level address is the
> framing that makes every later port question straightforward.
