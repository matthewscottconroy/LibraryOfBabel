# 28.3 SLAAC, DHCPv6 and Neighbor Discovery

IPv4 hosts get their addresses from DHCP or from a person. IPv6 offers three mechanisms,
they can be combined, and the resulting matrix is the single most confusing part of IPv6
operations.

This section sorts it out. NDP itself was covered in Chapter 18 §18.4; here the concern
is address **assignment**.

## The three mechanisms

| Mechanism | Address from | Also provides |
|---|---|---|
| **SLAAC** | the host computes it from a router's prefix | prefix, gateway, MTU, (DNS via RDNSS) |
| **DHCPv6 stateful** | a server assigns it | address, DNS, domain, everything |
| **DHCPv6 stateless** | SLAAC | **only** DNS and other options |
| Static | a person | everything |

They are not alternatives so much as ingredients, and most real deployments use two
at once.

## SLAAC

**StateLess Address AutoConfiguration**, RFC 4862. The mechanism that has no IPv4
equivalent and is IPv6's most distinctive feature.

**The sequence, on an interface coming up:**

```
   1. Generate a link-local address        fe80::[interface ID]
   2. Duplicate Address Detection          NS to its own address; silence = unique
   3. Send a Router Solicitation           to ff02::2 (all routers)
   4. Receive a Router Advertisement       carrying the /64 prefix
   5. Form a global address                prefix + interface ID
   6. DAD on the global address
   7. Install the default route            = the RA's source (a link-local address)
```

Steps 1, 2 and 7 need no server of any kind. The host has a working link-local
address and a default gateway from a router that was not configured to serve anybody.

> **A host with no DHCP server, no configuration and no administrator gets a working
> global address and a default route.** That is SLAAC, and it is why IPv6 networks can be
> brought up with startlingly little.

### Forming the interface ID

**The original method — EUI-64**, deriving 64 bits from the 48-bit MAC address:

```
   MAC:            00:1b:21:3c:4d:5e

   1. Split in half, insert ff:fe:
                   00:1b:21 : ff:fe : 3c:4d:5e

   2. Flip the 7th bit of the first byte (the U/L bit):
                   00 = 00000000  →  00000010 = 02

   3. Result:      021b:21ff:fe3c:4d5e

   Address:        fe80::21b:21ff:fe3c:4d5e
```

`ff:fe` in the middle of an interface ID is the EUI-64 signature, and it is
instantly recognisable in a capture.

**And it is a privacy disaster.** The MAC address is embedded, so **the low 64 bits of a
host's address are constant wherever it goes.** A laptop's addresses at home, at work and
in a café share the same interface ID, which makes the device trivially trackable across
networks — a capability IPv4 never gave anyone.

### Privacy addresses

**RFC 4941** (originally RFC 3041): generate a **random** interface ID instead, and
rotate it periodically — typically daily, with old addresses kept valid for a week so
existing connections survive.

**Now the default on every major operating system.** A modern host typically has several
addresses at once:

```
$ ip -6 addr show eth0
    inet6 2001:db8:1:1:a1b2:c3d4:e5f6:7890/64 scope global temporary dynamic
    inet6 2001:db8:1:1:5c2e:9f11:3a08:bd44/64 scope global dynamic mngtmpaddr
    inet6 fe80::21b:21ff:fe3c:4d5e/64 scope link
```

**Three addresses, and this is normal.** The temporary one is used for outbound
connections; the stable one for inbound; the link-local for NDP.

**RFC 7217** adds *stable-privacy* addresses: stable **per network** but different on
each network — so a server keeps a consistent address while remaining untrackable across
networks. The best of both, and increasingly the default.

**The operational consequence:** *"which address is this host using?"* has no single
answer, and firewall rules or ACLs written against a specific address will break when it
rotates. Filter on the /64, not on the address.

## Router Advertisements

The messages SLAAC depends on, and they carry more than a prefix.

| Field | Purpose |
|---|---|
| Prefix Information | the /64, and whether to autoconfigure from it |
| **Router Lifetime** | how long to use this as a default router — **0 means "not a router"** |
| MTU | the link MTU, so hosts need not discover it |
| **M flag** | *Managed* — use **DHCPv6 for addresses** |
| **O flag** | *Other* — use **DHCPv6 for other information** (DNS) |
| **A flag** (per prefix) | *Autonomous* — use this prefix for SLAAC |
| RDNSS / DNSSL | **DNS servers** and search domains (RFC 8106) |

### The flag matrix

This is the part that confuses everyone, and it is worth a table:

| M | O | A | Host behaviour |
|---|---|---|---|
| 0 | 0 | 1 | **Pure SLAAC.** Address from the prefix. **DNS only if RDNSS is present.** |
| 0 | 1 | 1 | **SLAAC + stateless DHCPv6.** Address from the prefix, DNS from DHCPv6. **Very common.** |
| 1 | 0 | 0 | **Stateful DHCPv6.** Everything from the server, like IPv4. |
| 1 | 1 | 0 | Stateful DHCPv6 for everything. |
| 0 | 0 | 0 | **No address configuration.** A router announcing itself as a gateway only. |

**The historically important gap:** for years, **SLAAC had no way to convey DNS
servers.** RA carried the prefix and the gateway and nothing else, so a pure-SLAAC network
gave hosts a working address and no name resolution — which meant nothing worked.

The result was that DHCPv6 was required even when SLAAC handled addressing, purely
for DNS. RFC 8106's **RDNSS option** closed the gap, and support is now good — but not
universal, and Android has never implemented DHCPv6 at all, on the deliberate
position that SLAAC plus RDNSS is the right design.

