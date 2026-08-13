# 30.1 Configuring a Static Route

A static route is a person telling a router something it could not work out for itself.
It is the simplest way to populate a routing table, it is the right answer more often
than its reputation suggests, and it is the wrong answer in ways that are worth
understanding precisely.

## The anatomy

Every static route says the same three things:

$$\underbrace{\texttt{10.5.0.0/16}}_{\text{destination}} \quad \underbrace{\texttt{via 192.168.1.2}}_{\text{next hop}} \quad \underbrace{\texttt{dev eth0}}_{\text{interface}}$$

**Destination and one of the other two are mandatory.** Give a next hop and the router
works out the interface by looking up the next hop; give an interface only and — on a
broadcast link — you get the failure of Chapter 29 §29.4.

## Three platforms

**Cisco IOS:**

```
ip route 10.5.0.0 255.255.0.0 192.168.1.2
ip route 10.5.0.0 255.255.0.0 GigabitEthernet0/1 192.168.1.2
ip route 10.5.0.0 255.255.0.0 192.168.1.2 200          ! AD 200
ip route 10.5.0.0 255.255.0.0 Null0                    ! blackhole
ipv6 route 2001:db8:5::/48 2001:db8:1::2
```

Note the **dotted-decimal mask**, not prefix notation. IOS is one of the last places you
must still write `255.255.0.0`, and it is why Chapter 25 §25.3's conversion table remains
worth knowing.

**Linux:**

```bash
ip route add 10.5.0.0/16 via 192.168.1.2
ip route add 10.5.0.0/16 via 192.168.1.2 dev eth0 metric 100
ip route add blackhole 10.6.0.0/16
ip route add unreachable 10.7.0.0/16       # blackhole, but sends ICMP
ip route del 10.5.0.0/16
```

**Linux routes added this way do not survive a reboot.** Persistence is the
distribution's business — `/etc/network/interfaces`, a `netplan` YAML, a NetworkManager
connection, or a `systemd-networkd` `.network` file. **A route that works until the next
maintenance window and then vanishes** is one of the more annoying self-inflicted faults,
and it is entirely avoidable by writing the route where the distribution expects it.

**Windows:**

```
route add 10.5.0.0 mask 255.255.0.0 192.168.1.2 -p
```

**The `-p` is the whole trick** — without it the route is not persistent, and Windows
gives no warning.

## The next hop must be reachable

**The rule that catches everyone once.**

```
ip route 10.5.0.0 255.255.0.0 192.168.99.7
```

If `192.168.99.7` is not on a subnet this router is directly connected to, the route is
**invalid**. The router cannot build a frame for it — it has no interface on which to ARP.

**What happens depends on the platform, and the variation is itself a trap:**

| Platform | Behaviour |
|---|---|
| Cisco IOS | **Recursively resolves** the next hop against the rest of the table. If that fails, the route is not installed. |
| Linux | **Refuses**: `Error: Nexthop has invalid gateway.` |
| Some platforms | Accept it and silently blackhole |

**Recursive resolution** on IOS is worth understanding, because it makes a class of
configuration work that looks as though it should not:

```
   ip route 10.5.0.0 255.255.0.0 192.168.99.7      ← next hop is not connected
   ip route 192.168.99.0 255.255.255.0 10.0.0.2    ← but this route reaches it
```

The router resolves `192.168.99.7` through the second route, finds `10.0.0.2`, resolves
*that* against a connected route, and forwards accordingly. **It works, and it costs a
second lookup**, and it breaks in a confusing way when the intermediate route disappears.

**Prefer a directly-connected next hop.** Recursive routes are legitimate — BGP depends
on them (Chapter 32 §32.2) — and in a static configuration they are usually a sign that
the design has drifted.

## The three forms, and when each is right

### Next hop only

```
ip route 10.5.0.0 255.255.0.0 192.168.1.2
```

**The default choice on Ethernet.** The router ARPs for `192.168.1.2` and sends frames
there. Unambiguous.

### Interface only

```
ip route 10.5.0.0 255.255.0.0 Serial0/0/0
```

**Correct on a point-to-point link and wrong on Ethernet.** On a serial link there is
exactly one possible neighbour, so no resolution is needed. On Ethernet the router
concludes the entire destination range is directly connected and **ARPs for every
address in it** (Chapter 29 §29.4).

### Both

```
ip route 10.5.0.0 255.255.0.0 GigabitEthernet0/1 192.168.1.2
```

**The most explicit and the safest.** No recursion, no ambiguity about which interface,
and the route is withdrawn automatically if that interface goes down — which the next-hop
only form does *not* guarantee, because the next hop may still be resolvable another way.

**Recommended for anything that matters.**

## Blackhole routes

A route that deliberately discards:

