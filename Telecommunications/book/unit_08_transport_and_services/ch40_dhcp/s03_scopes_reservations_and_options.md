# 40.3 Scopes, Reservations and Options

The configuration side. A scope is a subnet's address plan expressed to a server, and the
option catalogue is how a great deal of enterprise infrastructure is quietly configured.

## The scope

A scope is one subnet's pool, plus everything a client on it should be told.

```
   Subnet:        10.1.5.0/24
   Range:         10.1.5.100 – 10.1.5.200      ← the dynamic pool
   Excluded:      10.1.5.1 – 10.1.5.99         ← infrastructure and statics
                  10.1.5.201 – 10.1.5.254      ← reserved for growth
   Mask:          255.255.255.0
   Gateway:       10.1.5.1
   DNS:           10.1.1.53, 10.1.1.54
   Domain:        eng.example.com
   Lease:         8 days
```

The layout follows Chapter 27 §27.4's convention, and the reason for the exclusions is
the same: a DHCP pool must not overlap anything assigned by hand.

**Which is the single most common DHCP misconfiguration.** A server given a static address
inside the pool will eventually collide with a leased client — **and the failure is
intermittent, because it only occurs when the pool reaches that address.** A pool
configured in January can produce a duplicate in June.

> **Decide the split between static and dynamic once, write it down, and keep the pool
> strictly inside its half.**

**A sensible /24 layout:**

| Range | Use |
|---|---|
| `.1` | gateway |
| `.2` – `.9` | network infrastructure |
| `.10` – `.99` | **static servers and devices** |
| `.100` – `.200` | **DHCP pool** |
| `.201` – `.250` | **reserved for future static** |
| `.251` – `.254` | spare |

**The reserved block matters** (Chapter 27 §27.4): without it, the pool eventually abuts the
static range and any new static device must be squeezed in.

## Reservations

**A fixed address, delivered by DHCP.**

```
   MAC aa:bb:cc:dd:ee:ff  →  always  10.1.5.50
```

> **The device is configured for DHCP. The *server* guarantees which address it gets.**

This is almost always better than configuring the device statically, and the reason is
management:

| | Static on the device | DHCP reservation |
|---|---|---|
| Where the address lives | **on the device** | **on the server** |
| Changing it | touch the device | edit the server |
| Other options (DNS, gateway) | **also on the device** | **centrally managed** |
| Visible in one place | no | **yes** |
| Survives a device rebuild | **no** | **yes** |

**Use reservations for:** printers, cameras, access points, telephones, appliances — **any
device that needs a stable address and is not a server.**

**Use true static configuration for:** DHCP servers themselves (obviously), DNS servers,
routers, and anything that must work when DHCP does not.

**Two cautions:**

A reservation must be inside a range the server controls, and platforms differ on
whether it may be inside the dynamic pool. **Keeping reservations in their own excluded
block is cleaner** and avoids the question.

**MAC randomisation breaks reservations.** Modern phones and laptops randomise their MAC
address per network by default (Chapter 45 §45.4), so **a reservation keyed on MAC may stop
matching after an update.** For managed devices, disable randomisation; for unmanaged ones,
do not rely on reservations.

## Options

**The catalogue is large. The ones that matter in practice:**

| Option | Name | Notes |
|---|---|---|
| **1** | Subnet mask | |
| **3** | **Router** | the default gateway |
| **6** | **DNS servers** | |
| 15 | Domain name | the client's own domain |
| **42** | NTP servers | |
| **51** | **Lease time** | |
| **53** | **Message type** | **DISCOVER/OFFER/REQUEST/ACK — this is what makes DORA distinguishable** |
| **54** | **Server identifier** | the mechanism of §40.2's REQUEST |
| 55 | **Parameter request list** | **what the client asked for** |
| **66 / 67** | TFTP server / boot file | **PXE boot** |
| **60 / 43** | Vendor class / vendor-specific | **VoIP phones, access points** |
| **119** | Domain search list | |
| **121** | **Classless static routes** | see below |
| 252 | WPAD proxy URL | Microsoft convention |

### Option 55 explains a common confusion

The client sends a list of what it wants; the server returns only those.

So an option configured on the server may never reach a device — because that device
never asked for it. `ipconfig getpacket` on macOS or a capture is how you find out, and
"the option is configured but the phone does not have it" is usually this.

### Options 43 and 60 — how devices self-configure

**The mechanism behind a great deal of zero-touch deployment.**

**Option 60** is the client saying *what kind of device it is* — `Cisco AP c1140`,
`PXEClient`, a vendor string.

**Option 43** is the server's reply *for that kind of device* — vendor-specific data whose
format the vendor defines.

**In practice:**

| Device | Uses option 43 for |
|---|---|
| **Wireless access point** | **the controller's address**, so it can join |
| **VoIP telephone** | **the call manager's address** |
| PXE client | boot server details |
| Thin client | broker address |

