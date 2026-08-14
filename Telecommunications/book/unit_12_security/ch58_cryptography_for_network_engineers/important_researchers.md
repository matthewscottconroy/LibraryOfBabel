# Chapter 58 — The People

Whitfield Diffie (b. 1944) and Martin Hellman (b. 1945). *New Directions in Cryptography*,
1976 — and the paper opens with "We stand today on the brink of a revolution in
cryptography", which turned out to be understatement.

What they did was harder than it sounds in retrospect. They did not merely invent a
protocol. They invented the idea that a protocol of that kind could exist — that two parties
with no prior secret could establish one over a public channel — at a time when the entire
field's assumption was that key distribution required a trusted channel.

Ralph Merkle's contribution is generally acknowledged alongside theirs. Merkle's puzzles,
developed as an undergraduate project that his professor did not understand and initially
rejected, contain the essential idea in a less efficient form, and Hellman has repeatedly
said the scheme should be called Diffie–Hellman–Merkle.

The paper also introduced the concept of a digital signature and of a public key
cryptosystem — **without providing one.** They stated the requirement; RSA supplied it a
year later.

> **Diffie and Hellman received the Turing Award in 2015.** Hellman has spent much of the
> intervening period working on nuclear risk reduction, on the grounds that it is a larger
> problem, which is a reasonable position for someone who has already solved one of the
> largest problems in his own field.

Ron Rivest (b. 1947), Adi Shamir (b. 1952) and Leonard Adleman (b. 1945). **RSA, 1977.**

**The story is worth knowing.** Rivest and Shamir proposed candidate schemes and Adleman broke
them — **repeatedly, over months.** The published scheme is the one Adleman could not
break, and the roles are the reason Adleman's name is on it, which he has said he
initially thought unwarranted.

Its significance is that it was the first concrete instance of what Diffie and Hellman had
described, and it does both encryption and signature — which Diffie–Hellman does not.

**And there is a prior claim.** Clifford Cocks, at GCHQ, described the RSA algorithm in
1973 — **three years earlier** — and James Ellis had described the concept of "non-secret
encryption" in 1970, and Malcolm Williamson had described Diffie–Hellman key exchange in 1974.
All three were classified until 1997.

> **Which is a genuinely interesting counterfactual.** **The classified work was correct and
> changed nothing**, because it could not be published, could not be reviewed, and could not be
> built on. **The public work created an industry.** A result that cannot be shared is, for
> almost every practical purpose, a result that does not exist.

Neal Koblitz (b. 1948) and Victor Miller (b. 1947). Elliptic curve cryptography, proposed
independently in 1985.

The observation was that the discrete logarithm problem can be posed in any suitable
mathematical group, and elliptic curve groups are harder per bit than integers modulo a
prime — because the index calculus algorithms that attack the integer case have no elliptic
curve analogue.

**Deployment took two decades.** Patents, unfamiliarity, and the difficulty of implementing
curve arithmetic safely all delayed it, and elliptic curves became mainstream only when
mobile devices made RSA's key sizes painful.

**Daniel J. Bernstein (b. 1971).** Curve25519, Ed25519, ChaCha20, Poly1305, NaCl — and a
design philosophy.

**Bernstein's contribution is not primarily new mathematics.** It is the argument that
cryptographic primitives should be designed so that implementations cannot easily be wrong.

| Conventional design | Bernstein's |
|---|---|
| **The specification says "validate the input"** | **every input is valid** |
| **The specification says "use constant-time arithmetic"** | **the arithmetic is naturally constant-time** |
| **The specification says "use a fresh random nonce"** | **the nonce is derived deterministically** |
| Parameters from unexplained seeds | **every constant has a stated derivation** |

> This is the same argument as Chapter 55 §55.2's commit timers and Chapter 53 §53.2's
> location-based labels: design so that the failure cannot occur, rather than documenting that
> it must not. It is a general engineering principle and Bernstein applied it to a field
> that badly needed it.

**He also litigated.** *Bernstein v. United States* established that source code is
protected speech, which ended the export restrictions that had kept strong cryptography out
of software — and Chapter 23 §23.4's account of why the early Internet had no encryption is
partly a consequence of those restrictions.

**Bernstein is contentious.** His public criticism of NIST's processes and of other
researchers' work is direct in a way that has made him enemies, and the substance of the
criticism has more often than not been correct.

**Taher Elgamal (b. 1955).** The Elgamal cryptosystem, and — the part that matters here —
SSL at Netscape.

Elgamal was Netscape's chief scientist and led the work that produced SSL, which became
TLS — SSL 3.0's design is generally credited to Paul Kocher with Netscape's team. Elgamal
is reasonably described as the person who put encryption on the web.

SSL's early versions were flawed — SSL 2.0 badly so — and the honest reading is that
they were built quickly, under commercial pressure, by people solving a problem nobody had
solved before. The corrections took twenty-five years and are described in §58.4.

> The counterfactual is worth considering: if SSL had waited for a correct design, e-commerce
> would have arrived later and encryption might have arrived with it. A flawed thing that
> shipped and was fixed beat a correct thing that did not exist — Chapter 22's argument, in
> a field where it is much less comfortable.

**Phil Zimmermann (b. 1954).** PGP, 1991 — and the criminal investigation.

Zimmermann released PGP as free software for human rights and privacy reasons, and it left
the United States, and he was investigated for three years for violating munitions export
controls. The investigation was dropped in 1996 without charges.

PGP's technical legacy is mixed — the web of trust never worked at scale, and the tooling
has been criticised for decades as unusable — and its historical significance is not in
doubt.

> Zimmermann's argument was that cryptography's availability is a political question rather
> than a technical one, and the crypto wars of the 1990s were about exactly that. They
> are being refought now, under the heading of lawful access to encrypted messaging, and
> **the technical arguments have not changed since 1993** — the Clipper chip's key escrow
> proposal and the current proposals have the same structure and the same objections.

Ralph Merkle, Ivan Damgård and the hash function tradition — and the length extension
property that HMAC exists to defeat.

The Merkle–Damgård construction builds a hash of arbitrary input from a fixed-size compression
function, and it is the structure of MD5, SHA-1 and SHA-2. Its known weakness — that the
digest is the internal state, so a hash can be extended — was understood early, and
SHA-3's sponge construction, and BLAKE2, deliberately avoid it.

## What this chapter's history establishes

**Two patterns worth carrying.**

Every serious failure in the chapter is an implementation or protocol failure, not an
algorithmic one. AES, RSA, ECDSA and SHA-2 are unbroken. What has failed is nonce reuse,
padding oracles, entropy, certificate validation, key storage and export-weakened parameters.
The mathematics is the part that works.

And the field's most important recent contributions are about making misuse difficult, not
about new mathematics. Bernstein's designs, AEAD replacing hand-composed constructions, TLS
1.3's removal of options, Ed25519's determinism — all of them remove a way to be wrong.

> Which is the same conclusion as Chapters 53, 55 and 56 reached from operational evidence:
> the leverage is not in stronger mechanisms but in mechanisms that fail less often when
> handled by ordinary people under pressure.
