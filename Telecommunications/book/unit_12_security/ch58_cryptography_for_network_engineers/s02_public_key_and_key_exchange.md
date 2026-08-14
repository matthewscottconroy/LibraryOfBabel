# 58.2 Public Key and Key Exchange

§58.1 ended with a problem: two parties who have never met cannot share a symmetric key over
a channel an adversary is watching. In 1976 that problem was solved, and this section is
how.

## Diffie–Hellman, worked

Whitfield Diffie and Martin Hellman, *New Directions in Cryptography*, 1976 — which opens
with "We stand today on the brink of a revolution in cryptography", a claim that turned out
to be understatement.

The mechanism, with small numbers so you can follow the arithmetic yourself.

**Public parameters, agreed in the open:**

$$p = 23 \quad\text{(a prime)}, \qquad g = 5 \quad\text{(a generator)}$$

| | **Alice** | **Bob** |
|---|---|---|
| **Chooses secretly** | $a = 6$ | $b = 15$ |
| **Computes** | $A = g^a \bmod p = 5^6 \bmod 23 = \mathbf{8}$ | $B = g^b \bmod p = 5^{15} \bmod 23 = \mathbf{19}$ |
| **Sends, in the clear** | **8** → | ← **19** |
| **Computes the shared secret** | $B^a \bmod p = 19^6 \bmod 23 = \mathbf{2}$ | $A^b \bmod p = 8^{15} \bmod 23 = \mathbf{2}$ |

**Both arrive at 2.** The adversary saw $p = 23$, $g = 5$, $A = 8$ and $B = 19$, and to
compute the secret must find $a$ from $5^a \bmod 23 = 8$.

**Why it works:**

$$(g^b)^a = g^{ab} = (g^a)^b \pmod p$$

**Exponentiation commutes. That is the whole trick.**

**Why the adversary cannot:** recovering $a$ from $g^a \bmod p$ is the discrete logarithm
problem, and for a 2048-bit or larger $p$ no efficient classical algorithm is known.
With $p = 23$ you can solve it by trying seven values; with a 3072-bit prime you cannot.

> Two parties who have never met, communicating entirely over a channel an adversary is
> recording in full, arrive at a shared secret the adversary cannot compute. Not obscure —
> the adversary sees every message — but genuinely cannot derive. It is one of the
> delightful results in applied mathematics, and it takes ten minutes to verify by
> hand.

## The critical limitation

> Diffie–Hellman gives you a shared secret with somebody. It does not tell you with whom.

An adversary who can intercept and modify — not merely observe — can run the exchange twice:

```
   Alice ◀──── DH ────▶ Mallory ◀──── DH ────▶ Bob
        shared key K1            shared key K2

   Alice believes she shares a key with Bob.
   Bob believes he shares a key with Alice.
   Mallory reads and modifies everything, re-encrypting in each direction.
```

Neither party can detect it from the exchange itself, because every message is
well-formed and the mathematics is correct.

So Diffie–Hellman must be authenticated, and that is what §58.3's signatures and §58.4's
certificates are for. The key exchange establishes secrecy; something else must establish
identity.

> Every real protocol — TLS, IPsec, SSH, WireGuard — combines an unauthenticated key exchange
> with an authentication mechanism, and understanding that they are two separate problems is
> the most clarifying idea in applied cryptography.

## Forward secrecy

The operational consequence that matters most, and it is why ephemeral exchange is
mandatory.

| | **Static key exchange** | **Ephemeral (DHE / ECDHE)** |
|---|---|---|
| The session key derives from | **the server's long-term private key** | **a fresh key pair, per session, discarded after** |
| **If the long-term key is later compromised** | **every recorded past session is decryptable** | **past sessions remain secure** |
| Cost | slightly cheaper | **a fresh exchange per session** |

> An adversary who records encrypted traffic today and obtains the server's private key in
> three years can decrypt all of it — unless the exchange was ephemeral.

And "records traffic now, decrypts later" is not hypothetical. It is the assumed model for
well-resourced adversaries, and it is the same argument as §58.4's post-quantum discussion:
data encrypted today may need to remain secret for decades, and the adversary's capabilities
are not fixed.

TLS 1.3 made forward secrecy mandatory by removing every non-ephemeral key exchange. This
is the clearest single improvement in the protocol's history, and it is why "does it support
TLS 1.3" is a better question than any cipher-suite audit.

## RSA, and why it is being retired

Rivest, Shamir and Adleman, 1977 — the first practical public key system, and it can both
encrypt and sign.

**The mechanism, in outline:**

| | |
|---|---|
| **Key generation** | choose two large primes $p, q$; $n = pq$; derive $e$ and $d$ |
| **Public key** | $(n, e)$ |
| **Private key** | $d$ |
| **Encrypt** | $c = m^e \bmod n$ |
| **Decrypt** | $m = c^d \bmod n$ |
| **Security rests on** | **the difficulty of factoring $n$ into $p$ and $q$** |

