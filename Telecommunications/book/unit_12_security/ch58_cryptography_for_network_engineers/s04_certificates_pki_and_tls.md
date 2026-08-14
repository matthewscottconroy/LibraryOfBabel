# 58.4 Certificates, PKI and TLS

§58.2 left a problem: a key exchange establishes secrecy with somebody, and does not tell you
with whom. A certificate is the answer, and it is the part of cryptography that network
engineers actually operate.

## What a certificate is

> A certificate is a signed statement that a particular public key belongs to a particular
> name.

**That is all it is.** Everything else — chains, authorities, revocation, transparency — is
machinery for deciding whether to believe the statement.

```
   ┌──────────────────────────────────────────┐
   │  Subject:      CN=www.example.com        │  ← the name
   │  SAN:          www.example.com,          │  ← the names it actually covers
   │                example.com               │
   │  Public key:   <the key being bound>     │  ← the key
   │  Issuer:       CN=Example CA R3          │  ← who says so
   │  Valid:        2026-03-01 to 2026-05-30  │  ← for how long
   │  Serial, extensions, key usage, …        │
   ├──────────────────────────────────────────┤
   │  Signature by the issuer's private key   │  ← the assertion
   └──────────────────────────────────────────┘
```

**Two practical notes that catch people.**

**The Common Name is obsolete.** Modern clients ignore CN entirely and use the Subject
Alternative Name extension — a certificate with the right CN and no matching SAN will be
rejected, and this is a recurring cause of "it worked on the old system."

**Key usage extensions are enforced.** A certificate marked for server authentication cannot
be used for code signing, and a CA certificate without the basic constraints CA flag cannot
issue. These are checked, and misconfigurations here produce errors that look mysterious.

## The chain

Trust is delegated, and the chain is how.

```
   ┌─────────────────────┐
   │   Root CA           │  self-signed; in the trust store; offline
   └──────────┬──────────┘
              │ signs
   ┌──────────┴──────────┐
   │  Intermediate CA    │  online; does the actual issuing
   └──────────┬──────────┘
              │ signs
   ┌──────────┴──────────┐
   │  End-entity cert    │  your server
   └─────────────────────┘
```

**Why an intermediate exists at all:** the root's private key is the crown jewel. It is
kept offline, in hardware, in a safe, and used a handful of times a year — because
compromise of a root is unrecoverable: it is in millions of trust stores and cannot be
replaced quickly. The intermediate does the daily work and can be revoked and replaced if it
is compromised.

**What the client validates, in order:**

1. The chain reaches a trusted root in its store
2. **Each signature** verifies against the parent's public key
3. **Validity dates** — every certificate in the chain
4. **The name matches** — SAN against the requested hostname
5. Key usage and basic constraints permit this use
6. **Revocation status** — with the caveat below
7. The signature algorithm is acceptable — SHA-1 is not

> The commonest server misconfiguration by a very wide margin: the intermediate certificate is
> not sent. Browsers frequently recover by fetching it or by having cached it; other clients
> — `curl`, Java, mobile applications, IoT devices — do not. The symptom is "it works in my
> browser and fails everywhere else", and it is diagnosed in one command:
> `openssl s_client -connect host:443 -showcerts`.

## Revocation, which does not really work

The honest section, because the mechanisms are widely deployed and largely ineffective.

| Mechanism | How | Problem |
|---|---|---|
| **CRL** | **a list of revoked serials, downloaded** | **large, cached, and out of date** |
| **OCSP** | **ask the CA about one certificate** | **latency, a privacy leak to the CA, and it fails open** |
| **OCSP stapling** | **the server presents a recent signed status** | **better, and optional, so absence proves nothing** |
| **Must-staple** | **the certificate requires stapling** | **the actual fix, and barely deployed** |
| **Short lifetimes** | **the certificate expires before revocation matters** | **the answer the industry chose** |

**The fatal flaw is soft-fail.**

