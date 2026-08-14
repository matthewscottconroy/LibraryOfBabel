# 39.1 From HOSTS.TXT to Hierarchy

Until 1983, the entire Internet's name-to-address mapping was **one text file, maintained
by one person, distributed by FTP.**

Understanding why that worked, and precisely how it stopped working, is the best possible
introduction to what DNS is for — because every feature of DNS is an answer to a specific
way HOSTS.TXT failed.

## HOSTS.TXT

**A single file at SRI-NIC**, in Menlo Park, California:

```
NET : 10.0.0.0 : ARPANET :
GATEWAY : 10.0.0.77 : SRI-NIC : PDP-11/23 : MOS : TCP/FTP :
HOST : 10.0.0.51 : SRI-NIC,NIC : PDP-10 : TOPS20 : TCP/TELNET,TCP/FTP :
HOST : 10.1.0.5 : UCLA-CCN : IBM-3033 : OS/MVS : TCP/TELNET :
```

**The process:**

1. You wanted a hostname → **you telephoned or emailed SRI-NIC**
2. **Elizabeth Feinler and her team** edited the file by hand
3. Everyone else **downloaded it by FTP**, periodically
4. Your machine consulted its local copy

And it worked well for over a decade, which is worth acknowledging. For a few hundred
hosts administered by people who all knew each other, a central file is simple, correct,
and easy to reason about.

Its descendant is still on your machine — `/etc/hosts`, or
`C:\Windows\System32\drivers\etc\hosts` — and it is still consulted **before** DNS, which
is occasionally very useful and occasionally the cause of a fault nobody can find.

## The four ways it broke

Each one maps directly onto a DNS design decision, which is why this history is worth
knowing.

### 1. Traffic

**Every host on the Internet downloaded the entire file, repeatedly.**

As the network grew, the file grew, and the number of machines downloading it grew — so
the load on SRI-NIC rose with the *product* of the two.

By 1982 a single machine was serving the whole Internet's naming, and the growth was
visibly unsustainable.

> **A centralised resource whose load grows with the square of the network cannot scale.**

**DNS's answer: distribute the data**, so no single server holds or serves it all.

### 2. Name collisions

**One flat namespace, and everyone wanted the obvious names.**

Two universities both wanting the host `ORION` had to negotiate — through SRI-NIC, by
telephone. As the network grew, every good name was taken, and there was no principled
way to allocate them.

> **A flat namespace has no way to delegate authority over parts of itself.**

**DNS's answer: hierarchy**, so `orion.mit.edu` and `orion.stanford.edu` can both exist and
neither institution needs the other's permission.

### 3. Consistency

**Everyone had a different copy.**

You downloaded the file on Tuesday; a host was added on Wednesday; you could not reach it
until you downloaded again. There was no way to know your copy was stale, and no way to
learn about a change without re-fetching everything.

**DNS's answer: query on demand**, so you always ask for what you need when you need it,
plus **caching with an explicit expiry** (§39.4) so the cost is bounded and staleness is
bounded too.

### 4. The bottleneck of authority

**One organisation approved every name on the Internet.**

Slow, and — more fundamentally — it did not scale as a *human* process. Feinler's team
was not the constraint because they were slow; they were the constraint because **there is
no number of people who can approve every name for a global network.**

> **The binding constraint was administrative, not technical.**

**DNS's answer: delegation.** `.edu` delegates to MIT; MIT delegates to its departments;
**and nobody above needs to be told about names below.**

## Mockapetris's design

**Paul Mockapetris, at USC/ISI, 1983.** RFCs 882 and 883, revised in 1987 as **RFC 1034**
(concepts) and **RFC 1035** (implementation) — which are still the specification.

Four ideas, and each answers one of the four failures above:

| Idea | Answers |
|---|---|
| **Hierarchical namespace** | collisions |
| **Delegation of authority** | the administrative bottleneck |
| **Distributed servers** | the traffic problem |
| **Caching with a TTL** | consistency and load |

**The design brief was demanding and unusual:** it had to work **before** the thing it names
is reachable, it had to be **incrementally deployable** alongside HOSTS.TXT, and it had to
have **no single point of failure** — while still producing one authoritative answer.

## The namespace

```
                         .  (root)
                         │
        ┌────────┬───────┼───────┬────────┐
       com      org     edu     uk       arpa
        │        │       │       │         │
    example   example   mit    co        in-addr
        │                │       │
      www              www    example
                               │
                             www
```

**Read right to left.** `www.example.com` is:

- `www`, within
- `example`, within
- `com`, within
- **the root** — the trailing dot that is almost always omitted

