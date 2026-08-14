# Chapter 23 — TCP/IP and Encapsulation

In the spring of 1973, Robert Kahn came to Vint Cerf with a problem that was
practical rather than theoretical. ARPA now had three packet networks: the
ARPANET itself, a packet radio network, and a satellite network. Each worked.
None could talk to the others. They had different packet sizes, different error
characteristics, different addressing, different everything, and each was
administered by different people who had no intention of changing their design to
suit anyone else.

Kahn and Cerf worked on it through 1973, reportedly doing a good deal of the
thinking in a hotel lobby in Palo Alto, and in May 1974 published *A Protocol for
Packet Network Intercommunication* in the *IEEE Transactions on Communications*.

The paper's central move is a piece of engineering humility, and it is worth
stating carefully because it is the thing that made the Internet possible.

**They did not try to make the networks the same.** They did not propose a standard
that all three should adopt. Instead they proposed a **gateway** sitting between
networks, and a common packet format that gateways would understand, and a rule
that each network would carry these packets by whatever internal means it liked.
The interconnecting protocol would assume *almost nothing* about the networks
beneath it — only that they could, sometimes, move a packet from one attached
device to another.

Almost nothing. Not reliability, not ordering, not a particular packet size, not
flow control, not even a guarantee of delivery. Whatever the weakest constituent
network could promise was the most the internetwork could promise, and the honest
thing to do was to promise exactly that and build everything else at the edges.

## The hourglass

The consequence is the shape usually drawn as an hourglass, and it is the most important diagram about the Internet's architecture:

```
   Applications:   HTTP  SMTP  DNS  SSH  RTP  DHCP  SIP  ...  (many, and growing)
                        \    \    |    /    /    /
   Transport:              TCP    UDP    SCTP  QUIC
                              \    |    /
   Internet:                      IP                  ← exactly one
                              /    |    \
   Link:            Ethernet  Wi-Fi  PPP  DOCSIS  LTE  ...  (many, and growing)
                    /    |    \    \    \
   Physical:      copper  fibre  radio  ...
```

Wide at the top, wide at the bottom, and a single protocol at the waist. **IP is the
narrow point, and it is narrow on purpose.**

The design rule that follows is stated in RFC 1958 as *"everything over IP, and IP
over everything."* Any new link technology needs only to carry IP to be instantly
useful to every existing application. Any new application needs only to run over IP
to work across every existing network. Neither has to ask permission, and neither
has to coordinate with the other.

This is why you can invent an application today and deploy it worldwide without
negotiating with a single network operator. It is also why IPv6 has taken thirty
years to deploy: the waist of an hourglass is the hardest thing in the entire
structure to change, precisely because everything above and below depends on it.
Chapter 28 tells that story.

## The four layers

TCP/IP's model has four layers rather than seven, and the mapping to OSI is
approximate by design:

| TCP/IP layer | OSI equivalent | Contents |
|---|---|---|
| Application | 5, 6, 7 | HTTP, DNS, SMTP, SSH — including anything session- or presentation-like |
| Transport | 4 | TCP, UDP, QUIC, SCTP |
| Internet | 3 | IP, ICMP, routing protocols |
| Link | 1, 2 | Ethernet, Wi-Fi, PPP, and the physical media |

Note that the TCP/IP model deliberately declines to say much about the bottom
layer. RFC 1122 essentially says: whatever your network is, provide a way to send
an IP datagram over it. That refusal to specify is not an omission. It is the
mechanism by which the model accommodated Wi-Fi in 1997, LTE in 2009, and whatever
arrives next, without amendment.

## Encapsulation, traced

§23.3 does the thing this chapter exists for: it follows one HTTP request from a
browser's `write()` call down to voltage on a wire and back up again on the far
side, naming every header added and removed, with the real byte counts.

```
  Application:  GET / HTTP/1.1\r\nHost: example.com\r\n\r\n         (39 bytes)
  Transport:    [TCP hdr 20B][ HTTP data 39B ]                      → segment, 59 B
  Internet:     [IP hdr 20B][ TCP segment 59B ]                     → packet, 79 B
  Link:         [Eth hdr 14B][ IP packet 79B ][FCS 4B]              → frame, 97 B
  Physical:     preamble + 97 bytes of PAM/NRZ symbols + gap
```

Each layer treats the thing above it as **opaque payload**. The Ethernet driver has
no idea it is carrying HTTP and no need to know; it reads the EtherType, wraps, and
transmits. Chapter 2 §2.4's principle — *a field is a location plus an agreement* —
is exactly what makes this work, and the EtherType is the self-describing marker
that lets the receiver undo it.

The vocabulary for the units at each layer — **PDU**, protocol data unit — is worth
holding precisely, because this book uses the terms strictly and so do good
engineers: *frame* at the link layer, *packet* at the internet layer, *segment* for
TCP and *datagram* for UDP at the transport layer, *message* or *data* above. When
someone says "the switch dropped the packet," they have said something imprecise,
and in a diagnostic conversation imprecision costs time.

## The end-to-end argument

The chapter closes with a paper: Saltzer, Reed and Clark's *End-to-End Arguments in
System Design* (1981/1984), which is the closest thing the Internet has to a
constitution.

Its claim, roughly: **a function should be implemented at the endpoints, and placed
in the network only when it can be justified as a performance optimisation** — and
even then, the endpoints must still implement it, because the network's version can
never be complete.

The canonical illustration is reliable file transfer. Suppose the network guarantees
reliable delivery of every packet. Is the file transfer now reliable? No: the disk
at the far end might fail to write, or the memory might corrupt the data before the
network sees it, or the application might crash mid-transfer. To be genuinely
reliable, the application must verify end to end anyway — at which point the
network's guarantee is redundant for correctness, and can be justified only if it
makes things faster.

The argument explains a great deal at once: why IP is best-effort, why TCP lives on
the host rather than in routers, why the Internet's core is comparatively simple and
its edges comparatively complicated, and why the telephone network — which put
intelligence in the network and kept the endpoints dumb — produced a system that was
excellent at one thing and could not be extended to do others.

It also explains why NAT (Chapter 33), deep packet inspection, and transparent
proxies are *controversial* rather than merely unpleasant. They are not aesthetic
objections. They are violations of the principle that made the architecture
extensible, and each one narrows the set of applications that can be invented
without permission.

## By the end you will be able to

- Explain the hourglass and why the waist is deliberately narrow.
- Map the four TCP/IP layers onto the seven OSI layers and state where the
  correspondence is loose.
- Trace encapsulation and decapsulation through a real exchange, naming every
  header and its size.
- Use *frame*, *packet*, *segment* and *datagram* correctly and consistently.
- State the end-to-end argument and use it to evaluate whether a proposed function
  belongs in the network or at the edge.
