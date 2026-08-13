# 27.3 Unicast, Broadcast, Multicast and Anycast

Four delivery models. Everything so far in this book has been unicast; the other three
answer questions unicast cannot, and the fourth — anycast — is quietly the mechanism
behind most of the modern Internet's performance and resilience.

## The four

| Model | Delivered to | Diagram |
|---|---|---|
| **Unicast** | one specific host | one → one |
| **Broadcast** | every host on a segment | one → all |
| **Multicast** | every host that asked | one → many |
| **Anycast** | **the nearest of several** | one → nearest |

```
   Unicast              Broadcast            Multicast           Anycast

   S ──▶ ● A            S ──▶ ● A            S ──▶ ● A           S ──▶ ● A
         ○ B                 ▶ ● B                  ○ B                ○ B'
         ○ C                 ▶ ● C           ──▶ ● C                ○ C'

   one target          everyone            subscribers        nearest instance
                                                              (all share one address)
```

## Unicast

The default. One source, one destination, one copy.

**Everything in Chapters 24–26 is unicast**, and roughly 95% of Internet traffic is.

**The cost:** sending the same data to *n* recipients requires *n* copies traversing the
network. For a video stream to 10,000 viewers, that is 10,000 streams — which is why
multicast and CDNs both exist.

## Broadcast

**Every host on the local segment.** Address: the all-ones host portion
(`192.168.10.255`) or `255.255.255.255`.

**Where it is genuinely needed:**

| Use | Why it must be broadcast |
|---|---|
| **ARP** (Chapter 18) | the asker does not know whom to ask |
| **DHCP discovery** (Chapter 40) | the client has no address and no server address |
| Older NetBIOS name resolution | same bootstrap problem |
| Wake-on-LAN | the target is powered off and has no state |

**Every case is a bootstrap problem**: something must be found before enough is known to
address it directly (Chapter 18 §18.2).

**Its limits:**

- **Never crosses a router.** A broadcast domain ends at a Layer 3 boundary, which is the
  entire point of Chapter 20's VLANs.
- **Every host processes it**, whether or not it cares — a CPU interrupt each time
  (Chapter 17 §17.3).
- **Scales badly**, roughly with the square of host count for ARP.
- **No IPv6 equivalent.** IPv6 removed broadcast entirely and replaced every use with
  multicast, which is a strictly better design (Chapter 18 §18.4).

## Multicast

**Delivered to every host that has asked to receive it.** One copy per link, not one per
recipient — which is the efficiency argument.

**Address range: `224.0.0.0/4`** (`224.0.0.0` – `239.255.255.255`).

| Range | Purpose |
|---|---|
| `224.0.0.0/24` | **link-local control**, never forwarded |
| `224.0.0.1` | all hosts on this subnet |
| `224.0.0.2` | all routers on this subnet |
| `224.0.0.5`, `.6` | **OSPF** routers, designated routers |
| `224.0.0.9` | RIPv2 |
| `224.0.0.18` | VRRP |
| `224.0.0.251` | mDNS |
| `224.0.1.1` | NTP |
| `232.0.0.0/8` | source-specific multicast (SSM) |
| `239.0.0.0/8` | **administratively scoped** — private, like RFC 1918 |

**Note how many of those are protocols already met.** OSPF's hellos, VRRP's
advertisements and mDNS all use multicast so that only the interested devices are
interrupted — the same argument as IPv6's solicited-node addresses (Chapter 18 §18.4).

### The MAC mapping

Multicast IP maps to a multicast MAC address by a fixed rule:

```
   01:00:5e  +  the low 23 bits of the IP address
```

**Note: 23 bits, not 24.** The 24-bit IP group field is one bit larger than the space
available, so **32 different multicast IP addresses map to the same MAC address**.

The consequence is real: a host subscribed to one group may receive frames for another
that happens to collide, and must discard them in software. Rare, and worth knowing when
a capture shows multicast traffic nobody asked for.

### How it works

**IGMP** — Internet Group Management Protocol — is how a host tells its local router
*"I want group 239.1.1.1"*. **PIM** distributes the traffic between routers.

**IGMP snooping** on switches is the operationally important part: without it, a switch
treats multicast like broadcast and floods it to every port, which defeats the purpose
entirely. **A multicast deployment without IGMP snooping is a broadcast deployment with
extra steps**, and it is a very common misconfiguration.

### Where it succeeded and where it did not

| Succeeded | Failed |
|---|---|
| **Enterprise LANs** — IPTV, video distribution, trading floors | **The public Internet** |
| **Routing protocols** — OSPF, EIGRP, RIPv2, VRRP | Internet video |
| **Service discovery** — mDNS, SSDP, Bonjour | |
| **Financial market data** — the canonical use | |
| Data-centre replication | |

**Internet multicast essentially does not exist.** The reasons are instructive:

- **No business model.** A provider carrying multicast bears cost and receives no
  revenue; the sender saves the bandwidth.
