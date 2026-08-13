# Chapter 58 — Further Reading

## Read these first

**Diffie, W. & Hellman, M. (1976). "New Directions in Cryptography."** *IEEE Transactions on
Information Theory*.
**Free, and the opening paragraph alone is worth it.** **Read section III** for the key exchange
in the authors' own framing.

**Ferguson, N., Schneier, B. & Kohno, T. — *Cryptography Engineering* (2010).**
**The book to read if you read one.** **It is explicitly about deploying cryptography rather
than designing it**, which is this chapter's purpose, and **it is honest about how things go
wrong.** The chapters on key management and on implementation issues are the ones that will
change your practice.

**Aumasson, J.-P. — *Serious Cryptography* (2nd ed., 2024).**
**More current and slightly more mathematical.** **Excellent on AEAD, on elliptic curves and on
the post-quantum transition.** **The "how things go wrong" boxes throughout are the best part.**

**Rescorla, E. — *SSL and TLS: Designing and Building Secure Systems*, and RFC 8446.**
**Rescorla edited TLS 1.3.** **RFC 8446's sections 1 and 2 are the readable overview**, and F7
uses them.

## Specifications

**RFC 8446 — TLS 1.3.** **RFC 8447** for the IANA registries. **Read §1.2 ("Major Differences
from TLS 1.2") first** — it is a page and it is the summary of §58.4.

**RFC 5116 — AEAD**, **RFC 5869 — HKDF**, **RFC 2104 — HMAC.**
**Short, and each explains why the construction is shaped as it is.** **RFC 2104's rationale
section on length extension is the clearest statement of §58.3's argument.**

**RFC 8032 — EdDSA (Ed25519)**, and **RFC 7748 — X25519/X448.**
**Read the security considerations** for the design-so-misuse-is-hard argument in the authors'
own words.

**RFC 5280 — X.509 certificates and CRL profile**, and **RFC 6960 — OCSP.**
**Consult, do not read.** **RFC 5280's name-matching and path-validation sections are what to
look up when a certificate error makes no sense.**

**RFC 6962 / RFC 9162 — Certificate Transparency.**

**RFC 8555 — ACME.**
**The automation protocol.** **Short, and understanding it makes D3 straightforward.**

**NIST FIPS 197 (AES), FIPS 180-4 (SHA-2), FIPS 202 (SHA-3), FIPS 203/204/205 (ML-KEM, ML-DSA,
SLH-DSA).**
**The post-quantum standards were finalised in 2024** and are the reference for §58.4's last
section.

**NIST SP 800-57 — key management recommendations.**
**The source of the key-size equivalence table in §58.2**, and the reference to cite when
someone asks how long a key should be.

## The mathematics, if you want it

**Katz, J. & Lindell, Y. — *Introduction to Modern Cryptography*.**
**The standard rigorous textbook.** **Provable security, definitions, reductions.** **Read it if
you want to know why the constructions are believed secure**, not if you want to deploy them.

**Boneh, D. & Shoup, V. — *A Graduate Course in Applied Cryptography*.**
**Free, in progress, and excellent.**

**Dan Boneh's Cryptography course** (Coursera/Stanford Online).
**Free, and the single best structured introduction available.** **Weeks 1–4 cover this
chapter's §58.1 and §58.3 properly.**

## Failures, which teach more than the constructions

**Bleichenbacher (1998), Vaudenay (2002), BEAST, CRIME, BREACH, Lucky 13, POODLE, FREAK,
Logjam, DROWN, ROBOT, Heartbleed.**
**Search any of them.** **Each has a clear write-up**, and **reading five of them teaches the
difference between a cipher and a protocol better than any exposition.**

**The Debian OpenSSL entropy bug (2008).**
**A two-line change to silence a static-analysis warning reduced the key space to 32,767
possibilities**, for two years, across every Debian-derived system. **F8 uses it.**

**The DigiNotar incident (2011)** — the Fox-IT investigation report is public and detailed.

**Sony's PlayStation 3 ECDSA failure (2010)**, and the **Android SecureRandom Bitcoin thefts
(2013)** — **§58.3's nonce argument, twice.**

**"Imperfect Forward Secrecy: How Diffie-Hellman Fails in Practice" (Adrian et al., 2015).**
**The Logjam paper.** **It quantifies what a nation-state could do with precomputation against
common 1024-bit DH groups**, and it is the reason those groups were retired.

## Tools

**`openssl`** — `s_client`, `x509`, `req`, `speed`, `ciphers`. **F2 and F6 use it.** **Learn
`openssl s_client -connect host:443 -showcerts` by heart**; it diagnoses most certificate
problems in one command.

**`testssl.sh`** and **Qualys SSL Labs**. **F6 uses one.** **Read every finding critically** —
both produce results that are technically accurate and operationally irrelevant, and telling the
difference is the skill.

**Wireshark**, for F3. **TLS 1.3 handshakes are largely encrypted**, which is itself the lesson;
**setting `SSLKEYLOGFILE` lets you decrypt your own.**

**`step-ca`** (smallstep) or **HashiCorp Vault's PKI engine** — **F5 uses one.** **Both make a
correct private CA achievable in an afternoon**, which `openssl` scripts do not.

**`certbot` / `lego` / `acme.sh`** — ACME clients. **D3 is mostly a matter of choosing one.**

**`crt.sh`** — certificate transparency search. **F4 uses it, and it takes thirty seconds.**

**`age` and `minisign`** — modern, deliberately minimal file encryption and signing tools.
**Worth using once to see what a design that removes options looks like.**

## Following the field

**The IACR ePrint archive** (eprint.iacr.org) — where cryptographic results appear first.
**Not for casual reading**, and useful for checking whether something you have been sold is
real.

**Real World Crypto** conference talks — **the applied end of the field**, and the talks are
public and consistently good.

**Bernstein's, Aumasson's and Filippo Valsorda's writing** — **the current practitioner-facing
commentary**, and Valsorda's newsletter in particular tracks what is actually changing in
deployed cryptography.

**NIST's post-quantum project pages** — **for the transition timeline**, which is the thing to
watch rather than the mathematics.

## Where to look next

**Chapter 59** uses this chapter's primitives for authentication; **Chapter 60 §60.3** covers
what TLS inspection costs and why; **Chapter 61** builds tunnels from §58.2's pattern and
compares IPsec's complexity with WireGuard's minimalism; and **Chapter 62** covers what happens
when the deployment is wrong.
