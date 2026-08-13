# 62.2 Spoofing, Poisoning and On-Path Attacks

**§62.1's attacks require access to the segment. These do not**, and their reach is
correspondingly larger.

## IP spoofing

**Chapter 24 §24.2: the source address is asserted by the sender and verified by nobody.**

**What it enables:**

| | |
|---|---|
| **Reflection and amplification** | §62.3 — **the single largest use** |
| **Bypassing address-based access control** | **which is why source IP is not authentication** |
| **Blind injection into a session** | **hard, and possible where sequence numbers are guessable** |
| **Attribution failure** | **the traffic appears to come from an innocent party** |

**The defence is Chapter 57 §57.4's BCP 38 — ingress filtering — and it is deployed at the
network that gains nothing from deploying it.**

**What you can do on your own network:**

```
   ! Do not let your own users spoof
   ip verify unicast source reachable-via rx        ! strict uRPF, at the access edge
   
   ! Do not accept spoofed traffic from outside
   deny ip 10.0.0.0/8 any
   deny ip 172.16.0.0/12 any
   deny ip 192.168.0.0/16 any
   deny ip 127.0.0.0/8 any
   deny ip <your own prefixes> any                   ! the important one
```

**Strict uRPF — "would I route back out of the interface this arrived on?" — is correct at an
access edge and wrong where routing is asymmetric** (Chapter 60 §60.2), **where loose mode or
an ACL is used instead.**

## On-path attacks

**Formerly "man in the middle", and the current term is better because it describes the
position.**

**Getting into the path:**

| Method | Scope | Chapter |
|---|---|---|
| **ARP spoofing** | one segment | §62.1 |
| **Rogue DHCP** (gateway or DNS) | one segment | §62.1 |
| **Rogue access point / evil twin** | radio range | Chapter 45 |
| **DNS manipulation** | wherever the resolution goes | below |
| **BGP hijack** | **potentially global** | Chapter 32 §32.4 |
| **Compromised infrastructure** | whatever it carries | |

**And what the position buys depends entirely on whether the traffic is encrypted:**

> **Against plaintext, an on-path attacker reads and modifies everything.** **Against properly
> validated TLS, they can drop traffic and observe metadata and cannot read or modify it.**
> **The whole of Chapter 58 exists to make the second sentence true.**

**Which is why the attacks that matter now are about defeating the validation rather than
breaking the cryptography.**

## Downgrade and stripping attacks

**Do not break the encryption. Prevent it.**

### SSL stripping

**Moxie Marlinspike's 2009 demonstration, and it remains effective wherever the mitigations are
absent.**

```
   User types:      example.com                    (no scheme)
   Browser sends:   http://example.com             ← plaintext
   Server replies:  301 → https://example.com
   
   With an attacker on path:
   Browser sends:   http://example.com             ← attacker intercepts
   Attacker fetches https://example.com themselves
   Attacker returns the page over HTTP, rewriting every https:// link to http://
   
   The user never sees a certificate error, because there is no certificate.
```

> **The attack does not break TLS. It ensures TLS is never used**, and **the only visible
> difference is the absence of a padlock**, which users do not check.

**The mitigations, and all three are now widely deployed:**

**HSTS** — **`Strict-Transport-Security`** — **the server tells the browser to use HTTPS for this
domain for the next year**, and the browser refuses plaintext thereafter. **Its weakness is the
first visit**, which is what **HSTS preloading** — a list compiled into browsers — addresses.

**HTTPS-first browsing**, now default in major browsers: **try HTTPS, fall back only with a
warning.**

**And DNS-level:** **HTTPS/SVCB resource records let the DNS answer say "this host speaks
HTTPS"**, before any connection is attempted.

### Protocol version and cipher downgrade

**A negotiation an attacker can influence is a negotiation they can steer downwards.**

| Attack | Downgraded to |
|---|---|
| **POODLE** | **SSL 3.0** |
| **FREAK** | **export-grade RSA**, deliberately weakened in the 1990s |
| **Logjam** | **512-bit Diffie–Hellman** |
| **DROWN** | SSLv2, used to attack a TLS connection sharing the key |

> **FREAK and Logjam are the most instructive**, because **the weakness was legislated.**
> **Export-grade cryptography was mandated by 1990s US export control, the code stayed in
> implementations for twenty years, and attackers found they could force its use.** **A
> deliberately weakened option that nobody removed became a vulnerability two decades later**,
> which is Chapter 55 §55.1's accumulation argument with a legal cause.

**TLS 1.3's answer is to remove the options** (Chapter 58 §58.4): **five suites, no negotiable
key exchange, and a downgrade-detection mechanism in the handshake.**

## DNS attacks

**Chapter 39's protocol is the most attractive target in the stack** (Chapter 57 §57.4),
**because controlling name resolution controls where traffic goes without touching routing.**

### Cache poisoning

**The classical attack, and Kaminsky's 2008 work is why it is now hard.**

```
   Attacker triggers the resolver to query for random.example.com
   Attacker floods forged replies, guessing:
       the 16-bit query ID          — 65,536 possibilities
       (and originally, nothing else)
   One correct guess poisons the cache — and Kaminsky's insight was that
   the forged reply could include an authority record for example.com itself,
   poisoning the entire domain rather than one name.
```

