# 39.4 Caching, TTL and DNS Security

Caching is what makes DNS affordable, and it is also what makes DNS changes slow and DNS
attacks durable. This section covers both, and the security half is where DNS's original
design shows its age most sharply.

## TTL — the contract

**Every record carries a time-to-live**: *how long a resolver may cache this answer.*

It is a promise from the zone's operator to the world, and it is the only mechanism
controlling how quickly a change propagates.

| TTL | Change propagates in | Query load | Typical use |
|---|---|---|---|
| **60 s** | **1 minute** | **high** | during a migration; health-checked failover |
| 300 s | 5 minutes | moderate | records that change |
| **3600 s** | **1 hour** | low | **a sensible default** |
| 86400 s | **1 day** | very low | stable records |
| 172800 s | 2 days | minimal | **NS and delegation records** |

The trade is direct: a shorter TTL costs query volume and buys agility.

### The migration procedure

The standard technique, and getting the order right is the whole thing:

```
   Day -2:  lower the TTL from 3600 to 60
            → wait at least the OLD TTL (1 hour) for the world to see it
   Day  0:  change the record
            → the world follows within 60 seconds
   Day +1:  raise the TTL back to 3600
```

> **You must lower the TTL *before* the change, and wait out the *old* TTL.** Lowering it
> at the moment of the change achieves nothing — caches already hold the old record with
> the old, long TTL, and will keep it.

This is the most useful operational fact in the chapter, and forgetting it turns a
five-minute migration into an eight-hour one during which half the world reaches the old
address.

### Negative caching

**"This name does not exist" is cached too** — RFC 2308, using the **SOA's minimum field**
(§39.3).

**Which produces a distinctive fault:** you create a record that did not exist, and **it
does not work for the negative-cache duration** even though the zone is correct.

Everything looks right at the authoritative server and wrong everywhere else, and the
only remedy is waiting. **Keep the SOA minimum modest** — 300 to 900 seconds — precisely so
this window is short.

### Who ignores your TTL

**TTLs are advisory in practice.**

**Some resolvers impose a minimum** (ignoring very short TTLs to reduce load) or a
**maximum** (capping very long ones). **Browsers cache separately** and historically
ignored TTLs entirely — Chrome caches for about a minute regardless. **Operating systems
cache, and Java historically cached DNS forever** by default, which caused a great many
production incidents.

So a TTL is a request, not a guarantee — the same shape as Chapter 20 §20.2's QoS
marking, and worth remembering when a migration does not propagate as predicted.

## The security problem

**DNS as designed has no authentication of any kind.**

A response is believed because it arrived and matched a query. The matching is:

| Field | Entropy |
|---|---|
| **Query ID** | 16 bits |
| Question name and type | must match |
| **Source port** | originally **fixed**; now randomised |

With a fixed source port, an attacker needs only to guess a 16-bit ID — 65,536
possibilities, and they can send many forged responses.

If a forged response arrives before the real one and the ID matches, it is cached — and
**every client using that resolver is sent to the attacker's address until the TTL
expires.**

### The Kaminsky attack

**Dan Kaminsky, 2008**, and it is the most consequential DNS vulnerability ever found.

**The prior understanding** was that an attacker got one attempt per cached record, and had
to wait for the TTL to expire to try again. Slow, and largely impractical.

**Kaminsky's insight: query for names that do not exist.**

```
   Attacker asks the resolver for:  aaaa1.example.com
                                    aaaa2.example.com
                                    aaaa3.example.com   ...

   Each is a cache miss, so the resolver queries upstream EVERY TIME.
   The attacker floods forged responses for each — and each forged response
   carries an ADDITIONAL section:

       example.com.  NS  ns1.attacker.net
```

**So a successful forgery does not poison one name — it poisons the entire zone's
delegation**, redirecting *everything* under `example.com`.

And the attacker gets unlimited attempts, because every new nonsense name is a fresh
cache miss with a fresh query ID to guess.

> **Kaminsky turned cache poisoning from a slow, one-shot attack into one that succeeds in
> seconds and takes the whole domain.**

