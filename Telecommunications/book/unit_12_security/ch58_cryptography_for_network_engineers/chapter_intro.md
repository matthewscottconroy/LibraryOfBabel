# Chapter 58 — Cryptography for Network Engineers

A warning before anything else, and it is the most important sentence in the chapter:

> **Do not implement cryptography.**

Not because it is beyond you, but because the failure mode is uniquely bad.
Cryptographic code that is subtly wrong produces output that is indistinguishable from
correct output. It encrypts. It decrypts. Tests pass. There is no error message, no
alarm, and no symptom — right up until someone who knows what they are doing reads
your ciphertext in an afternoon.

Every other bug in this book announces itself eventually. A wrong subnet mask breaks
connectivity. A spanning tree loop takes the network down. A cryptographic mistake
looks exactly like success.

So this chapter's purpose is not to make you a cryptographer. It is to give you enough
working knowledge to **deploy** cryptography correctly — to read a cipher suite, to
know why a certificate error appeared, to choose between IPsec and WireGuard on
grounds other than familiarity, and to recognise when something you are being sold is
nonsense.

## The four primitives

Everything in this chapter is one of four things, and keeping them distinct is most of
the battle.

**Symmetric encryption.** One key, shared, used for both encryption and decryption.
Fast — AES on modern hardware with AES-NI instructions runs at gigabytes per second —
and therefore what actually protects bulk data. AES is the standard; ChaCha20 is the
alternative used where AES hardware acceleration is absent, notably on some mobile
devices.

The problem is in the word "shared." If two parties already share a secret key they
can communicate securely; the difficulty is arriving at that shared key over a channel
an adversary is listening to. For most of history the answer involved couriers, and it
did not scale.

**Asymmetric (public key) encryption.** Two mathematically related keys: one published
freely, one kept secret. What one encrypts, the other decrypts. This solves key
distribution — you can publish your public key on a billboard — at the cost of being
two to three orders of magnitude slower than symmetric.

So the universal pattern, and it is worth stating explicitly because it explains the
structure of every protocol in Chapters 59 through 61: use asymmetric cryptography
to establish a symmetric key, then use symmetric cryptography for the data. TLS does
this. IPsec does this. SSH does this. WireGuard does this.

**Hash functions.** A one-way transformation from arbitrary input to a fixed-length
digest. Fast, deterministic, infeasible to reverse, and infeasible to find two inputs
with the same digest. SHA-256 and SHA-3 are current; MD5 and SHA-1 are broken for
collision resistance — practical collisions have been demonstrated for both — and
their continued presence in production systems is a real finding.

A hash alone provides no authentication: an adversary who alters data can recompute
the hash. Combining a hash with a secret key gives a **MAC** (HMAC), which does.

**Digital signatures.** Asymmetric cryptography run backwards: sign with the private
key, verify with the public. Provides authentication, integrity and non-repudiation —
only the holder of the private key could have produced it, and they cannot plausibly
deny doing so.

## The idea worth understanding properly

If you take one mechanism from this chapter in detail, take **Diffie–Hellman key
exchange**, published by Whitfield Diffie and Martin Hellman in 1976 in a paper called
*New Directions in Cryptography*, which opens with the sentence "We stand today on the
brink of a revolution in cryptography" — a claim that turned out to be
understatement.

It permits two parties who have never met, communicating entirely over a channel an
adversary is recording in full, to arrive at a shared secret that the adversary cannot
compute. Not obscure it — the adversary sees every message — but genuinely cannot
derive it, because doing so requires solving the discrete logarithm problem.

§58.2 works it through with small numbers so you can follow the arithmetic yourself.
It takes ten minutes and it is one of the genuinely delightful results in applied
mathematics.

The consequence that matters operationally is **forward secrecy**: if a fresh
Diffie–Hellman exchange is performed for each session, then compromising a server's
long-term private key later does not decrypt previously recorded traffic, because that
key was used only to *authenticate* the exchange and not to establish the session key.
This is why TLS 1.3 removed static RSA key exchange entirely and mandates ephemeral
Diffie–Hellman, and it is why "record now, decrypt later" attacks are defeated by
modern configurations and were not by older ones.

## Certificates and the trust problem

Diffie–Hellman gives you a shared secret with *somebody*. It does not tell you who,
and an adversary in the middle can perform two separate exchanges — one with each
party — and relay between them, reading everything. Key exchange without
authentication is defeated trivially.

So the public key must be bound to an identity, by someone you already trust. That
binding is a **certificate**: a document containing a public key and an identity,
signed by a **certificate authority**.

§58.4 covers the chain of trust — root CAs pre-installed in your operating system and
browser, intermediate CAs, and the end-entity certificate — and the validation steps a
client performs: signature chain, validity dates, revocation status, hostname match,
and key usage.

It also covers the model's genuine weaknesses honestly, because they are structural
rather than incidental. Any CA can issue a certificate for any name. Your browser
trusts several hundred roots, operated in many jurisdictions, and a compromise or
coercion of any one of them produces a certificate your browser will accept. This has
happened — the DigiNotar compromise in 2011 produced fraudulent certificates for
Google domains that were used against Iranian users, and the CA was destroyed as a
business by the incident.

The mitigations — Certificate Transparency's public append-only logs, CAA records
(Chapter 39's record types) restricting which CAs may issue for a domain, and pinning —
are covered, along with an assessment of how much each actually helps.

## TLS 1.3

§58.4 finishes with the handshake in full, because it is the protocol you will see
most often and because its design is a case study in learning from failure.

TLS 1.3 (RFC 8446, 2018) removed a great deal: static RSA key exchange, all
non-AEAD ciphers, compression, renegotiation, MD5 and SHA-1, and the CBC constructions
that produced a decade of padding-oracle attacks. The changes are almost entirely
removals, each corresponding to a specific published attack — BEAST, CRIME, POODLE,
Lucky13, FREAK, Logjam, DROWN — and knowing that history is what makes "why is this
option gone" answerable.

It also reduced the handshake from two round trips to one, with an optional zero
round-trip resumption. That is Chapter 3 §3.4's lesson again: round trips are the
expensive thing, and a substantial fraction of modern protocol engineering is spent
removing them.

## By the end you will be able to

- Distinguish the four primitives and state what each does and does not provide.
- Explain why hybrid encryption is universal, and identify the pattern in any protocol.
- Work a Diffie–Hellman exchange with small numbers.
- Explain forward secrecy and why TLS 1.3 mandates it.
- Read a certificate, describe its chain, and diagnose the common validation failures.
- Explain the CA model's structural weakness and what CT and CAA do about it.
- Read a cipher suite string and say what each component provides.
