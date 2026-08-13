# Lab 07 — Routing Between Networks

**Corresponds to:** Chapters 29, 30, 31, 33, 34
**Week:** 7
**Time:** 120 minutes

---

## Objectives

- Build three routed networks and make them reach each other with static routes.
- Read a routing table on two platforms and predict, correctly, which route wins.
- Demonstrate longest-prefix match with a deliberately overlapping table.
- Observe a static route black-holing traffic, and fix it with a floating route.
- Watch NAT translate, and read the translation table.
- Diagnose the two classic routing symptoms from the outside.

---

## You will need

- Three routers, or three Linux hosts with forwarding enabled acting as routers,
  or a Packet Tracer/GNS3 topology.
- At least one host on each of three subnets.
- `netcalc.py` from [tools/](../tools/).

**Fallback:** Linux hosts make excellent routers for this lab and cost nothing:

```bash
sudo sysctl -w net.ipv4.ip_forward=1
```

The `ip route` syntax is arguably clearer than a vendor CLI for learning
purposes, and Part 2 asks you to read both.

---

## Procedure

### Part 1 — Three networks

**1.** Build this topology:

```
  HostA ---- R1 ----/30---- R2 ----/30---- R3 ---- HostC
 10.1.1.0/24    10.1.1.0/24      HostB       10.3.3.0/24
                      \        10.2.2.0/24
                       192.0.2.0/30    198.51.100.0/30
```

Concretely: `10.1.1.0/24` behind R1, `10.2.2.0/24` behind R2, `10.3.3.0/24`
behind R3, with `/30` (or `/31`) links R1–R2 and R2–R3.

**2.** Address everything. Verify each host can ping its own gateway, and each
router can ping its directly-connected neighbours.

**3.** Now ping HostA to HostC. Record the result and the exact error.

**4.** Examine R1's routing table:

```bash
ip route            # Linux
show ip route       # Cisco-style
```

Explain the failure from the table.

---

### Part 2 — Static routes

**5.** Add the routes needed for full reachability. Work out which routers need
which routes **before typing**, and count them.

```bash
# Linux
sudo ip route add 10.3.3.0/24 via <next-hop>
```

```
! Cisco-style
ip route 10.3.3.0 255.255.255.0 <next-hop>
```

**6.** Verify A to C. Then verify **C to A** — and note that these require
different routes, which is where students most often stop one short.

**7.** Count the total route statements you added. Now compute, from Chapter 30
§30.4's arithmetic, how many would be needed for ten such networks.

**8.** Read both a Linux and a vendor routing table for the same router. For each
entry, identify: the destination prefix, the next hop, the exit interface, and
how the route was learned (connected, static, dynamic).

---

### Part 3 — Longest-prefix match

**9.** On R2, deliberately add overlapping routes:

```bash
sudo ip route add 10.0.0.0/8 via <R1-side>
sudo ip route add 10.3.0.0/16 via <R3-side>
sudo ip route add 10.3.3.0/24 via <R3-side>
```

**10.** **Predict**, before testing, the next hop for each of:
`10.3.3.7`, `10.3.9.1`, `10.7.0.1`, `10.3.3.255`.

**11.** Verify:

```bash
ip route get 10.3.3.7
ip route get 10.3.9.1
ip route get 10.7.0.1
```

**12.** Now add a default route and repeat for an address matching nothing else.
Confirm that `0.0.0.0/0` loses to every other match.

---

### Part 4 — Failure and floating routes

**13.** With A-to-C working, **shut down** the R2–R3 link (unplug it, or
`ip link set <iface> down` on R2's side only, so R2's interface stays up).

**14.** Ping A to C continuously and record what happens. Look at R2's table —
is the static route still there?

**15.** Now build a redundant path. Add a second R2–R3 link on a different
subnet, with a **floating static route**:

```bash
# Linux uses metric; higher is worse
sudo ip route add 10.3.3.0/24 via <backup-next-hop> metric 200
```

```
! Cisco-style: administrative distance 200
ip route 10.3.3.0 255.255.255.0 <backup-next-hop> 200
```

**16.** Verify the backup route is **not** in the forwarding table while the
primary is up. Then fail the primary and record the failover time.

**17.** Now the important case: fail the link **beyond** R3 — so that R2's
interface stays up but the path is broken. Does the floating route activate?
Record and explain.

---

### Part 5 — NAT

