# 29.2 Reading a Routing Table

The routing table is the router's entire knowledge of the world. Reading one fluently is
the most transferable skill in this unit, and this section works through real output on
three platforms.

## Linux

```
$ ip route
default via 192.168.1.1 dev eth0 proto dhcp metric 100
10.8.0.0/24 dev tun0 proto kernel scope link src 10.8.0.6
169.254.0.0/16 dev eth0 scope link metric 1000
192.168.1.0/24 dev eth0 proto kernel scope link src 192.168.1.50 metric 100
192.168.50.0/24 via 192.168.1.254 dev eth0 proto static metric 100
```

**Field by field, using the last line:**

| Part | Meaning |
|---|---|
| `192.168.50.0/24` | **the destination prefix** — what this route is for |
| `via 192.168.1.254` | **the next hop** — hand the packet to this router |
| `dev eth0` | **the outgoing interface** |
| `proto static` | how the route was learned |
| `metric 100` | preference among routes of equal prefix length |

**The distinctions worth internalising:**

**`via X` versus no `via`.** A route with `via` is **indirect** — the destination is
somewhere else and this is the next router toward it. A route **without** `via` (like
`192.168.1.0/24 dev eth0 scope link`) is **directly connected**: the destination is on
this link, so ARP for the destination itself, not for a gateway.

**This is the mechanism behind Chapter 18 §18.1's local-or-remote decision.** The
"decision" is simply which kind of route matched.

**`proto kernel`** means the route was created automatically when the interface got an
address. **You do not configure connected routes** — configuring an address configures
the route, and this is true on every platform.

**`scope link`** means the destination is reachable directly on this link.

**`src`** is the source address the host will use for traffic taking this route, which
matters on a multihomed host and is a common cause of "why is my traffic coming from the
wrong address".

**The newer command**, showing what the kernel would actually do:

```
$ ip route get 8.8.8.8
8.8.8.8 via 192.168.1.1 dev eth0 src 192.168.1.50 uid 1000
```

**`ip route get` is the tool to reach for.** It performs the real lookup — longest-prefix
match, policy rules, everything — and reports the answer. It settles arguments that
reading the table by eye does not.

## Cisco IOS

```
Router# show ip route
Codes: L - local, C - connected, S - static, R - RIP, B - BGP
       O - OSPF, IA - OSPF inter area, D - EIGRP, EX - EIGRP external
       * - candidate default

Gateway of last resort is 203.0.113.1 to network 0.0.0.0

S*    0.0.0.0/0 [1/0] via 203.0.113.1
      10.0.0.0/8 is variably subnetted, 4 subnets, 3 masks
O        10.1.0.0/16 [110/20] via 10.255.0.2, 00:14:22, GigabitEthernet0/1
D        10.2.0.0/16 [90/2195456] via 10.255.0.6, 01:02:11, GigabitEthernet0/2
C        10.255.0.0/30 is directly connected, GigabitEthernet0/1
L        10.255.0.1/32 is directly connected, GigabitEthernet0/1
B     198.51.100.0/24 [20/0] via 203.0.113.1, 2d03h
```

**The code letter is the first thing to read.** It tells you *how the router learned
this*, which is usually the first question during an incident.

**The bracketed pair `[110/20]` is the second thing**, and it is two different numbers
that people constantly conflate:

$$[\underbrace{110}_{\text{administrative distance}} / \underbrace{20}_{\text{metric}}]$$

- **Administrative distance** — how much this router **trusts the source** of the route.
  Compared **between protocols**. Lower wins.
- **Metric** — how good the path is **according to that protocol**. Compared **only
  within** the same protocol. Lower wins.

Chapter 30 §30.2 develops both. The point here: **`[90/2195456]` is not worse than
`[110/20]`** — the 90 is a *better* administrative distance, and the two metrics are in
different units and are not comparable at all.

**The `C` and `L` pair** appears for every connected interface:

| | |
|---|---|
| `C 10.255.0.0/30` | the **subnet** on that interface |
| `L 10.255.0.1/32` | the router's **own address**, as a host route |

The `/32` local route exists so the router can recognise traffic addressed to itself
without a special case, and it appears on modern IOS only — older output shows just the
`C` line.

**"Gateway of last resort"** is the default route, stated separately at the top because
it is the one people look for.

## Windows

```
> route print
IPv4 Route Table
===========================================================================
Active Routes:
Network Destination        Netmask          Gateway       Interface  Metric
          0.0.0.0          0.0.0.0      192.168.1.1    192.168.1.50     35
        127.0.0.0        255.0.0.0         On-link         127.0.0.1    331
      192.168.1.0    255.255.255.0         On-link      192.168.1.50    291
    192.168.1.255  255.255.255.255         On-link      192.168.1.50    291
        224.0.0.0        240.0.0.0         On-link      192.168.1.50    291
  255.255.255.255  255.255.255.255         On-link      192.168.1.50    291
```