**Which produces a real operational trap:** a network using stateful DHCPv6 works for
Windows and Linux and **silently fails for Android devices**. If phones cannot get IPv6
on your network, this is why.

## DHCPv6

**RFC 8415.** Similar to DHCPv4 in purpose and different in several examinable details.

| | DHCPv4 | DHCPv6 |
|---|---|---|
| Server port | 67 | **547** |
| Client port | 68 | **546** |
| Discovery | broadcast `255.255.255.255` | **multicast `ff02::1:2`** |
| Client identity | MAC address | **DUID** |
| Messages | DISCOVER/OFFER/REQUEST/ACK | **SOLICIT/ADVERTISE/REQUEST/REPLY** |
| Default gateway | **provided** | **never** — always from the RA |

**Two of those matter a great deal.**

**DHCPv6 never provides a default gateway.** Not an omission — a deliberate decision, on
the reasoning that the router is the authority on whether it is a router, and it says so
in its RA. **So RAs are always required**, even in a fully stateful deployment. A network
with DHCPv6 and no RAs gives hosts addresses and no route.

**DUID instead of MAC.** A **DHCP Unique Identifier**, generated once per *machine* and
intended to be stable across interfaces and reinstalls. In principle better; in practice
it means a DHCPv6 reservation is keyed on something you cannot read off a label, and
a machine that is reimaged may present a new DUID and get a different address.

**IPv4 reservations are keyed on the MAC address, which is printed on the device. IPv6
reservations are keyed on a value you must first ask the machine for.** This is a genuine
operational regression and it is a common complaint.

## Prefix delegation

**DHCPv6-PD**, and it has no IPv4 equivalent worth the name.

A router asks its upstream not for an address but for a **prefix**:

```
   ISP ──── /56 delegated ────▶ home router
                                  │
                                  ├── 2001:db8:abcd:00::/64   LAN
                                  ├── 2001:db8:abcd:01::/64   guest
                                  └── 2001:db8:abcd:02::/64   IoT
```

**The home router receives a /56 and subnets it.** Every device on every internal segment
gets a **globally routable address**, with no NAT anywhere.

This is what IPv6 is actually for, and it is worth pausing on. The IPv4 home is one
address with NAT hiding everything behind it. The IPv6 home is **256 subnets of globally
addressable devices**. Every peer-to-peer application, every inbound connection, every
protocol broken by NAT (Chapter 21 §21.4) simply works.

What replaces NAT's accidental security is the firewall, which is where it belonged
all along — and the default on every consumer IPv6 router is to permit outbound and deny
inbound, which gives the same protection without the address mangling.

## Choosing a deployment model

| Situation | Use |
|---|---|
| Home, small office | **SLAAC + RDNSS**, prefix delegated from the ISP |
| Enterprise wanting IPv4-like control | **Stateful DHCPv6** — but see the Android warning |
| Enterprise wanting simplicity | **SLAAC + stateless DHCPv6** (M=0, O=1) |
| Servers | **Static**, always. You want a known address in DNS. |
| Point-to-point links | Static /127 (RFC 6164) or link-local only |

The most common enterprise choice is SLAAC + stateless DHCPv6: hosts autoconfigure
addresses, DHCPv6 supplies DNS and domain search, and everything works including Android.

**Servers are always static.** The same as IPv4, for the same reason.

## Diagnosing it

```bash
ip -6 addr show                 # all addresses, and their states
ip -6 route show                # is there a default route?
rdisc6 eth0                     # dump the RAs in full — the key tool
ip -6 neigh                     # the neighbour cache
ping6 ff02::2%eth0              # which routers are on this link?
ping6 ff02::1%eth0              # which hosts?
sysctl net.ipv6.conf.eth0.accept_ra
```

**`rdisc6` is the tool to reach for.** It shows the M and O flags, the prefix, the
lifetimes and the DNS options — which is exactly the information that determines what a
host will do, and it is otherwise invisible.

**The diagnostic sequence:**

| Observation | Conclusion |
|---|---|
| Only `fe80::` | **No RA received.** Router not sending, or `accept_ra` off. |
| Global address, no default route | RA received with **router lifetime 0** |
| Address and route, no DNS | **No RDNSS, and DHCPv6 not configured or not supported** |
| `dadfailed` | Duplicate address on the link |
| Works on Windows, fails on Android | **Stateful DHCPv6** — Android does not implement it |

## What breaks here

**No RA.** No global address, no route. `rdisc6` shows nothing. Check the router's RA
configuration and the host's `accept_ra`.

**A rogue RA.** Any device can send one, and hosts believe it. Usually a Windows machine
with connection sharing, or a hypervisor. **RA Guard** on access ports (Chapter 18
§18.4).

**Addresses but no DNS.** The RDNSS gap. Add RDNSS, or configure stateless DHCPv6.

**Android devices without IPv6.** Stateful DHCPv6. Use SLAAC.

**ACLs that stop matching.** Privacy addresses rotated. **Filter on the /64.**

**A DHCPv6 reservation that does not work.** DUID changed after a reimage.

**Everything working, then IPv6 breaking after a firewall change.** ICMPv6 filtered —
**which breaks IPv6 entirely** (Chapter 18 §18.4). RFC 4890.

> **Network+ note.** Objective 2.3 expects SLAAC and objective 1.8 expects IPv6
> configuration. Over-learn: **SLAAC forms an address from the RA's prefix plus an
> interface ID**; **EUI-64 inserts `ff:fe` and flips the U/L bit**; **DHCPv6 uses ports
> 546/547**; and — the most examined and most operationally important — **DHCPv6 never
> supplies a default gateway; it always comes from the Router Advertisement.**
