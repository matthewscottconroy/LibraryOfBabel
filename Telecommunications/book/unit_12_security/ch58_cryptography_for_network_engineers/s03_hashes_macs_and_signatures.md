# 58.3 Hashes, MACs and Signatures

Three mechanisms that are constantly confused, and the confusion produces real
vulnerabilities. They provide different things and require different inputs.

| | Requires | Provides |
|---|---|---|
| **Hash** | **nothing** | **integrity against accident** |
| **MAC** | **a shared secret** | **integrity and origin, between key-holders** |
| **Signature** | **a private key** | **integrity, origin, and non-repudiation, to anyone** |

## Hash functions

A one-way transformation from arbitrary input to a fixed-length digest.

**The four required properties:**

| Property | Meaning |
|---|---|
| **Deterministic** | same input, same output, always |
| **Fast** | to compute |
| **Preimage resistant** | **given $h$, infeasible to find $m$ with $H(m) = h$** |
| **Second preimage resistant** | **given $m_1$, infeasible to find $m_2 \ne m_1$ with the same digest** |
| **Collision resistant** | **infeasible to find any $m_1 \ne m_2$ with the same digest** |

Collision resistance is the weakest of the three and the first to fall, because of the
birthday bound:

$$\text{collisions appear after roughly } 2^{n/2} \text{ attempts for an } n\text{-bit digest}$$

| Digest | Bits | **Collision effort** |
|---|---|---|
| MD5 | 128 | $2^{64}$ — **and in practice, seconds** |
| **SHA-1** | 160 | $2^{80}$ nominally; **$2^{63}$ with the 2017 attack** |
| **SHA-256** | 256 | $2^{128}$ — **infeasible** |
| SHA-512, SHA-3 | 512 | more so |

![The birthday bound computed: collision probability against hashes tried, for 128- and 160-bit digests. The curve is essentially zero and then essentially one — the transition sits at $2^{n/2}$, and moving from a 128-bit to a 160-bit digest buys five orders of magnitude, not a different shape.](../../figures/birthday_bound.svg){width=90%}

### MD5 and SHA-1 are broken

And "broken" means specifically: collision resistance has failed.

| | |
|---|---|
| **MD5** | **collisions demonstrated in 2004; now computable in seconds on a laptop** |
| **SHA-1** | **SHAttered, 2017 — two different PDFs with the same digest**; then **chosen-prefix collisions in 2020** |

> **The chosen-prefix collision is the one that matters practically.** It permits an attacker
> to construct two documents with arbitrary different content sharing a digest — which is what
> makes a forged certificate possible, and it is why certificate authorities were required to
> abandon SHA-1.

What is still safe about them, and why they persist:

**Preimage resistance has not fallen.** You still cannot, given an MD5 digest, find an input
producing it. Which is why MD5 remains acceptable for non-adversarial integrity checking —
a file transfer checksum, a cache key — and unacceptable anywhere an adversary chooses the
input.

The distinction matters because "MD5 is broken, remove it everywhere" is over-broad advice
that gets ignored, and "MD5 must not be used where an adversary influences the input" is
precise and actionable.

Where finding SHA-1 or MD5 is a genuine finding:

- **Certificate signatures** (§58.4)
- Code signing and software update verification
- **Password storage** — and it was always wrong there, for a different reason
- Any protocol where an attacker supplies data that is hashed and compared

### Password hashing is a different problem

Worth its own note because the requirement is inverted.

> A hash function's speed is a virtue everywhere except password storage, where it is the
> vulnerability.

A fast hash means an attacker with a stolen database can try billions of candidate passwords
per second on a GPU. The answer is a deliberately slow, memory-hard function:

| Use | |
|---|---|
| **Argon2id** | **the current recommendation** |
| **scrypt**, **bcrypt** | acceptable |
| **PBKDF2** | acceptable with a high iteration count; **required by some standards** |
| **SHA-256, MD5, SHA-1** | **wrong, regardless of salting** |

**And salting is necessary and not sufficient.** A salt defeats precomputed rainbow tables; it
does nothing about the speed of the guessing. **Both are required.**

## MACs

**A hash keyed with a shared secret.**

$$\mathrm{HMAC}(K, m) = H\big((K \oplus \text{opad}) \,\|\, H((K \oplus \text{ipad}) \,\|\, m)\big)$$

**The nested construction is not decoration.** The naive $H(K \| m)$ is vulnerable to a length
extension attack — an attacker who knows $H(K \| m)$ and $|m|$ can compute $H(K \| m \|
\text{padding} \| m')$ without knowing $K$ — because Merkle–Damgård hashes (MD5, SHA-1,
SHA-2) expose their internal state as the digest.

> **HMAC exists specifically to defeat length extension**, and it is the reason you should
> never construct a MAC yourself from a hash function. SHA-3 and BLAKE2 are not vulnerable
> to length extension and can be keyed directly — which is one of SHA-3's design goals.