The coordinated response in July 2008 — patches released simultaneously by every major
DNS vendor before disclosure — is one of the largest coordinated security efforts in
Internet history.

The fix was source port randomisation, adding **~16 bits** of entropy:

$$2^{16} \times 2^{16} = 2^{32} \approx 4.3 \times 10^{9} \text{ combinations}$$

**And it is a mitigation, not a solution.** It makes the attack expensive rather than
impossible, and Kaminsky said so at the time. **0x20 encoding** (§39.1) adds a little more.

**The actual solution is cryptographic.**

## DNSSEC

**DNS Security Extensions** — RFC 4033–4035. **Sign the records.**

**What it provides:**

| Provides | Does not provide |
|---|---|
| **Authenticity** — this came from the zone's owner | **confidentiality** — queries are still plaintext |
| **Integrity** — it was not modified | protection of the last hop by default |
| **Authenticated denial** — "this name really does not exist" | |

**The records:**

| Type | Purpose |
|---|---|
| **RRSIG** | the signature over a record set |
| **DNSKEY** | the zone's public key |
| **DS** | a **hash of the child's key, published in the parent** |
| **NSEC / NSEC3** | authenticated denial of existence |

**The chain of trust:**

```
   Root zone — its key is the TRUST ANCHOR, configured in every validator
     │  signs a DS record for .com
     ▼
   .com
     │  signs a DS record for example.com
     ▼
   example.com
     │  signs its own records
     ▼
   www.example.com  A  93.184.216.34   + RRSIG
```

Each level vouches for the next, and the root's key is the one thing a validator must
know in advance.

The root was signed in July 2010, in a ceremony with multiple witnesses and physical
key shares — deliberately theatrical, because the whole system's trust rests on that one
key.

### Why deployment stalled

Signed zones are perhaps 5–10% of the total, and validation is more common than signing
because large public resolvers do it.

**The reasons are the ones this book keeps arriving at:**

**No benefit to being early** (Chapter 28 §28.1). Signing your zone protects users of
validating resolvers, of which there were few; validating protects your users from signed
zones, of which there were few.

**Operationally dangerous.** An expired signature makes your domain vanish for every
validating resolver — a harder failure than not signing at all. Several large organisations
have taken themselves offline this way.

**Key management is real work.** Rollovers must be staged correctly, and a mistake is an
outage.

**Response sizes grow**, running into §39.2's fragmentation problems.

**It does not protect the last hop.** Between the validating resolver and your machine,
the answer is still plaintext and unauthenticated — **which is where most users' actual
exposure is.**

That last point is why encrypted transport has moved faster than DNSSEC.

## Encrypted DNS

**A different problem: DNSSEC authenticates; it does not conceal.**

Every query you make is visible to your network, your ISP, and anyone on the path — and
the list of names you look up is a detailed record of what you do.

| | Transport | Port | Character |
|---|---|---|---|
| **DoT** — DNS over TLS | TLS | **853** | **a distinct port, so it is blockable and visible as DNS** |
| **DoH** — DNS over HTTPS | HTTPS | **443** | **indistinguishable from web traffic** |
| **DoQ** — DNS over QUIC | QUIC | 853 | lower latency |
| **DNSCrypt** | custom | varies | pre-standard, still used |

DoT and DoH differ in one respect that generated enormous argument:

DoT is on its own port, so a network operator can see that DNS is happening, can
require their own resolver, and can block it.

DoH is on 443 and looks like HTTPS, so **the network cannot distinguish it from any
other web traffic** — which is precisely the point for a user evading surveillance or
censorship, and precisely the problem for an enterprise that needs DNS visibility for
security monitoring, or for a school applying content filtering.

**The controversy is genuine and unresolved:**

| For DoH | Against DoH |
|---|---|
| Protects users from ISP surveillance and manipulation | **Bypasses enterprise security monitoring** |
| Resists censorship | **Bypasses parental and school filtering** |
| Prevents on-path tampering | **Centralises DNS in a few large providers** |
| | Breaks split-horizon and internal name resolution |