> **This is why an access point unboxed and plugged in finds its controller.** The switch
> port gives it a VLAN, DHCP gives it an address and option 43 tells it where to go.

And it is why "the new AP will not join" is usually a DHCP question, not a wireless one.

### Option 121 — classless static routes

**Underused, and it solves a real problem.**

**DHCP gives one default gateway (option 3).** A client needing an additional route — to a
VPN range, a management network, a partner subnet — has no way to learn it.

Option 121 supplies a list of prefix-and-gateway pairs, so a client can be given
specific routes alongside its default.

**Two cautions:**

RFC 3442 says that if option 121 is present, option 3 should be ignored — so a static
route list must include the default route explicitly, or the client will have none. **This
catches people.**

**Support varies.** Windows and Linux honour it; some embedded devices do not.

Option 249 is Microsoft's pre-standard equivalent, and many servers are configured to
send both.

## Failover and redundancy

**DHCP is infrastructure, so its failure matters — though §40.2's renewal timers mean the
failure is slow rather than immediate.**

### Split scopes — the simple approach

**Two servers, each with part of the range:**

```
   Server A:  10.1.5.100 – 10.1.5.160     (80%)
   Server B:  10.1.5.161 – 10.1.5.200     (20%)
```

**Both answer; the client takes the first offer.** If one fails, the other continues from
its share.

**The conventional split is 80/20** — the primary holds most of the pool and the secondary
enough to cover an outage.

Simple, requires no coordination, and wastes pool capacity: neither server can use the
other's addresses, so the effective pool is smaller than the range.

### DHCP failover — the proper approach

RFC 3074's protocol, implemented by ISC DHCP, Kea and Windows Server. The two servers
share the pool and synchronise lease state, so either can serve any address and both
know what is leased.

| Mode | Behaviour |
|---|---|
| **Load balance** | both serve simultaneously, splitting the load |
| **Hot standby** | one serves; the other takes over on failure |

**Better use of the pool and more configuration.** Worth it for anything above a few hundred
clients.

### The Kea model

ISC's replacement for the original ISC DHCP server, which reached end of life in 2022.
Kea stores leases in a database — MySQL, PostgreSQL, or memory — which makes redundancy
a database problem rather than a protocol one, and makes the lease table queryable.

Worth knowing about if you are choosing a server today, because ISC DHCP is no longer
maintained and a great deal of documentation still assumes it.

## Configuration, concretely

**ISC DHCP / Kea style:**

```
subnet 10.1.5.0 netmask 255.255.255.0 {
    range 10.1.5.100 10.1.5.200;
    option routers 10.1.5.1;
    option domain-name-servers 10.1.1.53, 10.1.1.54;
    option domain-name "eng.example.com";
    default-lease-time 691200;          # 8 days
    max-lease-time 691200;
}

host printer-eng-01 {
    hardware ethernet aa:bb:cc:dd:ee:ff;
    fixed-address 10.1.5.50;
}
```

**Cisco IOS, for a small site:**

```
ip dhcp excluded-address 10.1.5.1 10.1.5.99
ip dhcp excluded-address 10.1.5.201 10.1.5.254
!
ip dhcp pool ENG
 network 10.1.5.0 255.255.255.0
 default-router 10.1.5.1
 dns-server 10.1.1.53 10.1.1.54
 domain-name eng.example.com
 lease 8
```

**Note the exclusions come first** and are configured globally rather than inside the pool —
an IOS quirk that catches people.

## Monitoring

DHCP's slow failure mode (§40.2) means it needs deliberate monitoring, because nobody
notices for hours.

**What to watch:**

| Metric | Why |
|---|---|
| **Pool utilisation** | exhaustion is the commonest capacity failure |
| **Server availability** | it will not be noticed otherwise |
| **DECLINE count** | evidence of statics inside the pool |
| **Lease churn** | a spike suggests something is looping |
| **Rogue server detection** | §40.4 |

Pool utilisation deserves an alert at 80%, because the remedy — widening the pool or
shortening the lease — takes planning, and the failure is total for new clients.

## What breaks here

**Intermittent duplicate addresses.** A static inside the pool. Check the exclusions.

**A device with an address and no DNS.** The option is not configured, or the client did not
request it (55).

**An access point that will not join its controller.** Option 43, or the wrong VLAN.

**A client with routes missing after adding option 121.** Option 3 is ignored when 121 is
present; the default must be in the list.

**A reservation that stopped working.** MAC randomisation.

**Pool exhaustion on a guest network.** Leases too long for the churn. Shorten to hours.

**Both DHCP servers offering the same address.** Split scopes overlapping, or failover
misconfigured.

> **Network+ note.** Objective 2.3 expects scopes, reservations, exclusions and options.
> Over-learn: **a scope is a subnet's pool and settings**; **exclusions keep static
> addresses out of the pool**; **a reservation gives a fixed address by DHCP**; and the
> **option numbers 3 (gateway), 6 (DNS), 51 (lease), 66/67 (TFTP boot).** Options 3 and 6
> are examined directly.
