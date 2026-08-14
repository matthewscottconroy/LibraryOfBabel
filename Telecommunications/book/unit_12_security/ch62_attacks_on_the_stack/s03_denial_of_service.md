# 62.3 Denial of Service

Chapter 57 §57.2 called availability the hardest of the three properties to defend, because
a sufficiently large flood is indistinguishable from legitimate popularity. This section is
the arithmetic.

## Three kinds, requiring three defences

| Kind | Exhausts | Scale | Defence |
|---|---|---|---|
| **Volumetric** | **circuit bandwidth** | **Gb/s to Tb/s** | **upstream capacity or scrubbing** |
| **Protocol / state** | **connection tables, half-open slots** | **packets per second** | **SYN cookies, rate limits** |
| **Application** | **CPU, database, backend** | **requests per second — sometimes very few** | **rate limiting, caching, WAF** |

**The third is the one people underestimate.**

> A volumetric attack needs terabits. An application-layer attack may need a few hundred
> requests per second, if each one triggers an expensive database query or a report
> generation. The bandwidth is trivial and the service is down.

Which means "we have a 100 Gb/s scrubbing contract" answers one third of the problem.

## Amplification: the arithmetic that matters

The mechanism has two parts, and both are required:

1. Spoof the source address as the victim's (§62.2)
2. Send to a service that answers with far more than it was asked

```
   Attacker ──(small query, spoofed src = victim)──▶ Reflector
                                                        │
   Victim   ◀────────(large response)───────────────────┘
```

And the amplification factor is the whole story:

| Service | **Factor** | **Attacker bandwidth for a 100 Gb/s attack** |
|---|---|---|
| SSDP | ~30× | 3.3 Gb/s |
| DNS (ANY query) | **~54×** | **1.9 Gb/s** |
| CLDAP | ~56× | 1.8 Gb/s |
| chargen | ~358× | 280 Mb/s |
| **NTP `monlist`** | **~556×** | **180 Mb/s** |
| **memcached** | **~51,000×** | **2 Mb/s** |

> **The memcached figure is not a misprint.** In 2018 an attacker with a domestic connection
> could generate a 1 Tb/s attack, because memcached listened on UDP by default, required no
> authentication, and would return megabytes in response to kilobytes. The 1.35 Tb/s attack
> on GitHub in February 2018 used it.

**And the remedy was not clever.** Memcached's maintainers disabled UDP by default, and
operators filtered port 11211. Within weeks the vector was largely closed — which is the
pattern: amplification vectors are closed by fixing the reflector, and each closure is a
one-time fix that nobody has an incentive to prioritise until it is used.

**Two things follow:**

**Do not operate a reflector.** An open DNS resolver, an NTP server with `monlist` enabled, an
exposed memcached, an SSDP-responding device on a public address — each makes you part of
someone else's attack. This is Chapter 57 §57.4's externality: the cost falls elsewhere.

And BCP 38 would eliminate the entire category, because spoofing is step 1. It has been
specified since 2000.

## Protocol attacks

The state exhaustion of Chapter 60 §60.2, from the attacker's side.

**SYN flood.** Send SYNs, never complete the handshake. Each consumes a half-open slot on
the server and an entry in the firewall's table (Chapter 37 §37.2).

> **The defence is elegant: SYN cookies.** Encode the connection state in the initial sequence
> number, cryptographically, and allocate nothing until the ACK returns carrying it back.
> The server holds no state for an incomplete handshake.

Its cost is that some TCP options cannot be encoded, so SYN cookies are typically enabled
under attack rather than always — and "typically" means "if configured", which should be
checked.

**Other protocol attacks worth knowing:**

| | |
|---|---|
| **Slowloris** | **open many connections and send headers very slowly** — exhausts connection slots with almost no bandwidth |
| **RUDY / slow POST** | the same, with the body |
| **TLS renegotiation** | **the handshake is expensive for the server and cheap for the client** — an asymmetry |
| **HTTP/2 rapid reset (2023)** | **open and cancel streams faster than the server can free them** |

**The common shape is worth naming:**

> Every protocol attack exploits an asymmetry — the defender does more work than the
> attacker. Finding those asymmetries is the whole of protocol denial-of-service research,
> and the general remedy is to make the client prove effort or state before the server commits
> any.

## Application attacks

Small, cheap, and hard to distinguish from use.

| Attack | Cost to attacker | Cost to defender |
|---|---|---|
| **Search with an expensive query** | one request | a full table scan |
| **A report generation endpoint** | one request | seconds of CPU |
| **Login attempts** | one request | **a deliberately slow password hash** (Chapter 58 §58.3) |
| **Cache-busting requests** | one request | **an origin fetch every time** (Chapter 52 §52.4) |
| **File upload or image processing** | a small upload | substantial memory and CPU |

And the password hashing case is a genuine and under-appreciated trap:

