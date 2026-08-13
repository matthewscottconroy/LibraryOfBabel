# Chapter 61 — The People

**Steven Bellovin**, again — **and this time for what he said about IPsec rather than about
TCP/IP.**

**Bellovin's 1996 paper "Problem Areas for the IP Security Protocols"** found **serious flaws in
the then-current IPsec drafts**, including **the encryption-without-authentication modes that
permitted an attacker to modify traffic undetectably.**

> **The finding is the general one: ESP with encryption and no integrity protection is
> dangerous**, because **an attacker who can modify ciphertext can, in several modes, produce
> predictable changes in the plaintext.** **This is why AEAD exists** (Chapter 58 §58.1) **and
> why every modern configuration authenticates as well as encrypts.**

**Bellovin has also been among IPsec's more persistent critics** on the grounds that **its
complexity is itself a security property** — **a protocol nobody can configure correctly is not
secure regardless of its cryptography** — **which is §61.2's argument and Chapter 58's
implementation-versus-algorithm observation, together.**

**Niels Ferguson and Bruce Schneier.** **"A Cryptographic Evaluation of IPsec" (1999)** — **the
most-cited critique of the protocol, and it is unusually blunt.**

**Their conclusion is worth quoting because it is precisely calibrated:**

> **"IPsec was a great disappointment to us. Given the quality of the people that worked on it
> and the time that was spent on it, we expected a much better result… Our main criticism of
> IPsec is its complexity."**

**And then the sentence that matters:**

> **"We strongly discourage the use of IPsec in its current form. However, we are not aware of
> any other protocol that does a better job."**

**Which is an honest assessment rather than a rejection**, and **it remained the situation for
about eighteen years.**

**Their specific recommendations were largely adopted:** **eliminate transport mode, eliminate
AH, reduce the number of options, and simplify the key exchange.** **IKEv2 did most of this**,
and **WireGuard did all of it.**

**Hugo Krawczyk.** **The cryptographic core beneath IPsec, TLS and much else.**

**Krawczyk's contributions are the constructions rather than the protocols:**

| | |
|---|---|
| **HMAC** (with Bellare and Canetti, 1996) | Chapter 58 §58.3 |
| **HKDF** (RFC 5869) | **the key derivation used by TLS 1.3, IPsec and WireGuard alike** |
| **SIGMA** | **the "SIGn-and-MAc" design pattern underlying IKEv2 and TLS 1.3's handshake** |
| The "encrypt-then-MAC" analysis | **which settled the ordering question TLS 1.2 had got wrong** |

> **SIGMA is the interesting one for this chapter.** **It is the answer to §58.2's problem —
> Diffie–Hellman gives secrecy with somebody and does not say with whom** — **and it specifies
> exactly how to authenticate a key exchange without leaking the parties' identities to a
> passive observer.** **IKEv2's design is SIGMA; so is TLS 1.3's.**

**Krawczyk's work is the reason the modern protocols in this chapter are provably sound where
their predecessors were assembled by argument.**

**Jason A. Donenfeld (b. 1989).** **WireGuard, released 2016, merged into Linux 5.6 in 2020.**

**What is unusual about it is the goal.** **Donenfeld's stated objective was not better
cryptography** — **the primitives are all pre-existing and well-studied** — **but an
implementation small enough to be read.**

| | |
|---|---|
| **About 4,000 lines** | against IPsec's hundreds of thousands |
| **One cipher suite** | no negotiation, no downgrade |
| **Built on the Noise protocol framework** (Trevor Perrin) | **a formally analysed handshake pattern** |
| **Formally verified** | by several independent groups, in Tamarin and in F* |

> **"The entire codebase can be read in an afternoon" is a security claim**, and it is the same
> claim as Anderson's reference monitor requirement (Chapter 57's reading): **small enough to be
> verified.** **Twenty-two years after Ferguson and Schneier asked for a simpler IPsec, someone
> wrote one and did not call it IPsec.**

**Linus Torvalds's public comment on merging it** — that it was **"a work of art" compared with
the alternatives** — **is unusual praise from a source that does not give it**, and it
accelerated adoption considerably.

**Donenfeld has been explicit that WireGuard is deliberately incomplete** (§61.3): **key
distribution is out of scope**, and **that is a design decision rather than an omission.**

**Trevor Perrin.** **The Noise protocol framework, and the Signal protocol's double ratchet with
Moxie Marlinspike.**

**Noise is a framework for building handshake patterns from Diffie–Hellman operations**, with
**each pattern's security properties derived rather than asserted.** **WireGuard uses
`Noise_IKpsk2`; Signal, WhatsApp and others use Noise-derived constructions**, and **the framework's
value is that a designer chooses a pattern with known properties rather than inventing a
handshake.**

> **Which is the same argument as "do not implement cryptography"** (Chapter 58's opening),
> **applied one level up: do not design handshakes either.**

**Gurdeep Singh Pall and the PPTP team at Microsoft**, for the counter-example.

**PPTP shipped in Windows NT 4.0 in 1996 and made VPNs mainstream.** **Its cryptography was
broken by Bruce Schneier and Peter Mudge Zatko in 1998**, comprehensively, **and the revised
version (MS-CHAPv2) was broken again in 2012** — **reduced to a single DES key's worth of work,
which is hours.**

> **PPTP's contribution was demonstrating demand.** **Millions of people used a VPN because
> Windows included one**, and **the market that IPsec and TLS VPNs then served existed because
> PPTP created it.** **Its cryptography was poor and its effect was substantial**, which is an
> uncomfortable pattern this book has met before (Chapter 58's SSL).

## What this chapter's history establishes

**The critique was correct, was published in 1999, and was acted on in 2016.**

**Ferguson and Schneier asked for fewer options, no transport mode, no AH and a simpler key
exchange.** **The IETF delivered IKEv2 in 2005, which was a substantial improvement and kept the
architecture.** **WireGuard delivered the rest by starting again outside the standards process.**

> **Which raises the question of whether the standards process could have produced WireGuard,
> and the honest answer is probably not** — **because a working group's incentive is to
> accommodate every participant's requirement**, and **WireGuard's entire method is refusing
> to.** **Chapter 22's OSI story with the roles reversed: this time the committee produced the
> complex thing and an individual produced the simple one that shipped.**

**And the second observation is that simplicity became achievable rather than merely
desirable.** **WireGuard could be simple because the cryptography beneath it — Curve25519,
ChaCha20-Poly1305, BLAKE2s, Noise — had matured to the point where one good choice existed for
each decision.** **In 1998 it did not, and agility was a reasonable response to genuine
uncertainty.**
