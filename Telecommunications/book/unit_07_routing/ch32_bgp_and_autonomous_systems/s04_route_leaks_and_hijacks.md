# 32.4 Route Leaks and Hijacks

BGP has no way to tell whether an advertisement is true.

That sentence is the whole of this section. Everything below is its consequences, the
partial defences that have been built, and the reason the problem has taken thirty years
to begin fixing.

## The two failure modes

They are different and are often conflated.

| | **Route leak** | **Route hijack** |
|---|---|---|
| What is announced | routes that are **real, but should not have been passed on** | prefixes the announcer **does not hold** |
| The valley-free rule (§32.3) | **violated** | may be intact |
| Usually | **an accident** | accident **or** attack |
| Effect | traffic diverted through an undersized network | traffic diverted to the announcer |
| Symptom | **congestion, packet loss, latency** | **interception, or a black hole** |

**A leak is a policy failure. A hijack is an ownership failure.**

## Route leaks

**An AS advertises routes it learned from a provider or peer to another provider or
peer** — becoming, by accident, a transit provider for traffic it never intended to carry
and cannot possibly handle.

**The mechanism:**

```
   Big Provider A ──▶ Small AS ──▶ Big Provider B

   Small AS learns the full table from A, and advertises it to B.
   B now believes Small AS is a path to everything.
   Traffic for the whole Internet is offered to a network with a 10 Gb/s uplink.
```

**Why it is so damaging:** the leaked routes are often **more specific** or otherwise
preferred, so a great deal of traffic moves at once. The small AS's links saturate
immediately, and the traffic is not merely slow — **it is dropped**, so services that
depend on those prefixes fail globally while every router involved behaves correctly.

### The incidents

| Date | Event |
|---|---|
| **April 1997** | **The AS 7007 incident.** A misconfigured router at a small US ISP re-advertised a large part of the global table **as its own more-specific /24s**. Much of the Internet's traffic was drawn toward it. Outages lasted hours. **The first major demonstration that one misconfigured router could break the Internet.** |
| **June 2015** | Telekom Malaysia leaked ~179,000 prefixes to Level 3, causing global disruption |
| **November 2017** | Level 3 leaked routes, disrupting Comcast, Bell and others across the US |
| **June 2019** | A small Pennsylvania company leaked routes via Verizon; **Cloudflare, Amazon and others lost significant traffic for hours.** Cloudflare's public post-mortem is worth reading. |
| **June 2024** | Continuing incidents; the class of failure is not solved |

**The pattern is consistent:** a small network makes a configuration mistake, and a **large
network fails to filter it.** The leak is the small network's error; **the outage is the
large network's failure to filter.**

> **A leak requires two failures: someone must announce it, and someone must accept it.**
> The second is the preventable one, and it is the one that keeps not being prevented.

## Route hijacks

**Announcing a prefix you do not hold.** Chapter 29 §29.3 gave the mechanism:
longest-prefix match means **a more specific announcement always wins**, so announcing a
/25 against someone's /24 draws their traffic away — globally, within minutes.

### The incidents

**Pakistan Telecom and YouTube, February 2008.** The canonical case, and worth stating in
full because every element is instructive.

Pakistan's government ordered YouTube blocked. Pakistan Telecom implemented the block by
creating a route for `208.65.153.0/24` — **more specific than YouTube's `208.65.152.0/22`**
— pointing it at null, intending it to be internal.

**The route escaped into BGP**, to its upstream PCCW, which **did not filter it**. Within
minutes every router on the Internet preferred the /24, and **YouTube was globally
unreachable for about two hours.**

**Every element of the failure:**

- A local policy implemented with a routing change rather than a firewall
- **No outbound filter** on Pakistan Telecom's side (§32.2's mandatory line)
- **No inbound filter** on PCCW's side
- **No mechanism anywhere** for a router to check whether Pakistan Telecom was entitled to
  announce that prefix

**A domestic censorship decision took a global service offline**, and the protocol
performed exactly as specified throughout.

| Date | Event |
|---|---|
| **April 2010** | **China Telecom** announced ~50,000 prefixes; for about 18 minutes a substantial share of Internet traffic transited China |
| **April 2018** | **MyEtherWallet** — attackers hijacked Amazon's Route 53 DNS prefixes, served a fake site, and stole cryptocurrency. **A deliberate, financially-motivated BGP attack.** |
| **2020** | Rostelecom announced 8,800 prefixes including major cloud and financial networks |
| **2022** | KLAYswap — BGP hijack used to serve malicious JavaScript, ~$1.9M stolen |

**The MyEtherWallet and KLAYswap incidents changed the conversation.** Before them, BGP
incidents were understood as accidents. Both were **deliberate, targeted, and profitable**,
which made the case for defences commercial rather than merely architectural.

## Why the protocol cannot tell

**BGP has no notion of authorisation.** An advertisement is a claim, and there is nothing
in the protocol against which to check it.

**Why it was built that way** is Chapter 23's answer: in 1989 the Internet was a few
hundred cooperating institutions, and the participants were known to each other. **Adding
authorisation would have required an ownership registry that did not exist and a
cryptographic infrastructure nobody had.**

**Why it has not been fixed** is Chapter 28's answer: **there is no benefit to being
early.** A network that validates routes gains nothing until others sign theirs, and a
network that signs gains nothing until others validate. **The benefit is entirely a
network effect, and the first mover captures none of it.**

