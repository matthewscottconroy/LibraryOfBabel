# 21.4 Layer Violations in the Wild

The model is clean. The network is not. This section catalogues the violations, and
the point is not to disapprove of them — several are essential and one runs the entire
Internet — but to be able to recognise them, because **most difficult network faults
happen at a place where the model does not hold.**

## Middleboxes

The general category: a device that inspects or modifies fields belonging to a layer
above the one at which it nominally operates.

The Internet's original architecture had **hosts** at the edges and **routers** in the
middle, with routers examining only the IP header. That model held until roughly 1994.

### NAT — the big one

A network address translator rewrites the **IP header** (a Layer 3 function, so far so
good) *and* the **TCP or UDP port numbers** (Layer 4). It must, because a single public
address is shared among many hosts and ports are what distinguish them (Chapter 33).

It also **recomputes the TCP checksum**, which covers a pseudo-header containing the IP
addresses it just changed. So a Layer 3 device is parsing and rewriting Layer 4.

**The consequences are structural:**

- **The end-to-end principle is broken.** The address a host believes it has is not
  the address its peer sees.
- **Inbound connections fail** without explicit configuration, which changed the
  Internet from a peer-to-peer network to a client-server one for most users.
- **Protocols carrying addresses in their payload break.** FTP's `PORT` command sends
  an IP address as ASCII text *inside* the data stream (Chapter 41 §41.2), and SIP does
  the same for voice. A NAT must therefore parse the **application layer** to rewrite
  them — which is what an **application-layer gateway** is, and it is a Layer 3 device
  reading Layer 7.
- **Encryption breaks the gateway.** If the payload is encrypted, the NAT cannot
  rewrite it. Which is why SIP over TLS needs STUN, TURN and ICE (Chapter 33 §33.3) — an
  entire protocol family that exists to work around NAT.

**NAT is the most consequential layer violation ever deployed, and it saved IPv4.**
Without it, address exhaustion would have forced IPv6 in about 2000. With it, IPv4 has
lasted twenty-five extra years, at the cost of a permanently more complicated Internet.

Chapter 33 argues both sides properly. Note here only that the violation was not
carelessness: it was a deliberate trade of architectural purity for survival, made by
people who understood exactly what they were giving up.

### Firewalls

A stateful firewall (Chapter 60 §60.2) tracks TCP connection state — sequence numbers,
flags, the handshake — to decide what to permit. That is transport-layer state held in a
network-layer device.

A next-generation firewall goes further: it identifies applications by inspecting
payloads, terminates TLS to inspect what is inside, and applies policy on Layer 7
content. It is a Layer 3 device operating at every layer simultaneously.

Necessary. Also the reason a protocol designer in 2020 must assume that anything not
encrypted **will** be inspected, and that anything not looking like HTTPS **may** be
blocked.

### Load balancers

An L4 load balancer rewrites addresses and ports. An **L7** load balancer terminates
the TCP connection entirely, reads the HTTP request, chooses a server, and opens a
*separate* connection to it. There is no end-to-end connection at all — there are two,
joined by a device in the middle.

Which is why a server sees the load balancer's address as the client, and why
`X-Forwarded-For` exists: an application-layer header carrying network-layer
information that the layering destroyed. **A layer violation whose remedy is another
layer violation.**

## Protocol-level violations

### The TCP pseudo-header

The most-cited example, and it is in the original specification.

TCP's checksum covers not only the TCP segment but a **pseudo-header** containing the
source and destination **IP addresses**:

```
   ┌─────────────────────────────┐
   │      Source IP address      │  ← from the IP header
   ├─────────────────────────────┤
   │   Destination IP address    │  ← from the IP header
   ├────────┬────────┬───────────┤
   │  zero  │ proto  │  TCP len  │
   └────────┴────────┴───────────┘
```

**TCP reads the IP header.** By specification. Since 1981.

The reason is sound: it detects a packet misdelivered to the wrong host — where the
addresses were corrupted but IP's own header checksum happened to pass, or where the
address was corrupted after IP checked it. Without the pseudo-header, a segment
delivered to the wrong machine could be accepted as valid.

The cost is that **TCP and IP cannot be separated**. Change the address size and TCP's
checksum computation changes, which is exactly what happened with IPv6 and is one small
reason dual-stack implementations are more than a recompile.

**Cerf and Kahn's original 1974 protocol did not separate them at all** — TCP and IP
were one protocol, split in 1978 precisely to allow UDP and other transports
(Chapter 23 §23.1). The pseudo-header is the seam left by that split.

