# Chapter 39 — Further Reading

## Primary sources

**RFC 1034 — Mockapetris, P. (1987). *Domain Names — Concepts and Facilities.***
**RFC 1035 — Mockapetris, P. (1987). *Domain Names — Implementation and Specification.***
**The specification, unchanged in its essentials for thirty-eight years.** RFC 1034 is the
readable one — **read §2 for the design rationale**, which states the four HOSTS.TXT
failures in the author's own words.

**RFC 2181 — Elz, R. & Bush, R. (1997). *Clarifications to the DNS Specification.***
The corrections that fifteen years of implementation experience produced, including the
CNAME and MX rules of §39.3. **Short, and it settles a great many arguments.**

**RFC 2308 — Andrews, M. (1998). *Negative Caching of DNS Queries.***
Why a newly-created record does not work immediately. Four pages.

**RFC 6891 — Damas, J., Graff, M. & Vixie, P. (2013). *Extension Mechanisms for DNS
(EDNS0).***
Escaping the 512-byte limit — and read **RFC 9715 (2025)** alongside it, which recommends
retreating to ~1,232 bytes because of fragmentation. **The two together are a complete arc.**

**RFC 4033 / 4034 / 4035 — Arends, R. et al. (2005). *DNS Security Extensions.***
DNSSEC. **RFC 4033 is the overview** and is the one to read; the others are for
implementers.

**RFC 7858 (DoT) and RFC 8484 (DoH).**
Encrypted transport. **RFC 8484's §8 on privacy considerations** is the honest treatment of
what DoH does and does not achieve, written by its authors.

**RFC 9460 — Schwartz, B., Bishop, M. & Nygren, E. (2023). *Service Binding and Parameter
Specification via the DNS (SVCB and HTTPS RRs).***
**The standard answer to the apex-CNAME problem**, plus protocol hints that save a round
trip. Worth knowing about; adoption is now substantial.

**Kaminsky, D. (2008). "Black Ops 2008: It's The End Of The Cache As We Know It."**
Black Hat presentation. The attack, explained by its discoverer, after the coordinated
patch. **The disclosure timeline is as instructive as the attack.**

## Books

**Liu, C. & Albitz, P. (2006). *DNS and BIND*, 5th ed. O'Reilly.**
**The reference.** Dated in its BIND specifics and correct in everything conceptual. The
zone-file, delegation and troubleshooting chapters are the best available treatment.

**Liu, C. (2011). *DNS and BIND on IPv6.* O'Reilly.**
The short companion for AAAA and `ip6.arpa`.

**Hunt, C. (2010). *DNS Security Management.*** and **Rose, S. et al. — NIST SP 800-81r2,
*Secure Domain Name System (DNS) Deployment Guide*.**
The operational security material. NIST's guide is free and is the more practical of the
two.

**Grigorik, I. (2013). *High Performance Browser Networking*, chapter 1.**
**Freely at hpbn.co.** DNS as a component of page-load latency — why the resolution walk
matters to anyone building on top of it.

## Applied

**`dig +trace` — run it now.**

```bash
dig +trace www.example.com
dig +trace mail.eng.example.co.uk
```

**Watching the referrals descend the tree is the single most instructive thing in this
chapter**, and it takes ten seconds.

**The diagnostic sequence of §39.4**, learned as a habit:

```bash
dig name                          # does it resolve?
dig name @ns1.authoritative       # is it right at the source?
dig name @8.8.8.8                 # is it my resolver specifically?
dig +trace name                   # is the delegation right?
dig name | grep flags             # cached (no 'aa') or authoritative?
```

**Watch a TTL count down** (exercise F3):

```bash
dig +noall +answer example.com; sleep 10; dig +noall +answer example.com
```

**The second TTL is ten lower.** That single observation makes caching concrete and proves
you are talking to a cache rather than an authority.

**`dig CHAOS TXT id.server @1.1.1.1`** and `dig +nsid` — which anycast instance answered.
Run it from two networks and compare (Chapter 27 §27.3).

**Run an authoritative server.** BIND, **Knot**, **NSD** or **PowerDNS**, for a test zone.
**Exercise F2** — add every record type, verify each with `dig`, then deliberately make each
of the zone-file errors and observe how each fails. **A morning's work, and it makes §39.3
permanent.**

**`unbound` or `knot-resolver`** for the recursive side, with `verbosity: 3` so you can
watch it walk.

**Zone checking tools:** **Zonemaster** (zonemaster.net), **DNSViz** (dnsviz.net) for
DNSSEC chain visualisation, and `named-checkzone`. **DNSViz in particular** turns a DNSSEC
failure from an opaque error into a picture of exactly which link in the chain broke.

**`dnsperf`** for load testing, and `dnstop` or a `tcpdump port 53` capture to see what your
network is actually asking for. **Exercise F6** — the most-queried names on a busy network
are usually not what people expect.

**Check your own domain:**

```bash
dig example.com AXFR @ns1.example.com     # should FAIL from an arbitrary host
dig +dnssec example.com                   # is it signed?
dig example.com NS                        # do parent and child agree?
```

**Lab 28** in this book's [labs/](../../../labs/) directory builds an authoritative server
and a recursive resolver, delegates a subzone between them, then stages a TTL migration and
measures propagation, and finally demonstrates cache poisoning against a deliberately
misconfigured resolver on an isolated network.

## For the certification-minded

**Objective 1.6 is DNS and it is examined heavily** — record types especially. Objective
4.2 expects DNS poisoning.

**The record types are the largest memorisation item:**

| Type | Purpose |
|---|---|
| **A** | IPv4 address |
| **AAAA** | IPv6 address |
| **CNAME** | alias |
| **MX** | mail exchanger — **lower preference first** |
| **NS** | delegation |
| **PTR** | reverse lookup |
| **SOA** | zone authority |
| **TXT** | arbitrary text — SPF, DKIM, DMARC |
| **SRV** | service location, **with a port** |

Six more things worth over-learning:

1. **DNS uses UDP/53, and TCP/53 for large responses and zone transfers.**
2. **The stub asks recursively; the resolver queries iteratively.**
3. **Root → TLD → authoritative.**
4. **TTL controls caching**, and **lower it before a migration, not during.**
5. **A zone is what a server is authoritative for; a domain is the whole subtree.**
6. **DNSSEC provides authenticity and integrity, not confidentiality.**

The most-missed items are **the MX preference direction** (lower wins) and
**recursive-versus-iterative** (which is frequently reversed).

And the three operational facts worth more than the objective:

**`ping 8.8.8.8` works and `ping google.com` does not** — this is DNS, it is one of the
commonest reported "network" faults, and two commands identify it.

**Lower the TTL before the change and wait out the old one.** Forgetting this turns a
five-minute migration into an eight-hour one.

**Use two DNS providers for anything that matters.** Dyn 2016 is the argument, and it is
one of the cheapest resilience measures available.
