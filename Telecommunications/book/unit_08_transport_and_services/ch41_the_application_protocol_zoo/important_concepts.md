# Chapter 41 — Important Concepts

**HTTP is request and response** *(§41.1)* — Text, line-oriented, a blank line between
headers and body. **You can type it by hand with `nc`**, and doing so once is worth more
than reading about it.

**Safe versus idempotent** *(§41.1)* — Safe means no side effects; idempotent means
repeating changes nothing further. **POST is neither**, which is why **a proxy must not
retry it** and **QUIC's 0-RTT is unsafe for it** — a replayed POST can charge a card twice.

**4xx is the client's fault; 5xx is the server's** *(§41.1)* — **The distinction that
matters during an incident.** And **502/504 are proxy errors pointing past the thing you are
talking to**, which makes them characteristic of load-balanced services. **304 is caching
working**, and a high rate of it is good.

**The HTTP evolution** *(§41.1)* — 0.9 was one line. 1.0 added headers and **one request per
connection**, so **transport cost dominated content cost**. 1.1 added **persistent
connections** and **the mandatory `Host` header, which is what allows virtual hosting** and
let the web outgrow the address space. 2 added **multiplexing**, which removed
application-layer head-of-line blocking **and exposed TCP's**. 3 replaced the transport.

> **HTTP's methods and status codes have not changed since 1997. Everything else has been
> replaced twice.** The semantics were right; the transport was the problem.

**Pipelining and server push both failed** *(§41.1)* — Pipelining because a slow first
response still blocked the rest; server push because it pushed resources clients already
had. **Both sounded good and measurement retired them.**

**TLS provides three things** *(§41.1)* — Confidentiality, integrity, **and
authentication** — the last being the underestimated one. **Encryption without
authentication gives you a private conversation with an attacker.**

**TLS 1.3's design principle was to remove options** *(§41.1)* — RSA key exchange, CBC
ciphers, RC4, 3DES, MD5, SHA-1, compression and renegotiation all deleted. **Every
negotiable parameter is a downgrade opportunity**, and TLS's attack history — BEAST, CRIME,
POODLE, FREAK, Logjam — is largely a history of forcing a connection back to a weak option.
**And 1.3 is one round trip rather than two, with the certificate encrypted.**

**The CA trust model is a logical OR** *(§41.1)* — **Any of ~150 trusted roots can issue for
any name**, so **the security of every site is the security of the weakest CA your client
trusts.** DigiNotar (2011) demonstrated it. **Certificate Transparency** makes fraudulent
issuance **public rather than impossible**; **CAA** lets a domain name its permitted CAs;
**short lifetimes** limit exposure.

**Let's Encrypt changed the web** *(§41.1)* — Free certificates issued automatically via
ACME, proving control with a **DNS or HTTP challenge**. **HTTPS went from ~30% to over 95%
of page loads in under a decade**, because the barrier was cost and effort rather than
belief.

**SNI is plaintext in TLS 1.2** *(§41.1)* — Which is what network filtering uses to identify
destinations, and what **Encrypted Client Hello** conceals.

**Certificate expiry is a scheduled outage nobody scheduled** *(§41.1)* — "Everything worked
yesterday and nothing changed." **Monitoring expiry is the cheapest possible check.**

**The plaintext era's protocols did not become insecure; the context did** *(§41.2)* —
Telnet, FTP, rsh and TFTP were reasonable for a few hundred machines administered by
colleagues. **Packet sniffers on shared Ethernet ended that**, publicly, by 1995.

**SSH's TOFU model** *(§41.2)* — You accept a host key once and any later change warns
loudly. **So it protects against a man-in-the-middle appearing later, not one present on
your first connection.** SSHFP records or SSH certificates close the gap.

**`PasswordAuthentication no` is the single highest-value line** *(§41.2)* — It eliminates
brute forcing entirely, and **automated attempts against port 22 on any public address are
continuous.** **Changing the port to 2222 reduces log noise substantially and provides no
security.**

**SSH carries more than shells** *(§41.2)* — scp, sftp, rsync, **local and reverse port
forwarding**, and a SOCKS proxy. **Forwarding turns SSH into a VPN for one service** — and
is a security consideration, because a user with shell access can forward any port.

**SFTP is not FTPS** *(§41.2)* — **SFTP is a subsystem of SSH on port 22** and shares no
code, design or ports with FTP. **FTPS is FTP with TLS added.** Prefer SFTP: one port, no
separate data connection, and **no application-layer gateway problem** — FTPS breaks ALGs
because the encrypted control channel cannot be rewritten.

**FTP's two modes** *(§41.2)* — **Active** has the server connect back, which fails behind
NAT and client firewalls. **Passive** has the client connect for data too, which works for
clients and requires the server to open an inbound port range.

**RDP should never be exposed** *(§41.2)* — **Among the most common initial access vectors
for ransomware**, with continuous internet-wide scanning of 3389. BlueKeep was wormable and
pre-authentication, and Microsoft patched out-of-support Windows for it.

