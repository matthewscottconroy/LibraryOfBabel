# 60.3 Next-Generation Firewalls and Proxies

"Next-generation" is a marketing term that has outlived its novelty by fifteen years.
Beneath it is a real change: matching on application identity and content rather than on
addresses and ports.

## What changed, and why it had to

§60.1's ACL matches port 443. In 2005 that meant "web traffic". It now means "essentially
everything."

| | 2005 | Now |
|---|---|---|
| Port 443 carried | HTTPS to websites | **everything** — SaaS, APIs, VPNs, tunnels, file sync, video, malware C2 |
| **A port-based policy could** | **distinguish applications** | **distinguish almost nothing** |
| Applications used | their own ports | **whatever gets through** |

> **Applications deliberately migrated to 443 because it was the port firewalls permitted.**
> **Which destroyed the port as a classifier**, and the arms race between filtering and
> evasion has been running ever since — Chapter 38 §38.4's QUIC being the current instalment.

**So a next-generation firewall matches on:**

| | |
|---|---|
| **Application identity** | **"this is Dropbox", not "this is TCP 443"** |
| **User identity** | integrated with the directory (Chapter 59) |
| **Content** | intrusion signatures, malware, data patterns |
| **Reputation** | address, domain and file reputation feeds |
| **Decrypted traffic** | **where TLS inspection is deployed** — and see below |

## How applications are identified

Four techniques, in decreasing order of reliability and increasing order of what encryption
has taken away.

**Explicit identifiers in the clear.** TLS SNI, the certificate's subject (in TLS 1.2), DNS
queries, and HTTP Host headers. Reliable, cheap, and being closed — Encrypted Client
Hello removes SNI, and encrypted DNS removes the query (Chapter 39).

**Behavioural fingerprinting.** Packet sizes, timing, TLS handshake characteristics
(JA3/JA4 fingerprints). Works on encrypted traffic, is probabilistic, and changes when a
client updates.

**Deep packet inspection.** **Reading the payload.** Effective on unencrypted traffic and
largely irrelevant now — Chapter 52 §52.2 made the same observation about QoS classification.

**Decryption.** The only technique that sees content, and it is the subject of the next
section.

> **Application identification is now approximate, and its accuracy is falling.** A policy
> that depends on precise application classification is a policy whose accuracy depends on a
> vendor's signature updates (Chapter 51 §51.2's SD-WAN made the same observation), and
> which fails when an application changes its infrastructure.

## TLS inspection, and the honest trade

The firewall terminates TLS, inspects, and re-encrypts to the destination.

```
   Client ══TLS══▶ [ Firewall ] ══TLS══▶ Server
                        │
                   plaintext here
                        │
              presents a certificate signed by
              an internal CA the client trusts
```

> This is a man-in-the-middle attack that you perform against your own users, and calling it
> anything else obscures what must be reasoned about.

**What it buys:**

- Malware in encrypted downloads is detected
- Data loss prevention can see what is leaving
- Command-and-control channels using TLS become visible
- Application identification becomes accurate again

What it costs, and each item is real:

| Cost | |
|---|---|
| **Performance** | **decrypt and re-encrypt every session** — frequently the largest single load on the device |
| **A private CA on every client** | Chapter 58 §58.4 — **and its compromise is total** |
| **Certificate pinning breaks** | **many applications refuse to work** — banking apps, updaters, some mobile applications |
| **A single point of plaintext** | **every session in the organisation, in one device's memory** |
| **Privacy and legality** | **employee banking, medical and legal traffic** — and in many jurisdictions this is regulated |
| **Weaker crypto than the endpoints negotiated** | **some implementations downgrade**, and the client cannot tell |

And the last one is a genuine finding rather than a theoretical concern. Studies of
TLS-inspecting middleboxes have repeatedly found that a substantial fraction negotiate weaker
parameters with the server than the client would have, do not validate the server's
certificate properly, or fail to pass validation errors through to the client — so the
inspection device makes the connection less secure than it would have been.

> The client sees a green padlock signed by a CA it trusts, and has no way to know what the
> middlebox negotiated on its behalf. The security indicator is now measuring the wrong
> connection.

**The defensible position:**

**Inspect selectively.** Categories where the risk justifies it — unknown sites, file
downloads, high-risk categories — and bypass banking, health, legal and government, both for
legality and because pinning will break them anyway.

**Publish the policy.** Users and regulators should know it is happening, and in several
jurisdictions this is required.

**Verify the device's own TLS behaviour.** Test what it negotiates outbound and what it does
with an invalid server certificate. **Do not assume.**

