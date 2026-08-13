# Chapter 58 — Exercises

## A. Recall

**A1.** State the chapter's opening warning and explain precisely why cryptographic bugs are
worse than other bugs.

**A2.** Name the four primitives and say what each provides.

**A3.** State the universal pattern that structures TLS, IPsec, SSH and WireGuard.

**A4.** Why does AES-256 exist, given that AES-128 cannot be brute-forced? Give the good reason
and the poor one.

**A5.** What is ChaCha20's argument against AES? State it precisely — it is not that AES is
weak.

**A6.** Why does ECB have no legitimate use in a network protocol?

**A7.** What does AEAD provide that encryption plus a separate MAC does not?

**A8.** State the AES-GCM nonce rule and the consequence of breaking it.

**A9.** Work through Diffie–Hellman's algebra: why do both parties arrive at the same value, and
what must an adversary compute?

**A10.** State the critical limitation of Diffie–Hellman in one sentence.

**A11.** Define forward secrecy and state the threat model it addresses.

**A12.** Give the approximate RSA key size equivalent to a 256-bit elliptic curve key, and say
why the gap grows.

**A13.** Distinguish a hash, a MAC and a signature by what each requires and what each provides.

**A14.** In what sense are MD5 and SHA-1 "broken", and in what sense are they not?

**A15.** Why does HMAC use a nested construction rather than $H(K \| m)$?

**A16.** Why does a MAC provide no non-repudiation?

**A17.** What is a certificate, in one sentence?

**A18.** Why do intermediate CAs exist?

**A19.** Why does certificate revocation largely not work, and what did the industry do instead?

**A20.** What does Certificate Transparency prevent, and what does it merely make detectable?

## B. Apply

**B1.** Perform a Diffie–Hellman exchange by hand with $p = 47$, $g = 5$, $a = 8$, $b = 19$.

(a) Compute $A$, $B$ and the shared secret from both sides, showing they agree.
(b) An adversary sees $p$, $g$, $A$ and $B$. Find $a$ by trial. How many attempts?
(c) State what changes when $p$ is 3072 bits.

**B2.** For each, state whether it is a hash, a MAC, a signature or AEAD, and whether it is the
right choice:

(a) SHA-256 of a firmware image, published on the vendor's download page
(b) HMAC-SHA256 over an API request with a shared client secret
(c) Ed25519 signature over a software package, verified against a key in the package manager
(d) MD5 of a file, used as a cache key
(e) SHA-256 of a user's password, salted, stored in a database
(f) AES-GCM over a VPN packet with the header as associated data

**B3.** A TLS configuration offers these suites. Assess each and give the resulting
configuration you would deploy:

```
   TLS_RSA_WITH_AES_128_CBC_SHA
   TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA256
   TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256
   TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256
   TLS_DHE_RSA_WITH_3DES_EDE_CBC_SHA
```

(a) Which lack forward secrecy?
(b) Which lack AEAD?
(c) Which should be removed outright, and why?
(d) Which would you order first, and on what basis?

**B4.** Compute the collision effort for a 128-bit, a 160-bit, a 256-bit and a 512-bit digest.
State which are feasible today and which are not, and say what the birthday bound is.

**B5.** An organisation stores passwords as `SHA256(salt || password)`.

(a) What does the salt achieve?
(b) What does it not achieve?
(c) An attacker obtains the database and has a GPU capable of 10 billion SHA-256 operations per
second. How many 8-character lowercase-alphabetic passwords can they test per second, and how
long to exhaust that space?
(d) Recompute for Argon2id configured to take 100 ms per attempt.

**B6.** Examine a certificate chain for a website of your choosing using `openssl s_client
-showcerts`.

(a) How many certificates are presented?
(b) What are the SANs on the end-entity certificate?
(c) What signature algorithm is used at each level?
(d) When does each expire, and what is the shortest?
(e) Is the chain complete, or is the client completing it?

**B7.** For each error, give the most likely cause and the diagnostic command:

(a) "unable to get local issuer certificate"
(b) "certificate has expired"
(c) "hostname mismatch"
(d) "self-signed certificate in certificate chain"
(e) "certificate signature failure"
(f) Works in Chrome, fails in `curl` and in a Java application

**B8.** Design the certificate lifecycle for an organisation with 60 public certificates and 400
internal ones.

(a) State the issuance mechanism for each class.
(b) State the renewal mechanism and cadence.
(c) State what is monitored and at what thresholds.
(d) State what happens if the automation fails.

## C. Analyse

**C1.** The chapter says cryptographic bugs look exactly like success. Analyse the consequences
for testing, for code review and for procurement. What can a network engineer actually verify
about a cryptographic product?

**C2.** Analyse the argument that ChaCha20 and Curve25519 are preferable because they are harder
to implement wrongly. Is "difficult to misuse" a legitimate design criterion, or a concession to
poor engineering? Find two other examples of the principle in this book.

**C3.** Three constructions in this chapter have a value that must never repeat — GCM's nonce,
ECDSA's $k$, CBC's IV. Analyse why this pattern is so dangerous, why it recurs, and what
distinguishes designs that eliminate the requirement from those that merely state it.

