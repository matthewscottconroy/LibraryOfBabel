# 33.2 Static, Dynamic and Overload

Three kinds of NAT, and only one of them is what people mean when they say "NAT". This
section works the translation table explicitly, because the table is the mechanism and
everything else follows from it.

## Static NAT — one to one, permanent

**One private address maps to one public address, always, in both directions.**

```
   192.168.1.10   ⟷   203.0.113.10
   192.168.1.11   ⟷   203.0.113.11
```

```
ip nat inside source static 192.168.1.10 203.0.113.10
```

**It conserves nothing** — one public address per host, which is what you were trying to
avoid. **Its purpose is inbound reachability**: the mapping exists before any traffic
flows, so the outside can initiate.

**Use it for servers.** A web server needs a stable public address that DNS can point at
and that the outside can connect to. Static NAT gives it one while keeping the server
itself on a private address.

## Dynamic NAT — a pool, first come first served

**A pool of public addresses, assigned to internal hosts as they need one.**

```
ip nat pool PUBLIC 203.0.113.10 203.0.113.20 netmask 255.255.255.0
ip nat inside source list 1 pool PUBLIC
access-list 1 permit 192.168.1.0 0.0.0.255
```

**Eleven public addresses, shared among however many hosts** — but **only eleven at a
time**. The twelfth host to want outside access **gets nothing**, and its traffic is
dropped.

**Inbound is impossible**, because the mapping does not exist until the host sends
something, and there is no way to know in advance which public address it will get.

**Dynamic NAT is largely historical.** It conserves addresses only in proportion to how
many hosts are idle, which is a weak saving, and it fails hard when the pool exhausts.
§33.2's third form dominated it completely.

## PAT — the one everyone actually uses

**Port Address Translation**, also called **NAT overload**, **NAPT**, or — confusingly —
just "NAT".

> **Many private addresses share ONE public address, distinguished by port number.**

**This is the mechanism that saved IPv4**, and it is worth being precise about how.

### The insight

Chapter 35 will develop ports properly. The relevant fact now:

**A TCP or UDP conversation is identified by five things** — the *five-tuple*:

$$(\text{protocol},\ \text{src IP},\ \text{src port},\ \text{dst IP},\ \text{dst port})$$

**So a router translating an internal conversation can change the source port as well as
the source address**, and use the port to remember which internal host it belonged to.

**The port field is 16 bits — 65,535 values.** One public address can therefore support
**tens of thousands of simultaneous conversations**, and the practical limit is the port
space rather than the address.

### The table

```
ip nat inside source list 1 interface GigabitEthernet0/0 overload
```

**Three hosts browsing the web:**

| Inside local | Inside global | Outside global | Protocol |
|---|---|---|---|
| `192.168.1.10:52341` | `203.0.113.5:52341` | `93.184.216.34:443` | TCP |
| `192.168.1.11:49823` | `203.0.113.5:49823` | `93.184.216.34:443` | TCP |
| `192.168.1.12:60122` | `203.0.113.5:60122` | `142.250.180.14:443` | TCP |

**All three appear as `203.0.113.5`**, and the router tells them apart by port.

**Note rows 1 and 2:** two different internal hosts, both talking to the same server on
the same port. **Only the source port distinguishes them**, and it is the only thing that
can.

### Port collision

What happens when two internal hosts happen to choose the same ephemeral source port and
contact the same destination?

| Inside local | Inside global | Outside global |
|---|---|---|
| `192.168.1.10:52341` | `203.0.113.5:52341` | `93.184.216.34:443` |
| `192.168.1.11:52341` | `203.0.113.5:`**`52342`** | `93.184.216.34:443` |

**The router rewrites the second host's port** to something free. The host does not know
and does not need to — it sees replies arriving at the port it chose, because the router
translates back on the way in.

**This is why PAT is sometimes called *port* address translation rather than *address*
translation.** The port is not incidental; it is the identifier that makes the sharing
possible.

### Tracing a packet through

**Outbound:**

```
   Host sends:      192.168.1.10:52341  →  93.184.216.34:443
                              │
                       NAT rewrites SOURCE
                              ▼
   Router sends:    203.0.113.5:52341   →  93.184.216.34:443
                    and records the mapping
```

**Inbound — the reply:**

```
   Server sends:    93.184.216.34:443   →  203.0.113.5:52341
                              │
                    NAT looks up 203.0.113.5:52341 in the table,
                    finds 192.168.1.10:52341,
                    rewrites DESTINATION
                              ▼
   Router delivers: 93.184.216.34:443   →  192.168.1.10:52341
```

**And the checksums.** The router changed the IP addresses and the ports, so:

- **The IP header checksum** must be recomputed (it covers the addresses)
- **The TCP or UDP checksum** must be recomputed — because it covers a **pseudo-header
  containing the IP addresses** (Chapter 21 §21.4)

