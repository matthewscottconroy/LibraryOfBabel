# Chapter 58 — Important Concepts

Do not implement cryptography *(intro)* — Cryptographic code that is subtly wrong produces
output indistinguishable from correct output. It encrypts, it decrypts, tests pass, there is no
error message — until someone who knows what they are doing reads your ciphertext in an
afternoon. Every other bug in this book announces itself eventually. A cryptographic mistake
looks exactly like success.

**The universal pattern** *(intro, §58.2)* — Use asymmetric cryptography to establish a
symmetric key, then use symmetric cryptography for the data. TLS, IPsec, SSH and WireGuard
all do this, and the differences between them are almost entirely in how they authenticate the
exchange and derive the keys.

AES-128 will not be brute-forced *(§58.1)* — $2^{128}$ keys at $10^{18}$ per second is about
$1.1 \times 10^{13}$ years, roughly 800 times the age of the universe. AES-256 exists for
Grover's algorithm and for compliance, not for classical strength — and systems fail at key
management, protocol design and implementation, never at the cipher.

ChaCha20's argument is implementation safety, not AES's weakness *(§58.1)* — AES's table
lookups are timing side channels in software. ChaCha20 uses only additions, rotations and XORs
and is **constant-time by construction**, which is why it is preferred where there is no
AES-NI.

ECB has no legitimate use *(§58.1)* — Identical plaintext blocks produce identical
ciphertext blocks, so structure survives encryption. Its presence is a finding.

CBC was retired for its surroundings, not its cipher *(§58.1)* — Padding oracles, the
MAC-then-encrypt ordering error, and a decade of implementation failures. TLS 1.3 removed it
entirely, which is the clearest available signal.

Use AEAD; do not compose encryption and authentication yourself *(§58.1)* — It also
authenticates associated data that is not encrypted — headers and sequence numbers — which
is exactly what a network protocol needs. The history of attempts to compose them by hand is
the history of protocol vulnerabilities.

Never reuse a nonce with AES-GCM *(§58.1)* — Repetition does not merely leak a
relationship; it permits recovery of the authentication key, after which arbitrary forgery
follows. It has happened in production TLS stacks, in hardware VPNs, and in VM snapshots that
restore a counter.

Symmetric key distribution does not scale *(§58.1)* — $n(n-1)/2$ keys: 499,500 for a
thousand parties, each to be generated, distributed, stored, rotated and revoked.
Chapter 11's and Chapter 51's full-mesh arithmetic, a third time.

Exponentiation commutes, and that is the whole trick *(§58.2)* — $(g^b)^a = g^{ab} =
(g^a)^b$. Two parties who have never met, over a channel an adversary records in full, arrive
at a shared secret the adversary cannot compute — not obscure, genuinely cannot derive,
because that is the discrete logarithm problem.

Diffie–Hellman gives you a shared secret with somebody, and does not tell you with whom
*(§58.2)* — A man in the middle runs the exchange twice, and every message is well-formed and
the mathematics is correct. Understanding that secrecy and identity are two separate
problems is the single most clarifying idea in applied cryptography.

Forward secrecy: an adversary who records today and obtains the key in three years reads
everything — unless the exchange was ephemeral *(§58.2)* — "Record now, decrypt later" is the
assumed model for well-resourced adversaries, and TLS 1.3 made ephemeral exchange
mandatory, which is the clearest single improvement in the protocol's history.

RSA is not broken; it needs very large keys *(§58.2)* — RSA-3072 ≈ a 256-bit elliptic
curve key, twelve times larger and far slower. And the gap grows, because factoring
improves faster than elliptic curve discrete log does. RSA-1024 should be regarded as dead.

Curve25519's argument is that the mistakes are hard to make *(§58.2)* — The NIST curves
require constant-time arithmetic and validation that received points lie on the curve; an
implementation that skips the validation leaks the private key. Curve25519 has no invalid
inputs and no special cases — the same argument as ChaCha20's.

Raw Diffie–Hellman output is not a key *(§58.2)* — Use a KDF (HKDF) to extract uniform
key material and derive several keys. Using the shared value directly is a real implementation
error.

