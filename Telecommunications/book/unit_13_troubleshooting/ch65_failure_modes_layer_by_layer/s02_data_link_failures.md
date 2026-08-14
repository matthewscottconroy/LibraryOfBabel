# 65.2 Data Link Failures

Layer 1 is up. Frames are not going where they should. This section is the catalogue,
and the unifying property is that almost all of these faults produce "it has an address and
cannot reach anything" or "it works for some destinations and not others."

## Wrong VLAN

The commonest Layer 2 fault, and it presents as a Layer 3 or a DHCP problem.

> A device on the wrong VLAN sees the wrong DHCP server, or none. "No IP address" is very
> often a VLAN problem, not a DHCP problem (Chapter 20 §20.3).

The symptoms, in order of how the user reports them:

| Report | Actual |
|---|---|
| **"No network"** | **169.254.x.x — no DHCP server in that VLAN** |
| **"I got an address but nothing works"** | **an address from the wrong scope, with the wrong gateway** |
| **"It worked at my old desk"** | **the two ports are in different VLANs** |
| **"The phone works and the PC doesn't"** | **voice VLAN configured, data VLAN wrong or absent** |

**The check is one command:**

```
   $ show interface GigabitEthernet1/0/14 switchport
   Name: Gi1/0/14
   Administrative Mode: static access
   Operational Mode: static access
   Access Mode VLAN: 20 (STAFF)
   Voice VLAN: 240
```

And the mismatch to look for is between the operational mode and what you expected, not
between the configuration and the documentation — the documentation is frequently the thing
that is wrong.

## Trunk problems

**Three distinct faults with three distinct signatures.**

### VLAN not allowed on the trunk

```
   switchport trunk allowed vlan 10,20,30
   ! VLAN 40 exists at both ends and is not in the list
```

> The symptom is that VLAN 40 works within each switch and not between them, which looks
> like a routing problem and is not.

And the allowed list is the thing most often forgotten when a VLAN is added — the VLAN is
created on both switches, the ports are configured, and nobody updated the trunk.

### Native VLAN mismatch

Each end tags differently, so untagged frames land in different VLANs.

| | |
|---|---|
| **Switch A native VLAN 1** | untagged frames → VLAN 1 |
| **Switch B native VLAN 99** | **the same frames → VLAN 99** |

**The symptoms:** CDP or LLDP reports a native VLAN mismatch explicitly (which is the easy
case), and spanning tree may block, and traffic in the native VLAN crosses into the wrong
one — which is Chapter 62 §62.1's double-tagging vector, arriving accidentally.

### Mode mismatch

| A | B | Result |
|---|---|---|
| access | access | **works, one VLAN only** |
| trunk | trunk | works |
| **trunk** | **access** | **the access side receives tagged frames it cannot interpret** |
| **`dynamic auto`** | **`dynamic auto`** | **neither initiates — the link stays access** |
| `dynamic desirable` | `dynamic auto` | becomes a trunk |

The `auto`/`auto` case is worth knowing because it produces a link that works for one VLAN
and silently fails for the rest, and the configuration looks correct at both ends.

## Spanning tree

**Chapter 19's mechanism, seen as a fault.**

| Symptom | Cause |
|---|---|
| **A link that is up and carries nothing** | **it is blocking, which is correct** — but is it the link you intended? |
| **The root bridge is not the one you designed** | **another switch has a lower priority** (Chapter 19 §19.3) |
| **Traffic taking an unexpected path** | **the topology is not the one on the diagram** (Chapter 53 §53.1) |
| **Periodic brief outages across the whole VLAN** | **topology change notifications** — something is flapping |
| **A device takes 30–50 seconds to work after connecting** | **no PortFast** (Chapter 19 §19.2) |
| **Total broadcast storm** | **a loop with spanning tree disabled or defeated** |

**The two checks:**

```
   $ show spanning-tree vlan 20
   Root ID    Priority    4116
              Address     00:1a:2b:3c:4d:5e
              Cost        4
              Port        25 (GigabitEthernet1/0/25)
   
   $ show spanning-tree detail | include ieee|occurr|from
   VLAN0020 is executing the ieee compatible Spanning Tree protocol
     Number of topology changes 1847 last change occurred 00:00:42 ago
             from GigabitEthernet1/0/13
```

> **The topology change counter is the most useful spanning tree diagnostic.** 1,847
> changes with the last one 42 seconds ago names the port — and a port generating topology
> changes is a port that is flapping, or a port without PortFast where devices connect and
> disconnect.