RSA's problem is not that it is broken. It is that it needs very large keys.

| Security level | **RSA key** | **Elliptic curve key** |
|---|---|---|
| ~80-bit | 1,024 | 160 |
| **~112-bit** | **2,048** | **224** |
| **~128-bit** | **3,072** | **256** |
| ~192-bit | 7,680 | 384 |
| **~256-bit** | **15,360** | **512** |

> **RSA-3072 and a 256-bit elliptic curve key offer comparable security.** The elliptic curve
> key is twelve times smaller and its operations are dramatically faster, which matters for
> handshake latency, for certificate size, and for constrained devices.

**And the growth is unfavourable.** RSA key sizes must grow faster than elliptic curve ones to
keep pace, because factoring algorithms improve faster than elliptic curve discrete log
algorithms do. This is why new deployments use ECDSA or Ed25519 and why RSA is a legacy
choice.

**RSA-1024 should be regarded as dead.** It is within reach of a well-resourced adversary,
and certificates using it are rejected by modern browsers.

## Elliptic curves

**Same idea, different group.**

Diffie–Hellman needs a mathematical structure in which exponentiation is easy and the
logarithm is hard. Integers modulo a prime is one such structure. Points on an elliptic
curve are another, and a better one — the discrete logarithm problem is harder there per bit
of key.

**The curves you will actually meet:**

| Curve | Notes |
|---|---|
| **P-256 (secp256r1)** | **the NIST curve; ubiquitous in TLS certificates** |
| P-384 | where a higher security level is required |
| **Curve25519 / X25519** | **Bernstein's curve; used for key exchange in TLS 1.3, SSH and WireGuard** |
| **Ed25519** | **the signature scheme on the same curve** |
| secp256k1 | Bitcoin's curve; not used in TLS |

Curve25519's argument is implementation safety, and it is the same argument as ChaCha20's
(§58.1):

> **The NIST curves are hard to implement without side channels.** Correct implementations
> require careful constant-time arithmetic and validation that received points are actually on
> the curve — and an implementation that skips the validation is vulnerable to an invalid
> curve attack that recovers the private key.

Curve25519 was designed so that these mistakes are difficult to make: every 32-byte string
is a valid input, the arithmetic is naturally constant-time, and there are no special cases.

**And there is a provenance argument.** The NIST curves' parameters were generated from
unexplained seed values, and after 2013 that became a live concern rather than a curiosity.
No weakness has been demonstrated in them, and the preference for Curve25519 is partly
about being able to explain where every constant came from.

## The universal pattern

Worth stating explicitly, because it explains the structure of every protocol in Chapters 59
through 61:

> Use asymmetric cryptography to establish a symmetric key, then use symmetric cryptography
> for the data.

```
   1. Ephemeral key exchange (ECDHE)        ── establishes a shared secret
   2. Authenticate it (signature/certificate) ── establishes who
   3. Derive symmetric keys (KDF)            ── from the shared secret
   4. Encrypt the data (AES-GCM/ChaCha20)    ── fast, bulk
```

TLS does this. IPsec does this. SSH does this. WireGuard does this. The differences
between them are almost entirely in steps 2 and 3.

**And step 3 deserves a note.** The raw output of a Diffie–Hellman exchange is not a key —
it is a number with structure, unsuitable for direct use. A key derivation function (HKDF is
standard) extracts uniform key material from it and derives several keys — one per direction,
one for encryption, one for authentication where required. Using the raw DH output as a key is
a real implementation error.

## What breaks here

**An unauthenticated key exchange.** You have secrecy with somebody, and no idea whom.
Authentication is a separate problem and must be solved separately.

A static RSA key exchange in a TLS configuration. **No forward secrecy.** Every recorded
session is decryptable if the key is ever compromised. **Disable non-ephemeral suites.**

**RSA-1024 in production.** **Within reach.** Replace.

**A 15,360-bit RSA key chosen "for strength".** Handshakes will be slow enough to notice, and
a 384-bit elliptic curve key gives more security faster.

**An invalid curve attack.** The implementation did not validate that a received point is on
the curve. Not your bug to fix, and it is a reason to prefer implementations and curves where
it cannot happen.

Raw Diffie–Hellman output used directly as a key. **Use a KDF.**

**Key exchange succeeding against an attacker's certificate.** The exchange worked perfectly
— the failure is in §58.4's validation, and it is where almost all real-world TLS failures are.

> **Network+ note.** Objective 4.1. Over-learn: asymmetric encryption uses a public and a
> private key; **what one encrypts the other decrypts**; key exchange lets two parties
> derive a shared secret over an insecure channel; PFS means compromise of a long-term key
> does not expose past sessions; and **RSA and elliptic curve are the two families.** The
> public/private key roles are examined constantly and are frequently confused —
> encrypt to the public key; sign with the private one.