```
ip route 10.6.0.0 255.255.0.0 Null0            # IOS
ip route add blackhole 10.6.0.0/16             # Linux, silent
ip route add unreachable 10.6.0.0/16           # Linux, sends ICMP unreachable
```

**Silent versus ICMP matters.** A silent blackhole leaves the sender waiting for a
timeout; an `unreachable` route tells them immediately. **For legitimate traffic, tell
them.** For hostile traffic, silence is preferable — an ICMP reply confirms the address
exists and costs you bandwidth.

**Three real uses:**

**1. Remotely-triggered blackhole (RTBH).** Under a volumetric attack on one address, you
announce a /32 for it into your own network — and often to your upstream — so the traffic
is dropped at the network edge rather than saturating your access link. **You are
completing the denial of service against that one address in order to save everything
else**, which is a genuinely uncomfortable trade and is standard practice.

**2. Anchoring an aggregate.** To advertise `10.1.0.0/16` when only parts are live, hold
a `Null0` route for the /16. It gives the router something to advertise, and traffic for
the unused parts is discarded locally rather than following a default route back
outward — which would create a loop.

**3. Preventing loops in a default-route hierarchy.** A branch with a default route
pointing at head office, and head office with a default pointing at the Internet, will
loop traffic for an unrouted internal prefix back and forth until the TTL expires. A
`Null0` for the organisation's own aggregate at head office stops it.

## Deletion, and the danger of it

```
no ip route 10.5.0.0 255.255.0.0 192.168.1.2      # IOS
ip route del 10.5.0.0/16                          # Linux
```

**Deleting the route your management session is using disconnects you**, and if it was
the only route to the device, permanently — until someone attends with a console cable.

**Two habits worth adopting before you need them:**

- **`reload in 10`** on IOS before a risky change: the router will reboot to its saved
  configuration in ten minutes unless you cancel with `reload cancel`. If you lock
  yourself out, the change reverts by itself.
- **Out-of-band management** — a console server or a separate management network — so the
  device is reachable when the production path is not. Chapter 53 §53.4.

## Verifying

```bash
# Is it in the table?
show ip route 10.5.0.0
ip route show 10.5.0.0/16

# Would traffic actually take it?
ip route get 10.5.0.1
show ip cef 10.5.0.1

# Does it work end to end?
ping 10.5.0.1
traceroute 10.5.0.1
```

**Check the table before the ping.** If the route is absent, the ping tells you nothing
you did not already know, and the reason for its absence is the actual question.

**And check the return path.** Chapter 29 §29.1: forward and return are independent
decisions. **A static route configured on one router and not the other gives traffic that
arrives and never comes back**, and this accounts for a large share of "the static route
doesn't work" reports.

## When static is the right answer

Its reputation is poor and its reputation is unfair. Static routing is correct when:

| Situation | Why |
|---|---|
| **A stub network with one exit** | There is nothing to compute. A default route is all that exists. |
| **A default route to an ISP** | You have one upstream; a protocol would add nothing |
| **Fewer than a handful of routers** | The protocol's overhead exceeds its benefit |
| **A route that must not change** | Static is deterministic by construction |
| **Blackholes and exceptions** | No protocol expresses these naturally |
| **A last-resort backup path** | Floating statics (§30.3) |
| **Where predictability beats adaptability** | Some environments genuinely need this |

**The default route to an ISP is the most common static route in the world**, and it is
correct: a single-homed site has exactly one way out, and running BGP to learn that fact
would be absurd.

**Its virtues are real:** no protocol to run, no CPU, no bandwidth, no adjacency to
troubleshoot, no convergence behaviour to understand, and **completely deterministic** —
it does exactly what it says until a person changes it.

§30.4 covers where that stops being enough.

## What breaks here

**A route that vanished after a reboot.** Not persistent. Windows needs `-p`; Linux needs
the distribution's configuration file.

**"Invalid gateway" or a route that will not install.** The next hop is not reachable.

**A route that works and takes an odd path.** Recursive resolution through an
intermediate route.

**A route configured with an interface only, on Ethernet.** The router ARPs for
everything.

**Traffic arriving and nothing returning.** The return route is missing on the other
device.

**A route that stays in the table after the link dies.** The next-hop-only form, where
the next hop is still resolvable by another path. Use the interface-and-next-hop form.

**Locking yourself out.** Use `reload in`, and build out-of-band access before you need
it.

> **Network+ note.** Objective 2.2 expects static routing. Over-learn: **the destination,
> the next hop, and the interface**; **the next hop must be directly reachable**;
> **interface-only is for point-to-point links**; and **a static route on one router
> needs its counterpart on the other**. Expect a configuration question and a "why
> doesn't this work" scenario, which is nearly always the missing return route.
