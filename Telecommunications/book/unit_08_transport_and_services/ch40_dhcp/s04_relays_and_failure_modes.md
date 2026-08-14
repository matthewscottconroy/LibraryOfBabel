# 40.4 Relays and Failure Modes

§40.2 established that a DHCP client broadcasts, and Chapter 27 §27.3 established that
broadcasts do not cross routers. **This section closes that gap and then catalogues what
goes wrong** — because DHCP failures are common, distinctive, and mostly resolvable in a
few minutes once you know the pattern.

## The relay agent

**Without it, every subnet needs its own DHCP server.** For a network with sixty VLANs that
is sixty servers, sixty configurations, and sixty things to fail.

**The relay agent** — historically the "DHCP helper" — is a **router listening for DHCP
broadcasts and forwarding them, as unicast, to a central server.**

```
   Client            Router (relay)              Server
   VLAN 10           10.1.10.1                   10.1.1.53
     │                    │                          │
     │─ DISCOVER ────────▶│                          │
     │  broadcast         │                          │
     │                    │─ DISCOVER ──────────────▶│
     │                    │  UNICAST                 │
     │                    │  giaddr = 10.1.10.1  ←───┼── the key field
     │                    │                          │
     │                    │◀──────────── OFFER ──────│
     │                    │  unicast to the relay    │
     │◀─ OFFER ───────────│                          │
     │  broadcast         │                          │
```

**Configured in one line:**

```
interface Vlan10
 ip address 10.1.10.1 255.255.255.0
 ip helper-address 10.1.1.53
 ip helper-address 10.1.1.54          ! a second server, for redundancy
```

### giaddr — the field that makes it work

**The relay sets the `giaddr` (gateway IP address) field to its own address on the client's
subnet.**

> **`giaddr` is how the server knows which scope to allocate from.**

The server receives a unicast packet from `10.1.1.x`, looks at `giaddr = 10.1.10.1`, and
selects the scope for `10.1.10.0/24`. **Without it the server would have no idea which
subnet the client is on**, because the packet arrived from the relay rather than from the
client's segment.

And it is why the relay must be the router *on the client's subnet* — the address it
inserts must be one the server can map to a scope.

A missing or wrong scope for a `giaddr` value produces a distinctive symptom: the server
receives the request, has no matching scope, and **silently ignores it.** The client gets
nothing, and the server's logs — if you look — say so explicitly.

### Option 82 — relay agent information

**The relay may add information about *where* the client is:**

| Sub-option | Contents |
|---|---|
| **Circuit ID** | **the switch port** the client is on |
| **Remote ID** | the switch's identity |

Which lets a server assign based on physical location — useful for service providers
assigning by subscriber line, and for enterprises assigning by port.

It is also what DHCP snooping uses to build its binding table (below), and it is why
snooping and Dynamic ARP Inspection (Chapter 18 §18.3) fit together.

### `ip helper-address` forwards more than DHCP

**A Cisco quirk worth knowing**, because it surprises people.

`ip helper-address` forwards eight UDP broadcast services by default, not just DHCP:

| Port | Service |
|---|---|
| 37 | Time |
| **53** | **DNS** |
| **67/68** | **DHCP/BOOTP** |
| 69 | TFTP |
| 137/138 | NetBIOS |
| 49 | TACACS |

So configuring a helper for DHCP also forwards NetBIOS broadcasts to the same server —
usually harmless, occasionally a surprising traffic source.

```
no ip forward-protocol udp netbios-ns
no ip forward-protocol udp netbios-dgm
```

**Restricting it is good practice**, and it is one line per service.

## The failure modes

**DHCP's faults are unusually distinctive**, which makes them fast to diagnose once
recognised.

### `169.254.x.x` — no reply arrived

**Chapter 27 §27.2's signal**, and here are the causes, in the order worth checking:

| Cause | Check |
|---|---|
| **Wrong VLAN on the switch port** | `show interfaces switchport` — **the most common in enterprises** |
| **Missing relay** on the client's subnet | `ip helper-address` on the SVI |
| **Missing scope** for that `giaddr` | the server's configuration and logs |
| **Pool exhausted** | server statistics |
| Server down | monitoring |
| **PortFast missing** | the port took 30 s to forward and DHCP timed out (Chapter 19 §19.3) |
| Cable or link | Layer 1 |

**The first and last deserve emphasis.**

Wrong VLAN is the commonest enterprise cause, and it is invisible from the client: the
link is up, the port is fine, and the client is simply in a broadcast domain where no
server or relay exists.

**Missing PortFast** produces a distinctive variant — **the client fails on boot and
succeeds if you release and renew manually**, because by then the port has finished
listening and learning. **Any "it works if I retry" DHCP report should prompt a PortFast
check.**

### One host affected versus all hosts

**The first branch in the diagnosis:**

| Scope | Points at |
|---|---|
| **One host** | its port's VLAN, its cable, its own configuration |
| **One subnet** | **the relay**, or the scope for that `giaddr` |
| **Everything** | **the server**, or the path to it |

This single question eliminates most of the search space, and it should be the first
thing asked.

### Pool exhaustion

**Every address leased, and new clients get nothing.**

**The causes:**

