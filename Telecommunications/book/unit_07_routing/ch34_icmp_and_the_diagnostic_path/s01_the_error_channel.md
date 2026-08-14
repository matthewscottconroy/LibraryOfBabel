# 34.1 The Error Channel

IP is best-effort and drops packets without explanation (Chapter 24 §24.1). That is
correct, and it would be intolerable if there were no way at all to find out what happened.

**ICMP is the way.** It is IP's error channel, its diagnostic instrument, and — because it
is neither of those things reliably — the source of a specific class of failure that
appears throughout this book.

## What it is

Internet Control Message Protocol, RFC 792, 1981. Postel again, and again a document
that has never been revised.

It carries control and error messages for IP, and its position is awkward in a way
worth stating precisely:

- It is **carried inside IP** — protocol number **1** — so it sits above IP
- It is **part of IP** — a router implementing IP must implement ICMP (RFC 1122 says so)
- It reports on IP's operation, so it is IP's **control plane**

IP's control plane travelling inside IP's data plane. Chapter 22 §22.2 places it at
Layer 3; Chapter 21 §21.4 explains why the placement is uncomfortable.

**It is not a transport protocol.** There are no ports, no connections, no reliability.
An ICMP message is a single datagram, sent once, and if it is lost nobody retries.

## The message format

```
 0                   1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+---------------+---------------+-------------------------------+
|      Type     |      Code     |           Checksum            |
+---------------+---------------+-------------------------------+
|              rest of header (depends on type)                 |
+---------------------------------------------------------------+
|   the IP header + first 8 bytes of the offending datagram     |
+---------------------------------------------------------------+
```

Four bytes of header, then a payload that depends on the type.

**The last row is the important one.** An error message carries a copy of the packet
that caused it — its IP header plus the first 8 bytes of its payload.

**Why 8 bytes?** Because for TCP and UDP that is enough to include the source and
destination ports — which is exactly what the sender needs to work out which of its
connections the error refers to.

This is a deliberate layer violation, and it is necessary. An ICMP error arriving at a
host is useless unless the host can attribute it to a socket, and attributing it requires
reading the transport header of the original packet out of the ICMP payload. A Layer 3
message carrying enough Layer 4 information to be actionable.

It also means NAT must rewrite inside the ICMP payload (Chapter 33 §33.3) — the
embedded header contains translated addresses, and if the NAT does not fix them the error
is unattributable. Implementations that get this wrong produce path-MTU black holes that
appear only across the NAT.

## The types that matter

| Type | Name | Meaning |
|---|---|---|
| **0** | **Echo Reply** | *"I am here"* — `ping`'s reply |
| **3** | **Destination Unreachable** | delivery failed; **the code says why** |
| 5 | Redirect | *"use that router instead"* (Chapter 29 §29.4) |
| **8** | **Echo Request** | `ping` |
| **11** | **Time Exceeded** | **TTL hit zero** — `traceroute`'s mechanism |
| 12 | Parameter Problem | a malformed header field |
| 13 / 14 | Timestamp Request / Reply | largely disabled; leaks information |

Types 0, 3, 8 and 11 are the ones to know, and between them they cover essentially all
practical use.

### Type 3 — Destination Unreachable, by code

The type whose codes are the diagnosis, and they are worth memorising because each
names a different fault at a different layer:

| Code | Meaning | Diagnosis |
|---|---|---|
| **0** | Net unreachable | **no route to that network** |
| **1** | Host unreachable | route exists; **ARP failed** — the host is not there |
| 2 | Protocol unreachable | the host does not run that IP protocol |
| **3** | **Port unreachable** | **nothing is listening on that UDP port** |
| **4** | **Fragmentation needed and DF set** | **path MTU discovery** (§34.4) |
| 9 / 10 | Communication administratively prohibited | **a firewall said no, and admitted it** |
| 13 | Communication administratively filtered | same |

**Read the code, not just the type.** "Destination unreachable" is not a diagnosis; code
1 means the host is absent and code 3 means the host is present and not listening, and
those are entirely different problems.

Code 4 is the one this book has referred to repeatedly. It is the mechanism of path MTU
discovery, and §34.4 covers what happens when it is filtered.

Codes 9, 10 and 13 are a firewall being polite. A firewall that sends them tells you it
dropped your packet; a firewall that drops silently tells you nothing. §34.2 develops why
the difference matters.

## Rate limiting

Routers generate ICMP sparingly, and this shapes what you can conclude from its
absence.

RFC 1812 requires rate limiting, and typical defaults are a few messages per second per
destination. The reasons are sound:

- Generating ICMP is a control-plane task (Chapter 29 §29.1) — the CPU, not the
  forwarding hardware