> Argon2id is deliberately slow, which is correct for password storage and makes the login
> endpoint an amplifier. Rate limiting the login endpoint is therefore not optional, and
> it must be per-account as well as per-source.

**The defences are application-layer:** rate limiting per user and per source, caching,
query cost limits, CAPTCHAs (which are increasingly ineffective), and a WAF with behavioural
rules — and none of them is a network engineer's to configure, which is why this category
is frequently nobody's responsibility.

## DDoS, and why it is different

A distributed attack removes the defence of blocking the source.

**Where the capacity comes from:**

| Source | Notes |
|---|---|
| **Botnets of compromised devices** | **IoT in particular** (Chapter 47) — **Mirai was 600,000 devices** |
| **Reflectors** | above — no compromise required |
| **Booter / stresser services** | **rented, from about $10 for a short attack** |
| **Compromised cloud instances** | **very high bandwidth per node** |

The Mirai case is worth knowing precisely because it explains the current threat landscape:

> Mirai scanned for devices with default credentials — 62 username/password pairs in the
> original source — and found hundreds of thousands. **Cameras, routers and video recorders.**
> It produced attacks over 600 Gb/s against Krebs on Security and over 1 Tb/s against OVH in
> 2016, and took Dyn's DNS service down, which removed a large fraction of the web for
> several hours.

**Two lessons, both Chapter 57's:**

The devices were compromised by default credentials — §57.1's opportunist, at scale.

And the Dyn attack demonstrated dependency concentration (Chapter 52 §52.4): the target was
a DNS provider, and the casualties were hundreds of unrelated services.

## Defending, honestly

**Layered, and the layers do different things.**

| Layer | Handles | Limitation |
|---|---|---|
| **Upstream scrubbing service** | **volumetric** | **cost; and traffic must be diverted to it** |
| **Provider filtering / RTBH** | **volumetric, bluntly** | **black-holing completes the attack for that prefix** |
| **Anycast** | **volumetric, by distribution** | **needs global presence** (Chapter 52 §52.4) |
| **CDN in front** | **volumetric and some application** | **origin must not be reachable directly** |
| **Firewall / load balancer** | **protocol** | **it is downstream of the circuit** |
| **Application controls** | **application** | **someone must own them** |

**The two mechanisms worth understanding:**

**Remotely Triggered Black Hole (RTBH).** You announce a route for the attacked address with a
community that tells your provider to discard it. The attack traffic is dropped in the
provider's network, and so is the legitimate traffic — you have completed the denial of
service against yourself, deliberately, to protect everything else. It is a real and
appropriate tool, and it must be understood as what it is.

**Diversion to a scrubbing centre.** BGP or DNS redirects traffic to a cleaning service, which
forwards the clean subset to you over a tunnel (Chapter 61). Effective, and it must be
pre-arranged, tested, and its activation time known — which is Chapter 56 §56.2's argument
about untested failover.

**And the preparation that actually matters:**

- **Know your normal** (Chapter 54 §54.1) — you cannot recognise an attack without a
  baseline
- Know your circuit's capacity and what fraction an attack must reach to matter
- Have the scrubbing arrangement in place before you need it — arranging one during an
  attack takes hours
- Know who to telephone at your provider, out of hours (Chapter 53 §53.2)
- Do not make your origin reachable directly if you are behind a CDN — and check, because
  the address is frequently discoverable through certificate transparency, DNS history or an
  unprotected mail server

## What breaks here

A 40 Gb/s attack against a 1 Gb/s circuit. Nothing you own can help (Chapter 57 §57.2).
Upstream, or scrubbing.

A scrubbing contract that has never been activated. **Untested failover.** Rehearse it.

A CDN in front and the attack reaching the origin directly. The origin address is
discoverable. Restrict it to the CDN's ranges.

An attack of a few hundred requests per second taking a service down. **Application layer.**
No amount of bandwidth helps.

**The login endpoint used as an amplifier.** The password hash is deliberately slow. Rate
limit per account and per source.

**A firewall failing before the server.** Chapter 60 §60.2 — **state table exhaustion.**

**Your own network reflecting someone else's attack.** An open resolver, exposed memcached, or
NTP `monlist`. Close it; it costs you nothing and it is your traffic.

RTBH announced and the service is still down. That is what RTBH does. It protects
everything else.

A DDoS "protection" appliance on premises for a volumetric attack. It is downstream of the
full circuit. It addresses protocol attacks and not the flood.

> **Network+ note.** Objective 4.2 covers DoS and DDoS. Over-learn: a DoS attack comes from
> one source and a DDoS from many; amplification and reflection use spoofed source addresses
> and services that reply with more than they receive; **botnets provide the distributed
> capacity**; and mitigation includes upstream filtering, rate limiting and scrubbing
> services. The reflection/amplification mechanism is examined and the arithmetic is worth
> being able to reproduce.