**What a MAC gives you:**

- **Integrity** — the message was not modified
- **Origin authentication** — it came from someone holding the key

**And what it does not:**

> A MAC cannot prove to a third party who sent the message, because **both parties hold the
> same key** — **either could have produced the MAC.** **This is why a MAC gives no
> non-repudiation**, and it is the essential difference from a signature.

**Which is sometimes a feature.** Deniability is desirable in some protocols — messaging
systems deliberately use MACs so that a leaked transcript proves nothing.

## Signatures

Asymmetric cryptography run backwards: sign with the private key, verify with the public.

```
   Signing:      message ──▶ hash ──▶ [ sign with private key ] ──▶ signature
   Verification: message ──▶ hash ──┐
                 signature ─▶ [ verify with public key ] ──▶ match?
```

Note that the signature is over the hash, not the message. Which is why a hash collision
breaks signatures: two messages with the same digest have the same valid signature, and
the attacker signs the innocuous one and presents the malicious one.

**The three properties:**

| | |
|---|---|
| **Integrity** | the message was not modified |
| **Authentication** | **only the private key holder could have produced it** |
| **Non-repudiation** | **and they cannot plausibly deny doing so** |

**The algorithms:**

| | Notes |
|---|---|
| **RSA-PSS** | **the modern RSA signature padding**; PKCS#1 v1.5 is legacy and has had implementation problems |
| **ECDSA** | **elliptic curve; ubiquitous** — **and it has a dangerous requirement** |
| **Ed25519** | **deterministic, fast, and safe by construction** |

### The ECDSA nonce disaster

Worth knowing because it is the clearest example of a correct algorithm destroyed by
implementation.

> **ECDSA requires a random value $k$ per signature.** Reuse $k$ across two signatures and the
> private key can be recovered algebraically from the two signatures alone.

**And it has happened repeatedly and expensively:**

- Sony's PlayStation 3 (2010) — **a fixed $k$**, and the console's signing key was
  recovered and published
- Android's Bitcoin wallets (2013) — a weak random number generator produced repeated
  $k$, and funds were stolen
- **Multiple embedded devices** with insufficient entropy at first boot

**Ed25519's answer is to remove the requirement:** it derives $k$ deterministically from the
private key and the message. There is no random value to get wrong, and this is the
argument for preferring it.

> **The pattern recurs throughout this chapter.** **AES-GCM's nonce, ECDSA's $k$, CBC's IV** —
> three cases where a correct algorithm has a value that must never repeat, and three
> catalogues of production failures. Prefer constructions that remove the requirement rather
> than constructions that state it.

## Choosing between them

A decision table, because the confusion in the introduction is the practical problem.

| You need to | Use |
|---|---|
| **Detect accidental corruption** | **a hash — or a CRC, which is faster** (Chapter 15 §15.4) |
| **Verify a download matches what the publisher intended** | **a signature** — a published hash is only as trustworthy as the channel that published it |
| **Authenticate messages between two systems that share a secret** | **a MAC (HMAC)** |
| **Prove to a third party who sent something** | **a signature** |
| **Authenticate and encrypt together** | **AEAD** (§58.1) — do not compose it yourself |
| **Store passwords** | **Argon2id** — not a general-purpose hash |
| **Deduplicate or index content** | **a hash** — and MD5 is fine if no adversary chooses the input |

The second row deserves expansion, because it is a common real error:

> Publishing a SHA-256 of a download on the same website as the download proves nothing
> against an attacker who compromised the website. **They change both.** A signature with a
> key distributed separately is what actually helps, which is why package managers verify
> signatures rather than hashes.

## What breaks here

**SHA-1 in a certificate chain.** Rejected by modern clients. Reissue.

MD5 used where an adversary supplies the input. **A finding.** Where no adversary does, it
is fine and the noise about it is unhelpful.

**Passwords stored with SHA-256, salted.** **Still wrong.** The salt is not the problem; the
speed is. Argon2id.

**A MAC constructed as $H(K \| m)$.** **Length extension.** Use HMAC, or a hash that is not
Merkle–Damgård.

A signature verified against a certificate that was not itself validated. Extremely
common, and it is §58.4's subject. A valid signature from an unverified key proves nothing.

**ECDSA with a repeated nonce.** The private key is recoverable from two signatures. Ed25519
removes the possibility.

**A download's hash published beside the download.** It defends against corruption, not
against compromise.

"We hash the password before sending it, so it is secure." The hash is now the password.
An attacker who captures it can replay it, and TLS was required anyway.

> **Network+ note.** Objective 4.1 covers hashing. Over-learn: **hashing is one-way and provides
> integrity, not confidentiality**; **MD5 and SHA-1 are deprecated, SHA-256 is current**;
> **salting defends against rainbow tables**; **a digital signature provides authentication,
> integrity and non-repudiation**; and **encryption is reversible, hashing is not.** The
> encryption-versus-hashing distinction is examined constantly.