**That second one is the layer violation.** A Layer 3 device is recomputing a Layer 4
checksum, and it must, because the transport checksum reaches down into the network layer.
**The pseudo-header decision of 1981 made NAT's job harder in 1994**, which is a small
example of how far design decisions propagate.

## Capacity and the timers

**How many hosts can share one public address?**

The theoretical limit is 65,535 ports, less the well-known range — call it **64,000
concurrent conversations**.

**But hosts open many connections at once.** A single modern web page may open several
dozen; a browser with twenty tabs may hold hundreds. **Practical guidance is a few hundred
to a thousand hosts per public address**, and the binding constraint is bursty connection
behaviour rather than the steady-state count.

**Which is why the timers matter:**

| Entry type | Typical timeout |
|---|---|
| TCP, established | 24 hours |
| **TCP, after FIN or RST** | 60 seconds |
| **UDP** | 300 seconds (5 minutes) |
| ICMP | 60 seconds |
| DNS | 60 seconds |

```
ip nat translation timeout 3600
ip nat translation udp-timeout 120
ip nat translation tcp-timeout 3600
```

**The 24-hour TCP timeout is the one that causes trouble.** A translation is held for a
day after the last packet, so a device with many short-lived connections accumulates
entries and can exhaust the table long before the port space is genuinely full.

**And the UDP timeout is why long-lived UDP applications send keepalives.** A VoIP call on
hold, a VPN with no traffic, an IoT device reporting every ten minutes — each risks its
translation being reaped, after which inbound packets have nowhere to go and the session
silently dies. **The application sends a packet every few minutes purely to keep the NAT
entry alive**, and this is why so many protocols have an otherwise-pointless keepalive.

## Port forwarding

**Making one internal service reachable from outside**, without a full static NAT.

```
ip nat inside source static tcp 192.168.1.50 8080 203.0.113.5 80
```

**Outside traffic to `203.0.113.5:80` is sent to `192.168.1.50:8080`.**

**A pre-populated table entry**, created by configuration rather than by outbound traffic,
so the outside can initiate. This is what every home router's "port forwarding" page does.

**Its limitations are structural:**

- **One external port maps to one internal host.** Two internal web servers cannot both
  have port 80 externally without different public addresses or different external ports.
- It requires configuration, so it does not work for dynamic peer-to-peer applications.
- **It is a hole**, and it should be as narrow as possible — one port, one host, one
  protocol.

**UPnP and NAT-PMP/PCP** let applications request forwarding automatically. Convenient,
and **a genuine security concern**: any application on the internal network — including
malware — can open an inbound hole with no human approval. **Disable UPnP on anything you
care about.**

## Hairpinning

The case that is broken on a surprising number of devices.

**An internal host tries to reach an internal server by its *public* address:**

```
   192.168.1.20  ──▶  203.0.113.5:80  ──▶  should reach 192.168.1.50:8080
```

**This requires the router to translate a packet that arrives and leaves on the same
interface** — turning it round, hence "hairpin", or "NAT loopback".

**Many cheap routers do not implement it.** The symptom is distinctive and maddening: **the
service works from outside the network and fails from inside**, using the same name and
address, so users on the office wifi cannot reach the company's own website while everyone
else can.

**The workarounds:** split-horizon DNS (return the internal address to internal
clients — the usual answer), or a router that supports hairpinning.

## Verifying

```
show ip nat translations
show ip nat statistics
clear ip nat translation *
debug ip nat
```

```bash
# Linux
iptables -t nat -L -n -v
nft list table nat
conntrack -L
```

**`show ip nat translations` is the command.** It is the table of §33.2, and every NAT
problem is visible in it: a missing entry, an entry pointing at the wrong host, or a table
that is full.

**`conntrack -L` on Linux** shows the same thing and is worth knowing — the connection
tracking table is what both NAT and the stateful firewall use.

## What breaks here

**A translation table that is full.** Too many hosts, or timers too long. Check
`show ip nat statistics` for the count and the drops.

**A UDP application dying after a few minutes of silence.** The translation was reaped.
Keepalives, or a longer UDP timeout.

**Two internal servers needing the same external port.** Not possible on one public
address. Use different external ports or another address.

**A service reachable from outside and not from inside.** Hairpinning. Split-horizon DNS.

**All connections dropping when the router restarts.** The table was the state.

**A protocol that works and reports the wrong address to its peer.** It embeds addresses.
§33.3.

> **Network+ note.** Objective 2.2 expects NAT, PAT and port forwarding. **This is
> examined directly.** Over-learn: **static NAT is one-to-one and permits inbound**;
> **dynamic NAT uses a pool and fails when it exhausts**; **PAT/overload shares one public
> address using port numbers and is what almost everyone runs**; and **port forwarding is
> a manually-created translation entry.** The "how many hosts can share one address"
> question appears, and the answer is bounded by ports.
