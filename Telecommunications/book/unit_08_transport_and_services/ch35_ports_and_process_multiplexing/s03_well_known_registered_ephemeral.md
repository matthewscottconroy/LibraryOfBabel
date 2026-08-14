# 35.3 Well-Known, Registered and Ephemeral

Sixty-five thousand numbers, divided into three ranges by convention. The division is
worth understanding as a piece of governance as much as a technical fact — it is one of
the clearest examples in this book of an unenforceable convention that everyone follows.

## The three ranges

| Range | Name | Assigned by | Character |
|---|---|---|---|
| **0 – 1023** | **Well-known** / System | **IANA**, strictly | the services everyone must agree on |
| **1024 – 49151** | **Registered** / User | IANA, on request | vendors and applications |
| **49152 – 65535** | **Dynamic / Ephemeral** | **nobody** | clients, temporarily |

The boundaries are RFC 6335's, and they were formalised in 2011 after decades of
informal use.

## Well-known — 0 to 1023

The services a client must be able to find without asking.

The one you should know cold:

| Port | Service | Protocol |
|---|---|---|
| **20 / 21** | **FTP** data / control | TCP |
| **22** | **SSH** (and SFTP, SCP) | TCP |
| **23** | Telnet | TCP |
| **25** | **SMTP** | TCP |
| **53** | **DNS** | **UDP and TCP** |
| **67 / 68** | **DHCP** server / client | UDP |
| **69** | TFTP | UDP |
| **80** | **HTTP** | TCP |
| **110** | POP3 | TCP |
| **123** | **NTP** | UDP |
| **143** | IMAP | TCP |
| **161 / 162** | **SNMP** / SNMP trap | UDP |
| **389** | LDAP | TCP |
| **443** | **HTTPS** | TCP (**and UDP for QUIC**) |
| **445** | SMB | TCP |
| **514** | **syslog** | UDP |
| **636** | LDAPS | TCP |
| **993 / 995** | IMAPS / POP3S | TCP |

**Appendix B has the full table.** These are the ones examined and the ones you will use.

**Two entries deserve a note.**

**Port 53 uses both UDP and TCP.** UDP for ordinary queries, TCP for responses too large
for a datagram and for zone transfers (Chapter 39 §39.2). Firewall rules must permit
both, and permitting only UDP is a classic error that breaks DNSSEC and large responses.

Port 443 now carries UDP as well, because **QUIC** runs there (Chapter 38 §38.4). A
firewall permitting only TCP/443 blocks HTTP/3, and the symptom is subtle: browsers fall
back to TCP and everything works slightly worse, with nobody noticing.

### The privilege rule

On Unix systems, binding a port below 1024 traditionally requires root.

**The reasoning was social rather than technical.** In a multi-user system of the 1980s,
any user could run a program; if any user could bind port 25, any user could impersonate
the mail server. Restricting the low range meant that a service on a well-known port had
been started by an administrator, which is a weak form of authentication and was better
than nothing.

**Its modern consequences are awkward:**

- Web servers must start as root to bind 80 and 443, then **drop privileges** — a
  well-established but delicate pattern
- Containers and unprivileged services need `CAP_NET_BIND_SERVICE`, or a redirect, or
  `sysctl net.ipv4.ip_unprivileged_port_start`
- The security value is now essentially nil on a single-purpose server where the only
  user is the administrator

It survives as convention and as a source of deployment friction, which is a fair
description of a great deal of Unix's inheritance.

## Registered — 1024 to 49151

Assigned by IANA on request, and the register is advisory rather than enforced.

| Port | Service |
|---|---|
| **1433** | Microsoft SQL Server |
| 1521 | Oracle |
| **3306** | **MySQL / MariaDB** |
| **3389** | **RDP** |
| 5060 / 5061 | **SIP** / SIP-TLS |
| **5432** | **PostgreSQL** |
| 5900 | VNC |
| 6379 | Redis |
| **8080** | HTTP alternate |
| 8443 | HTTPS alternate |
| 9090, 9100 | Prometheus, node exporter |
| 27017 | MongoDB |

**Registration means IANA recorded your claim.** It does not mean anyone must respect it,
and collisions in this range are common — several applications use 8080, and nothing
stops them.

The database ports are worth memorising for a security reason, not an operational one:
3306, 5432, 1433 and 6379 should essentially never be reachable from the Internet, and
finding one open is a finding. Shodan scans for exactly these.

## Ephemeral — the client side

Assigned by the operating system, temporarily, for outbound connections. Nobody
registers them and nobody needs to know them in advance.

And the ranges differ by platform, which matters more than it should:

| System | Range | Count |
|---|---|---|
| **Linux** (default) | **32768 – 60999** | ~28,000 |
| Windows (Vista+) | 49152 – 65535 | ~16,000 |
| macOS / BSD | 49152 – 65535 | ~16,000 |
| **RFC 6335 recommendation** | 49152 – 65535 | ~16,000 |