**C4.** Analyse forward secrecy against a specific adversary: one who records all traffic and
may compel disclosure of a key years later. What does ephemeral key exchange actually protect,
what does it not, and what else would the adversary need?

**C5.** Certificate revocation fails open, and hard-fail was tried and abandoned. Analyse the
trade-off, and argue for a position. Is "short lifetimes instead" a solution or an admission of
defeat?

**C6.** Certificate Transparency publishes every hostname you certify. Analyse this as a
security trade: what it gains, what it costs, and what an organisation should do about the
disclosure.

**C7.** Analyse the claim that "systems fail at key management, protocol design and
implementation — never at the cipher." Test it against three real incidents you can find, and
say whether it holds.

**C8.** TLS 1.3 reduced hundreds of cipher suites to five and separated the key exchange and
authentication from the suite name. Analyse what class of attack this eliminated, and derive a
general principle about negotiable options in protocols.

**C9.** Analyse the post-quantum transition as an operational problem rather than a mathematical
one. What breaks first, who must act, and what is the network engineer's part?

## D. Design

**D1.** Design the cryptographic configuration standard for an organisation: TLS versions and
suites, certificate key types and sizes, hash algorithms, SSH configuration, and the review
cadence. Justify each choice and state what you deliberately permit for compatibility and why.

**D2.** Design a private CA for a 3,000-device estate using 802.1X: hierarchy, key protection,
issuance authorisation, distribution of the root, renewal automation, revocation, and the plan
for the root's own expiry. State what you would use to run it and why not `openssl` scripts.

**D3.** An organisation has 200 certificates renewed manually by two people. Public certificate
lifetimes are about to fall to 47 days. Design the transition programme: what you would
automate first, what would remain manual, the monitoring, and the failure modes you would
rehearse.

**D4.** Design the key management for a fleet of 50,000 IoT devices with a 10-year field life
(Chapter 47). Address: initial key provisioning, identity, rotation, revocation of a
compromised device, the algorithms you would choose given the post-quantum horizon, and what you
would do differently from an enterprise design.

**D5.** Write the one-page briefing you would give a non-technical executive who has been told
the organisation needs "quantum-safe encryption." Cover what is true, what is not urgent, what
should be started now, and what it would cost to do nothing.

## E. Troubleshoot

**E1.** A service works in browsers and fails from a mobile application and from `curl`.
Diagnose and give the exact fix.

**E2.** After a certificate renewal, a mobile application stops working while everything else is
fine. Give the likely cause and the design lesson.

**E3.** A VPN using AES-GCM is found to be forgeable. Investigation shows two packets with the
same nonce. Explain the mechanism and the three ways this could have arisen.

**E4.** A signature verifies correctly and the message is malicious. Explain how this is possible
and what was actually wrong.

**E5.** An internal PKI stops working entirely one morning, affecting every service
simultaneously. Give the most likely cause.

**E6.** TLS handshakes to a particular server take two round trips rather than one. Explain and
state whether it is a fault.

**E7.** An API accepts a request that was sent legitimately an hour earlier and processes it
again. The HMAC verifies. Explain what the protocol lacks.

**E8.** A security scanner reports "weak cipher: TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA384."
Assess the finding honestly: is it weak, in what sense, and what would you do?

**E9.** A certificate for a domain you own appears in a CT log and was not requested by your
team. Describe your response in the first hour.

## F. Extend

**F1.** Implement Diffie–Hellman with small numbers in any language, and then implement the
man-in-the-middle attack against it. Write a paragraph on what the attack required that the
passive adversary did not have.

**F2.** Use `openssl` to: generate an RSA-2048 and an Ed25519 key pair; time 1,000 signature and
1,000 verification operations for each (`openssl speed`); and compare the key and signature
sizes. Report the ratios and relate them to §58.2's table.

**F3.** Capture a TLS 1.3 handshake in Wireshark and identify each message. Determine what is
visible to an observer and what is not. Then capture a TLS 1.2 handshake to the same or a
similar server and list what 1.3 hid that 1.2 did not.

**F4.** Search a certificate transparency log for your own or your organisation's domain. Record
every hostname found. Identify any you did not expect to be public and assess what an attacker
would learn.

**F5.** Set up a private CA with `step-ca` or Vault, issue a certificate, and configure a client
to trust the root. Then revoke the certificate and determine experimentally whether the client
notices.

**F6.** Test a public service with `testssl.sh` or the Qualys SSL Labs test. Read every finding,
determine which are genuine and which are noise, and write the remediation you would actually
perform.

**F7.** Read RFC 8446 (TLS 1.3) sections 2 and 4.1. Compare the handshake described there with
what you captured in F3, and identify one thing in the RFC you had not realised from the
capture.

**F8.** Investigate one published cryptographic failure in a deployed system — Sony's ECDSA
nonce, the Debian OpenSSL entropy bug, DigiNotar, Heartbleed, or another. Determine which of the
four primitives failed, whether the failure was in the algorithm or the implementation, and how
long it went undetected.
