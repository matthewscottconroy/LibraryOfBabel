# 27.2 Special Ranges and APIPA

A handful of addresses do not behave like ordinary addresses. Each is a diagnostic
signal, and recognising them on sight converts a long investigation into a glance.

## `169.254.0.0/16` — link-local, and the most useful diagnostic in networking

**RFC 3927.** An address a host assigns **itself** when it cannot obtain one any other
way.

**The mechanism:**

1. DHCP is attempted, and fails or times out.
2. The host picks a **random** address in `169.254.1.0` – `169.254.254.255` (the first
   and last /24 are reserved).
3. It ARPs for that address to check nothing else holds it (Chapter 18 §18.3's ACD).
4. If something replies, it picks another and repeats.
5. It configures the address with mask `255.255.0.0`, **no default gateway**, and **no
   DNS server**.

**Microsoft calls this APIPA** — Automatic Private IP Addressing. Same thing.

### What it means when you see it

```
$ ip addr show eth0
    inet 169.254.201.44/16 scope link
```

> **This host did not get a DHCP reply.**

Not "DHCP is slow", not "the address is wrong" — **no reply arrived**, and the host gave
up and self-assigned. That single observation eliminates enormous parts of the search
space and points directly at a small set of causes:

| Cause | Check |
|---|---|
| DHCP server down or out of leases | server status, scope utilisation |
| **Wrong VLAN on the switch port** | `show interfaces switchport` |
| DHCP relay not configured on the router | the relay/helper address |
| Cable or link problem | Chapter 22 §22.4's Layer 1 checks |
| Port in spanning-tree listening/learning | **PortFast missing** (Chapter 19 §19.3) |
| Rogue DHCP server handing out nothing | DHCP snooping |

**The wrong-VLAN cause is the most common in enterprise networks**, and it is worth
noticing why: the port is up, the link light is on, everything at Layer 1 and 2 looks
perfect, and the host is simply in a broadcast domain where no DHCP server or relay
exists.

### What works and what does not

**Works:** communication with **other link-local hosts on the same segment.** Two
machines that both failed DHCP can reach each other.

**Does not work:** anything requiring a gateway or DNS. **There is no default route**,
so nothing off-segment is reachable at all.

Which produces a distinctive symptom: *"I can see the other computer in the office but I
can't get to the internet."*

### Where it is used deliberately

Not only a failure mode:

- **Two laptops with a crossover cable**, no DHCP anywhere, needing to exchange a file.
- **mDNS / Bonjour / Zeroconf** — printers and media devices discoverable without
  infrastructure.
- **AWS instance metadata** at `169.254.169.254` — a link-local address every cloud
  provider now uses for the same purpose, which is a small piece of accidental
  standardisation worth knowing.

### The IPv6 comparison

**IPv6 link-local (`fe80::/10`) is not a failure mode.** Every IPv6 interface has one,
always, in addition to any global address, and NDP depends on it (Chapter 18 §18.4).

**In IPv4 a link-local address means something went wrong. In IPv6 it means the interface
is up.** Confusing the two produces false alarms during IPv6 deployments.

## `127.0.0.0/8` — loopback

**The whole /8**, not just `127.0.0.1`. 16.7 million addresses for one purpose, which is
an allocation decision that looks extravagant now and reflects 1981's assumptions.

**Behaviour:**

- Traffic **never leaves the host**. The stack loops it back internally.
- The interface is always up, whatever the physical state of anything.
- A packet with a `127.x.x.x` **source or destination** arriving on a real interface is
  **martian** and must be discarded — it is spoofed by definition.

**Why the whole /8 is occasionally useful:** you can bind different services to
`127.0.0.2`, `127.0.0.3` and so on, giving each its own address without any external
visibility. Some test harnesses and container arrangements use this.

**`localhost` normally resolves to `127.0.0.1`** — and to `::1` on IPv6, which is a
common source of confusion. A service listening only on IPv4 `127.0.0.1` is unreachable
by a client that resolves `localhost` to `::1` first, and the symptom is *"connection
refused to my own machine"*.

**Check with `ss -tlnp`**: `127.0.0.1:8080` and `[::1]:8080` are different listeners.

## `0.0.0.0` — several meanings

Context-dependent, and worth separating:

| Context | Meaning |
|---|---|
| **Source address** | "I do not have an address yet" — a DHCP client (Chapter 40 §40.2) |
| **Destination in a routing table** | `0.0.0.0/0` — the **default route** |
| **A bind address** | "listen on **all** interfaces" |
| A route's next hop | "directly connected" |

**`0.0.0.0` as a bind address is worth understanding.** A service bound to `0.0.0.0:80`
accepts connections on every interface, including ones you did not intend. A service
bound to `127.0.0.1:80` accepts only local connections.

**This distinction is a security control**, and `ss -tlnp` showing `0.0.0.0:*` for a
database is a finding.

## Broadcast addresses

Two kinds, and they behave differently.

**Directed broadcast** — the all-ones host portion of a specific network, such as
`192.168.10.255` for `192.168.10.0/24`.

**Historically routable**, and it was a serious problem: the **Smurf attack** sent ICMP
echo requests to a network's directed broadcast address with a spoofed source, so every
host on that network replied to the victim, amplifying the attack by the host count.

**RFC 2644 (1999) made "do not forward directed broadcasts" the required default.** It is
`no ip directed-broadcast` on Cisco equipment and has been the default for over two
decades.

**Limited broadcast** — `255.255.255.255`. **Never forwarded by any router**, under any
configuration. Used by DHCP clients that have no address and no idea what network they
are on (Chapter 40 §40.2).

## Martian addresses

Addresses that must never appear as a source on a real interface:

| Range | Why |
|---|---|
| `0.0.0.0/8` | "this network" |
| `127.0.0.0/8` | loopback — must not be on the wire |
| `169.254.0.0/16` | link-local — not routable |
| `224.0.0.0/4` | multicast — not valid as a source |
| `240.0.0.0/4` | reserved |
| RFC 1918 ranges | **on an Internet-facing interface** |

**Filtering martians inbound at your border is basic hygiene**, and it is what
**BCP 38** (RFC 2827) formalises: a network should not emit packets with source addresses
it does not own.

**BCP 38 is twenty-five years old, universally recommended, and incompletely deployed** —
which is why source-address spoofing still works and why reflection and amplification
attacks remain viable (Chapter 62 §62.4). It is the clearest case in this book of a
security measure whose cost falls on one party and whose benefit falls on everyone else,
which is Arkko's observation from Chapter 18's notes.

## Recognition table

Worth committing to memory as a set of instant diagnoses:

| See this | Conclude |
|---|---|
| `169.254.x.x` | **DHCP failed** |
| `127.x.x.x` | loopback, never on the wire |
| `100.64.x.x` – `100.127.x.x` | **behind carrier-grade NAT** |
| `192.0.2.x`, `198.51.100.x`, `203.0.113.x` | documentation — someone copied an example |
| `0.0.0.0` as a source | a DHCP client with no address |
| `255.255.255.255` | limited broadcast, local only |
| `224.x.x.x` – `239.x.x.x` | multicast |
| `172.32.x.x` | **public**, and probably a mistake |

## What breaks here

**A host on `169.254.x.x`.** DHCP failed. Check the VLAN first, then the relay, then the
server.

**A whole subnet on `169.254.x.x`.** The DHCP relay is missing on that segment's router,
or the scope is exhausted.

**A service unreachable on `localhost`.** IPv4/IPv6 mismatch — the client resolved to
`::1` and the server is listening on `127.0.0.1`.

**A service unexpectedly reachable from outside.** Bound to `0.0.0.0` instead of
`127.0.0.1`.

**Documentation addresses in production.** Somebody copied an example without changing
it. Harmless until it is not.

**Traffic from RFC 1918 sources arriving from the Internet.** Spoofed. Filter it.

> **Network+ note.** Objective 1.7 expects APIPA/link-local and loopback. **This is
> examined and it is genuinely useful.** Over-learn: **`169.254.x.x` means the host could
> not reach a DHCP server**; **`127.0.0.1` is loopback**; **`255.255.255.255` is never
> forwarded**; and **`100.64.0.0/10` means carrier-grade NAT**. Expect a scenario where a
> user has an APIPA address and you must identify the cause.
