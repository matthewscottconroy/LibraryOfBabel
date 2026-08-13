# 30.3 Default and Floating Routes

Two applications of the machinery just built. The default route is the most common static
route in existence; the floating static is the cheapest redundancy available, and both
depend on the arithmetic of §30.2.

## The default route, configured

Chapter 29 §29.4 covered what it is. Here it is as a configuration decision.

```
ip route 0.0.0.0 0.0.0.0 203.0.113.1                  ! IOS
ip route add default via 203.0.113.1                  ! Linux
ipv6 route ::/0 2001:db8:ffff::1                      ! IOS, IPv6
```

**One line, and every destination the router does not otherwise know about is handled.**

**When it is right:** a **single-homed** site. There is exactly one way out; there is
nothing to decide; a routing protocol would learn one fact you already know.

**When it is wrong:** anywhere with more than one exit and a reason to choose between
them. Then you need either several statics with different administrative distances (§30.3
below), or a protocol.

### Propagating it

A default route configured on the edge router is useless to the routers behind it unless
they hear about it. Every protocol has a way to say *"I am the way out"*:

```
router ospf 1
 default-information originate            ! advertise my default into OSPF
 default-information originate always     ! even if I do not have one myself

router eigrp 100
 redistribute static                      ! or: network 0.0.0.0

router rip
 default-information originate
```

**The `always` keyword deserves care.** Without it, the router advertises a default only
while it *has* one — so if its own upstream link fails and its default is withdrawn, the
advertisement stops and the rest of the network learns to stop sending traffic there.
**That is the behaviour you want.**

With `always`, the router advertises a default unconditionally, **including when it has no
way out itself** — so the whole network keeps sending it traffic that it then drops.
**`always` converts a working failover into a black hole**, and it is used correctly only
where the router genuinely is the exit regardless of what it can currently see.

## Floating static routes

**The cheapest redundancy in networking**, and it is one line.

**The problem:**

```
                    ┌─── primary: MPLS ────┐
   Branch router ───┤                      ├─── Head office
                    └─── backup: Internet ─┘
```

You want the MPLS link normally, and the Internet link **only when MPLS fails**.

**The naive attempt:**

```
ip route 0.0.0.0 0.0.0.0 10.1.1.2          ! MPLS
ip route 0.0.0.0 0.0.0.0 203.0.113.1       ! Internet
```

**Both have AD 1.** They tie on prefix length and on administrative distance, so **both
install** and traffic is **split** between them (Chapter 29 §29.3). Half your traffic
takes the backup path all the time, which is not what was asked for — and worse, it is
intermittent-looking rather than obviously broken.

**The floating static:**

```
ip route 0.0.0.0 0.0.0.0 10.1.1.2               ! MPLS,     AD 1  (default)
ip route 0.0.0.0 0.0.0.0 203.0.113.1 200        ! Internet, AD 200
```

**Now only the MPLS route installs**, because AD 1 beats AD 200. The Internet route sits
in the configuration, **not in the routing table**, doing nothing.

**When the MPLS route is withdrawn** — because its interface went down, or its next hop
became unreachable — the router has no route with AD 1, so **the AD 200 route installs
automatically.** When MPLS returns, the AD 1 route reappears and displaces it again.

**"Floating" because it floats above the table until it is needed.**

```
   Normal:                          MPLS failed:
   S*  0.0.0.0/0 [1/0] via 10.1.1.2      S*  0.0.0.0/0 [200/0] via 203.0.113.1
       (the AD 200 route is invisible)       (the AD 1 route is gone)
```

**Choosing the AD:** any value worse than the primary's and better than 255. Convention
is to leave a gap — 200 or 250 — so that another route can be inserted between them later
without renumbering. If the primary is learned by OSPF (AD 110), the floating static must
be **above 110**, which is a detail people miss when the primary is dynamic rather than
static.

## The failure this does not handle

**The important limitation**, and it is the reason floating statics are cheap rather than
good.

**A static route is withdrawn when its next hop becomes unreachable.** In practice that
means **when the local interface goes down.**

**But a link can fail without the local interface going down**, and this is common:

```
   Branch ──── switch ──── [ the carrier's network ] ──── Head office
              ↑                       ↑
      interface stays UP        the failure is here
```