And "the root is not where it should be" is a design problem that presents as a performance
one (Chapter 56 §56.2's FHRP alignment): traffic crosses the inter-switch link twice, and
nothing is broken.

## Duplex mismatch

Chapter 66 §66.2 treats it properly. Its Layer 2 signature:

> **One side full duplex, the other half.** The full-duplex side transmits whenever it likes;
> the half-duplex side detects that as a collision.

| On the half-duplex side | On the full-duplex side |
|---|---|
| **Late collisions** | **CRC errors, runts** |
| Collisions | FCS errors |

**The symptom is performance, not failure:** the link works, small transfers are fine, and
throughput collapses under load — frequently to a few per cent of the link rate.

And it is now rare and not extinct. Auto-negotiation works; the fault occurs where one
side is forced and the other is not, because a forced side does not participate in
negotiation and the auto side falls back to half duplex.

> The rule: both sides auto, or both sides forced identically. Never one of each.

## MAC address table

| Symptom | Cause |
|---|---|
| **A device unreachable, its MAC absent from the table** | **it has not transmitted**, or it is on a different VLAN, or Layer 1 |
| **A MAC learned on the wrong port** | **a loop, a spoofing attack** (Chapter 62 §62.1), **or the device moved** |
| **A MAC flapping between two ports** | **a loop** — and the log will say so explicitly |
| **All traffic flooding** | **table exhaustion** (Chapter 62 §62.1) **or a topology change ageing the table** |

MAC flapping messages are unambiguous and are the fastest loop diagnosis available:

```
   %SW_MATM-4-MACFLAP_NOTIF: Host 00:1a:2b:3c:4d:5e in vlan 20 is
   flapping between port Gi1/0/13 and port Gi1/0/24
```

> **Two ports, one MAC, alternating** — there is a path between those two ports that should
> not exist. This message identifies a loop faster than any topology analysis, and it is
> frequently ignored because it is a level-4 informational message.

## Port security and 802.1X

Faults created by the controls of Chapter 59 and Chapter 62.

| Symptom | Cause |
|---|---|
| **The port shuts down when a laptop is docked** | **port security maximum exceeded** (Chapter 62 §62.1) |
| **A device works and its replacement does not** | **sticky MAC learning retained the old one** |
| **Link light and no network, indefinitely** | **802.1X authentication failing** (Chapter 59 §59.2) |
| **Works after 30–90 seconds** | **802.1X timing out to MAB or a guest VLAN** |
| **Everything disconnects at once** | **RADIUS unreachable and re-authentication timers expiring** |
| **A phone works and the PC behind it does not** | **multi-domain authentication not configured** |

## ARP and neighbour discovery

Chapter 18's mechanism, as a fault (Chapter 64 §64.2's states).

| Symptom | Cause |
|---|---|
| **`INCOMPLETE` for the gateway** | **the gateway is down, or not in this VLAN, or the mask is wrong** |
| **Two IPs, one MAC** | **a router (normal), a proxy ARP device, or spoofing** |
| **One IP, two MACs over time** | **duplicate address** (Chapter 53 §53.3) **or spoofing** |
| **Gratuitous ARP storms** | **a clustering product, a failover event, or an attack** |
| **IPv6 hosts with only link-local** | **no Router Advertisement** — the RA is blocked or the router is not sending |

And proxy ARP deserves a mention because it produces baffling behaviour:

> A router answering ARP for addresses that are not its own makes a wrongly-masked host work
> anyway, which hides the actual fault and produces a network where the mask can be wrong
> for years without symptom — until proxy ARP is disabled, or a device that does not do it
> is installed, and then everything breaks at once.

## The diagnostic sequence

```
   1.  What VLAN is this port actually in?        show interface switchport
   2.  Is the MAC learned, and on the right port? show mac address-table
   3.  Is the port forwarding in STP?             show spanning-tree interface
   4.  Are there errors, and of what kind?        show interface
   5.  Does the trunk carry this VLAN?            show interface trunk
   6.  Do the native VLANs match?                 show interface trunk / CDP log
   7.  Is 802.1X or port security involved?       show authentication / port-security
```

> **Steps 1 and 2 resolve the majority.** "Which VLAN is it in, and has the switch seen its
> MAC?" answers most Layer 2 complaints in ten seconds, and both are read-only.

## What breaks here

**"No IP address" after a desk move.** The VLAN, not DHCP.

A VLAN that works within a building and not between buildings. The trunk's allowed list.

A link that works for one VLAN only, with correct-looking configuration at both ends.
`dynamic auto` at both ends — neither initiated.

Devices taking 45 seconds to work after connecting. **No PortFast.**

**Brief outages across an entire VLAN, repeatedly.** **Topology changes** — read the counter and
the port it names.

Throughput at 3% of the link rate, with the link healthy. **Duplex mismatch.** Late
collisions on one side, CRC on the other.

**A MAC flapping between two ports.** **A loop.** The log message names both ports.

A port that disables whenever a specific user connects. Port security, and their docking
station presents extra MAC addresses.

A host with a wrong subnet mask that works. Proxy ARP is covering for it, and it will
break later for an unrelated reason.

**IPv6 hosts with only link-local addresses.** **No RA** — the router is not sending, or RA
Guard is blocking it (Chapter 62 §62.1).

> **Network+ note.** Objective 5.2 and 5.3. Over-learn: VLAN misconfiguration prevents
> communication and is a common cause of "no address"; **native VLAN mismatch and allowed-VLAN
> omissions break trunks**; **duplex mismatch causes late collisions and poor performance**;
> switching loops cause broadcast storms and MAC table instability; and **spanning tree
> blocks redundant paths.** The VLAN-versus-DHCP misattribution is examined and is the single
> most useful thing in this section.
