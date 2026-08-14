# Chapter 62 — Further Reading

## Read these first

Bellovin, S. (1989). "Security Problems in the TCP/IP Protocol Suite."
Recommended in Chapter 60 and it belongs here most of all. Read it and check the date on
each fix.

Spafford, E. (1989). "The Internet Worm Program: An Analysis." Purdue TR-823.
The founding document of incident analysis. Detailed, and it reads like a modern
post-mortem written thirty-six years early.

Marlinspike, M. (2009). "New Tricks for Defeating SSL in Practice." Black Hat DC.
**The sslstrip presentation.** The slides and video are public, and the argument about
security that depends on users noticing is the transferable part.

Kaminsky's 2008 DNS work — the Black Hat presentation and the contemporaneous accounts of
the coordinated disclosure.
Read the disclosure story as well as the technique; it is the model for how this is done and
a demonstration of how fragile the model is.

## Attack references

**MITRE ATT&CK** (attack.mitre.org).
The catalogue of what attackers actually do, by technique, with references to real
incidents. Consult it as a coverage check — for each technique relevant to your
environment, do you prevent it, detect it, or neither?

**OWASP Top Ten** and the **OWASP Testing Guide** — for the application row this chapter treats
briefly.

The CVE and CISA KEV databases (Chapter 55's reading) — KEV is the actionable one.

**Vendor threat research** — Unit 42, Talos, Mandiant, CrowdStrike annual reports.
Read them for the technical detail and the initial-access statistics, and discount the
conclusions.

**The Verizon DBIR** — for the distribution of what actually happens, which consistently
differs from what is discussed.

## Specific attacks and defences

RFC 2827 / BCP 38, and RFC 3704. Two pages of the actual argument, and F9 uses them.
**The Spoofer project** (caida.org) measures deployment.

**RFC 5961** — TCP robustness against blind in-window attacks. The mitigations for §62.2's
injection attacks, specified.

RFC 6797 — HSTS, RFC 9460 — HTTPS/SVCB records, and the **HSTS preload list**
(hstspreload.org). **F6 uses these.**

RFC 4033–4035 — DNSSEC, and Chapter 39's reading. **RFC 8945 (TSIG)** for the
zone-transfer and update authentication that is far easier to deploy than full DNSSEC.

**IEEE 802.11w** — protected management frames. The specification is behind a paywall; the
vendor documentation is adequate, and the point is simply to enable it.

RFC 4732 — "Internet Denial-of-Service Considerations."
A taxonomy, and it is better than most treatments of the subject.

CableLabs, NIST SP 800-61 (incident handling) and SP 800-83 — for the process around all
of this.

## Denial of service

The GitHub 1.35 Tb/s incident (2018) — GitHub's own engineering blog post is detailed and
honest, including the timeline and the scrubbing activation. F8 uses it.

The Dyn incident (2016) — Dyn's post-incident statement and the subsequent analyses.
The dependency-concentration lesson is the valuable one (Chapter 52 §52.4).

Antonakakis, M. et al. (2017). "Understanding the Mirai Botnet." USENIX Security.
The rigorous analysis: how it spread, how large it was, what it attacked, and how the variants
evolved. **Read section 4** for the credential list and what it says about device
manufacturing.

Cloudflare's and Akamai's quarterly DDoS reports — for current vectors and volumes,
which change. The figures in §62.3 will be out of date; the arithmetic will not.

Rossow, C. (2014). "Amplification Hell: Revisiting Network Protocols for DDoS Abuse." NDSS.
Where the amplification factor table comes from, measured rather than asserted.

## Hardening

**CIS Benchmarks** for your platforms — directly usable, and §62.4's checklist is a summary of
their network sections.

NSA/CISA "Network Infrastructure Security Guidance."
Free, short, and the management-plane sections are written by people who investigate the
consequences of ignoring them.

**DISA STIGs** for network devices — more prescriptive than most organisations need, and
useful as a completeness check.

**Vendor hardening guides** — Cisco's "Network Device Security Hardening" and equivalents.
The control-plane policing sections are the least read and among the most valuable.

## Economics and disclosure

Anderson, R. (2001). "Why Information Security is Hard — An Economic Perspective."
Recommended in Chapter 57 and it is this chapter's conclusion.

Geer, D. et al. (2003). "CyberInsecurity: The Cost of Monopoly."
**The monoculture argument**, and the circumstances of its publication are part of the lesson.

Geer's later essays and talks (available at geer.tinho.net) — on vulnerability economics,
disclosure and the limits of the field. Consistently the most honest writing available about
what security can and cannot achieve.

The literature on coordinated vulnerability disclosure — ISO/IEC 29147 and 30111, and
the debates around disclosure timelines. Google Project Zero's 90-day policy and the arguments
about it are the live version.

## Tools

Only against infrastructure you own or are explicitly authorised to test. This is not a
formality; unauthorised testing is a criminal offence in most jurisdictions.

**`macof`, `arpspoof`, `dnsspoof`** (from `dsniff`), **`yersinia`** — F1, F2 and F3.
Yersinia in particular demonstrates the Layer 2 attacks of §62.1 interactively, and watching
BPDU Guard fire is instructive.

**`bettercap`** — the modern equivalent, and better documented.

**`hping3`, `scapy`** — for constructing the packets in §62.2 and §62.3 yourself, which
teaches more than any tool with a menu.

**`sslstrip`, `mitmproxy`** — for §62.2's stripping demonstration, and for observing what
HSTS actually prevents.

**Suricata, Zeek** (Chapter 60's reading) — to see the attacks from the defender's side.

`dig`, `dnsviz.net`, `crt.sh`, `hstspreload.org`, `dnssec-analyzer` — **F6's toolkit**, and
all of them are free web services requiring nothing to be installed.

Shadowserver's free reports for your ASN, and the **Open Resolver Project** — **F5.**
They will tell you what of yours is reflectable, at no cost, and most organisations have
never looked.

**A lab.** containerlab, GNS3 or EVE-NG with virtual switches. Every attack in §62.1 can be
demonstrated safely on a laptop, and doing so once makes the mitigations memorable in a way
that reading does not.

## Where to look next

**Chapter 63** begins the troubleshooting unit, and the same layered method that structures
§57.4's attack surface structures the diagnosis; **Chapter 65** covers the failure modes
layer by layer, several of which are indistinguishable from the attacks here; and **Chapter 71**
covers where this is going — post-quantum, AI-assisted attack and defence, and the regulatory
response.