If the branch router connects to the carrier through a switch or a media converter — which
it usually does — then **the router's interface stays up when the carrier's network
fails.** The next hop appears reachable. The static route stays in the table. **Traffic
continues to be sent into a black hole, and the floating static never activates.**

**This is not an edge case.** Ethernet handoffs from carriers are the norm, and the
symptom — total outage with every interface showing "up" — is one of the most confusing in
this book.

### The fixes

**IP SLA with object tracking** (Cisco; equivalents elsewhere):

```
ip sla 1
 icmp-echo 10.1.1.2 source-interface GigabitEthernet0/0
 frequency 5
ip sla schedule 1 life forever start-time now
!
track 1 ip sla 1 reachability
!
ip route 0.0.0.0 0.0.0.0 10.1.1.2 track 1
ip route 0.0.0.0 0.0.0.0 203.0.113.1 200
```

**Now the primary route is conditional on the far end actually answering.** The router
pings the next hop every five seconds; if it stops responding, `track 1` goes down, the
route is withdrawn, and the floating static takes over. Detection in about fifteen
seconds.

**Better targets than the next hop:** ping something *beyond* it — a well-known address
on the far side — so that a failure anywhere in the carrier's network is detected, not
only a failure of the immediate neighbour.

**BFD** (Bidirectional Forwarding Detection, RFC 5880) is the proper answer where both
ends support it: sub-second detection, negotiated between the two routers, and usable by
routing protocols as well as by static routes. Chapter 31 §31.4.

**A routing protocol** across the link is the other answer: if the protocol's adjacency
depends on hearing from the far end, the adjacency dies when the far end becomes
unreachable, and the route goes with it. **This is one of the strongest arguments for
running a protocol even on a two-router topology** — not for path selection, but for
failure detection.

## Combining defaults with specifics

The pattern that appears in most branch designs:

```
ip route 0.0.0.0 0.0.0.0 203.0.113.1                 ! Internet, via the firewall
ip route 10.0.0.0 255.0.0.0 10.1.1.2                 ! all corporate, via MPLS
ip route 10.99.0.0 255.255.0.0 203.0.113.1           ! except this, via Internet
```

**Longest-prefix match resolves it** with no further mechanism (Chapter 29 §29.3):

| Destination | Matches | Wins | Path |
|---|---|---|---|
| `8.8.8.8` | /0 | /0 | Internet |
| `10.5.1.1` | /0, /8 | **/8** | MPLS |
| `10.99.4.1` | /0, /8, /16 | **/16** | Internet |

**Three lines expressing a policy** — corporate traffic over MPLS, Internet traffic over
the Internet, and one corporate range that is reached over the Internet because it is
hosted in a cloud. **This is why longest-prefix match's exception behaviour matters in
practice**, and it is a very common real configuration.

## Verifying a floating static

**Test the failover, deliberately, before you need it.**

```bash
# What is in the table now?
show ip route 0.0.0.0

# Take the primary down
interface GigabitEthernet0/0
 shutdown

# Did the backup install?
show ip route 0.0.0.0

# Does traffic actually work over it?
ping 10.5.0.1
traceroute 10.5.0.1

# Restore, and confirm it fails back
interface GigabitEthernet0/0
 no shutdown
show ip route 0.0.0.0
```

**Untested failover is not failover.** The most common discovery during this test is that
the backup path works for routing and fails for something else — the firewall does not
permit the traffic, NAT is configured for one path only, or the far end has no return
route over the backup. **All three are invisible until you test.**

## What breaks here

**Both defaults installed and traffic splitting.** Same AD. Add a worse one to the
backup.

**The floating static never activates.** The primary's next hop is still "reachable"
because the local interface stayed up. Add tracking or BFD.

**The floating static activates and nothing works.** The return path, the firewall, or
NAT is not configured for the backup. Test failover properly.

**`default-information originate always` black-holing the network.** The router
advertises a default it cannot honour.

**A branch with no default at all after a change.** The default was learned from a
protocol that lost its adjacency.

**Failover works, failback does not.** Some tracking configurations are asymmetric; check
both directions of the transition.

> **Network+ note.** Objective 2.2 expects default routes; objective 3.3 expects
> redundancy. Over-learn: **a default route is `0.0.0.0/0`**; **a floating static has a
> higher administrative distance and installs only when the primary is withdrawn**; and
> **a static route is only withdrawn when its next hop becomes unreachable, which an
> Ethernet handoff can hide.** The last point is the most valuable and the least
> examined.
