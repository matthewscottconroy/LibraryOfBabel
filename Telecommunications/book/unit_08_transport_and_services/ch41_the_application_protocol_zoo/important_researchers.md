# Chapter 41 — The People

**Tim Berners-Lee (b. 1955).** HTTP, HTML and URLs, at CERN in 1989–91.

**The design's restraint is the achievement.** HTTP/0.9 was one line — `GET /path` — and the
system worked. **Compare with the contemporaneous alternatives**: Gopher was more
structured, WAIS was more capable, and both lost to something simpler that anyone could
implement in an afternoon.

**And the decision that mattered most was giving it away.** CERN placed the specification
and implementation in the public domain in 1993, **with no licensing and no royalties** —
which is why it spread and why the competing hypertext systems of the period did not.

**His later work on the Semantic Web has been less successful**, and it is worth noticing
why: it requires everyone to agree on structure, which is a coordination problem of exactly
the kind Chapter 28 describes.

**Roy Fielding (b. 1965).** Co-author of **HTTP/1.0 and 1.1**, and his 2000 dissertation
introduced **REST** as an architectural style.

**HTTP/1.1's persistent connections, caching semantics and the `Host` header are largely
his**, and the `Host` header is the one with the largest consequence — **before it, one
address served one website.**

**REST is widely misused** to mean "an HTTP API with JSON". **Fielding's actual argument is
about constraints** — statelessness, uniform interface, cacheability — and about *why* the
web scaled when other distributed systems did not. Worth reading in the original.

**Taher Elgamal (b. 1955), and the Netscape SSL team.** **SSL 2.0 (1995) and 3.0 (1996)**,
which became **TLS** at the IETF.

**Elgamal is sometimes called "the father of SSL"**, and the significant thing about the
work is its context: **Netscape needed commerce on the web to be possible**, and no
mechanism existed. **SSL was built quickly, for a commercial deadline, and it showed** —
SSL 2.0 had serious flaws and 3.0 was a substantial rewrite.

**Every version since has been a response to attacks on the previous one**, which is the
normal way cryptographic protocols mature and is not a criticism.

**Eric Rescorla.** **TLS 1.3** (RFC 8446), and much of the modern security architecture of
the web.

**TLS 1.3's design principle — remove options rather than add features — is his**, and it is
a genuine reversal of how protocols usually evolve. **Every negotiable parameter had proved
to be a downgrade opportunity**, and the response was to delete rather than to add
protections around them.

He also co-authored **DoH** (Chapter 39 §39.4) and much of WebRTC's security, and his
*SSL and TLS: Designing and Building Secure Systems* remains the standard reference.

**Tatu Ylönen (b. 1968).** **SSH**, 1995, written in response to a password-sniffing attack
on his own university network in Finland.

**It replaced Telnet, rsh, rlogin and FTP essentially completely within a decade**, which is
extraordinarily fast for a protocol transition — and the reason is that **it was a drop-in
replacement that solved an immediate, demonstrated problem.** Chapter 28's incentive
argument, inverted: **the benefit was immediate, local, and required nobody else's
cooperation.**

**The original was free; the company that followed was not**, and **OpenSSH** — from the
OpenBSD project, led by **Theo de Raadt** and **Markus Friedl** — is what actually shipped
everywhere.

**Jon Postel (1943–1998).** **SMTP** (RFC 821, 1982) and **Telnet** and **FTP** before it.

**SMTP's store-and-forward design was right for its era** — intermittently connected hosts,
relaying through intermediates — **and its complete absence of sender authentication is the
single most expensive omission in this book.** Every anti-spam mechanism since is a
retrofit onto a protocol that assumes the sender is honest.

**Marshall Rose (b. 1961).** **POP3** and **SNMP** — Chapter 22's notes cover his OSI work.

**SNMP's design brief was "simple"**, and it succeeded: an agent is small enough for a
1990 switch. **The security was left for later**, and later took until SNMPv3 in 2002 —
during which time **`public` and `private` became the most widely known passwords in
networking.**

**Mark Crispin (1956–2012).** **IMAP**, 1986, at Stanford.

**The insight was that mail should stay on the server** — which was contrarian when storage
was expensive and prescient once people had more than one device. **IMAP's complexity
relative to POP3 is the price of server-side state**, and it is what makes reading mail on a
phone and a laptop coherent.

**Meng Weng Wong, Mark Delany, and the DMARC authors.** **SPF**, **DKIM** and **DMARC** —
the retrofit.

**Each addresses a different part of the problem** and none is sufficient alone: SPF
authorises senders and breaks on forwarding; DKIM signs and survives it; **DMARC ties either
to the header the user actually sees**, which is the part that matters and which neither of
the others did.

**The deployment took fifteen years** and is still incomplete, and the reason is Chapter
28's: **a domain publishing SPF protects other people's users, not its own.**

**David L. Mills (1938–2024).** **NTP**, and Chapter 36's notes cover his design insight.

**What belongs here is the statistical machinery.** NTP does not simply ask a server the
time; it **polls several, measures each one's delay and dispersion, discards outliers, and
weights the rest** — because a clock that is confidently wrong is worse than no clock.

**Forty years of production use with essentially unchanged algorithms** is an unusual record
for anything involving statistics.

**Henning Schulzrinne (b. 1961).** **SIP** and **RTP** — his third appearance.

**SIP's deliberate resemblance to HTTP** was a design choice with a purpose: **make it
implementable by people who already knew how to write HTTP code**, and make it debuggable by
reading it.

**And the signalling/media separation** is the decision that shapes all VoIP
troubleshooting — it is what makes "the call connects and there is no audio" a coherent and
common diagnosis rather than a contradiction.

**Eric Allman (b. 1955).** **Sendmail**, and **syslog**.

**Syslog is the more universal contribution.** It was written as a logging facility for
sendmail and became **the logging mechanism for essentially every Unix system and network
device** — because it was simple, it was there, and nothing better was proposed until the
need for reliability and encryption produced RFC 5424 and its transports.

**Its facility/severity model has survived unchanged since the early 1980s**, and the
severity levels of §41.4 are his.

**The Let's Encrypt founders — Josh Aas, Eric Rescorla, Peter Eckersley (1979–2022), and
the ISRG.**

**The insight was that the barrier to HTTPS was operational, not technical.** Certificates
cost money and required manual work at renewal, so most sites did not bother.

**Removing both — free, and automated via ACME — moved HTTPS from about 30% of page loads to
over 95% in under a decade.** No new cryptography, no protocol change, no standards battle.

> **The largest security improvement to the web in twenty years came from removing friction
> rather than from inventing anything**, which is a lesson this book's other security
> chapters could stand to absorb.

**Peter Eckersley** also created HTTPS Everywhere and Certificate Transparency's monitoring
ecosystem, and his work on measuring the CA system is what made §41.1's trust-model critique
empirical rather than theoretical.