Linux's range starts well below the recommended boundary, and it overlaps the
registered range. Which means a Linux client's outbound connection may use a source port
that is somebody's registered service number — harmless, and occasionally confusing when
reading a firewall log.

```bash
# Check and change on Linux
sysctl net.ipv4.ip_local_port_range
sysctl -w net.ipv4.ip_local_port_range="10240 65535"
```

### Ephemeral exhaustion

The failure mode of this range, and it is a real one.

A machine making many rapid outbound connections — a load generator, a proxy, an API
gateway, a monitoring system — can run out of source ports.

**The arithmetic:** with ~28,000 ports and TCP's `TIME-WAIT` holding each for **60
seconds** after close (Chapter 37 §37.5):

$$\frac{28{,}000}{60} \approx 466 \text{ new connections per second, sustained}$$

Beyond that, connections fail — and the error is `EADDRNOTAVAIL`, "cannot assign
requested address", which does not obviously mean "out of ports".

**The remedies, in order of preference:**

| Fix | Effect |
|---|---|
| **Connection reuse / keep-alive** | **the real answer** — stop opening so many |
| Widen the range | 10240–65535 gives ~55,000, roughly doubling the rate |
| `net.ipv4.tcp_tw_reuse=1` | reuse `TIME-WAIT` sockets for outbound; safe |
| **More destination addresses** | the tuple includes the destination, so a second server address doubles the space |
| `tcp_tw_recycle` | **removed from Linux; was dangerous behind NAT** |

The fourth row is worth understanding, because it follows directly from §35.2: the
limit is not on *ports*, it is on **unique five-tuples**. Since the destination address and
port are part of the tuple, connecting to two different server addresses doubles the
available combinations with the same local port range.

And the first row is the actual answer. An application opening a new TCP connection
per request, at hundreds per second, is doing something HTTP keep-alive was designed to
avoid twenty-five years ago.

## The convention has no enforcement

Worth stating plainly, because it explains both a security posture and an attack.

**Nothing stops a program using any port.** SSH on 2222, HTTP on 8080, a database on 443 —
all work, and all are done.

**The consequences:**

Running SSH on a non-standard port is not security. It reduces automated scanning
noise, which is a genuine operational benefit for log volume, and a targeted attacker
finds it in seconds with a port scan. Security through obscurity, honestly labelled,
is fine; mistaking it for a control is not.

**Port-based firewall rules are weak.** A rule permitting TCP/443 outbound permits
**anything** that chooses to use 443 — which is why malware command-and-control channels
use it almost universally, and why next-generation firewalls inspect content rather than
trusting port numbers (Chapter 58 §58.3).

**Port-based service identification is a guess.** `nmap` reporting "443/tcp open https" is
inferring from convention. `nmap -sV` actually probes, and sometimes the answer is
different.

> The port number is a convention that almost everyone follows and nobody enforces.
> It is enormously useful and it is not evidence.

## Reading a port in context

**Given a port number, four questions:**

1. **Which range?** Well-known suggests a standard service; ephemeral suggests a client
   side of a connection.
2. **Source or destination?** In a captured packet, the destination port on the first
   packet of a connection names the service; the source port is the client's ephemeral
   choice. On the reply they swap.
3. **Which protocol?** TCP/53 and UDP/53 are different (§35.1).
4. Is it actually that service? Convention, not evidence.

Question 2 is the one people get wrong when reading captures. A packet from
`10.0.0.5:51234` to `203.0.113.10:443` is a client reaching a web server. The reply from
`203.0.113.10:443` to `10.0.0.5:51234` is the same conversation. The service is whichever
end has the low, stable, well-known number.

## What breaks here

**A DNS firewall rule permitting only UDP/53.** Large responses and DNSSEC fail; zone
transfers fail. Permit TCP too.

**HTTP/3 not working.** UDP/443 blocked. The fallback to TCP hides it.

**`EADDRNOTAVAIL` on a busy client.** Ephemeral exhaustion. Reuse connections first.

A service that will not start as an unprivileged user. The port is below 1024. Grant
the capability or use a higher port with a redirect.

**A database reachable from the Internet.** 3306, 5432, 1433 or 6379 should not be. This
is a finding.

**Trusting a port-based firewall rule.** Anything can use any port.

> **Network+ note.** Objective 1.4 expects the well-known ports, and this is one of the
> most directly examined items on the test. Memorise the table at the top of this
> section — 20/21, 22, 23, 25, 53, 67/68, 69, 80, 110, 123, 143, 161/162, 389, 443, 445,
> 514, 636, 993/995 — with their protocols. Also know **the three ranges and their
> boundaries**, and that **53 uses both TCP and UDP.**
