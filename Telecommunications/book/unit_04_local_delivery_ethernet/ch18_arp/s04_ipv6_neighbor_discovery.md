# 18.4 IPv6 Neighbor Discovery

By 1998 the IPv6 working group had watched ARP for sixteen years and knew precisely
what was wrong with it. Neighbor Discovery Protocol — RFC 2461, revised as RFC 4861 in
2007 — is the redesign, and it is instructive because almost every change is
traceable to a specific complaint.

This section covers NDP as the ARP-replacement it is. Chapter 28 covers IPv6
addressing properly, and Chapter 29 covers SLAAC; here the concern is only address
resolution.

## What changed, and why

| ARP | NDP | Because |
|---|---|---|
| Own EtherType (`0x0806`) | **ICMPv6 messages** | ARP fitted nowhere in the layer model; NDP is properly Layer 3 |
| **Broadcast** requests | **Solicited-node multicast** | Broadcasts interrupt every station; multicast interrupts roughly one |
| No authentication | **SEND** available (RFC 3971) | The spoofing problem was known and understood |
| Resolution only | Resolution **+ router discovery + autoconfiguration + redirect + MTU** | These were separate mechanisms in IPv4 (ARP, ICMP, DHCP); consolidating them simplified the stack |
| Duplicate detection optional | **DAD mandatory** | Duplicate addresses were a recurring, hard-to-diagnose failure |
| No reachability tracking | **Neighbour Unreachability Detection** | ARP could not tell a working neighbour from a dead one |

The last row is the underappreciated one. ARP's cache records that a mapping *was*
correct at some point; it says nothing about whether the neighbour is still
functioning. NDP tracks reachability continuously, and a host knows when its default
router has stopped responding — which is what makes automatic failover to a second
router possible without any additional protocol.

## The messages

Four ICMPv6 types carry NDP:

| Type | Name | Purpose |
|---|---|---|
| 133 | Router Solicitation (RS) | *Are there any routers?* |
| 134 | Router Advertisement (RA) | *I am a router; here is the prefix, the MTU, and how to configure* |
| 135 | Neighbor Solicitation (NS) | *Who has this address?* — **the ARP request** |
| 136 | Neighbor Advertisement (NA) | *I do, at this MAC* — **the ARP reply** |
| 137 | Redirect | *There is a better first hop for that destination* |

NS/NA replace ARP. RS/RA replace nothing in IPv4 — the equivalent function is DHCP,
or manual configuration, and consolidating it into the neighbour protocol is one of
IPv6's larger structural changes (Chapter 29).

## Solicited-node multicast

The cleverest part, and the one worth understanding in detail because it is the
mechanism by which NDP avoids ARP's broadcast cost.

An ARP request interrupts **every** station on the segment. In a broadcast domain of
500 hosts, 499 receive the frame, examine it, find it irrelevant, and discard it. That
is 499 interrupts to reach one host.

NDP instead sends the solicitation to a **solicited-node multicast address**, derived
from the target address by a fixed rule:

```
   ff02::1:ff  ++  the low-order 24 bits of the target address
```

So a host at `2001:db8::1:2:3:4` joins `ff02::1:ff02:0304`.

Every IPv6 interface automatically joins the solicited-node group for each of its
addresses. When a host needs to resolve an address, it computes that group and sends
there.

The multicast IPv6 address maps to a multicast **MAC** address by an equally fixed
rule — `33:33:` followed by the low 32 bits — so the Ethernet frame carries a
multicast destination that **network interface hardware filters**. A station whose
low-24 bits do not match discards the frame *in silicon*, without interrupting the
CPU.

The result:

> **An ARP request costs every host on the segment a CPU interrupt. A neighbour
> solicitation costs only the target — and, rarely, a host that happens to share the
> same low 24 bits.**

The collision probability is 1 in 2²⁴, about one in sixteen million, so in practice
exactly one host is disturbed. On a large segment this is the difference between
broadcast traffic that scales with host count and broadcast traffic that does not.

## A resolution, traced

Host A (`2001:db8::a`, `aa:aa:aa:aa:aa:aa`) resolves `2001:db8::1:2:3:4`.

**Neighbor Solicitation:**