- Without limits, a flood of undeliverable packets becomes a flood of ICMP, **amplifying
  the problem**
- ICMP has been used for amplification attacks (Chapter 27 §27.2's Smurf)

**The diagnostic consequence is large:**

> A router that does not reply is not necessarily a router that dropped your packet.

`traceroute` showing `* * *` at a hop almost always means that router is rate-limiting
or declining to generate ICMP, while forwarding traffic perfectly (Chapter 24 §24.4).
Enormous amounts of time are wasted by people who read stars as loss.

## Why filtering all ICMP is wrong

One of the most persistent bad practices in network security, and it is worth
addressing directly because it appears in real firewall policies constantly.

**The reasoning behind it:** ICMP can be used for reconnaissance (ping sweeps), for covert
channels (data in echo payloads), and historically for attacks (Smurf, Ping of Death).
**All true.**

**What blocking all ICMP actually breaks:**

| Blocked | Consequence |
|---|---|
| **Type 3 code 4** | **Path MTU discovery fails.** Large transfers hang; small ones work. §34.4 |
| Type 11 | `traceroute` stops working — you lose your primary path diagnostic |
| Type 3 codes 0–3 | Failures become timeouts instead of immediate errors |
| Type 0/8 | You cannot verify basic reachability |

**The first row is the serious one.** Chapter 24 §24.3's black hole is caused by exactly
this, it is common, and the symptom looks nothing like a firewall problem — which is
why it costs so much time.

**And in IPv6 it is fatal.** ICMPv6 carries NDP, router discovery and address resolution
(Chapter 18 §18.4), so blocking ICMPv6 breaks IPv6 entirely. RFC 4890 exists to say
which types must pass.

**The correct policy:**

| Permit | Why |
|---|---|
| **Type 3 code 4** inbound and outbound | **non-negotiable** — PMTUD |
| Type 3 (other codes) | fast failure instead of timeouts |
| **Type 11** | traceroute, and TTL diagnosis |
| Type 0/8, rate-limited | reachability testing |
| Type 12 | malformed header reporting |

| Consider blocking | Why |
|---|---|
| Type 5 (Redirect) | a host-routing attack (Chapter 29 §29.4) |
| Types 13–18 | information leaks, no modern use |
| Unrestricted echo inbound | reconnaissance, and it is a reasonable trade |

> Rate-limit ICMP. Do not block it. And never block type 3 code 4.

## ICMPv6

IPv6's version is **not** a straight port. It absorbed several functions that were
separate protocols in IPv4:

| Function | IPv4 | IPv6 |
|---|---|---|
| Errors and echo | ICMP | ICMPv6 |
| **Address resolution** | **ARP** (its own EtherType) | **ICMPv6 — NDP** |
| **Router discovery** | ICMP router discovery, rarely used | **ICMPv6 — RA/RS** |
| **Multicast group management** | **IGMP** (its own protocol) | **ICMPv6 — MLD** |

Three protocols folded into one, which is why ICMPv6 is load-bearing in a way ICMP
never was.

**The types:**

| Type | Meaning |
|---|---|
| 1 | Destination Unreachable |
| **2** | **Packet Too Big** — PMTUD, **and a separate type rather than a code** |
| 3 | Time Exceeded |
| 4 | Parameter Problem |
| 128 / 129 | Echo Request / Reply |
| **133–137** | **NDP** — RS, RA, NS, NA, Redirect (Chapter 18 §18.4) |

**Note type 2.** IPv6 gave "packet too big" its own type rather than burying it as a code,
which reflects how important PMTUD became once routers were forbidden to fragment
(Chapter 24 §24.3). In IPv6 the source *must* discover the path MTU; there is no fallback.

Under 128 is an error; 128 and above is informational — a clean split that IPv4's
numbering lacks.

## What breaks here

**Large transfers hanging while small ones work.** Type 3 code 4 filtered. §34.4, and it
is the single most valuable diagnosis in this chapter.

**`traceroute` showing stars mid-path with connectivity working.** Rate limiting. **Not a
fault.**

**A connection failing slowly instead of quickly.** ICMP errors filtered, so a refusal
became a timeout.

IPv6 not working at all after a firewall change. ICMPv6 blocked. RFC 4890.

An ICMP error that cannot be attributed to a connection. NAT did not rewrite the
embedded header.

> **Network+ note.** Objective 1.4 expects ICMP. Over-learn: **it is IP protocol 1**;
> type 8 echo request, type 0 echo reply, type 11 time exceeded, type 3 destination
> unreachable; the type 3 codes, especially 4 (fragmentation needed) and 3 (port
> unreachable); and blocking all ICMP breaks path MTU discovery, and breaks IPv6
> entirely. The type numbers are examined directly.