**18.** Configure R1 to NAT `10.1.1.0/24` behind its uplink address:

```bash
sudo iptables -t nat -A POSTROUTING -s 10.1.1.0/24 -o <uplink> -j MASQUERADE
```

**19.** From HostA, open several connections to a host beyond R1. Read the
translation table:

```bash
sudo conntrack -L
# or
cat /proc/net/nf_conntrack
```

**20.** Record, for three connections: the internal address and port, the
translated address and port, and the destination. Confirm that two connections
from different hosts to the same destination got different translated ports.

**21.** From outside, attempt to open a connection **inbound** to HostA. Record
what happens and why.

---

### Part 6 — ICMP and the diagnostic path

**22.** From HostA, traceroute to HostC. Record every hop.

**23.** On R2, rate-limit or drop ICMP time-exceeded:

```bash
sudo iptables -A OUTPUT -p icmp --icmp-type time-exceeded -j DROP
```

Traceroute again. Record what changed and what did **not**.

**24.** Restore, then set an MTU of 1400 on the R2–R3 link and block ICMP
fragmentation-needed on R2:

```bash
sudo ip link set <iface> mtu 1400
sudo iptables -A OUTPUT -p icmp --icmp-type fragmentation-needed -j DROP
```

**25.** From HostA: `ping -c 2 -s 1000 <HostC>` then `ping -c 2 -s 1400 -M do <HostC>`.
Then try a large TCP transfer. Record all three.

---

## Expected observations

- **Step 3 fails** with "Network is unreachable" generated by R1 — R1 has no route
  to `10.3.3.0/24`. Note that this is a *different* message from an ARP failure.
- **Step 6:** return routes are required. A one-way route produces one-way
  connectivity, which manifests as a total failure because the reply cannot get
  back.
- **Step 11:** `10.3.3.7` takes the /24; `10.3.9.1` takes the /16; `10.7.0.1`
  takes the /8. Longest prefix wins regardless of the order you added them.
- **Step 14: traffic is black-holed silently.** The static route stays in the
  table because the *local* interface is still up. R2 cheerfully forwards toward a
  dead neighbour. Nothing tells anyone.
- **Step 17: the floating route does not activate**, because the primary never
  withdrew. This is the caveat in Chapter 30 §30.3 and it is the difference
  between a design that works in the lab and one that works at 3 a.m.
- **Step 20:** two internal hosts choosing the same source port receive different
  translated ports.
- **Step 21: inbound fails** — there is no translation entry, so the packet has
  nowhere to go.
- **Step 23: asterisks appear at hop R2, and every hop beyond it still responds.**
  The path is fine. This is the artefact that generates incorrect escalations.
- **Step 25: small pings work, `-s 1400 -M do` fails, and the TCP transfer hangs
  after establishing.** You have built a PMTUD black hole.

---

## Break it

Integrated above — steps 13, 17, 23 and 25 are the deliberate faults. One more:

**A. Wrong default gateway on a host.** Set HostA's gateway to an address on its
subnet that nobody holds. Record the symptom: local works, remote fails
completely, and the ARP cache shows an incomplete entry for the phantom gateway.

**B. Off-by-one gateway.** Set it to a *real* address on the subnet that is not a
router. Record how this differs from A — and note that it is harder to diagnose,
because ARP succeeds.

---

## Debrief

**1.** Count the static routes you added for three networks. Extrapolate to ten
networks using Chapter 30 §30.4's arithmetic. At what number would you switch to
a dynamic protocol, and what would you weigh against the operability of the team
running it?

**2.** In step 11 you predicted three next hops. State the rule that decided each,
and explain why the order in which the routes were added is irrelevant.

**3.** Describe precisely what happened in step 14 and why the router did not
notice. Then explain what the floating route in step 15 fixed, and what step 17
demonstrated that it did **not** fix. Name the mechanism that closes that gap.

**4.** From your NAT translation table, explain how the return traffic for two
simultaneous connections from different hosts to the same server is
disambiguated. Then state what breaks about inbound connectivity and what
architectural change to the Internet that side effect produced.

**5.** In step 23, traceroute showed asterisks at one hop and normal responses
beyond. State what a technician who escalated this to a provider would have got
wrong, and give the rule that prevents the error.

**6.** In step 25 you produced a connection that establishes and then hangs. Write
the incident record for it as if it had happened in production: symptom, evidence,
cause, fix, and — the field that matters — what made it hard to find.