**SMTP pushes; POP3 and IMAP pull** *(§41.3)* — Not alternatives to each other. **Every mail
system uses SMTP plus one of the other two.**

**The three SMTP ports are three jobs** *(§41.3)* — **25 server-to-server** and almost
universally blocked outbound by ISPs; **587 submission with authentication**; **465
submission with implicit TLS**. **A client should never use 25**, and **a server accepting
unauthenticated mail on 25 is an open relay** — found by spammers within hours.

**The envelope is not the headers** *(§41.3)* — `MAIL FROM`/`RCPT TO` route the message;
`From:`/`To:` are displayed text. **A message can be addressed to one person and display
another, and nothing in SMTP prevents it.** This is the basis of essentially all mail
spoofing, and it is why bounces go one place and replies another.

**SPF, DKIM, DMARC** *(§41.3)* — **SPF** lists permitted senders, is checked against the
**envelope**, and **breaks on forwarding**. **DKIM** signs the message and **survives
forwarding**. **DMARC** says what to do on failure **and adds alignment** — requiring the
validated domain to match the `From:` the user sees, which closes the envelope/header gap.
**Deploy `none` → reports → fix → `quarantine` → `reject`.**

**IMAP won because people have several devices** *(§41.3)* — POP3 downloads and deletes;
IMAP keeps mail on the server.

**LDAP is OSI's survivor** *(§41.3)* — X.500's data model without its protocol stack. **Its
dominant use is authentication** — a successful bind means the password was right — and
**Active Directory is LDAP plus Kerberos plus DNS**. **Port 389 without TLS sends the bind
password in clear text.**

**NTP's offset assumes a symmetric path** *(§41.3)* — When it is not, **the offset is wrong
by half the asymmetry and NTP cannot detect it.** This is why NTP achieves milliseconds and
**PTP, with hardware timestamping, achieves sub-microsecond.**

**NTP consults several servers deliberately** *(§41.3)* — It discards outliers, because **a
single server that is confidently wrong is worse than none.**

**Time failures are dramatic** *(§41.3)* — TLS certificates rejected or wrongly accepted;
**Kerberos fails entirely beyond a 5-minute skew**; DNSSEC signatures appear invalid; log
correlation becomes impossible; TOTP codes rejected. **"Nobody can log in" is a time problem
surprisingly often**, and it takes one command to check.

**Signalling and media are separate** *(§41.4)* — **SIP sets up the call; RTP carries the
audio**, on different ports, over different transports, and usually by a different path.
**Which is why "the call connects and there is no audio" is a distinct and common
diagnosis.**

**SIP is modelled on HTTP** *(§41.4)* — Text, headers, methods, status codes. **And its SDP
body embeds an IP address**, which is Chapter 33 §33.3's problem in one line and the reason
STUN, TURN and ICE exist.

**The jitter buffer trade** *(§41.4)* — Too small discards late packets (gaps); too large
adds latency. **Jitter is worse than latency for voice**, because the buffer must be sized
for the worst case — **so the jitter sets the latency.** A consistent 150 ms is workable; a
mean of 80 ms varying between 20 and 200 is not.

**The voice targets** *(§41.4)* — **< 150 ms one-way** (ITU G.114, about conversational
turn-taking rather than audio quality), **< 30 ms jitter**, **< 1% loss**.

**VoIP bandwidth is dominated by headers** *(§41.4)* — G.729's 8 kb/s codec consumes about
**39 kb/s on the wire** — **a factor of five.** G.711's 64 kb/s becomes ~95 kb/s. **Capacity
planning must use the on-the-wire figure.**

**Streaming is not a networking problem** *(§41.4)* — **HLS and DASH cut video into segments
fetched over HTTP**, with the client measuring its own throughput and choosing the next
segment's quality. **Reliability beats timeliness because there is no conversation**, which
reverses Chapter 36 §36.2's criterion.

**SNMP's community string is a plaintext password** *(§41.4)* — **v1 and v2c with `public`
is equivalent to no authentication**, and a writable community permits reconfiguration.
**Use v3**; where v2c is unavoidable, restrict by source, make it read-only, and never use a
default.

**A trap is fire-and-forget** *(§41.4)* — **Which fails precisely when it matters**, because
a device in trouble sends onto a network that may be congested. **Use INFORM, and poll as
well** — traps report events, polling establishes that the device is alive.

**Syslog severities run 0–7 with 0 most severe** *(§41.4)* — Emergency, Alert, Critical,
Error, Warning, Notice, Informational, Debug. **PRI = facility × 8 + severity.**
**Centralise** (logs on a failed device are lost when you need them), **use TCP or TLS**,
**synchronise clocks**, and **alert on the absence of logs** as well as their content.

**NetFlow answers what SNMP cannot** *(§41.4)* — **Who talked to whom**, rather than
counters. **Always on, unlike a capture that must be started before the problem** — which
makes it the most valuable troubleshooting input after packet capture.
