# 14.3 Clients, Servers, and Peers

The network's shape follows from what the applications do with it, and the
application architectures have swung between centralisation and decentralisation
several times. Each swing had consequences for the network beneath, and the current
position is not the one the Internet's architecture anticipated.

## Client–server

One party requests; another responds. The **server** listens on a well-known
address and port (Chapter 35 §35.3); the **client** initiates.

The asymmetry is the point, and it has network consequences:

- **The server must be reachable at a stable, known address.** Hence DNS
  (Chapter 39), hence static addressing or reservations for servers, hence the
  entire business of publishing a service.
- **The client need not be reachable at all.** It initiates, so return traffic
  follows an existing conversation. This is what makes NAT (Chapter 33) viable for
  clients and impossible for servers without configuration.
- **Traffic is asymmetric** — small requests, large responses — which is why
  consumer access technologies are asymmetric (Chapter 49) and why that asymmetry
  became a problem when video calling made the uplink matter.

This is the dominant model and has been since the web. Nearly everything in
Chapter 41 is client–server.

## Peer-to-peer

Every participant is both client and server. No party is privileged; any can
initiate to any.

**Instances:** the original ARPANET applications (every host was a peer), file
sharing from Napster onward, BitTorrent, blockchain networks, WebRTC media paths,
and — significantly — most local service discovery.

**Advantages:** capacity scales with participants rather than being provisioned
centrally; no single point of failure; no central cost.

**Network consequences, and they are substantial:**

- **Every participant must be reachable inbound**, which NAT breaks comprehensively
  (Chapter 33 §33.3). The entire apparatus of STUN, TURN, ICE and hole punching
  exists to work around this, and every video-calling application contains it.
- **Traffic is symmetric and unpredictable**, which access networks provisioned for
  asymmetric client traffic handle badly.
- **Traffic patterns are many-to-many**, which is difficult to engineer capacity
  for.

## The re-centralisation, and its consequences

Here is the observation that matters for anyone designing networks now.

The Internet's architecture is peer-to-peer at the network layer. Every host has an
address; any host can address any other; the network does not distinguish clients
from servers. That is the model Chapter 14 §14.2's four requirements produce.

The **application layer has re-centralised comprehensively**, and it happened in
three stages:

**Stage one: NAT.** Chapter 33 §33.2 makes the argument — a NAT'd host cannot
receive unsolicited connections, so it can consume services and cannot offer them.
Address exhaustion converted a network of peers into a network of clients as a side
effect of an addressing workaround, without anyone deciding to.

**Stage two: the cloud.** Applications moved from machines organisations operated
to services a small number of providers operate. The traffic followed.

**Stage three: content delivery.** Chapter 52 §52.4's CDNs, and the fact that a
large majority of consumer bytes now originate from a small number of very large
sources.

The consequences for network design are direct:

**Traffic goes outward, not inward.** A branch office's traffic is overwhelmingly to
the Internet rather than to headquarters, which is Chapter 51 §51.1's tromboning
argument and the entire case for local breakout.

**The perimeter model stops making sense.** If the applications are not inside and
the users are not inside, a boundary between inside and outside protects little —
which is Chapter 59 §59.4's zero trust argument.

**Latency to a handful of destinations dominates the experience.** Optimising the
path to five providers matters more than optimising the general case.

**And resilience concentrates.** A CDN or cloud provider outage takes down large
numbers of apparently unrelated services simultaneously, which has happened
repeatedly and which is a systemic risk the architecture did not anticipate.

## Three-tier and microservices

Within a data centre, the application architecture has moved in the opposite
direction, and this has network consequences of its own.

**Three-tier** — presentation, application, data — was the standard enterprise
pattern. Traffic is predominantly **north–south**: a request comes in, passes through
the tiers, and a response goes out.

**Microservices** decompose the application layer into many small services
communicating over the network. One external request now triggers dozens of internal
calls.

The consequence is Chapter 11 §11.4's and Chapter 67's: **east–west traffic came to
dominate**, by an order of magnitude, and the three-tier network topology optimised
for north–south stopped fitting. Leaf-spine fabrics are the response.

Note the irony worth naming: **the application layer decentralised inside the data
centre at the same time as it centralised across the Internet**, and both movements
changed what the network underneath had to do.

## The pattern

Worth extracting, because it recurs:

| Era | Application model | Network consequence |
|---|---|---|
| 1970s | Peer hosts | Symmetric, low volume |
| 1980s–90s | Client–server, on-premises | North–south within the site |
| 2000s | Web, plus P2P file sharing | Asymmetric access; NAT traversal industry |
| 2010s | Cloud and CDN | Traffic leaves the site; perimeter erodes |
| 2010s–20s | Microservices, inside | East–west dominates the data centre |

**The network follows the application architecture**, always, and with a lag. A
network designed for the previous era works and is wrong in ways that show up as
performance problems attributed to capacity.

The design question this produces, and it is the useful one:
**where does the traffic actually go?** Not where the org chart says it goes, not
where it went when the network was designed — where it goes now, measured. Chapter 54
§54.4's flow data is how you find out, and Chapter 72's Deliverable 1 asks for it
first for exactly this reason.

## What breaks here

**Backhauling cloud traffic to a central gateway.** Tromboning, and it is the most
common consequence of a network designed for an application architecture that has
changed.

**A perimeter firewall protecting applications that are no longer behind it.**

**Capacity planned north–south in a data centre running microservices.** Uplinks
saturate with traffic the model never anticipated.

**Peer-to-peer applications behind carrier-grade NAT.** Double translation defeats
most traversal techniques (Chapter 56 §56.2), which is why some applications
degrade to relayed media and consume far more bandwidth than they should.

**Assuming resilience because the architecture is distributed.** The Internet's
architecture is; its current application layer is not, and a CDN outage demonstrates
the difference.

> **Network+ note.** Objective 1.6 expects client–server and peer-to-peer as network
> architectures. Objective 1.3 expects cloud models. The connection worth carrying:
> **the application architecture determines where traffic goes, and where traffic
> goes determines what the network should look like** — which is why Chapter 72's
> design process begins with a communication matrix rather than with a topology.