### ARP

Chapter 18 §18.1 covered it: resolves Layer 3 addresses, is carried directly in Layer 2
frames, belongs to neither. "Layer 2.5" is the polite designation.

### ICMP

Carried **inside IP** (so it is above IP), but it is a **control protocol for IP**
(so it is part of IP). Its messages contain a copy of the offending packet's header, so
it reads the payload of what it reports on.

It is IP's control plane travelling inside IP's data plane. Chapter 34 §34.1 covers it.

### MPLS

Labels sit **between** the Ethernet header and the IP header. Not Layer 2, not Layer 3.
The industry calls it **"Layer 2.5"** — the same name as ARP, for a completely
different thing, which tells you how much the numbering means.

MPLS is a good illustration that the model is a description rather than a constraint:
the designers needed something with the forwarding speed of Layer 2 and the topology
awareness of Layer 3, and built it, and the model had no slot for it. Chapter 51.

### QUIC

The deliberate one, and the most significant recent development.

QUIC (Chapter 38) puts a reliable, multiplexed, encrypted transport **inside UDP,
implemented in user space**. It merges the transport layer, the security layer, and
part of the session layer into a single protocol that the kernel does not implement.

**The reasons are precisely §21.3's costs:**

- TCP's head-of-line blocking penalised HTTP/2's multiplexing, and the boundary made
  it impossible to fix from either side
- TCP's handshake and TLS's handshake cost separate round trips because they were
  separate layers
- **TCP could not be changed**, because middleboxes drop TCP they do not recognise —
  the ossification that middleboxes caused

That third point closes the loop. **Middleboxes violated layering; the ossification
they caused made TCP unevolvable; the response was a new transport that violates
layering deliberately and encrypts nearly everything so middleboxes cannot see it.**

QUIC encrypts most of its header specifically to prevent the next generation of
middleboxes from ossifying it the same way. That is a design decision made in direct
response to thirty years of layer violations, and it is the clearest evidence that this
section is not a list of curiosities.

## Optimisation violations

From §21.3, in brief: **TSO/LRO/GRO** (the NIC segments TCP), **checksum offload**
(the NIC computes transport checksums), **RDMA** (the NIC writes to remote application
memory), **DPDK/XDP** (user space drives the hardware). All ship in volume; all are
invisible until you take a packet capture and find frames that were never on the wire.

## How to think about this

Three positions, and the third is right.

**"Violations are bad and should be eliminated."** Untenable. NAT saved IPv4;
firewalls are necessary; offloads are how line rate is achieved.

**"The model is useless because it is violated."** Also wrong. Every one of these
violations is *recognisable as a violation*, which requires the model. And Chapter 22
§22.4's diagnostic method works — it is the most effective troubleshooting technique in
this book — precisely because the model describes the common case well.

**"The model is a map."** Useful, simplified, accurate for most of the territory, and
**wrong in specific known places that you should learn**. A map that omits a bridge is
still worth carrying; you simply need to know about the bridge.

> **Layering is a design discipline, not a law of physics. It is violated where the
> cost of honouring it exceeds the benefit — and the interesting engineering is
> almost always at those places.**

Which is also where the hard faults are. A fault that respects layering is usually
easy: one layer is broken, the method of §22.4 finds it. **A fault that crosses layers
— a NAT mangling a payload, a firewall dropping fragments, an offload corrupting a
checksum, a middlebox rejecting an unfamiliar TCP option — is hard**, because the
symptom appears at a layer other than the cause.

## What breaks here

**A protocol working until it crosses a NAT.** It carries addresses in its payload.
FTP, SIP, and older peer-to-peer protocols.

**A capture showing 64 KB TCP segments.** TSO. `ethtool -K eth0 tso off gso off gro
off` before capturing.

**A connection establishing and then hanging on large transfers.** ICMP filtered, path
MTU discovery broken. A layer violation by a firewall breaking a mechanism at another
layer.

**A new TCP option causing connections to fail on some paths.** Middlebox
ossification. This is why QUIC exists.

**An application seeing the load balancer's address as the client.** Layer 7 proxying.
`X-Forwarded-For`.

> **Network+ note.** Objectives 1.2 and 1.4 cover middleboxes as devices; objective
> 5.5 covers capture. Worth over-learning: **NAT rewrites Layer 4 ports as well as
> Layer 3 addresses**; **an application-layer gateway parses payloads to fix protocols
> that embed addresses**; and **offloads make host captures differ from the wire**. All
> three appear in troubleshooting scenarios.
