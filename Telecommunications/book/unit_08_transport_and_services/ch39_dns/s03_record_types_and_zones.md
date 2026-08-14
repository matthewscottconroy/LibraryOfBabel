# 39.3 Record Types and Zones

DNS stores **resource records**, and there are far more types than the name-to-address
mapping suggests. This section covers the ones you will meet, the zone file that holds
them, and the mistakes each type invites.

## The anatomy of a record

```
   www.example.com.        3600    IN      A       93.184.216.34
   └───────┬───────┘       └─┬─┘   └┬┘    └┬┘     └──────┬─────┘
        NAME               TTL   CLASS   TYPE         RDATA
```

| Field | Meaning |
|---|---|
| **NAME** | the name this record is about |
| **TTL** | **how long it may be cached**, in seconds (§39.4) |
| CLASS | **always `IN`** in practice (Internet) |
| **TYPE** | what kind of record |
| **RDATA** | the value |

**`CH` (Chaos) survives for one purpose:** `dig CHAOS TXT id.server @8.8.8.8` asks an
anycast server which instance you reached (Chapter 27 §27.3) — a diagnostic worth knowing.

## The address records

### A and AAAA

```
www.example.com.    3600    IN  A       93.184.216.34
www.example.com.    3600    IN  AAAA    2606:2800:220:1:248:1893:25c8:1946
```

**A is IPv4; AAAA ("quad-A") is IPv6.** The name comes from the address being four times
larger.

