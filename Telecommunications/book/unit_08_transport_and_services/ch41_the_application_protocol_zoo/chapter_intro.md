# Chapter 41 — The Application Protocol Zoo

Now, and only now, the port table.

The conventional order is to hand students a list of forty port numbers in the first
week and require them to be memorised. This book has spent forty chapters refusing
to do that, for a reason that should by now be self-evident: **a number attached to
nothing is a number you will forget.** Port 53 learned as a fact about the world is
a fact you look up again in six months. Port 53 learned after you understand that DNS
is a small query/response exchange that needed process addressing, that chose UDP for
the reasons Chapter 38 §38.2 gave, and that falls back to TCP when responses exceed
the datagram limit — that is a number with somewhere to live.

So this chapter is a survey, organised by **what each protocol is for**, and the
ports are attached at the end of each discussion rather than at the beginning.

## The organising groups

§41.1 — **The web.** HTTP's request/response model and its statelessness; the
evolution from 1.0's connection-per-request through 1.1's persistent connections and
pipelining to HTTP/2's multiplexing and HTTP/3 over QUIC; and TLS, examined as the
layer it actually is — where it sits, what the handshake costs in round trips, what
SNI reveals, and why HTTPS is now the default rather than the exception. This section
also covers the parts of HTTP that operate a network rather than a website: status
code families, the headers that control caching, and what a proxy does to each.

§41.2 — **Remote access and file transfer.** Telnet, present only as a cautionary
tale and as a diagnostic tool for poking at text protocols; SSH properly — key
exchange, host key verification and the trust-on-first-use problem, public key
authentication, port forwarding and tunnels; and the file transfer family, where the
distinctions genuinely matter. FTP with its two connections and its NAT
incompatibility (Chapter 33 §33.3); FTPS versus SFTP, which are unrelated protocols
with confusingly similar names; TFTP, which is trivial and is how network devices
load firmware; and SMB, which is the file protocol most enterprises actually run and
whose historical security record is why port 445 should never cross a perimeter.

§41.3 — **Mail, directory, and time.** SMTP's store-and-forward model and why mail is
the last major protocol still designed around intermittent connectivity; the
retrieval split between POP3 and IMAP and what each assumes about the client; the
anti-abuse layer of SPF, DKIM and DMARC, which are all DNS `TXT` records and which
are the reason Chapter 39's record types matter here. LDAP and Active Directory as
the directory that most enterprise authentication rests on. And NTP — which deserves
more respect than it gets, because **clock skew breaks things in ways that do not
look like clock problems**: Kerberos tickets fail, TLS certificates appear invalid,
log correlation becomes impossible, and multi-factor codes are rejected.

§41.4 — **Voice, video, and management.** The signalling/media split that Chapter 12
§12.3 promised would reappear: SIP negotiates, RTP carries, and they take different
paths with different requirements — SIP wants reliability, RTP wants timeliness, and
Chapter 36 explains why they therefore use different transports. Then the management
protocols: SNMP's versions and why v1 and v2c community strings are indefensible on a
modern network, syslog's severity levels, and NetFlow — all of which Chapter 54
develops operationally.

## The ports, grouped by purpose

The full table is Appendix B. The core set, arranged as this chapter arranges them:

| Purpose | Protocol | Port | Transport |
|---|---|---|---|
| Naming | DNS | 53 | UDP, TCP for large |
| Addressing | DHCP | 67/68 | UDP |
| Web | HTTP / HTTPS | 80 / 443 | TCP (443 also UDP for QUIC) |
| Remote access | SSH / RDP | 22 / 3389 | TCP |
| File transfer | FTP / SFTP / TFTP / SMB | 20,21 / 22 / 69 / 445 | TCP, TCP, UDP, TCP |
| Mail | SMTP / submission / IMAPS / POP3S | 25 / 587 / 993 / 995 | TCP |
| Directory | LDAP / LDAPS | 389 / 636 | TCP |
| Time | NTP | 123 | UDP |
| Voice | SIP / SIP-TLS / RTP | 5060 / 5061 / dynamic | UDP/TCP, TCP, UDP |
| Management | SNMP / SNMP trap / syslog | 161 / 162 / 514 | UDP |

Note `22` appearing twice: SFTP is a subsystem of SSH, not a separate service, which
is exactly the kind of fact that makes sense once and is otherwise memorised
wrongly.

## The synthesis

§41.1 closes with the exercise this unit has been building toward, and it is the
capstone of the first two-thirds of the book: **trace a complete web page load,
packet by packet, from a cold start.**

```
  1. DHCP DORA                     → address, mask, gateway, resolver   (Ch 40)
  2. ARP for the gateway           → gateway's MAC address              (Ch 18)
  3. DNS query for example.com     → 93.184.216.34                      (Ch 39)
  4. TCP three-way handshake       → connection established             (Ch 37)
  5. TLS 1.3 handshake             → keys agreed, identity verified     (Ch 58)
  6. HTTP GET /                    → request sent                       (Ch 41)
  7. HTTP 200 + HTML               → response received
  8. Repeat 3–7 for each asset     → often to different hosts
```

Eight steps, six protocols, five chapters, and every one of them can be watched in
Wireshark in about ninety seconds. Counting the round trips in that sequence — and
noticing that steps 3, 4 and 5 each cost at least one before a single byte of content
moves — is the practical form of Chapter 3 §3.4's lesson, and it explains at a
glance why connection reuse, DNS caching, TLS session resumption and QUIC's 0-RTT
exist.

## By the end you will be able to

- Identify the transport, port and purpose of every protocol in the table above, and
  explain *why* each chose its transport.
- Explain what HTTP's statelessness means and how sessions are built despite it.
- Distinguish FTPS from SFTP and explain why FTP breaks through NAT.
- Explain how SPF, DKIM and DMARC use DNS and what each proves.
- Explain three distinct failures that clock skew causes and which look like
  something else.
- Trace a complete page load, name every protocol involved, and count the round
  trips.