> **If a browser cannot reach the OCSP responder, it proceeds.** Which means an attacker who
> can intercept the connection — the exact attacker revocation exists to stop — can also block
> the OCSP query, and the check is skipped.

Hard-fail was tried and abandoned, because a CA responder outage would break the web,
and that failure mode was judged worse than the attack.

So the industry's actual answer is short certificate lifetimes. Maximum public certificate
validity has fallen from five years, to three, to two, to 398 days, and is heading towards
weeks. A certificate that lives 47 days does not need effective revocation — and this
makes automated issuance and renewal mandatory rather than merely convenient.

> If your certificate renewal process involves a human, a calendar reminder and a copy-paste,
> it will fail — and shorter lifetimes convert that from an annual risk to a monthly
> certainty. ACME (Let's Encrypt's protocol, RFC 8555) is the answer, and it is available
> from several CAs including commercial ones.

## Certificate transparency

The response to a real failure mode: a CA issuing a certificate it should not have.

**Which has happened**: **DigiNotar (2011)** was compromised and issued a valid certificate for
`*.google.com`, used to intercept Iranian users' traffic. The company did not survive.
Symantec, TrustWave and others have had their own incidents.

**Certificate Transparency's mechanism:**

> Every issued certificate is logged to public, append-only, cryptographically verifiable
> logs, and browsers require proof of logging (an SCT) before accepting a certificate.

Which does not prevent misissuance. It makes it detectable.

And the practical consequence for you is twofold.

You can monitor for certificates issued for your domains — `crt.sh` and commercial services
will alert you — and an unexpected certificate is a serious finding.

And every hostname you obtain a certificate for is public, permanently. Internal
hostnames, staging environments, customer names in subdomains — all of it is in a public,
searchable log, and Chapter 57 §57.4's reconnaissance uses exactly this. Use wildcards or an
internal CA for names you do not want published.

## Running a private CA

Where an internal CA is right, and what it costs.

**Right for:** internal services, device authentication (802.1X, Chapter 59 §59.2), VPN client
certificates, mutual TLS between internal systems, anything where you control both ends.

**Wrong for:** anything a browser or an uncontrolled client must trust, because the root
must be distributed to every client and it will not be.

**What it costs, honestly:**

| | |
|---|---|
| **Root key protection** | **offline, ideally in an HSM.** A root private key on a file server is not a CA |
| **Distribution of the root** | to every device that must trust it — **and to every device you acquire later** |
| **Issuance process** | **who may request a certificate, and how is the request authorised** |
| **Revocation** | **which you must operate, with the same soft-fail problem** |
| **Renewal** | **automated, or it will fail** |
| **The root's own expiry** | **and this is the one that catches people** |

> A private root certificate with a 20-year lifetime expires in 20 years, and the plan for
> that day is not written down anywhere. **It has produced large outages** — every device
> trusting it stops trusting everything at once. **Put the root's expiry in the lifecycle
> register** (Chapter 55 §55.3).

**And the operational advice:** **use a tool.** `step-ca`, HashiCorp Vault's PKI, Microsoft
AD CS or EJBCA — not a directory of `openssl` commands in a wiki, which is how most private
CAs are run and why most of them are in poor condition.

## The TLS 1.3 handshake

Worth walking through, because it is the protocol you will actually debug.

```
   Client                                              Server
     │                                                    │
     │──── ClientHello ──────────────────────────────────▶│
     │     supported versions, cipher suites,             │
     │     KEY SHARE (an ephemeral public key, guessed)   │
     │                                                    │
     │◀─── ServerHello ───────────────────────────────────│
     │     chosen suite, KEY SHARE                        │
     │     ── from here, everything is encrypted ──       │
     │◀─── EncryptedExtensions, Certificate,              │
     │     CertificateVerify, Finished ───────────────────│
     │                                                    │
     │──── Finished ─────────────────────────────────────▶│
     │──── application data ─────────────────────────────▶│
```

Four things changed from TLS 1.2, and each matters.

