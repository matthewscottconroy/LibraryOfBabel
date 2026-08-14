# 29.4 The Default Gateway

`0.0.0.0/0` is the shortest prefix that exists, matches every address, and therefore
loses to every other route. That combination makes it the *route of last resort*, and it
is the most consequential entry in most routing tables.

## The idea

A host cannot hold a route to every network on the Internet. It holds:

```
   192.168.1.0/24   directly connected
   0.0.0.0/0        via 192.168.1.1
```

**Two entries covering four billion addresses.** Anything not on the local subnet goes to
the gateway, and the gateway's problem is what happens next.

**Why it works with no special case:** `0.0.0.0/0` has zero network bits, so
`address AND 0.0.0.0 = 0.0.0.0` for every address — it always matches. And being the
shortest prefix, longest-prefix match rejects it whenever anything else applies (§29.3).

> **The default route needs no mechanism of its own.** It falls out of longest-prefix
> match. This is the kind of economy that marks a good design.

## Recursive delegation

Follow a packet from a laptop to a distant server and the same pattern repeats at
increasing scale:

```
   Laptop           →  default → home router
   Home router      →  default → ISP edge
   ISP edge         →  default → ISP core
   ISP core         →  ⟵ HERE IT STOPS
```

**At the ISP core the default route disappears.** A router in the Internet's core, or in
any provider running BGP with full tables, has **no default route at all** — it holds
roughly 950,000 prefixes covering the entire routed Internet, and if a destination
matches none of them, **there is nowhere to send it.**

**These routers are called *default-free*** (the "default-free zone", DFZ), and the term
is a precise description rather than jargon:

| | Edge router | DFZ router |
|---|---|---|
| Routes held | a handful + default | **~950,000, no default** |
| Unknown destination | send to the default | **drop, and send ICMP unreachable** |
| Memory required | trivial | **TCAM-limited** (§29.1) |

**The whole Internet is a hierarchy of "I don't know, ask upstream" — until you reach the
level where there is no upstream, and someone must actually know.**

That is where Chapter 32's BGP lives, and it is why the routing table's size is an
Internet-wide concern rather than an operator's local one.

## Configuring it

```bash
# Linux
ip route add default via 192.168.1.1
ip route add default via 192.168.1.1 dev eth0 metric 100
ip route del default

# Cisco IOS
ip route 0.0.0.0 0.0.0.0 203.0.113.1
ip route 0.0.0.0 0.0.0.0 GigabitEthernet0/0 203.0.113.1   # interface + next hop

# Windows
route add 0.0.0.0 mask 0.0.0.0 192.168.1.1 -p
```

**A note on the IOS form.** Specifying **only an interface** on a multi-access link
(Ethernet) is a mistake worth knowing about:

```
ip route 0.0.0.0 0.0.0.0 GigabitEthernet0/0        ← don't
```

The router now believes **every** Internet destination is directly connected to that
interface, so it **ARPs for each one individually**. With proxy ARP on the far side it
appears to work, while filling the ARP cache with thousands of entries; without it,
nothing works. **Always give a next-hop address on a broadcast link.** On a genuine
point-to-point link (serial, some tunnels) interface-only is correct, because there is
only one possible neighbour.

## How a host learns it

| Method | Where |
|---|---|
| **DHCP option 3** | almost every host (Chapter 40 §40.3) |
| **IPv6 Router Advertisement** | always, in IPv6 (Chapter 28 §28.3) |
| Static configuration | servers, infrastructure |
| A routing protocol | routers |
| ICMP redirect | discouraged; see below |

**The IPv4/IPv6 asymmetry is worth restating** (Chapter 28 §28.3): DHCPv4 supplies the
gateway; **DHCPv6 never does** — it always comes from the Router Advertisement. A network
with DHCPv6 and no RAs hands out addresses and no route.

## When one gateway is not enough

An SVI or a router is the gateway for a whole subnet, so **its failure removes off-subnet
connectivity for every host on it** (Chapter 20 §20.4) — even though the network around
them is fine.

Hosts do not, in general, fail over. A statically configured or DHCP-supplied gateway is
a single address, and if it stops responding the host keeps sending to it.

**First-hop redundancy protocols** solve this by making the gateway address **virtual**:

| Protocol | Origin |
|---|---|
| **VRRP** | standard, RFC 5798 |
| **HSRP** | Cisco |
| **GLBP** | Cisco, with load sharing |
| **CARP** | BSD |

Two routers share a **virtual IP and virtual MAC address**; one is active, one standby;
hosts point at the virtual address and never know which physical device is serving them.
Failover works by **gratuitous ARP** (Chapter 18 §18.3) — the newly active router
announces the virtual MAC, switches update their tables, and traffic follows within a
second.

**Chapter 56 §56.2 covers this properly.** Note here only that **IPv6 does not need it**:
NDP's Neighbour Unreachability Detection (Chapter 28 §28.3) lets a host notice its router
has died and switch to another by itself. Which is a genuine architectural improvement,
and one of the better answers to "what does IPv6 actually give me?"

## ICMP redirect

The mechanism for *"you sent that to me, but the better first hop is over there."*

```
   Host has one default: R1.
   Host sends a packet for 10.5.0.0/16 to R1.
   R1's own route for it points back out the same interface, to R2.
   R1 forwards the packet AND sends the host an ICMP Redirect:
        "for 10.5.0.0/16, use R2"
```

It works, and it is **discouraged and usually disabled**:

- **A host that accepts redirects accepts routing changes from anything on its segment**,
  which is a straightforward attack (Chapter 62)
- It masks a routing design problem rather than fixing it
- The resulting host routing table is invisible to network management
- It produces a path that is correct and inexplicable

**`net.ipv4.conf.all.accept_redirects=0`** is standard hardening, and **`no ip
redirects`** on router interfaces is standard configuration. If redirects are needed, the
gateway assignment is wrong.

## The failure modes

**No default route.**

```
$ ping 8.8.8.8
connect: Network is unreachable
```

**That exact message means no route matched** — not a timeout, not unreachable-host, but
the local stack refusing to send because it has nowhere to send to. **Local traffic still
works**, which makes the symptom distinctive: *"I can reach the printer and nothing
else."*

**Wrong default gateway.** An address that exists but is not a router. Packets are sent
and go nowhere; ARP succeeds, so Layer 2 looks fine. Symptom: **a timeout rather than an
error**, which is the harder case.

**Gateway not on the local subnet.** The classic mask error (Chapter 25 §25.3):

```
   Host:    192.168.1.50/25      network 192.168.1.0 - .127
   Gateway: 192.168.1.200        outside it
```

**The host cannot reach its own gateway.** Most stacks reject the configuration; some
accept it and fail silently. `ip route` shows the default as unreachable.

**Two default routes.** Not an error — the one with the better metric wins, and if both
are equal, ECMP (§29.3) splits traffic between them. That is desirable if both work and
catastrophic if one is a stale VPN route, because **half of all connections fail and half
succeed.**

This is a genuinely common laptop problem: a VPN adds a default route, the VPN
disconnects untidily, and the route remains.

**A default pointing at a router that has no default.** Traffic reaches it and stops
there. The `traceroute` shows one extra hop and then nothing.

## The diagnostic sequence

Chapter 22 §22.4's method, at this layer:

```bash
ip route | grep default        # is there one?
ping <gateway>                 # is it reachable?
arping -I eth0 <gateway>       # is it reachable at Layer 2? (Chapter 18 §18.3)
ip route get 8.8.8.8           # what would actually happen?
traceroute 8.8.8.8             # where does it stop?
```

**In order, each eliminates a cause.** No default → configuration. Default present but
unreachable → mask or Layer 2. Reachable but nothing beyond → the gateway's own routing.

## What breaks here

**"Network is unreachable".** No default route. Distinct from a timeout.

**Local works, nothing else does.** Same thing.

**Gateway outside the host's own subnet.** Mask error.

**Two defaults, half of connections failing.** A stale route, usually a VPN's.

**Interface-only default on Ethernet.** The router ARPs for every Internet destination.

**A path that is correct and makes no sense.** ICMP redirects, or proxy ARP (Chapter 18
§18.3).

> **Network+ note.** Objectives 1.7 and 2.2 expect the default gateway, and objective 5.3
> lists **incorrect default gateway** as a named cause. Over-learn: **`0.0.0.0/0` matches
> everything and loses to everything**; **"network is unreachable" means no route
> matched**; **the gateway must be on the host's own subnet**; and **core routers are
> default-free**. The last one is examined less and explains the most.