**`www.example.com.`** — with the final dot — is the **fully qualified domain name**, and
the dot is the root. Most software adds it for you, which is why you rarely see it and
why it occasionally matters (§39.2's search domains).

**The tree's properties are the design:**

**Each node's parent delegates authority over it.** `.com` says *"ask `example.com`'s
servers about anything under `example.com`"* — and then **stops caring.** The root does not
know `www.example.com` exists.

**Names are unique by construction.** `orion.mit.edu` and `orion.stanford.edu` cannot
collide because the path from the root differs.

**Administration is local.** Adding `newserver.example.com` requires **nobody's
permission** — you edit your own zone.

> **This is the same argument as Chapter 26 §26.3's route aggregation and Chapter 31
> §31.4's OSPF areas: hierarchy is what makes a large system's state tractable, and
> delegation is what makes its administration tractable.**

Two different problems — state and authority — solved by the same structure, and DNS is
the clearest example in this book because it solves both simultaneously and visibly.

## Zones versus domains

**The distinction that confuses everyone**, and it matters for §39.3.

**A domain** is a subtree of the namespace — **everything under a name.**

**A zone** is the part of a domain that one server is authoritative for, excluding
anything delegated away.

```
   Domain: example.com  — everything under it

   Zone: example.com    — managed in one file:
       www.example.com
       mail.example.com
       ftp.example.com
       eng.example.com   NS  ns1.eng.example.com   ← DELEGATED

   Zone: eng.example.com — a SEPARATE zone, possibly on other servers:
       build.eng.example.com
       test.eng.example.com
```

> **The domain `example.com` includes `build.eng.example.com`. The zone `example.com` does
> not.**

A zone is an administrative unit; a domain is a naming unit. The zone stops where
delegation begins, and the boundary is created by an **NS record** (§39.3).

## Case, and other properties

**DNS names are case-insensitive** for matching — `WWW.Example.COM` and `www.example.com`
are the same name.

**But case is preserved** in responses, and **0x20 encoding** exploits this: a resolver
randomises the case of its query, and a valid response must echo it exactly — **adding
entropy against spoofing** (§39.4) at no protocol cost.

Labels are limited to 63 characters, and a full name to **255**.

**Internationalised names** are encoded into ASCII with **Punycode** (RFC 3492):
`münchen.de` becomes `xn--mnchen-3ya.de`. **Which created a security problem** —
homograph attacks, where Cyrillic `а` renders identically to Latin `a` — and browsers now
display Punycode when a name mixes scripts suspiciously.

## What DNS is used for beyond names

Worth flagging early, because it explains the record types of §39.3:

- **Mail routing** — MX records
- **Service discovery** — SRV records; how a client finds an LDAP or SIP server
- **Anti-spam** — SPF, DKIM and DMARC, all in TXT records
- **Certificate issuance** — ACME's DNS-01 challenge proves domain control
- **Reverse lookup** — address to name, via `in-addr.arpa`
- **Load distribution and failover** — multiple A records, low TTLs
- **CDN steering** — the resolver's location determines which address is returned

> **DNS became the Internet's general-purpose distributed database**, which is far beyond
> what Mockapetris designed and is a consequence of its being the one lookup service every
> host already trusts.

## Why it still matters

**DNS is on the critical path of essentially every connection.**

Before a browser can open a TCP connection, before TLS, before HTTP — **a name must
resolve.** Which makes DNS:

The most common cause of "the network is broken" (Chapter 22 §22.4). `ping 8.8.8.8`
working while `ping google.com` fails identifies it in two commands, and a very large share
of reported network faults are exactly this.

**A single point of failure with global consequences.** Chapter 39 §39.4's outages —
Dyn 2016, Facebook 2021 — took down large parts of the Internet through DNS alone.

**A security-critical system.** Control the answer and you control where the traffic goes,
which is why §39.4's DNSSEC and encrypted-transport work exists.

## What breaks here

**`/etc/hosts` overriding DNS.** Checked first, and an entry nobody remembers adding
produces a fault that no DNS diagnostic will find.

**A name that resolves for you and not for others.** Local cache, local hosts file, or a
split-horizon configuration.

**A stale answer after a change.** Caching and TTLs (§39.4) — the change is correct and
the world has not heard yet.

**Confusing a zone with a domain.** The zone stops at delegation, and a record in the wrong
zone file is served by nobody.

> **Network+ note.** Objective 1.6 expects DNS and its hierarchy. Over-learn: **DNS
> resolves names to addresses**; **the namespace is hierarchical with delegation**; **a FQDN
> ends at the root**; and **a zone is what one server is authoritative for while a domain is
> the whole subtree.** The zone/domain distinction is examined and commonly confused.