**One round trip instead of two.** The client guesses the server's preferred group and sends
its key share in the first message. If it guesses wrong, a HelloRetryRequest costs an extra
round trip — which is why the guess is worth getting right, and why clients offer X25519
first.

**The certificate is encrypted.** In TLS 1.2 the server's certificate crossed in the clear,
so an observer learned which site you were visiting. In 1.3 it does not — though SNI
in the ClientHello still leaks the hostname, which Encrypted Client Hello addresses and which
is still being deployed.

**Forward secrecy is mandatory.** Every non-ephemeral key exchange was removed (§58.2).

**The cipher suite list is short.** TLS 1.2 had hundreds of suites, many weak, and negotiating
them safely was genuinely hard. TLS 1.3 has five, all AEAD:

```
   TLS_AES_128_GCM_SHA256
   TLS_AES_256_GCM_SHA384
   TLS_CHACHA20_POLY1305_SHA256
   TLS_AES_128_CCM_SHA256
   TLS_AES_128_CCM_8_SHA256
```

> Note what is no longer negotiable: the key exchange and the authentication algorithm are
> separated out, so **a suite name no longer encodes four choices.** This alone eliminated a
> large class of downgrade attacks.

**And 0-RTT deserves its warning.** TLS 1.3 permits sending application data in the first
flight using a key from a previous session — **zero round trips.** It is not replayable-safe:
an attacker who captures the 0-RTT data can replay it. Use it only for idempotent requests,
and never for anything that changes state.

## The transition ahead

Two lifecycles that belong in Chapter 55 §55.3's register.

**Post-quantum.** A sufficiently large quantum computer breaks RSA and elliptic curve
cryptography entirely — Shor's algorithm solves both factoring and discrete logarithms
efficiently. Symmetric cryptography is only halved (Grover's), so AES-256 remains
adequate.

No such machine exists, and the transition has begun anyway, for the reason §58.2 gave:
harvest now, decrypt later. NIST standardised ML-KEM (Kyber) for key encapsulation and
ML-DSA (Dilithium) and SLH-DSA for signatures in 2024, and hybrid key exchange — classical
plus post-quantum, so that breaking either alone is insufficient — is already deployed in
Chrome, Cloudflare and OpenSSH.

**The practical position for a network engineer:** you do not need to act yet, and you need to
know it is coming, and the first thing that will bite is that post-quantum keys and signatures
are much larger — which affects handshake size, MTU, and constrained devices.

**Shortening certificate lifetimes.** **Already under way**, and it makes automation mandatory.
An organisation whose renewal is manual should treat this as a project with a deadline, not
as a future consideration.

## What breaks here

"Certificate not trusted" in one client and fine in a browser. The intermediate is not
being sent. `openssl s_client -showcerts`.

**"Name mismatch" with a correct Common Name.** **No matching SAN.** CN is ignored.

**A certificate that expired.** The most predictable outage in this book (Chapter 55 §55.3),
and still the most common self-inflicted one. **Monitor and automate.**

A private CA root expiring and everything failing at once. It was not in the lifecycle
register.

**Revocation not working.** It largely does not. Short lifetimes are the actual answer.

An unexpected certificate for your domain in a CT log. **A serious finding.** Investigate
immediately.

**Internal hostnames discovered in CT logs.** Expected, and it is a reconnaissance source.
Wildcards or an internal CA.

**0-RTT data replayed.** **Documented behaviour.** Idempotent requests only.

A pinned certificate breaking after a legitimate renewal. Pinning is a foot-gun — it
protects against a real attack and it has taken more services offline than it has protected.
Pin to a CA or a key that outlives the certificate, if at all.

> **Network+ note.** Objective 4.1 covers PKI. Over-learn: a certificate binds a public key to
> an identity and is signed by a CA; a chain of trust runs from an end-entity certificate
> through intermediates to a trusted root; **self-signed certificates are not trusted by
> default**; **CRL and OCSP are the revocation mechanisms**; and **TLS uses certificates to
> authenticate the server.** The chain-of-trust concept is examined and the intermediate
> misconfiguration is what you will actually meet.
