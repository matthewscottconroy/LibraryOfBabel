# Chapter 34 — Further Reading

## Primary sources

**RFC 792 — Postel, J. (1981). *Internet Control Message Protocol.***
Twenty-one pages, never revised. Read it for the message formats and for the reasoning
behind carrying the offending packet's header plus eight bytes.

**RFC 1122 §3.2.2 — Braden, R. (1989). *Requirements for Internet Hosts.***
What a host must do with each ICMP message it receives. The normative statement behind
§34.1's "ICMP is part of IP".

**RFC 1812 §4.3 — Baker, F. (1995). *Requirements for IP Version 4 Routers.***
What a router must generate, and the rate-limiting requirement that produces §34.3's
`* * *`.

**RFC 1191 — Mogul, J. & Deering, S. (1990). *Path MTU Discovery.***
The mechanism, **and its authors' own discussion of what happens when the ICMP is
filtered.** They predicted the black hole and recommended a fallback nobody implemented.

**RFC 4821 — Mathis, M. & Heffner, J. (2007). *Packetization Layer Path MTU Discovery.***
The robust alternative. **Build a mechanism that does not depend on a third party who
frequently does not cooperate.**

**RFC 8201 — McCann, J., Deering, S., Mogul, J. & Hinden, R. (2017). *Path MTU Discovery
for IPv6.***
Where PMTUD is the only mechanism, because routers must not fragment.

**RFC 4443 — Conta, A., Deering, S. & Gupta, M. (2006). *ICMPv6.***
The IPv6 version, including the under-128 / 128-and-above split.

**RFC 4890 — Davies, E. & Mohacsi, J. (2007). *Recommendations for Filtering ICMPv6
Messages in Firewalls.***
**Read this before writing any IPv6 firewall rule.** Type by type, what must pass and why.
It exists because people kept breaking IPv6 with IPv4 habits.

**RFC 8900 — Bonica, R. et al. (2020). *IP Fragmentation Considered Fragile.***
Why DF is set on everything now, which is why PMTUD matters more than it used to.

## Books

**Stevens, W. R. (1994). *TCP/IP Illustrated, Volume 1*, chapters 6, 7, 8 and 11.**
ICMP, ping, traceroute and path MTU discovery, with real captures throughout. **Chapter 8
on traceroute is the clearest explanation of the TTL trick anywhere**, and chapter 11's
treatment of PMTUD predates the black hole becoming common.

**Fall, K. & Stevens, W. R. (2011). *TCP/IP Illustrated, Volume 1*, 2nd ed., chapter 8.**
The revision, covering ICMPv6 and modern PMTUD failure modes.

**Cheswick, W., Bellovin, S. & Rubin, A. (2003). *Firewalls and Internet Security*,
2nd ed.**
**The authoritative treatment of what ICMP filtering actually buys**, from people whose
professional interest is security. Their conclusion — selective filtering and rate
limiting, not blanket blocking — is worth citing when this argument arises at work.

**Limoncelli, T., Hogan, C. & Chalup, S. (2016). *The Practice of System and Network
Administration*, 3rd ed.**
The debugging discipline around these tools.

## Applied

**`ping -M do -s 1472 <host>`. Learn this before you need it.** Two minutes with this
command identifies a PMTUD black hole with certainty, and the day you need it you will be
under pressure and unable to look it up calmly.

**`tracepath <host>`** — does the binary search automatically and reports where the MTU
changes.

**Run all four traceroute variants** to the same destination and compare:

```bash
traceroute host
traceroute -I host
traceroute -T -p 443 host
mtr --report --report-cycles 100 host
```

**Exercise F4**, and the differences are §34.3's point. **Do it once and the "traceroute
shows nothing but the site works" scenario becomes instantly recognisable.**

**`mtr --report --report-cycles 200`** to several distant destinations. For every hop
showing loss, apply §34.3's rule and decide whether it is real. **Most is not**, and being
able to say so with confidence is worth a great deal.

**`arping -I eth0 <host>`** when ping fails. Chapter 18 §18.3's tool, and the fastest way
to separate "not there" from "there and filtered".

**Wireshark filters:** `icmp`, `icmp.type == 3 && icmp.code == 4`, `icmpv6`,
`icmpv6.type == 2`. **Capture a fragmentation-needed message and decode the embedded
original header by hand** (exercise F6) — confirming that the first 8 bytes contain the
ports makes §34.1's design decision concrete.

**`sysctl net.ipv4.tcp_mtu_probing`** — check whether your systems have PLPMTUD enabled.
Many do not.

**`tc qdisc` and a reduced-MTU veth pair** to build the lab of exercise F1 on a single
Linux machine, without any hardware.

**Lab 23** in this book's [labs/](../../../labs/) directory builds a three-router topology
with a constrained middle link, verifies PMTUD working, then blocks ICMP type 3 code 4 and
reproduces the black hole with a real file transfer — then fixes it three ways and compares
what each solves.

## Public measurement

**RIPE Atlas** — thousands of probes worldwide, and you can run traceroutes and pings
**from other people's networks toward yours**, which is the only way to see the reverse
path of §34.3.

**Public looking glasses** — most large providers run one. A traceroute from their network
to yours costs nothing and answers questions your own traceroute cannot.

**`bgp.he.net`, RIPEstat** — for correlating a path change with a routing change.

## For the certification-minded

Objective 1.4 expects ICMP; objective 5.5 expects `ping` and `traceroute`; objective 5.2
expects MTU issues as a troubleshooting scenario.

Seven things worth over-learning:

1. **ICMP is IP protocol 1.**
2. **Type 8 echo request, type 0 echo reply, type 11 time exceeded, type 3 destination
   unreachable.**
3. **Type 3 code 4 is fragmentation needed** — the PMTUD mechanism.
4. **Windows `tracert` uses ICMP; Unix `traceroute` uses UDP by default.**
5. **`* * *` means no reply, not no path.**
6. **A successful ping proves Layer 3 in both directions and nothing above it.**
7. **Blocking all ICMP breaks path MTU discovery, and breaks IPv6 entirely.**

**The scenario that appears most often, essentially verbatim:** *connectivity works, small
transfers work, large transfers hang — what is wrong?* The answer is MTU, the mechanism is
the PMTUD black hole, and the diagnostic is `ping -M do -s`.

And the three operational habits worth more than the whole objective:

**When ping fails, use `arping` before concluding the host is down.**

**When traceroute shows loss at one hop and none after it, it is rate limiting.**

**When small things work and large things hang, it is MTU.** Every time.