**`On-link` is Windows for "directly connected"** — the same as Linux's absent `via`.

Windows lists more routes than the others because it makes explicit what other systems
handle implicitly: the broadcast address, the multicast range, and the limited broadcast
each get their own entry. **Informative rather than different** — every stack does these
things; Windows shows its working.

## What every table has in common

Whatever the platform, every route carries the same five things:

| | |
|---|---|
| **Destination prefix** | what this route is for |
| **Next hop** | where to send it — or "directly connected" |
| **Interface** | which port it leaves by |
| **Source** | how it was learned |
| **Preference** | metric, and on some platforms administrative distance |

**Learn to read those five in any format** and the platform differences become
cosmetic.

## Where routes come from

The `proto` field on Linux and the code letter on IOS answer the same question:

| Source | Linux `proto` | IOS code | Chapter |
|---|---|---|---|
| **Connected** — an interface has an address | `kernel` | `C`, `L` | — |
| **Static** — a person configured it | `static` | `S` | 30 |
| **DHCP** — supplied with the address | `dhcp` | — | 40 |
| **RIP** | `rip` / `zebra` | `R` | 31 |
| **OSPF** | `ospf` / `zebra` | `O`, `IA`, `E1`, `E2` | 31 |
| **EIGRP** | — | `D`, `EX` | 31 |
| **BGP** | `bgp` / `zebra` | `B` | 32 |
| **Redistributed** — imported from another protocol | varies | varies | 31 |

**Connected routes come first in every sense.** They need no configuration, they are the
most trusted, and without them nothing else works — a static route pointing at a next hop
that is not on a connected subnet is invalid, and the router will reject or ignore it.

## Reading a table during an incident

The questions, in the order worth asking:

**1. Is there a route to the destination at all?**

```
ip route get 203.0.113.50            # Linux — does the real lookup
show ip route 203.0.113.50           # IOS — shows the matching route
```

**"Network is unreachable" means no route matched, not even a default.**

**2. Is it the route you expected?**

If a more specific route exists that you did not intend, it wins (§29.3). If the default
is being used where a specific route should exist, the specific route is missing or was
withdrawn.

**3. Where did it come from?**

A route learned from OSPF when you configured it statically means your static route lost
on administrative distance — or was never installed.

**4. Is the next hop reachable?**

**A route whose next hop is unreachable is worse than no route**, because it matches and
then fails silently. The router must be able to reach the next hop *by some other route*
— usually a connected one — and if it cannot, the route should not be installed. Some
platforms are stricter about this than others.

**5. Does the return path exist?**

Chapter 29 §29.1's point. **Forward and return are independent.** Half of all
"intermittent" routing problems are a missing return route, and the giveaway is that
traffic reaches the destination and nothing comes back.

## The commands

```bash
# Linux
ip route                       # the table
ip route get 8.8.8.8           # what would actually happen — use this
ip -6 route                    # IPv6
ip route show table all        # every table, including policy tables
ip rule show                   # policy routing rules, checked before the table

# Cisco IOS
show ip route                        # the RIB
show ip route 203.0.113.50           # the matching route, in detail
show ip route ospf                   # one protocol only
show ip cef 203.0.113.50             # the FIB — what hardware uses
show ip protocols                    # what is running and what it advertises

# Windows
route print
Get-NetRoute                         # PowerShell
Find-NetRoute -RemoteIPAddress 8.8.8.8   # the equivalent of ip route get
```

**`ip rule show` is worth knowing about**, because policy routing rules are consulted
**before** the main table and are invisible in `ip route`. A host whose traffic goes
somewhere inexplicable, with a table that says otherwise, usually has a policy rule — VPN
clients and multihomed systems add them routinely.

## What breaks here

**"Network is unreachable".** No matching route, not even a default.

**Traffic taking an unexpected path.** A more specific route exists. Use
`ip route get` rather than reading the table.

**A configured static route that is not in the table.** Its next hop is not reachable, or
a route with better administrative distance won.

**Traffic arriving and nothing returning.** No return route. Check the *other* end's
table.

**Comparing metrics across protocols.** `[90/2195456]` and `[110/20]` are not comparable.
The first number is what decides.

**A table that looks right while forwarding does not.** RIB/FIB mismatch (§29.1). Check
the FIB.

> **Network+ note.** Objective 2.2 expects reading a routing table, and objective 5.5
> expects `route`/`ip route`. Over-learn: **directly connected versus via a next hop**;
> **the five fields every route carries**; and **administrative distance is the first
> number, metric the second, and they are not comparable.** Expect a table-reading
> question giving several routes and one destination.
