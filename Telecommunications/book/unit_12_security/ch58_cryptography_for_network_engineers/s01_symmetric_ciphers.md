# 58.1 Symmetric Ciphers

One key, shared, used for both encryption and decryption. Fast, well understood, and it
protects essentially all bulk data on the Internet — every TLS session, every VPN tunnel,
every encrypted disk.

## The shape

```
   plaintext ──▶ [ E ] ──▶ ciphertext ──▶ [ D ] ──▶ plaintext
                  ▲                        ▲
                  └────── same key ────────┘
```

**The security property:** an adversary who sees the ciphertext and knows the algorithm
completely, but not the key, learns nothing about the plaintext. **Kerckhoffs's principle**
(Chapter 57's reading) — the enemy knows the system.

## AES, and why the key size argument is over

The Advanced Encryption Standard, selected in 2001 after an open five-year competition, and
it is the answer.

| | |
|---|---|
| Block size | **128 bits** |
| Key sizes | **128, 192, 256 bits** |
| Structure | substitution–permutation, 10/12/14 rounds |
| **Hardware support** | **AES-NI instructions on essentially every modern CPU** |
| **Throughput** | **gigabytes per second per core** |
| Status | **no practical attack against the full cipher after 25 years of public analysis** |

**On key size, the arithmetic settles it:**

| Key | Keys to search | |
|---|---|---|
| **56-bit (DES)** | $7.2 \times 10^{16}$ | **brute-forced in 1998, in 56 hours, for \$250,000** |
| **128-bit** | $3.4 \times 10^{38}$ | **beyond any conceivable brute force** |
| 256-bit | $1.2 \times 10^{77}$ | more so |

> **AES-128 will not be brute-forced.** Enumerating $2^{128}$ keys at a billion billion
> ($10^{18}$) keys per second takes about $1.1 \times 10^{13}$ years — roughly 800 times the
> age of the universe — and the energy required to run the counter alone exceeds what is
> available.

![Exhaustive key search at $10^{18}$ keys per second. The scale is logarithmic, so the straight line is exponential growth: DES's 56 bits fall in under a second, and 128 bits sit thirteen orders of magnitude above the age of the universe. Key length is the one place in security where a small number settles an argument.](../../figures/keyspace.svg){width=90%}

**So why does AES-256 exist?** Two honest reasons and one poor one.

**Quantum computing.** Grover's algorithm halves the effective key length (§58.4), so
**AES-256 becomes AES-128-equivalent** and remains adequate. This is the good reason, and
it is why AES-256 is specified for long-lived secrets.

**Compliance.** Some regimes require it. Legitimate, if uninteresting.

**And the poor reason:** "bigger is more secure" as a procurement instinct. AES-256 costs
about 40% more compute than AES-128 for no practical gain against classical adversaries, and
neither is where any real system fails. Systems fail at key management, at protocol
design, and at implementation — never at the cipher.

## ChaCha20, and why it exists

A stream cipher by Daniel Bernstein, and the standard alternative.

| | **AES** | **ChaCha20** |
|---|---|---|
| Type | **block** | **stream** |
| **With hardware acceleration** | **faster** | comparable |
| **Without hardware acceleration** | **slow, and vulnerable to timing attacks in software** | **fast and constant-time** |
| Implementation | **hard to implement safely in software** — table lookups leak timing | **simple, naturally constant-time** |

> ChaCha20's argument is not that AES is weak. It is that AES is hard to implement safely in
> software, because the table lookups that make it fast are also timing side channels.
> **ChaCha20 uses only additions, rotations and XORs**, which run in constant time by
> construction.

Which is why it is preferred on devices without AES-NI — many mobile processors, embedded
devices, older hardware — and why TLS clients frequently offer ChaCha20-Poly1305 first when
they detect no hardware acceleration.

## Modes: where symmetric encryption actually goes wrong

A block cipher encrypts 128 bits. Real data is longer. The mode is how you chain blocks
together, and the mode is where the failures are.

### ECB, and the penguin

**Electronic Codebook: encrypt each block independently.**

> **Identical plaintext blocks produce identical ciphertext blocks.**

**Which means structure survives encryption.** The canonical demonstration is an image
encrypted with ECB, in which the picture remains clearly visible — usually a Linux penguin,
and it has convinced more engineers than any argument.

ECB has no legitimate use in network protocols. Its presence in a system is a finding,
and it appears in legacy industrial protocols and in badly-written applications with dispiriting
regularity.

### CBC, and why it is being retired

Cipher Block Chaining: XOR each plaintext block with the previous ciphertext block before
encrypting.

```
   P1 ─(+)─ [E] ──▶ C1 ──┐
        ▲                 │
       IV                 ▼
   P2 ─(+)─ [E] ──▶ C2 ──┐
```

Identical blocks now encrypt differently, and the IV must be random and unpredictable —
a predictable IV was the BEAST attack against TLS.

CBC is not broken as a cipher mode. It is fragile in practice, and its problems are all
about what surrounds it:

| Problem | |
|---|---|
| **Padding oracle attacks** | **the error message distinguishing "bad padding" from "bad MAC" leaks the plaintext** — Vaudenay 2002, and Lucky 13 in 2013 |
| **No built-in authentication** | **must be combined with a MAC, in the right order** |
| **MAC-then-encrypt vs encrypt-then-MAC** | **TLS chose the wrong one and paid for it for fifteen years** |
| Sequential | cannot be parallelised for encryption |

> **TLS 1.3 removed CBC entirely**, and this is the clearest signal available: the mode was
> retired not because the cipher failed but because too many implementations of the surrounding
> protocol failed.

### AEAD: the modern answer

Authenticated Encryption with Associated Data — encryption and authentication in one
construction, designed together.

| Mode | Notes |
|---|---|
| **AES-GCM** | **the standard.** Parallelisable, hardware-accelerated, fast |
| **ChaCha20-Poly1305** | **the software alternative**, same properties |
| AES-CCM | used in 802.11 (Chapter 44) and constrained devices |
| **AES-GCM-SIV** | **nonce-misuse resistant** — see the warning below |

What AEAD provides that encryption alone does not:

1. **Confidentiality** of the plaintext
2. **Integrity and authenticity** of the plaintext
3. Integrity and authenticity of associated data that is not encrypted — headers, sequence
   numbers, addresses

**Point 3 is the elegant part.** A packet's header must be readable to be routed and must not
be modifiable. AEAD covers it with the authentication tag without encrypting it, which is
exactly what a network protocol needs.

> Use AEAD. There is no longer a good reason to compose encryption and authentication
> yourself, and the history of attempts to do so is the history of protocol vulnerabilities.

### The GCM nonce warning

The one thing about AES-GCM every engineer should know:

> **Never reuse a nonce with the same key.** Repeating a nonce in GCM does not merely leak
> the relationship between two messages — it permits recovery of the authentication key, after
> which the attacker can forge arbitrary messages.

**This is a catastrophic and easy failure.** It has occurred in production TLS
implementations, in **hardware VPN products**, and in virtual machine snapshots that restore
a counter to a previous value.

**The defences:** counters rather than random nonces where message ordering permits;
rekeying before the counter space is exhausted; and AES-GCM-SIV where nonce uniqueness
cannot be guaranteed, which degrades gracefully instead of catastrophically.

## The problem symmetric cryptography cannot solve

**The word "shared" is doing the work.**

> If two parties already share a secret key they can communicate securely. The difficulty is
> arriving at that shared key over a channel an adversary is listening to.

**And it does not scale.** For $n$ parties to communicate pairwise, each pair needs its own
key:

$$\frac{n(n-1)}{2} \text{ keys}$$

| Parties | Keys |
|---|---|
| 10 | 45 |
| 100 | **4,950** |
| **1,000** | **499,500** |
| 1,000,000 | $5 \times 10^{11}$ |

Which is Chapter 11's and Chapter 51's full-mesh arithmetic, arriving a third time — and
each of those keys must be generated, distributed securely, stored securely, rotated and
revoked.

For most of history the answer involved couriers, and it did not scale then either. §58.2
is how the problem was solved.

## What breaks here

**ECB found in a production system.** **A finding.** Structure survives encryption; there is no
legitimate use.

**A padding oracle in a CBC-based protocol.** The error handling leaks the plaintext. Move
to AEAD.

**A nonce reused with AES-GCM.** **Catastrophic** — the authentication key is recoverable.
Counters, rekeying, or GCM-SIV.

**A VM snapshot restoring a GCM counter.** A real and non-obvious failure, and it is why
counter state must be considered when snapshotting anything cryptographic.

**AES-256 specified everywhere "to be safe".** Harmless and pointless against classical
adversaries. The system will fail elsewhere.

**"Military-grade encryption" in a product description.** A marketing phrase with no
technical content. Ask which cipher, which mode, which key size, and how keys are managed —
the last question is the one that distinguishes real products from bad ones.

**A proprietary cipher.** **Kerckhoffs.** Unreviewed cryptography has an excellent record of
being broken quickly, and a proprietary cipher is an admission that the vendor could not use a
standard one.

Encryption deployed and the keys stored beside the data. The commonest real failure, and
no cipher choice affects it.

> **Network+ note.** Objective 4.1 covers encryption concepts. Over-learn: symmetric
> encryption uses one shared key and is fast; asymmetric uses a key pair and is slow; **AES is
> the current symmetric standard**; **DES and 3DES are deprecated**; and the key exchange
> problem is what asymmetric cryptography solves. The symmetric/asymmetric distinction is
> examined constantly.