| Layer | Field | Value |
|---|---|---|
| Ethernet | Destination | `33:33:ff:02:03:04` ← multicast, hardware-filtered |
| Ethernet | EtherType | `0x86DD` (IPv6) |
| IPv6 | Source | `fe80::…` (A's link-local) |
| IPv6 | Destination | `ff02::1:ff02:0304` |
| IPv6 | Hop limit | **255** |
| ICMPv6 | Type | 135 |
| ICMPv6 | Target | `2001:db8::1:2:3:4` |
| Option | Source link-layer addr | `aa:aa:aa:aa:aa:aa` |

**Neighbor Advertisement:**

| Layer | Field | Value |
|---|---|---|
| IPv6 | Destination | A's link-local (unicast) |
| IPv6 | Hop limit | **255** |
| ICMPv6 | Type | 136 |
| ICMPv6 | Target | `2001:db8::1:2:3:4` |
| Flags | Router / Solicited / Override | |
| Option | Target link-layer addr | the answer |

**The hop limit of 255 is a security mechanism**, and an elegant one. Every NDP
message is sent with hop limit 255 — the maximum — and a receiver **discards any NDP
message arriving with a hop limit below 255**.

Since every router decrements the hop limit, a message that has crossed even one
router cannot arrive at 255. So the check proves the sender is **on the same link**,
using nothing but a field that already existed. It is called the **GTSM** (Generalized
TTL Security Mechanism, RFC 5082) and the same trick protects BGP sessions (Chapter 32
§32.2).

It does not authenticate — an attacker on the same link still passes — but it
eliminates every off-link attacker for free.

## Duplicate Address Detection

Mandatory in IPv6, and performed **before any address may be used**.

On configuring an address, a host sends a neighbour solicitation for **its own
address**, from the unspecified source `::`. If anything replies, the address is a
duplicate and the host must not use it.

This is why an IPv6 interface briefly shows `tentative`:

```
$ ip -6 addr show dev eth0
    inet6 2001:db8::a/64 scope global tentative
```

A **permanently** `dupaddrdetectfailed` address means something else on the link holds
it. In IPv4 the same condition produces mysterious intermittent failures for weeks; in
IPv6 the stack tells you at configuration time. This is a genuine operational
improvement, and it exists because the designers had spent a decade diagnosing IPv4
duplicates.

## Neighbour Unreachability Detection

The other genuine improvement.

A host tracks each neighbour's state and, crucially, accepts **upper-layer
confirmation** as evidence of reachability. If TCP is receiving acknowledgements from
a neighbour, that neighbour is demonstrably alive, and no probe is needed.

```
   REACHABLE ──(timer)──▶ STALE ──(packet sent)──▶ DELAY
        ▲                                            │
        │                                            │ (no upper-layer
   (upper-layer or NA confirmation)                  │  confirmation)
        │                                            ▼
        └────────────────── PROBE ◀───────────────── │
                              │
                              │ (no response after retries)
                              ▼
                          unreachable → try another router
```

The consequence: **a host whose default router fails will detect it and switch to
another**, with no first-hop redundancy protocol at all. IPv4 needs VRRP or HSRP
(Chapter 56 §56.2) to achieve the same thing, because ARP has no way to notice that a
cached mapping now points at a dead router.

This is the answer to "IPv6 is just IPv4 with bigger addresses". The address size is
the headline; the protocol restructuring is the substance.

## Router Advertisements

RAs do considerably more than announce a router. A single RA carries:

- **Prefix information** — the /64 for the link, and whether to autoconfigure from it
- **Router lifetime** — how long to treat this as a default router (0 means *stop*)
- **MTU** — the link MTU, so hosts do not have to discover it
- **Flags** — `M` (use DHCPv6 for addresses), `O` (use DHCPv6 for other information)
- **DNS servers** (RFC 8106) — added later, closing the last gap that forced DHCPv6

That is essentially what DHCP delivers in IPv4, arriving unsolicited from the router.
Chapter 29 covers the consequences, including the **rogue RA** problem: since a host
believes any RA it receives, one misconfigured device advertising itself as a router
can black-hole a whole segment. **RA Guard** is the switch feature that filters RAs on
ports where no router should exist, and it is to IPv6 what DAI is to IPv4.

## SEND, and why nobody uses it

**SEcure Neighbor Discovery** (RFC 3971) fixes the authentication gap properly, using
**cryptographically generated addresses** — the interface identifier is a hash of a
public key, so a host can prove it owns the address it claims by signing with the
corresponding private key. No PKI is required; the address *is* the credential.

It is elegant, and deployment is essentially zero. The reasons are worth noting
because they recur:

- Requires certificate infrastructure for the router-authorisation part
- Poor support in operating systems and network equipment
- The switch-based defences (RA Guard, ND Inspection) address most of the practical
  risk with none of the complexity
- The threat model — an attacker already on your link — is one many operators accept

**A better mechanism lost to a good-enough one deployed at a different layer.** The
same pattern appears with IPsec (Chapter 61), which was designed as the universal
answer and was largely displaced by TLS.

## The commands

```
ip -6 neigh                        # Linux, the neighbour cache
ip -6 neigh flush all              # clear it
ip -6 addr show                    # see tentative / dadfailed states
ndp -a                             # macOS / BSD
netsh interface ipv6 show neighbors     # Windows
show ipv6 neighbors                # Cisco
rdisc6 eth0                        # solicit and dump router advertisements
ping6 ff02::1%eth0                 # all nodes on the link
ping6 ff02::2%eth0                 # all routers on the link
```

The last two have no IPv4 equivalent worth having. `ping6 ff02::2%eth0` asks every
router on the link to identify itself — an instant answer to *"what is routing this
segment?"* that in IPv4 requires guesswork or a capture.

## What breaks here

**`dadfailed`.** A duplicate address. IPv6 detected at configuration time what IPv4
would have let you discover through weeks of intermittent failures.

**Rogue RAs.** A device advertising itself as a router — often a Windows machine with
internet connection sharing enabled, or a misconfigured hypervisor. Hosts configure
addresses from a prefix that routes nowhere. **RA Guard** on access ports.

**ICMPv6 filtered.** Someone applied IPv4 firewall habits — where blocking ICMP is
merely unwise — to IPv6, where it is fatal. **Blocking ICMPv6 breaks IPv6
entirely**: no NDP, no address resolution, no router discovery, no path MTU discovery.
RFC 4890 specifies exactly which types must be permitted.

**Neighbour cache exhaustion.** A /64 contains 2⁶⁴ addresses; scanning it makes a
router attempt resolution for each, exhausting its neighbour table. Mitigated by rate
limiting and by RFC 6164's /127 point-to-point links.

**Address resolution working, connectivity not.** Link-local NDP succeeds while
global routing fails, so `ping6 fe80::…%eth0` works and `ping6` to a global address
does not. This is a normal and informative intermediate state, and it localises the
fault above the link immediately.

> **Network+ note.** Objective 1.4 expects NDP as ARP's IPv6 counterpart and
> objective 2.3 expects SLAAC. Worth over-learning: **NDP uses ICMPv6, not a separate
> EtherType**; **NS/NA replace ARP request/reply**; **multicast, not broadcast**; and
> **blocking ICMPv6 breaks IPv6**. The last is examined and is also the single most
> common real-world IPv6 deployment mistake.
