# Chapter 41 — Further Reading

## Primary sources

**RFC 9110 / 9111 / 9112 — *HTTP Semantics*, *HTTP Caching*, *HTTP/1.1* (2022).**
The current, reorganised HTTP specifications. **RFC 9110 is the one to read** — semantics
separated from any particular version, which makes clear what has and has not changed since
1997.

**RFC 8446 — Rescorla, E. (2018). *The Transport Layer Security (TLS) Protocol Version
1.3.***
**Read §1.2 (major differences from 1.2)** — the list of what was *removed* is the security
argument in one page.

**RFC 6962 — Laurie, B., Langley, A. & Kasper, E. (2013). *Certificate Transparency.***
Making fraudulent issuance public rather than impossible. A good example of a mechanism that
does not prevent an attack and makes it undeployable.

**RFC 4251–4254 — the SSH protocol architecture and its layers.**
**RFC 4251 §4 on trust relationships** states the TOFU model and its limitations explicitly.

**RFC 5321 (SMTP) and RFC 5322 (message format).**
**Two documents because the envelope and the message are different things** (§41.3) — which
is the clearest possible demonstration of the distinction.

**RFC 7208 (SPF), RFC 6376 (DKIM), RFC 7489 (DMARC).**
**Read DMARC's §3 on alignment** — it is the part that makes the other two useful, and it is
frequently skipped.

**RFC 5905 — Mills, D. et al. (2010). *Network Time Protocol Version 4.***
The algorithms, including the outlier rejection of §41.3.

**RFC 3261 (SIP) and RFC 3550 (RTP).**
SIP is long; **§4 gives the overview**. RFC 3550 is the better read and is Chapter 36's
reference too.

**RFC 5424 — Gerhards, R. (2009). *The Syslog Protocol.***
The structured format, superseding the informal one of RFC 3164. **RFC 5425** for TLS
transport and **RFC 6587** for TCP.

**RFC 3411–3418 — SNMPv3.**
The security model. Read the architecture document if you must configure it; otherwise know
that v3 exists and v2c does not authenticate.

## Books

**Grigorik, I. (2013). *High Performance Browser Networking.* O'Reilly.**
**Freely at hpbn.co, and it is the best book in this list.** HTTP, TLS, WebRTC and the
transport underneath, from the perspective of what actually makes a page load. **The
chapters on HTTP/1.1's limitations and HTTP/2's design explain §41.1 better than the RFCs
do.**

**Rescorla, E. (2001). *SSL and TLS: Designing and Building Secure Systems.*
Addison-Wesley.**
Dated on versions and unmatched on *why* the protocol is shaped as it is.

**Ristić, I. *Bulletproof TLS and PKI*, 2nd ed.**
**The practical companion.** Configuration, cipher selection, the attack history, and the
certificate ecosystem. Also runs **SSL Labs** (ssllabs.com/ssltest), which is the standard
way to assess a server's TLS configuration.

**Barrett, D., Silverman, R. & Byrnes, R. (2005). *SSH, The Secure Shell: The Definitive
Guide*, 2nd ed. O'Reilly.**
Everything SSH does beyond shells — the forwarding, agent and certificate material of
§41.2.

**Costales, B. et al. *sendmail*, 4th ed.**
Enormous, and the mail chapters are the standard reference for SMTP's operational reality
regardless of which server you run.

**Mauro, D. & Schmidt, K. (2005). *Essential SNMP*, 2nd ed. O'Reilly.**
MIBs, OIDs and what to actually monitor.

## Applied

**Do HTTP by hand** (exercise F1). It takes one minute and it makes the protocol concrete:

```bash
printf 'GET / HTTP/1.1\r\nHost: example.com\r\nConnection: close\r\n\r\n' | nc example.com 80
```

**Then the same over TLS:**

```bash
openssl s_client -connect example.com:443 -servername example.com
# then type the same request
```

**`openssl s_client` is the TLS diagnostic.** Certificate chain, expiry, protocol version,
cipher — all of it, in one command. **Learn it before you need it during a certificate
incident.**

**SSL Labs** (ssllabs.com/ssltest) for a full external assessment of any public server, and
**testssl.sh** for the same thing locally.

**Check your own certificates' expiry**, and **alert on it**:

```bash
echo | openssl s_client -connect host:443 2>/dev/null \
  | openssl x509 -noout -dates
```

**crt.sh** — search Certificate Transparency logs for your own domains. **You may find
certificates you did not know existed**, which is precisely what CT is for.

**Capture a SIP call** (exercise F3). Wireshark's **Telephony → VoIP Calls** menu will
reconstruct the signalling and let you **play the audio back**, which is a startling
demonstration of what RTP over UDP without encryption means.

**Mail authentication testing:** send a message to **check-auth@verifier.port25.com** or use
**mail-tester.com**, and both will report exactly what SPF, DKIM and DMARC did with it.
**Faster than reading the specifications**, and it tells you what receivers actually see.

**`chronyc sources -v` and `chronyc tracking`.** Look at your own machine's time sources and
the selection markers. **Then block one upstream and watch the selection change**
(exercise F6).

**Set up a syslog collector** — rsyslog or syslog-ng — and **flood it over UDP** to quantify
the loss (exercise F5). **Then repeat over TCP.** The numbers are the argument of
Chapter 36 §36.3.

**`snmpwalk -v2c -c public host`** against a device you own, and then **against the same
device with v3 configured.** The difference in setup effort is real and so is the difference
in what an attacker can do.

**Lab 30** in this book's [labs/](../../../labs/) directory works through this chapter's
protocols by hand — an HTTP request with `nc`, a TLS handshake examined with `s_client`, an
SMTP conversation typed manually, a SIP call captured and its media identified — and then
requires a migration plan for a deliberately insecure lab environment.

## For the certification-minded

**Objective 1.4 is ports and protocols, and this chapter carries most of the remaining
memorisation.**

| Protocol | Port | Protocol | Port |
|---|---|---|---|
| **HTTP** | 80 | **HTTPS** | 443 |
| **SSH / SFTP** | **22** | **Telnet** | 23 |
| **FTP** | **20/21** | **TFTP** | 69 |
| **SMTP** | **25** | SMTP submission | **587** / 465 |
| **POP3** | 110 | **POP3S** | 995 |
| **IMAP** | 143 | **IMAPS** | 993 |
| **LDAP** | 389 | **LDAPS** | 636 |
| **NTP** | **123** | **SNMP** | **161/162** |
| **SIP** | 5060/5061 | **syslog** | **514** |
| **RDP** | **3389** | VNC | 5900 |

Eight more things worth over-learning:

1. **Status code classes**: 2xx success, 3xx redirect, **4xx client error, 5xx server
   error**.
2. **TLS provides confidentiality, integrity and authentication.**
3. **SFTP is not FTPS** — SFTP is SSH; FTPS is FTP with TLS.
4. **SMTP sends; POP3 and IMAP retrieve.** IMAP leaves mail on the server.
5. **SPF, DKIM and DMARC are DNS TXT records.**
6. **SNMPv3 adds authentication and encryption**; v1/v2c use plaintext community strings.
7. **Syslog severities 0–7, with 0 most severe.**
8. **SIP signals; RTP carries media.**

And the four operational facts worth more than the memorisation:

**5xx means the fault is the server's; 502 and 504 mean it is further upstream still.**

**A fleet-wide certificate error is usually the clock, not the certificates.**

**"Nobody can log in" — check clock skew.** Kerberos's tolerance is five minutes.

**A call that connects with no audio means signalling worked and media did not** — and they
are different protocols on different ports, so look in the right place.
