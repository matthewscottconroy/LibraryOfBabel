# 40.1 Manual Configuration Does Not Scale

A host needs four things before it can use a network:

| | Chapter |
|---|---|
| **An IP address** | 25 |
| **A subnet mask** | 25 |
| **A default gateway** | 29 |
| **A DNS resolver** | 39 |

**Without all four, nothing works** — and three of the four produce distinctive failures
when wrong, which is why Chapter 22 §22.4's method checks them in order.

**Someone must supply them.** This chapter is about why that someone should not be a person.

## The arithmetic of doing it by hand

**For *n* hosts, a person must:**

- Choose an address, **verifying it is unused**
- Configure four values on the device
- **Record the assignment somewhere** (Chapter 27 §27.4)
- Repeat on every change, every reinstall, every replacement

**At *n* = 5**, this is fine. At *n* = 50 it is tedious. **At *n* = 500 it is a full-time
job**, and at *n* = 5,000 it is impossible.

**But the count is not the real problem.** As with static routing (Chapter 30 §30.4), it is
**the rate of change.**

### What actually breaks

**Duplicate addresses.** Two hosts, one address. **The symptom is intermittent and
host-dependent connectivity** that changes as ARP caches expire (Chapter 18 §18.3) — one of
the most confusing faults in networking, and it arises the moment two people assign
addresses without perfect coordination.

**Mobility.** A laptop moves between floors, buildings, offices and homes. **Each is a
different subnet**, so manual configuration means reconfiguring on every move. **This alone
made manual assignment untenable once portable computers existed.**

**Reinstallation and replacement.** Every rebuild is a reconfiguration, done by someone who
must first find out what the values were.

**Change.** Renumber a subnet, move a gateway, replace a DNS server — **and every host must
be touched.** Chapter 27 §27.4's renumbering problem, in its most acute form.

**Scale of error.** A wrong mask on one host produces Chapter 25 §25.3's selective
connectivity; a wrong gateway produces a timeout; **and both are configured by hand, by
people, hundreds of times.**

> **The failure is not that manual configuration is slow. It is that it is unreliable at
> scale, and its errors are the most confusing kind — partial, intermittent, and
> host-specific.**

## The predecessors

**DHCP did not appear from nothing**, and its ancestry explains several of its oddities.

### RARP (1984) — Reverse ARP

**"Here is my MAC address; what is my IP address?"** RFC 903.

**Its limitations were severe:**

- **Returns only an address** — no mask, no gateway, no DNS
- **Uses a Layer 2 broadcast**, so **it cannot cross a router** — a server was needed on
  every segment
- Required a manually-maintained MAC-to-IP table anyway, so it moved the work rather than
  removing it

**Obsolete, and it established the pattern**: broadcast a request, have a server answer.

### BOOTP (1985) — Bootstrap Protocol

**RFC 951, and this is DHCP's direct ancestor.**

**The improvements:**

- **Runs over UDP/IP**, so a **relay agent** can forward it across a router (§40.4)
- **Returns the mask, gateway and a boot file name** — enough for a diskless workstation to
  fetch its operating system by TFTP (Chapter 36 §36.3)
- **Ports 67 and 68** — which DHCP inherited

**The limitation that mattered:** **BOOTP assignments were static.** A table mapped MAC
addresses to IP addresses, maintained by hand. **It automated configuration and not
allocation.**

> **BOOTP solved "how does a host learn its settings". DHCP solved "who decides what they
> are".**

**And the inheritance is visible:** DHCP uses BOOTP's ports, its packet format, and its
relay mechanism. **A DHCP packet is a BOOTP packet with the vendor-extensions field
repurposed** — which is why §40.3's option encoding looks the way it does, and why BOOTP
relay agents work for DHCP unchanged.

## DHCP's addition — the lease

**RFC 2131 (1997), building on RFC 1531 (1993).**

**The idea that makes dynamic allocation work:**

