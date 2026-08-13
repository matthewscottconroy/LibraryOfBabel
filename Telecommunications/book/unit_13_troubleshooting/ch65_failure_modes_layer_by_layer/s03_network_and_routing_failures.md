# 65.3 Network and Routing Failures

**Layers 1 and 2 are working. Packets are not reaching their destination**, or are reaching it
and not returning.

## The four host-side faults

**All four produce "cannot reach anything" or "can reach some things", and they are
distinguishable in one command each.**

### Wrong address

**Duplicate, or in the wrong range entirely.**

| Symptom | |
|---|---|
| **Intermittent, affecting two machines** | **duplicate address** (Chapter 53 §53.3) |
| **Nothing works at all** | wrong range for the VLAN |
| **`169.254.x.x`** | **DHCP failed** (Chapter 40 §40.4) |

### Wrong mask

**The subtle one, and the symptom depends on the direction of the error.**

**Real subnet 10.20.4.0/22; host configured 10.20.5.14/24:**

> **The host believes its subnet is 10.20.5.0/24.** **A destination at 10.20.6.9 — which is on
> the same physical segment — is treated as remote**, so **the host sends it to the gateway**,
> which routes it back out of the same interface. **It works, with an ICMP redirect and an extra
> hop, and it looks fine.**

**Now the reverse. Real subnet 10.20.5.0/24; host configured 10.20.5.14/16:**

> **The host believes 10.30.0.5 is on its own segment**, so **it ARPs for it and nothing
> answers.** **Communication to that destination fails completely**, while everything inside
> 10.20.5.0/24 works.

| Mask too **narrow** | Mask too **wide** |
|---|---|
| **Local destinations treated as remote** | **remote destinations treated as local** |
| **Works, via the gateway, with redirects** | **fails — ARP goes unanswered** |
| **A performance and elegance problem** | **a hard failure to specific destinations** |

**Which is why "it works for some destinations and not others, on the same subnet" is a mask
question**, and `ip route get <destination>` (Chapter 64 §64.2) answers it immediately.

### Wrong gateway

**Or none.**

| Symptom | |
|---|---|
| **Local subnet works, nothing else does** | **no gateway, or a wrong one** |
| **Some remote destinations work** | **a static route covers them and the default is wrong** |
| **`ip neigh` shows the gateway `FAILED`** | **the gateway address is not on this segment** |

### Wrong DNS

**Chapter 64 §64.2, and it is Layer 7 presenting as Layer 3** — **`ping 10.9.0.5` works and
`ping app.example.com` does not.**

## Routing faults

### Missing route

```
   $ ip route get 10.9.0.5
   RTNETLINK answers: Network is unreachable
```

**On a router:**

```
   R1# show ip route 10.9.0.0
   % Network not in table
```

**Which is unambiguous**, and the question becomes **why** — an adjacency down, a route filtered,
a summarisation that excludes it, a redistribution not configured.

### Asymmetric routing

**Traffic goes one way and returns another.**

> **Not a fault in itself. IP permits it, and it is common** (Chapter 24 §24.1).

**It becomes a fault when something stateful is in the path:**

| Device | Consequence |
|---|---|
| **Stateful firewall** | **the return traffic has no state entry and is dropped** (Chapter 60 §60.2) |
| **NAT** | **the return has no translation entry** |
| **Load balancer** | the return bypasses it and the source address is wrong |
| **Strict uRPF** | **drops the packet on arrival** (Chapter 62 §62.2) |

**The signature: some connections work and some do not, varying by flow**, and **it appears
after a redundancy change or during a failover.**

**And it is diagnosed by traceroute from both ends** (Chapter 64 §64.1), **because a single
traceroute cannot show it.**

### Routing loops

| Symptom | |
|---|---|
| **`Time to live exceeded` from ping** | |
| **Traceroute showing the same addresses repeating** | |
| **High CPU on the routers involved** | |
| **A prefix flapping in and out of the table** | |

**Causes:** **a static route pointing at a device that routes it back**, **redistribution between
protocols without filtering**, **a summary route pointing to null missing**, or **a transient
during convergence** (Chapter 31 §31.4) — **which is normal and self-correcting.**

### Convergence and flapping

> **A route that appears and disappears repeatedly is worse than one that is missing**, because
> **every change is recomputed everywhere and the CPU cost is real.**

**Dampening exists for this** (Chapter 32), **and the underlying cause is usually a flapping
interface** (§65.1) **or an unstable adjacency** — **and the fix is at the flapping element, not
at the routing protocol.**

### Adjacency failures

**A routing protocol that will not form a neighbour relationship**, and the causes are a short
list:

| Protocol | Common causes |
|---|---|
| **OSPF** | **area mismatch, subnet/mask mismatch, hello/dead timer mismatch, authentication mismatch, MTU mismatch, network type mismatch** |
| **BGP** | **wrong AS number, TCP 179 blocked, source address mismatch, TTL security, authentication** |
| **All** | **the interface is passive, or the protocol is not enabled on it** |