A checksum detects accident; a hash detects accident; only a key detects intent *(§58.3)* —
Hash requires nothing, MAC requires a shared secret, signature requires a private key — and
they provide integrity, integrity-plus-origin, and integrity-plus-origin-plus-non-repudiation
respectively.

Collision resistance falls first, at $2^{n/2}$ *(§58.3)* — The birthday bound. MD5's
collisions take seconds; SHA-1's chosen-prefix collisions arrived in 2020 — and the
chosen-prefix version is the one that permits a forged certificate.

"MD5 is broken" is over-broad advice that gets ignored *(§58.3)* — Preimage resistance has
not fallen, so MD5 remains acceptable for non-adversarial integrity checking and is
unacceptable wherever an adversary influences the input. The precise statement is the
actionable one.

A hash's speed is a virtue everywhere except password storage *(§58.3)* — Argon2id,
scrypt or bcrypt. A salt defeats precomputed tables and does nothing about guessing speed;
both are required. 10 billion SHA-256/s exhausts the 8-character lowercase space in 21
seconds.

HMAC's nesting exists to defeat length extension *(§58.3)* — Merkle–Damgård hashes expose
their internal state as the digest, so $H(K \| m)$ can be extended without knowing $K$.
Never construct a MAC yourself from a hash function.

A MAC gives no non-repudiation, because both parties hold the same key *(§58.3)* — Either
could have produced it. **Sometimes a feature**: messaging systems use MACs deliberately so
that a leaked transcript proves nothing.

A signature is over the hash, which is why a collision breaks it *(§58.3)* — Two messages
with one digest share one valid signature.

ECDSA's $k$ must never repeat, and repetition yields the private key algebraically
*(§58.3)* — Sony's PlayStation 3, Android's Bitcoin wallets, and multiple embedded devices
with poor entropy at first boot. Ed25519 derives it deterministically, removing the
possibility. Prefer constructions that eliminate a must-never-repeat value over ones that
state the requirement.

A published hash beside a download proves nothing against a compromised website *(§58.3)* —
**They change both.** A signature with a separately distributed key is what helps, which is why
package managers verify signatures.

A certificate is a signed statement that a public key belongs to a name *(§58.4)* — Chains,
authorities, revocation and transparency are all machinery for deciding whether to believe it.

CN is obsolete; SAN is what is checked *(§58.4)* — A certificate with the right Common Name
and no matching Subject Alternative Name is rejected, and this is a recurring cause of "it
worked on the old system."

The commonest server misconfiguration is not sending the intermediate *(§58.4)* — Browsers
recover; `curl`, Java, mobile applications and IoT devices do not. The symptom is "it works
in my browser and fails everywhere else", diagnosed by `openssl s_client -showcerts`.

Revocation fails open, which defeats it *(§58.4)* — An attacker who can intercept the
connection can also block the OCSP query. Hard-fail was tried and abandoned because a CA
outage would break the web. The industry's actual answer is short lifetimes — five years to
398 days and falling — which makes automated renewal mandatory rather than convenient.

Certificate Transparency does not prevent misissuance; it makes it detectable *(§58.4)* —
DigiNotar issued a valid `*.google.com` certificate in 2011 and did not survive. And every
hostname you certify is public and permanent — internal names, staging environments, customer
subdomains — which is a reconnaissance source (Chapter 57 §57.4).

A private root certificate expires, and the plan for that day is not written down *(§58.4)*
— Every device trusting it stops trusting everything at once. Put it in Chapter 55 §55.3's
lifecycle register. And use a CA tool, not a directory of `openssl` commands in a wiki.

TLS 1.3: one round trip, encrypted certificate, mandatory forward secrecy, five suites
*(§58.4)* — The suite name no longer encodes four choices, which eliminated a large class of
downgrade attacks. SNI still leaks the hostname, which Encrypted Client Hello addresses.
And 0-RTT is replayable — idempotent requests only.

Post-quantum: RSA and elliptic curve break entirely; symmetric is merely halved *(§58.4)* —
No such machine exists and the transition has begun anyway, because of harvest-now-decrypt-
later. NIST standardised ML-KEM, ML-DSA and SLH-DSA in 2024, and hybrid key exchange is
already deployed. The first thing that will bite a network engineer is key and signature
size, which affects handshake size, MTU and constrained devices.