- **Genuine growth** — more devices than planned
- Leases too long for the churn — a guest network with 8-day leases holds addresses for
  a week after each visitor leaves
- **A device requesting many addresses** — a misbehaving client, or a **DHCP starvation
  attack** (below)

**The remedies, in order:** shorten the lease, widen the pool, add a subnet. **Shortening
the lease is immediate and reversible**, which makes it the right first move.

### Rogue DHCP servers

The attack, and the accident — and the accident is more common.

**There is no authentication in DHCP.** A client believes whichever server answers first
(§40.2), and anything on the segment can be a server.

**The accidental cases:**

- A home router plugged in as a switch — and it is serving DHCP on its LAN ports
- A virtualisation host with a NAT network bridged to the physical one
- A Windows machine with internet connection sharing
- A test lab device connected to the production network

**The malicious case is the same mechanism used deliberately:** answer first, supply
your own address as the gateway, and every packet the victim sends off-subnet passes
through you. A man-in-the-middle established by answering a broadcast.

**The symptoms are distinctive:**

- Some hosts get addresses from the wrong range — often `192.168.x.x` where the network is
  `10.x.x.x`
- Affected hosts have the wrong gateway and cannot reach anything
- **Which hosts are affected varies**, because it depends on which server answered first
- Rebooting sometimes fixes it and sometimes does not

**The defence: DHCP snooping.**

```
ip dhcp snooping
ip dhcp snooping vlan 10,20,30
!
interface GigabitEthernet0/24              ! toward the real server or relay
 ip dhcp snooping trust
!
interface range GigabitEthernet0/1 - 23    ! access ports
 ip dhcp snooping limit rate 10
```

**The switch drops DHCP *server* messages arriving on untrusted ports.** A client may send
DISCOVER and REQUEST; only a trusted port may send OFFER and ACK.

> **DHCP snooping is one line per switch plus one trusted port**, and it eliminates an
> entire class of both accident and attack.

And it builds the binding table — MAC, IP, VLAN, port, lease — **which Dynamic ARP
Inspection depends on** (Chapter 18 §18.3). **This is why DAI without DHCP snooping drops
everything**: there is no table to validate against.

**Port security** (Chapter 17 §17.2) and **DHCP starvation**: an attacker requesting
thousands of addresses with spoofed MAC addresses exhausts the pool, **so that legitimate
clients fall back to the attacker's rogue server.** The rate limit above bounds it, and
port security bounds the MAC addresses.

### Duplicate addresses despite DHCP

A static address inside the pool (§40.3), and a client that does not perform the
ARP check.

**The symptom is intermittent** — it appears only when the pool reaches that address, which
may be months after the static was configured.

**`arping -D`** (Chapter 18 §18.3) identifies it in seconds, and the fix is an exclusion.

### The slow failure

§40.2's renewal timers mean a DHCP outage is invisible for hours.

**Running hosts renew at T1 and, failing that, continue happily.** **Only booting hosts and
new arrivals fail**, so the first report comes from whoever restarts a machine — often the
morning after.

> A DHCP server can be down for half a lease time before anyone notices, which is why
> it needs monitoring rather than complaint-driven discovery.

## The diagnostic sequence

```bash
# 1. What does the client have?
ip addr show                      # 169.254.x.x means no reply
ipconfig /all                     # Windows: lease and server address

# 2. One host, one subnet, or everything?
#    (ask, before touching anything)

# 3. Is the port in the right VLAN?
show interfaces gi0/5 switchport

# 4. Is there a relay?
show run interface vlan 10 | include helper

# 5. Is the server receiving the request?
#    On the server:
journalctl -u isc-dhcp-server -f
tail -f /var/log/syslog | grep dhcp

# 6. Capture, if still unclear
tcpdump -i eth0 -nn port 67 or port 68
```

**Steps 2 and 5 do most of the work.** Knowing the scope of the failure narrows it to one of
three causes, and the server's log says explicitly whether it saw the request and what it
did.

And the server's log is the most under-used DHCP diagnostic — it reports "no free
leases", "unknown network segment for giaddr", and every DISCOVER it declined, in plain
text.

## What breaks here

**`169.254.x.x` on one host.** Its VLAN, its cable, or PortFast.

**`169.254.x.x` on a whole subnet.** The relay, or a missing scope for that `giaddr`.

**`169.254.x.x` everywhere.** The server, or the path to it.

"It works if I release and renew." PortFast missing.

**Addresses from an unexpected range.** A rogue DHCP server. Enable snooping.

**Intermittent duplicates.** A static inside the pool.

**New clients failing while existing ones are fine.** Pool exhaustion, or a server outage
that existing leases are hiding.

**DAI dropping everything after being enabled.** No DHCP snooping, so no binding table.

> **Network+ note.** Objective 2.3 expects relay agents; objective 4.2 expects rogue DHCP
> servers and DHCP snooping. Over-learn: **a relay forwards client broadcasts as unicast to
> a central server and sets `giaddr` so the server knows the scope**; **`ip helper-address`
> is the configuration**; **a rogue DHCP server is defeated by DHCP snooping with a trusted
> port**; and **`169.254.x.x` means no DHCP reply arrived.** The relay's purpose is
> examined and the `giaddr` mechanism explains why it works.