**A name may have both**, and a dual-stack client resolves both and prefers IPv6
(Chapter 28 §28.4's Happy Eyeballs).

**And several of each:**

```
www.example.com.    300     IN  A       93.184.216.34
www.example.com.    300     IN  A       93.184.216.35
www.example.com.    300     IN  A       93.184.216.36
```

**"Round-robin DNS"** — the server returns them in rotating order, giving crude load
distribution.

**Its limitations are worth stating plainly**, because it is often deployed as though it
were high availability:

- **No health checking.** A dead server's address is returned exactly as often as a live
  one's.
- **Clients cache**, so the rotation does not reach them.
- **Clients choose**, and many simply use the first.
- **Recovery is bounded by the TTL**, not by detection.

> **Round-robin DNS distributes load. It does not provide failover.** Anything requiring
> failover needs health checking — a load balancer, or a DNS service that removes dead
> addresses.

## The indirection records

### CNAME — an alias

```
www.example.com.    3600    IN  CNAME   webserver.example.com.
webserver.example.com. 3600 IN  A       93.184.216.34
```

**A resolver following a CNAME issues a second lookup**, and most servers return both
records together to save the round trip.

**Three rules, and breaking them causes real faults:**

**1. A CNAME cannot coexist with other records at the same name.**

```
example.com.    IN  CNAME   something.        × INVALID
example.com.    IN  MX      10 mail.          ← conflicts
```

**Because a CNAME means "this name is entirely an alias"**, so nothing else may be said
about it.

**2. Therefore you cannot CNAME a zone apex.**

**The apex — `example.com` itself — must carry SOA and NS records**, so it cannot be a
CNAME.

**Which is a genuine practical problem**, because CDNs and cloud load balancers want you to
CNAME to their name, and you cannot do it at the apex.

**The workarounds:**

| Approach | Character |
|---|---|
| **ALIAS / ANAME / CNAME-flattening** | a **non-standard** provider feature: the server resolves the target and returns its A record |
| A redirect from apex to `www` | a small extra round trip; universally supported |
| **SVCB / HTTPS records** (RFC 9460) | **the standard answer**, and now widely supported |

**HTTPS records are the proper fix** and worth knowing about — they let a name at the apex
point at a service endpoint, and they also carry protocol hints (ALPN, port, IP hints) that
let a client skip a round trip.

**3. Chains cost round trips and can loop.** Each CNAME is another lookup; a chain of four
is four lookups. **And two CNAMEs pointing at each other is a loop** that resolvers detect
and refuse.

## The mail records

### MX

```
example.com.    3600    IN  MX  10  mail1.example.com.
example.com.    3600    IN  MX  20  mail2.example.com.
example.com.    3600    IN  MX  30  backup.elsewhere.net.
```

**The number is the *preference*, and lower is more preferred** — which reverses most
people's intuition and is worth checking twice.

**A sender tries preference 10 first**, falling back to 20 and then 30.

**Two rules:**

**An MX target must be a name with an A or AAAA record, never a CNAME.** RFC 2181 forbids
it, and some mail servers refuse such a domain outright.

**No MX record means mail goes to the A record** — the "implicit MX" — which is usually not
what anyone wants and is a common cause of mail arriving at a web server.

### TXT — and what it became

```
example.com.  3600  IN  TXT  "v=spf1 include:_spf.google.com ~all"
_dmarc.example.com.  IN  TXT  "v=DMARC1; p=reject; rua=mailto:d@example.com"
sel._domainkey.example.com. IN TXT "v=DKIM1; k=rsa; p=MIGfMA0..."
_acme-challenge.example.com. IN TXT "gfj9Xq...Rg85nM"
```

**TXT was "arbitrary text" and became the Internet's general-purpose assertion mechanism:**

| Use | What it asserts |
|---|---|
| **SPF** | which servers may send mail as this domain |
| **DKIM** | the public key for verifying mail signatures |
| **DMARC** | what to do when SPF or DKIM fails |
| **ACME (`_acme-challenge`)** | **proof of domain control**, for certificate issuance |
| Domain verification | "the holder of this domain also controls this service account" |

**The pattern in every case:** *only the domain's owner can create a TXT record under it*,
**so a TXT record proves control of the domain.** Chapter 60's certificate issuance depends
entirely on this, and so does every "verify your domain" flow.

> **DNS became an authentication mechanism because it is the one namespace whose control is
> already established** — which is also why DNS hijacking is so damaging (§39.4).

## Service discovery

### SRV

```
_sip._tcp.example.com.  3600  IN  SRV  10  60  5060  sipserver.example.com.
                                       ↑   ↑    ↑          ↑
                                  priority weight port    target
```

**The name encodes the service and protocol**, both underscore-prefixed.

**This is what an A record cannot do: return a *port*.** A client looking for the SIP
service learns the host **and the port**, without either being configured.

**Priority works like MX (lower first); weight distributes among equal priorities.**

**Used by:** Active Directory (extensively — domain controller location is entirely SRV),
SIP, XMPP, Minecraft, and much else. **Not by HTTP**, which is why the web needed the
HTTPS/SVCB records above.

## The zone's own records

### SOA — Start of Authority

**Every zone has exactly one, at the apex:**

```
example.com.  3600  IN  SOA  ns1.example.com. admin.example.com. (
                  2024031501  ; serial
                  7200        ; refresh
                  3600        ; retry
                  1209600     ; expire
                  3600 )      ; minimum / negative TTL
```

| Field | Purpose |
|---|---|
| **Primary NS** | the authoritative source for transfers |
| **Admin email** | **with `@` written as `.`** — `admin.example.com` means `admin@example.com` |
| **Serial** | **incremented on every change**; secondaries compare it |
| Refresh | how often a secondary checks |
| Retry | how soon to retry a failed check |
| **Expire** | **how long a secondary serves data it cannot refresh** |
| **Minimum** | **the negative-caching TTL** (§39.4) |

**The serial is the mechanism that makes replication work**, and **forgetting to increment
it is the classic zone-file mistake**: the primary has the change, the secondaries do not,
and answers differ depending on which server a resolver happened to ask. **Intermittent,
inconsistent, and baffling** — and the fix is one number.

**The convention `YYYYMMDDNN`** — `2024031501` being the first change on 15 March 2024 —
makes staleness visible at a glance.

### NS — delegation

```
example.com.        172800  IN  NS  ns1.example.com.
example.com.        172800  IN  NS  ns2.example.com.
eng.example.com.    172800  IN  NS  ns1.eng.example.com.   ← delegating a subzone
```

**NS records appear in two places**: in the **parent** zone (creating the delegation) and in
the **child** zone (the authoritative list).

**When they disagree, resolution behaves unpredictably** — some resolvers use the parent's,
some the child's. **Keeping them in sync is basic zone hygiene** and is checked by tools
like `dnscheck` and Zonemaster.

### PTR — reverse

```
34.216.184.93.in-addr.arpa.  3600  IN  PTR  www.example.com.
```

§39.2's reverse hierarchy. **Controlled by the address block's holder**, not the domain's.

## The zone file

**Putting it together** — BIND's format, which most tools accept:

```
$TTL 3600
$ORIGIN example.com.

@       IN  SOA     ns1.example.com. admin.example.com. (
                    2024031501 7200 3600 1209600 3600 )

@       IN  NS      ns1.example.com.
@       IN  NS      ns2.example.com.
@       IN  MX      10 mail.example.com.
@       IN  A       93.184.216.34
@       IN  TXT     "v=spf1 mx ~all"

ns1     IN  A       192.0.2.53
ns2     IN  A       198.51.100.53
mail    IN  A       192.0.2.25
www     IN  A       93.184.216.34
www     IN  AAAA    2606:2800:220:1::1946
ftp     IN  CNAME   www

eng     IN  NS      ns1.eng.example.com.
ns1.eng IN  A       192.0.2.60                  ← glue for the delegation
```

**Three syntax rules that cause most zone-file errors:**

**`@` means the zone's own name** — the apex.

**A name without a trailing dot has `$ORIGIN` appended.** So `www` becomes
`www.example.com.` — **and `www.example.com` without the dot becomes
`www.example.com.example.com.`**, which is the most common zone-file mistake and
produces a name nobody can resolve.

**A blank name reuses the previous record's name**, which is convenient and makes
diffs harder to read.

## Zone transfers

**How secondaries get the data:**

| | |
|---|---|
| **AXFR** | **full** transfer of the zone |
| **IXFR** | **incremental** — only changes since a given serial |
| **NOTIFY** | the primary tells secondaries to check now, rather than waiting for refresh |

**Transfers use TCP**, because a zone does not fit in a datagram.

**And they must be restricted.** An open AXFR hands an attacker your complete internal
network map — every hostname, every address, every service:

```
# Try it against your own zone
dig @ns1.example.com example.com AXFR
```

**If that returns the zone from an arbitrary source address, it is a finding.** Restrict by
address and authenticate with **TSIG** (a shared key signing the transfer).

## What breaks here

**A CNAME at the apex.** Invalid. Use ALIAS, a redirect, or HTTPS/SVCB records.

**A CNAME alongside other records.** Invalid; the other records will be ignored or the zone
rejected.

**An MX pointing at a CNAME.** Forbidden, and some mail servers refuse the domain.

**Mail arriving at the web server.** No MX record, so the implicit MX used the A record.

**Secondaries serving stale data.** The serial was not incremented.

**A record that resolves to `www.example.com.example.com`.** A missing trailing dot.

**Round-robin DNS not failing over.** It never did. Add health checking.

**An open zone transfer.** Restrict it and use TSIG.

> **Network+ note.** Objective 1.6 expects DNS record types, and **this is examined
> directly.** Over-learn: **A (IPv4), AAAA (IPv6), CNAME (alias), MX (mail, lower
> preference wins), NS (delegation), PTR (reverse), SOA (zone authority), TXT (arbitrary,
> used for SPF/DKIM/DMARC), SRV (service and port).** The MX preference direction and the
> A/AAAA distinction are the most-missed items.