- **Inter-domain complexity.** MSDP and inter-domain PIM are genuinely hard to operate.
- **No congestion control.** A multicast sender has no feedback from receivers and
  cannot slow down.
- **Security.** Anyone can join a group; anyone can send to one.
- **CDNs solved the problem differently** — by placing unicast copies near users, which
  requires no cooperation from anyone else's network.

**CDNs won because they need no cooperation.** That is the general lesson: **a solution
requiring every network to change loses to one that requires only your own.** The same
reasoning explains QUIC, and NAT, and much else in this book.

## Anycast

**One address, many hosts, each packet delivered to the nearest.**

The mechanism is startling in its simplicity: **advertise the same prefix from multiple
locations, and let routing do the rest.** Each router sends traffic toward whichever
instance its routing table considers closest.

**There is no anycast protocol.** No special address range, no configuration keyword,
nothing new. It is a consequence of how routing already works.

### Where it is used

| Service | Address | Instances |
|---|---|---|
| **DNS root servers** | `198.41.0.4` (a.root) and 12 more | **1,900+ worldwide** |
| Google Public DNS | `8.8.8.8` | dozens of sites |
| Cloudflare DNS | `1.1.1.1` | 300+ cities |
| Cloudflare, Fastly, Akamai edges | various | hundreds of locations |
| Many CDN and DDoS-scrubbing services | various | global |

**The thirteen DNS root "servers" are not thirteen machines.** They are thirteen
addresses served by more than 1,900 physical instances (Chapter 39 §39.2). Anycast is why
the root can absorb attacks that would obliterate thirteen machines, and why a query from
anywhere reaches a nearby responder.

### What it buys

**Latency.** Users reach a nearby instance automatically, with no client configuration,
no geolocation database and no DNS trickery.

**Load distribution.** Traffic spreads across instances by network topology.

**DDoS resilience.** An attack is absorbed by the instance nearest the attacker rather
than concentrating. **This is anycast's most valuable property** and the reason every
serious DDoS mitigation service is built on it.

**Failure handling.** An instance that fails withdraws its route, and traffic moves to the
next-nearest automatically, in seconds. No health-check system, no failover logic — the
routing protocol is the failover mechanism.

### The catch

**Anycast works cleanly for stateless, short exchanges and awkwardly for stateful ones.**

A routing change mid-connection sends subsequent packets to a **different instance**,
which has no knowledge of the connection state and will reset it.

| | Anycast |
|---|---|
| **DNS over UDP** | **ideal** — one query, one response, no state |
| Short HTTP requests | fine in practice; routing rarely changes that fast |
| **Long TCP connections** | **risky** — a route change resets them |
| Long-lived sessions, uploads | avoid, or handle the resets explicitly |

**Which is why DNS is anycast's canonical application** — it is exactly the stateless,
single-exchange, latency-sensitive case anycast serves perfectly.

For TCP, providers mitigate by keeping routing stable, by using consistent hashing at the
edge, and by accepting a small reset rate. **Cloudflare and Google both run anycast TCP
at enormous scale**, so the difficulty is manageable — it is just not free.

## The comparison

| | Unicast | Broadcast | Multicast | Anycast |
|---|---|---|---|---|
| Recipients | one | all on segment | subscribers | nearest one |
| Crosses routers | yes | **no** | with configuration | yes |
| Efficiency for one-to-many | poor | segment only | **excellent** | n/a |
| IPv6 | yes | **removed** | yes | yes |
| Internet-wide | yes | no | **effectively no** | **yes** |
| Special addresses | no | yes | `224.0.0.0/4` | **no** |
| Extra protocol needed | no | no | IGMP, PIM | **none** |

**Two rows are worth dwelling on.** Multicast has its own address range, its own
protocols and its own switch features — and does not work across the Internet. Anycast
has none of those — and does. **The mechanism that required nothing new is the one that
deployed**, which is Chapter 23's incremental-deployability argument appearing yet again.

## What breaks here

**Multicast flooding everywhere.** IGMP snooping not enabled. The switch treats it as
broadcast.

**Multicast working on one VLAN and not across.** PIM not configured; multicast routing
is not automatic.

**Anycast TCP connections resetting.** A routing change moved the flow to another
instance. Expected behaviour; design for it.

**A broadcast storm.** Chapter 19.

**Expecting broadcast to cross a router.** It does not, ever, and that is what routers
are for.

**Two multicast groups interfering.** The 32-to-1 MAC address collision.

> **Network+ note.** Objective 1.7 expects all four models. Over-learn: **unicast
> one-to-one, broadcast one-to-all-on-segment, multicast one-to-subscribers, anycast
> one-to-nearest**; **multicast is `224.0.0.0/4`**; **broadcast does not cross a router**;
> **IPv6 has no broadcast**; and **anycast requires no special addressing**. The anycast
> definition — *nearest*, not *all* — is the one most often confused with multicast.