Browsers shipping DoH enabled by default, sending queries to a resolver of the
*browser's* choosing rather than the network's, was the flashpoint — and the resolution has
mostly been **canary domains** (a network can publish a record signalling "do not use DoH
here") and enterprise policy controls.

> **It is a real conflict between the user's privacy and the network operator's visibility,
> and both interests are legitimate.** Chapter 61 returns to it.

## The outages worth knowing

**DNS failures are unusually consequential, because everything depends on them.**

**Dyn, October 2016.** A Mirai-botnet DDoS against a large managed DNS provider took
Twitter, Spotify, Reddit, GitHub, Netflix and many others offline simultaneously —
because they shared a DNS provider. None of their own infrastructure failed.

**The lesson is concentration risk:** a dependency shared by many services is a single
point of failure for all of them, and it is invisible until it fails. **Use two DNS
providers for anything that matters.**

**Facebook, October 2021.** A configuration change withdrew the BGP routes to Facebook's
own DNS servers. The DNS servers were healthy and unreachable, so every Facebook
property became unresolvable worldwide for six hours.

**And the failure compounded:** internal tools depended on the same DNS, badge readers
depended on internal tools, and engineers could not enter the building to fix it.

> **A dependency you did not know you had is still a dependency.** Facebook's is the
> canonical example, and it is worth asking of your own systems: *what breaks if DNS
> fails, and does the recovery path depend on DNS?*

## Diagnosing

```bash
# The basics
dig www.example.com
dig www.example.com @8.8.8.8          # bypass the local resolver
dig +short www.example.com

# The walk, step by step — the most instructive command
dig +trace www.example.com

# Is this from a cache or authoritative?
dig www.example.com | grep flags       # look for 'aa'

# Ask the authoritative server directly
dig www.example.com @ns1.example.com

# DNSSEC
dig +dnssec www.example.com            # look for 'ad' (authenticated data)
delv www.example.com                   # full validation with explanation

# Which anycast instance answered?
dig CHAOS TXT id.server @1.1.1.1
dig +nsid www.example.com @8.8.8.8

# Reverse
dig -x 93.184.216.34
```

**The sequence that resolves most DNS faults:**

| Question | Command |
|---|---|
| **1. Does it resolve at all?** | `dig name` |
| **2. Does the authoritative server have it right?** | `dig name @ns1...` |
| **3. If yes but the resolver disagrees — caching.** | check the TTL, wait or flush |
| **4. Is the delegation correct?** | `dig +trace name` |
| **5. Is it my resolver specifically?** | `dig name @8.8.8.8` |

Steps 2 and 5 together localise almost everything: right at the authority and wrong at
your resolver means caching or a resolver problem; wrong at the authority means the zone.

```bash
# Flushing
resolvectl flush-caches          # systemd
sudo dscacheutil -flushcache     # macOS
ipconfig /flushdns               # Windows
rndc flush                       # BIND
```

## What breaks here

**A change that has not propagated.** The TTL. Check what it was *before* the change.

**A new record that does not work.** Negative caching. Wait out the SOA minimum.

Everything worked until the certificate expired — or the DNSSEC signature did. A
signature expiry makes the domain vanish for validating resolvers.

**One resolver giving a different answer from another.** Caching, or split-horizon, or a
poisoned cache.

**Enterprise DNS monitoring going blind.** Browsers using DoH. Policy or canary domains.

**Everything down because one provider was attacked.** Concentration risk. Use two.

Unable to fix an outage because the tools depend on the broken thing. Ask this question
before the outage.

> **Network+ note.** Objective 1.6 expects DNS caching and TTL; objective 4.2 expects DNS
> poisoning as an attack. Over-learn: **TTL controls how long an answer is cached**;
> **lower the TTL before a migration, not during**; **DNSSEC provides authenticity and
> integrity but not confidentiality**; **DoT is port 853 and DoH is port 443**; and **cache
> poisoning is mitigated by source port randomisation and solved by DNSSEC.**
