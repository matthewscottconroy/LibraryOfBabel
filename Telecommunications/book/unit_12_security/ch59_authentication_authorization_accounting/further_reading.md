# Chapter 59 — Further Reading

## Read these first

NIST SP 800-63B — "Digital Identity Guidelines: Authentication and Lifecycle Management."
Free, current, and it is the document that reversed the password guidance. Read sections 5
and 10. F6 uses it. The appendix on the reasoning behind the changes is the persuasive
part when you have to argue with an existing policy.

NIST SP 800-207 — "Zero Trust Architecture."
Short, vendor-neutral, and the only authoritative definition. **F7 uses it.** Read the
seven tenets and the deployment models, and use it as the yardstick against which product
claims are measured.

**Google's BeyondCorp papers** (research.google, 2014–2018).
**Six papers, free.** Read "BeyondCorp: The Access Proxy" and the migration paper. They
describe a project rather than an architecture, which is what makes them useful.

Needham, R. & Schroeder, M. (1978). "Using Encryption for Authentication in Large Networks of
Computers." *CACM*.
Then read Lowe's 1995 paper finding the flaw. The pair together is the best available
argument for formal verification, and it takes an hour.

## Specifications

**IEEE 802.1X-2020.** Purchasable; the architecture is well described in vendor deployment
guides. What to know is the state machine and the port states.

RFC 3748 — EAP, and the method RFCs: **RFC 5216 (EAP-TLS)**, **RFC 5281 (EAP-TTLS)**,
**draft/RFC for PEAP.**
RFC 5216's security considerations explain the server-validation requirement that §59.2 says
is universally ignored.

RFC 2865 / 2866 — RADIUS and RADIUS Accounting, RFC 6614 — RadSec, RFC 5176 — Change
of Authorization.
CoA is the under-used one: it lets the server change a live session's authorisation, which
is what makes posture-driven quarantine possible.

**RFC 8907 — TACACS+.**
Standardised only in 2020, decades after deployment. The security considerations section
is unusually frank about the protocol's weaknesses.

**RFC 4120 — Kerberos V5.** **Long.** The introduction and the ticket flow are what to read.

**OpenID Connect Core**, and RFC 6749 — OAuth 2.0, with RFC 6819 (threat model) and
RFC 9700 (current best practice).
Read the OAuth security BCP rather than the base specification — it is where twenty years of
implementation failures are recorded.

W3C WebAuthn Level 3, and the **FIDO2/CTAP** specifications.
The origin-binding section is the one that explains §59.1's phishing resistance.

## Books

**Anderson, R. — *Security Engineering* (3rd ed.).**
**Free online.** Chapter 2 (usability and psychology) and the chapters on access control and
on multilateral security are directly this chapter, and Anderson is better than anyone on why
these systems fail in organisations rather than in protocols.

Gilman, E. & Barth, D. — *Zero Trust Networks*.
The practical book, and honest about what the model does not solve.

Garbis, J. & Chapman, J. — *Zero Trust Security: An Enterprise Guide*.
**More implementation-oriented**, and useful for D5's sequencing.

Bosworth, S. et al. (eds.) — *Computer Security Handbook*.
A reference rather than a read. The authentication and access control chapters are thorough.

Hunt, T. — the *Have I Been Pwned* project and associated writing.
**Not a book**, and the Pwned Passwords API is the practical implementation of SP 800-63B's
breach-list requirement, free, and usable in an afternoon.

## Papers and analysis

Lowe, G. (1995). "An Attack on the Needham-Schroeder Public-Key Authentication Protocol."
**Three pages.** Read it after the 1978 paper.

Anderson, R. & Needham, R. (1995). "Programming Satan's Computer."
The mindset required for protocol design, and it is entertaining.

Bonneau, J., Herley, C., van Oorschot, P. & Stajano, F. (2012). "The Quest to Replace
Passwords." IEEE S&P.
The framework that evaluates authentication schemes across usability, deployability and
security — and the conclusion that nothing dominates passwords on all three, which is why
passwords persist. Essential reading before proposing a replacement for anything.

Herley, C. (2009). "So Long, and No Thanks for the Externalities."
Recommended in Chapter 57 and relevant here: why users rationally reject security advice.

Adams, A. & Sasse, M. A. (1999). "Users Are Not the Enemy." *CACM*.
The paper that started usable security. Twenty-five years old and still routinely
ignored.

The Verizon DBIR's credential-related sections, annually — for the current proportion of
breaches involving stolen credentials, which is the number that justifies MFA spending.

## Tools

**FreeRADIUS** — F1 and F2 use it. The single best way to learn 802.1X properly, and it
runs on a laptop. The `radtest` and `eapol_test` utilities are what make experimentation
possible.

**`hostapd`** in EAP server mode, and **`wpa_supplicant`** — a complete 802.1X test bench
without any network hardware.

**Wireshark** with the EAP and RADIUS dissectors — **F2.** Watching a PEAP exchange and seeing
exactly where the TLS tunnel starts is worth an hour.

`step-ca` or a small internal CA (Chapter 58's reading) — for EAP-TLS certificates in
F1.

**Keycloak** — a free identity provider implementing SAML, OIDC and OAuth 2.0. Standing one
up and federating a test application teaches more about SSO than any amount of reading.

**A FIDO2 key**, or a passkey on a phone — F3 costs about £25 and demonstrates the phishing
resistance directly.

`ssh-audit`, and a script over `authorized_keys` files — **F4.** The result is
consistently worse than expected.

Sysinternals, BloodHound and their equivalents — for mapping actual privilege paths in a
Windows environment. BloodHound in particular visualises the gap between intended and actual
authorisation, and it is the tool that makes §59.3's argument concrete to management.

## Following the field

The IETF's `emu`, `radext` and `oauth` working groups — where EAP, RADIUS and OAuth are
still being developed.

The FIDO Alliance's material on passkeys — **the current transition**, and the one most
likely to change what you deploy in the next three years.

Troy Hunt's, Bruce Schneier's and Kelsey Hightower's writing on authentication practice;
and the annual reports from the major identity providers, read for their data rather than
their conclusions.

## Where to look next

**Chapter 60** implements the segmentation that §59.4 says zero trust does not replace;
**Chapter 61** covers the VPN that zero trust is replacing and the tunnels underneath both;
**Chapter 62** covers what an attacker does with a correctly authenticated credential; and
**Chapter 51 §51.4** is where this chapter's zero trust argument meets WAN design.