> **An address is not assigned. It is *leased* — granted for a bounded time, and reclaimed
> if not renewed.**

**Why a lease rather than an assignment:**

**Addresses are finite.** A subnet has a fixed pool. Without expiry, every device that ever
connected would hold an address forever, and a guest network would exhaust in days.

**Hosts vanish without saying goodbye.** A laptop closed and carried away sends no release.
**A permanent assignment would leak an address every time**, and there is no mechanism that
could detect it reliably.

**The network changes.** A lease's expiry is a natural point at which a host re-asks — so
changing a gateway or a DNS server propagates within a lease time, **with no need to touch
any host.**

**The lease is what makes DHCP self-correcting**, and it is the difference between DHCP and
everything before it.

**Choosing the duration is a real trade:**

| Lease time | Suits |
|---|---|
| **1–2 hours** | guest wireless, conference networks — **high churn, reclaim fast** |
| 8 hours | a working day |
| **1 day** | **a sensible default for corporate wireless** |
| **8 days** | **Windows Server's default** — wired office networks |
| 30+ days | stable wired estates |
| Infinite | **avoid** — it discards the mechanism |

**Short leases** reclaim addresses quickly and propagate changes fast, at the cost of more
DHCP traffic and more server dependence.

**Long leases** are quiet and resilient — **a host with a long lease survives a DHCP outage
for days** — at the cost of slow reclamation and slow propagation.

> **The lease time is the period over which a configuration change reaches every host, and
> also the period a host can survive without the server.** Those pull in opposite
> directions, and the choice is which matters more for that network.

## What DHCP supplies

**Far more than four values.** The option catalogue (§40.3) is large, and the commonly used
ones are:

| | Option |
|---|---|
| **IP address** | (in the packet header, not an option) |
| **Subnet mask** | 1 |
| **Default gateway** | 3 |
| **DNS servers** | 6 |
| Domain name | 15 |
| **Lease time** | 51 |
| **DHCP server identifier** | 54 |
| NTP servers | 42 |
| **TFTP server / boot file** | 66, 67 — **PXE boot** |
| **Vendor-specific** | 43, 60 — VoIP phones, access points |
| Classless static routes | **121** |

**Which is why "DHCP is broken" is often broader than it sounds.** A host may get an
address and still fail because option 6 is wrong, or because a phone did not receive its
option 43 and cannot find its call manager.

## Where it fits with IPv6

**Worth stating now, because it is the most common IPv6 confusion** (Chapter 28 §28.3):

| | IPv4 | IPv6 |
|---|---|---|
| Address | **DHCP** | **SLAAC, or DHCPv6, or both** |
| Gateway | **DHCP option 3** | **always the Router Advertisement** |
| DNS | DHCP option 6 | **RDNSS in the RA, or DHCPv6** |

> **DHCPv6 never supplies a default gateway.** It always comes from the RA, and a network
> running stateful DHCPv6 without RAs gives hosts addresses and no route.

**And Android does not implement DHCPv6 at all**, so a stateful-only IPv6 network silently
fails for phones.

## What breaks here

**Two hosts with the same address.** Manual assignment without coordination, or a static
address inside a DHCP pool (§40.3).

**A device that works in one office and not another.** Manually configured for one subnet.

**A configuration change that requires touching every host.** No DHCP, or leases too long.

**`169.254.x.x`** (Chapter 27 §27.2). **No DHCP reply arrived** — and the causes are
§40.4's.

**An address obtained and nothing working.** The address is only one of four values. Check
the mask, the gateway and the resolver.

> **Network+ note.** Objective 1.6 expects DHCP's purpose and objective 2.3 its operation.
> Over-learn: **DHCP supplies address, mask, gateway and DNS**; **it uses UDP ports 67
> (server) and 68 (client)**; **it descends from BOOTP and shares its ports**; and **the
> lease is what distinguishes it from BOOTP's static assignment.**