**Two deserve emphasis:**

> **OSPF's MTU mismatch is the one that produces a puzzling symptom.** **The adjacency reaches
> ExStart/Exchange and stops**, because **the database description packets are large and one side
> cannot receive them.** **The adjacency is not down and is not up**, and the state name is the
> diagnosis.

**And BGP over a path with a firewall:** **TCP 179 must be permitted in both directions**, and
**the session's source address must match what the peer expects** — **which is why `update-source`
exists and why a peering that works from one router and not its identical neighbour is an
address problem.**

## NAT faults

**Chapter 33's mechanism, as a fault.**

| Symptom | Cause |
|---|---|
| **Outbound works, inbound does not** | **expected** — NAT is asymmetric by design |
| **Port exhaustion at scale** | **PAT's 64,512 ports per public address** (Chapter 33 §33.2) |
| **Long-lived idle connections dropping** | **the translation timed out** |
| **A protocol that embeds addresses failing** | **FTP, SIP, some legacy protocols** (Chapter 33 §33.3) |
| **Two sites cannot be joined** | **overlapping address space** (Chapter 61 §61.1) |
| **Traffic to an internal service from inside failing** | **NAT hairpinning not configured** |

**Hairpinning is the one that produces the most confusing report:**

> **An internal host resolves the service's public name to its public address, and sends the
> packet to the firewall, which must translate it and send it back inside.** **Many devices do
> not do this by default**, so **the service works from outside and not from inside**, which is
> the opposite of what anyone expects.

**And split-horizon DNS is the better fix** (Chapter 39 §39.3): **give internal clients the
internal address and the problem does not arise.**

## IPv6-specific

**Faults with no IPv4 equivalent.**

| Symptom | Cause |
|---|---|
| **Link-local address only** | **no Router Advertisement** — router not configured, or RA Guard blocking |
| **Address obtained, no connectivity** | **RA sent with no default route flag**, or the router is not forwarding |
| **DNS works, connections fail** | **the client prefers IPv6 and the IPv6 path is broken** — Happy Eyeballs mitigates this |
| **Slow connections that eventually work** | **IPv6 attempted first and timing out** |
| **Everything broken after a firewall change** | **ICMPv6 filtered** (Chapter 60 §60.1) |
| **A rogue router advertising** | Chapter 62 §62.1's RA Guard |

> **"Turn off IPv6" is the reflexive fix and it is usually the wrong one** — **it hides a
> misconfiguration rather than fixing it, and the misconfiguration will be waiting when IPv6 is
> eventually required.** **Fix the RA, the routing or the firewall.**

## The diagnostic sequence

```
   1.  ip addr        — right address? right mask?
   2.  ip route       — gateway present? correct?
   3.  ip neigh       — is the gateway resolving?
   4.  ping gateway   — Layer 3 to the first hop
   5.  ping a remote IP — beyond the gateway
   6.  ping by name   — DNS (Chapter 63 §63.3's highest-yield test)
   7.  traceroute     — where does it stop?
   8.  traceroute from the far end — asymmetry
   9.  On the routers: show ip route <dest>, and the adjacency state
```

**Steps 1 to 4 are on the host, take twenty seconds, and resolve most of this section.**

## What breaks here

**Some destinations on the same subnet unreachable.** **Mask too wide** — the host is ARPing for
things that are not local.

**Everything works via the gateway including local traffic.** **Mask too narrow.** Functional,
inelegant, and it will confuse someone.

**Local works, remote does not.** **Gateway.**

**`ping` by IP works, by name does not.** **DNS**, and Chapter 63 §63.3 says so in one command.

**Some flows work and some do not, after a redundancy change.** **Asymmetric routing meeting a
stateful device.**

**Traceroute repeating the same two addresses.** **A routing loop.**

**An OSPF adjacency stuck at ExStart.** **MTU mismatch.** The state name is the diagnosis.

**A BGP session that works from one router and not its neighbour.** **Source address**, or a
filter on TCP 179.

**A service reachable from outside and not from inside.** **NAT hairpinning**, and split-horizon
DNS is the better answer.

**IPv6 hosts with link-local only.** **No RA.**

**A connection that takes 20 seconds and then works.** **IPv6 attempted first and timing out.**
Fix IPv6 rather than disabling it.

> **Network+ note.** Objective 5.3 covers these. Over-learn: **incorrect IP address, subnet
> mask, gateway or DNS server**; **duplicate IP address**; **routing loops and the TTL exceeded
> message**; **missing routes**; **asymmetric routing and its interaction with stateful
> devices**; and **APIPA indicating DHCP failure.** The four host-side faults and their distinct
> symptoms are examined constantly, and **the mask-too-wide versus mask-too-narrow distinction
> is the one candidates get wrong.**