And consider whether endpoint agents are the better answer. An agent on the device sees
plaintext without breaking TLS, does not create a central plaintext point, and does not break
pinning. Where the endpoints are managed, this is frequently the superior architecture —
and it is why the market has moved that way.

## IDS and IPS

Detection and prevention, and the difference is where they sit.

| | **IDS** | **IPS** |
|---|---|---|
| Position | **out of band, on a tap or span** | **in line** |
| On detection | **alerts** | **blocks** |
| **A false positive costs** | **an alert** | **an outage** |
| **A failure of the device costs** | **nothing** | **the network, unless bypassed** |
| Latency added | **none** | **some** |

**The false-positive asymmetry determines everything.**

> An IDS with a 1% false positive rate is noisy. An IPS with a 1% false positive rate blocks
> legitimate traffic 1% of the time, and the resulting pressure is to tune it until it
> blocks almost nothing — at which point it is an expensive IDS.

Which is the observed outcome in a great many deployments, and it should be planned for
rather than discovered: deploy in detection mode, tune for weeks, and promote individual
signatures to blocking as confidence in each is established. Blocking everything from day
one produces an outage and a permanent loss of organisational trust in the device.

Signature versus anomaly detection repeats Chapter 54's argument:

| | Detects | Misses | False positives |
|---|---|---|---|
| **Signature** | **known attacks, precisely** | **anything new** | **low** |
| **Anomaly** | **deviation from a baseline** | **attacks that look normal** | **high, and it is the binding constraint** |

Denning predicted the false-positive problem in 1987 (Chapter 57's reading), and it has not
been solved.

## Proxies

A different architecture with different properties, and it is being rediscovered.

| | **Firewall** | **Proxy** |
|---|---|---|
| Position | **in the path, transparently** | **the client connects to it explicitly** |
| Sees | packets | **complete application transactions** |
| Terminates the connection | no | **yes — two connections, not one** |
| Client configuration | **none** | **required** (or transparent interception) |
| **Protocol enforcement** | limited | **can enforce protocol correctness** |

**A proxy's structural advantage:** it terminates the connection, so a malformed or
malicious protocol interaction reaches the proxy and not the server. The server only ever
receives requests the proxy constructed, which is a stronger property than inspecting packets
in flight.

**Forward proxies** (client to Internet) largely died with HTTPS (Chapter 52 §52.4) and
have returned as cloud security services — which is exactly what SASE is (Chapter 51
§51.2). The architecture came back; the deployment location changed.

**Reverse proxies** (Internet to your servers) never went away and have grown — load
balancers, WAFs, API gateways and CDN edges are all reverse proxies, and a web application
firewall is a reverse proxy that applies application-layer rules.

## Where each belongs

| Requirement | Use |
|---|---|
| **Block by address, port, protocol** | **stateful firewall** |
| **Block by application or user** | **NGFW** |
| **See inside encrypted traffic** | **TLS inspection, or an endpoint agent** |
| **Detect known attacks** | **IPS, tuned** |
| **Protect a specific web application** | **WAF / reverse proxy** |
| **Control outbound web access at scale** | **cloud proxy / SASE** |
| **Enforce protocol correctness** | **proxy** |

## What breaks here

A port-based policy that no longer distinguishes anything. Everything is on 443.

An application misclassified after the vendor changed its infrastructure. Signature
databases go stale. Chapter 51 §51.2 said the same.

**A banking application refusing to work.** Certificate pinning, and TLS inspection. Bypass
the category.

TLS inspection negotiating weaker cryptography than the client would have. A real and
documented finding. Test it.

**An IPS tuned until it blocks nothing.** The predictable outcome of deploying it in blocking
mode on day one.

**An IPS failure taking the network down.** **In-line placement.** Configure fail-open or
fail-closed deliberately (Chapter 57 §57.2), and hardware bypass where the risk warrants
it.

A WAF blocking legitimate requests after a code deployment. The application changed and
the rules did not. WAF rules are coupled to the application.

**Encrypted DNS bypassing the firewall's category filtering.** **Expected** (Chapter 39).
Endpoint policy, or DNS controls that the endpoint honours.

**Users routing around the proxy.** Chapter 57 §57.4's psychological acceptability — if it is
slow enough, they will.

> **Network+ note.** Objective 4.3 covers firewall types, IDS/IPS and proxies. Over-learn:
> an NGFW inspects at the application layer and can identify applications and users; an
> IDS detects and alerts while an IPS detects and blocks; an IDS is out of band and an IPS
> in line; **a proxy terminates connections on behalf of clients**; and **a WAF protects web
> applications specifically.** The IDS/IPS distinction is examined in every form.