**The mitigations:**

| | |
|---|---|
| **Source port randomisation** | **adds ~16 bits — the emergency 2008 fix** |
| **0x20 encoding** | randomise the case of the query name; the reply must match |
| **DNSSEC** | **the actual answer, and its deployment is partial** (Chapter 39 §39.4) |
| **DoT / DoH** | **protects the client-to-resolver path**, not the resolver-to-authoritative one |

### The attacks that do not require poisoning

**And these are the ones that actually happen now.**

**Registrar compromise.** **Change the nameservers at the registrar, and the domain is yours** —
**no protocol weakness required.** **Registrar account MFA and registry lock are the controls**,
and registry lock in particular is under-used for high-value domains.

**Resolver substitution.** **Rogue DHCP** (§62.1) **or a modified device configuration.**

**Domain shadowing.** **Compromise the DNS account and add subdomains** — **the legitimate
records are untouched, so nothing looks wrong.**

**And subdomain takeover.** **A CNAME pointing at a cloud service that has been deleted**, which
an attacker then registers. **Chapter 53's documentation problem, with a security consequence**:
**dangling DNS records accumulate and nobody removes them.**

## Session hijacking

**Three distinct things share the name.**

**Network-level TCP hijacking.** **Predict the sequence number and inject.** **Largely historical
against modern stacks** — **randomised initial sequence numbers made blind injection
impractical** — **and trivially possible for an on-path attacker against plaintext.**

**Session token theft.** **The modern version, and the common one.** **Steal the cookie or token
and use it**, defeating even MFA because **the authentication already happened.**

> **This is why token theft has become the dominant attack against organisations with good
> authentication** (Chapter 59 §59.1). **A stolen session token bypasses every factor**, and the
> mitigations are **short token lifetimes, binding tokens to a device or a TLS channel, and
> re-authentication for sensitive actions.**

**Fixation.** **Set a session identifier the attacker knows before the victim authenticates.**
**Prevented by regenerating the identifier on authentication**, which is an application concern.

## Wireless-specific

**Chapter 45's material, from the attacker's side.**

| Attack | Mechanism | Mitigation |
|---|---|---|
| **Evil twin** | **an access point with the same SSID and a stronger signal** | **802.1X with server validation** (Chapter 59 §59.2) |
| **Deauthentication flood** | **management frames were unauthenticated** | **802.11w / PMF** |
| **Karma / preferred network** | **respond to any probe request** | **do not auto-join open networks; and modern clients randomise** |
| **WPA2 handshake capture** | **offline dictionary attack against a PSK** | **strong PSK, or 802.1X** |
| **KRACK** | **key reinstallation in the four-way handshake** | patched clients |

**802.11w — Protected Management Frames — is the one to insist on.** **It is mandatory in WPA3
and optional in WPA2**, and **it removes the deauthentication attack that every wireless
troubleshooting session has to consider.**

## What the defences have in common

**Every mitigation in this section is one of three things:**

| | Examples |
|---|---|
| **Authenticate what was not authenticated** | DNSSEC, RPKI, 802.11w, DAI |
| **Remove the option that can be downgraded to** | TLS 1.3's five suites, HSTS |
| **Verify against an out-of-band source of truth** | DHCP snooping's binding table, certificate transparency, RPKI |

> **None of them is "detect the attack".** **The successful mitigations all make the attack
> impossible rather than visible**, which is Chapter 60's default-deny argument in a different
> guise, and it is why detection-based approaches to these attacks have been consistently
> disappointing.

## What breaks here

**Traffic from your network with forged source addresses.** **No uRPF or egress filtering.** You
are contributing to §62.3's problem.

**A user who reached a plaintext version of a site that supports HTTPS.** **SSL stripping**, or
simply no HSTS. Deploy HSTS and preload.

**A TLS connection negotiating something ancient.** **The option was still enabled.** Chapter 55
§55.1 — remove what is not needed.

**A domain redirected with no protocol attack.** **Registrar compromise.** MFA and registry
lock.

**A CNAME pointing at a deleted cloud resource.** **Subdomain takeover waiting to happen.**
Audit dangling records.

**MFA in place and an account compromised anyway.** **Session token theft.** Short lifetimes and
token binding.

**Wireless clients repeatedly disconnecting.** **A deauthentication flood, or interference**
(Chapter 45 §45.4). **802.11w distinguishes them by eliminating one.**

**A rogue access point with your SSID collecting credentials.** **Chapter 59 §59.2's server
validation**, which is the actual fix; rogue AP detection is detection, not prevention.

> **Network+ note.** Objective 4.2 covers these. Over-learn: **on-path (man-in-the-middle)
> attacks intercept traffic between two parties**; **DNS poisoning inserts false records**;
> **ARP spoofing associates the attacker's MAC with another's IP**; **evil twin and rogue AP are
> wireless on-path attacks**; **deauthentication attacks disconnect clients**; and **session
> hijacking uses a stolen session token.** The attack-name-to-mechanism mapping is examined
> heavily.