## The defences

### Filtering — necessary, insufficient, and free

**The baseline, and it prevents most incidents.**

```
 neighbor 203.0.113.1 prefix-list ANNOUNCE-OURS out
 neighbor 203.0.113.1 prefix-list SANITY in
 neighbor 203.0.113.1 maximum-prefix 1000000 90 restart 15
```

**Filter outbound to exactly what you hold.** **Filter inbound to what the neighbour
should plausibly send** — from a customer, only their prefixes; from anyone, never your
own, never RFC 1918, never a default you did not ask for, never longer than /24.

**`maximum-prefix` is the blunt backstop**: if a neighbour sends far more than expected,
tear the session down. **Losing one session beats propagating a leak**, and this single
line has limited the blast radius of several major incidents.

**Filtering is free, it is entirely within your control, and incomplete deployment of it
is why every incident above happened.** It is the BCP 38 situation again (Chapter 27
§27.2): the cost falls on one party and the benefit on everyone.

### IRR — an honour-system registry

**Internet Routing Registries** — RADB, RIPE, and others — hold objects declaring who
intends to announce what, and many providers build filters from them automatically.

**The weakness is in the name.** Registration is voluntary, entries go stale, and **some
registries permit anyone to create an object for any prefix.** IRR data is better than
nothing and it is not evidence.

### RPKI — the actual fix

**Resource Public Key Infrastructure**, RFC 6480. **Cryptographic attestation of who may
announce what.**

A prefix holder publishes a **Route Origin Authorisation** — signed by the registry that
allocated the address space — stating *"AS 64500 may originate 203.0.113.0/24, with a
maximum length of /24."*

A router performing **Route Origin Validation** classifies each announcement:

| Result | Meaning |
|---|---|
| **Valid** | a ROA exists and this announcement matches it |
| **Invalid** | a ROA exists and this announcement **contradicts** it |
| **NotFound** | no ROA — the majority, still |

**And then drops Invalid.**

**This stops the Pakistan Telecom case outright.** A ROA for YouTube's /22 with max-length
/22 makes the /24 announcement provably Invalid, and every validating router discards it.

**Deployment has finally moved.** Roughly **half** of the global routing table is now
covered by ROAs, and major networks — Cloudflare, Google, AT&T, Telia, NTT, most large
European providers — drop Invalids. The tipping point was around 2019–2021, after the
MyEtherWallet incident made the risk concrete and after **MANRS** (Mutually Agreed Norms
for Routing Security) gave networks a public commitment to point at.

**What RPKI does not do:**

- It validates the **origin**, not the **path**. An AS can still announce a prefix with a
  forged AS_PATH claiming to have learned it legitimately.
- **It does not prevent leaks at all** — a leaked route has a valid origin.

**ASPA** (Autonomous System Provider Authorisation) is the in-progress work to attest
provider relationships, which would let a validator detect a valley and therefore a leak.
**BGPsec** signs the entire path and is deployed essentially nowhere, for the usual
reasons: it requires cryptographic operations per update at line rate and universal
adoption to be useful.

### Monitoring — knowing it happened

**You cannot prevent someone else announcing your prefix. You can find out quickly.**

| Service | What it does |
|---|---|
| **BGPStream** | public feed of detected hijacks and leaks |
| **RIPE RIS**, **RouteViews** | global BGP data, queryable and archived |
| **Cloudflare Radar**, **bgp.he.net** | route visibility and history |
| **BGPalerter** | open-source, alerts on changes to *your* prefixes |

**Set up monitoring for your own prefixes.** It costs nothing, and the difference between
noticing a hijack in five minutes and hearing about it from a customer in two hours is
most of the damage.

## The uncomfortable summary

> **The Internet's routing works because most participants are careful and honest most of
> the time.**

There is no central authority, no enforcement, and — until RPKI's partial deployment — no
technical means of verification. **A single misconfigured router in any of 75,000
autonomous systems can still disrupt a significant part of the Internet**, and this has
happened repeatedly and will happen again.

**It is simultaneously the Internet's greatest weakness and the reason it scaled.** A
protocol requiring authorisation from a central authority would have needed that authority
to exist, to be trusted by every government and every company on Earth, and to keep up.
**None of those was ever going to happen**, and the cost of not requiring it is the
incident list above.

Chapter 57 §57.1's argument — that the architecture's largest mistake was assuming a
trustworthy network — reaches its clearest form here.

## What breaks here

**Your prefix announced by someone else.** A hijack. Contact them, contact their upstream,
announce more specifics yourself as an emergency measure, and get a ROA published.

**Sudden congestion on your transit links with no traffic increase.** You may be receiving
a leak — traffic that is not yours has been redirected through you.

**Your traffic taking an extraordinary path.** Someone leaked, and your provider accepted
it.

**A session dropped by `maximum-prefix`.** Working correctly. Find out what the neighbour
sent before raising the limit.

**Routes rejected as RPKI Invalid.** Either someone is hijacking, or **your own ROA is
wrong** — a max-length set too short is a common self-inflicted outage.

> **Network+ note.** Objective 4.2 lists routing attacks. Over-learn: **a hijack is
> announcing a prefix you do not hold, and more-specific announcements win**; **a leak is
> passing on routes that should not have been passed on**; **prefix filtering is the
> baseline defence**; and **RPKI validates the origin cryptographically**. The
> distinction between leak and hijack is examined and is commonly confused.
