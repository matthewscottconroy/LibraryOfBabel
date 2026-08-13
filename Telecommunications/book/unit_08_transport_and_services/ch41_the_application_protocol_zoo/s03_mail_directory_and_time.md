# 41.3 Mail, Directory and Time

Three infrastructure services that most people never think about and that a great deal
depends on.

## Mail — the three-protocol split

**The distinction that confuses everyone**, and it is worth getting right because it
determines which protocol you troubleshoot:

| | Protocol | Direction | Port |
|---|---|---|---|
| **Sending, and server-to-server** | **SMTP** | **push** | 25, 587, 465 |
| **Reading, downloading** | **POP3** | pull | 110, 995 |
| **Reading, server-resident** | **IMAP** | pull | 143, 993 |

> **SMTP pushes mail toward its destination. POP3 and IMAP pull it from where it landed.**
> They are not alternatives to each other; every mail system uses SMTP plus one of the
> other two.

### SMTP — port 25, 587, 465

**A conversation you can hold by hand:**

```
S: 220 mail.example.com ESMTP
C: EHLO client.example.org
S: 250-mail.example.com
   250-STARTTLS
   250-AUTH PLAIN LOGIN
   250 SIZE 35882577
C: STARTTLS
   ... TLS negotiated, everything below is encrypted ...
C: MAIL FROM:<alice@example.org>
S: 250 OK
C: RCPT TO:<bob@example.com>
S: 250 OK
C: DATA
S: 354 End with <CRLF>.<CRLF>
C: Subject: Hello
C:
C: Message text.
C: .
S: 250 OK queued as ABC123
```

**Text, line-oriented, and readable** — the same shape as HTTP (§41.1), from the same era.

**The three ports are three different jobs**, and confusing them causes real problems:

| Port | Purpose | Notes |
|---|---|---|
| **25** | **server to server** | **Almost universally blocked outbound by ISPs** to limit spam |
| **587** | **submission** — client to its own server | **Authentication required**, STARTTLS |
| **465** | submission, **implicit TLS** | Deprecated, then un-deprecated (RFC 8314); now recommended |

**A client should use 587 or 465, never 25.** **A mail server accepting unauthenticated mail
on 25 from anywhere is an open relay** — and an open relay is found by spammers within
hours and blacklisted within days.

### The envelope versus the headers

**The detail that explains most mail forensics.**

**`MAIL FROM` and `RCPT TO` are the *envelope*** — what SMTP uses to route the message.

**`From:` and `To:` in the message are *headers*** — text, displayed to the user, **and
entirely independent of the envelope.**

> **A message can be addressed to one person on the envelope and display another in the
> headers, and nothing in SMTP prevents it.** This is the mechanism behind essentially all
> mail spoofing.

**And it is why bounce messages go to the envelope sender** while replies go to the header
`From:` — they are different fields with different purposes.

### The anti-spoofing trio

**SMTP has no sender authentication**, for the same reason as everything else from that era.
**Three mechanisms were retrofitted**, all using DNS TXT records (Chapter 39 §39.3):

**SPF** — *which servers may send mail for this domain.*

```
example.com.  TXT  "v=spf1 ip4:203.0.113.0/24 include:_spf.google.com ~all"
```

**Checked against the envelope sender.** **Breaks on forwarding**, because a forwarding
server relays with the original envelope sender from an unlisted address.

**DKIM** — *a cryptographic signature over the message.*

```
sel._domainkey.example.com.  TXT  "v=DKIM1; k=rsa; p=MIGfMA0GCSq..."
```

**The sending server signs headers and body; the receiver verifies against the published
key.** **Survives forwarding**, because the signature travels with the message.

**DMARC** — *what to do when SPF and DKIM fail, and where to report.*

```
_dmarc.example.com.  TXT  "v=DMARC1; p=reject; rua=mailto:reports@example.com"
```

**And it adds the crucial piece: alignment.** DMARC requires that the domain SPF or DKIM
validated **matches the domain in the `From:` header the user sees** — which closes the
envelope/header gap above.

| Policy | Effect |
|---|---|
| `p=none` | monitor only — **the correct starting point** |
| `p=quarantine` | to spam |
| **`p=reject`** | **refuse delivery** |

**The deployment path is `none` → collect reports → fix what breaks → `quarantine` →
`reject`**, and going straight to `reject` reliably breaks a mailing list or a forwarding
arrangement somebody depended on.

### POP3 versus IMAP

| | POP3 (110/995) | IMAP (143/993) |
|---|---|---|
| Model | **download and delete** | **the server holds the mail** |
| Multiple devices | **poor** | **the whole point** |
| Folders | no | yes |
| Server storage | minimal | substantial |
| Offline | naturally | with caching |

**IMAP won because people have several devices**, and POP3 survives where a single client
downloads everything and server storage is expensive.

**Both should be used only over TLS** — 993 and 995, or STARTTLS on 143 and 110.

## LDAP — 389 and 636

**A directory: a hierarchical, read-optimised database of people, groups, computers and
their attributes.**

**It is OSI's survivor** (Chapter 22 §22.1) — a simplification of X.500 that kept the data
model and discarded the protocol stack.

```
   dc=example,dc=com
     ├── ou=People
     │     ├── uid=alice
     │     └── uid=bob
     ├── ou=Groups
     │     └── cn=engineering
     └── ou=Computers
```

**A Distinguished Name is the full path**, read from the leaf:
`uid=alice,ou=People,dc=example,dc=com`

**The operations:** `bind` (authenticate), `search`, `compare`, `add`, `modify`, `delete`.

**Its dominant use is authentication.** An application binds to the directory with the
user's credentials; **success means the password was right.** Which is why LDAP is the
authentication back-end for a very large fraction of enterprise software.

**Active Directory is LDAP plus Kerberos plus DNS**, and Chapter 39 §39.3's SRV records are
how a client finds a domain controller.

> **LDAP over 389 without TLS sends the bind password in clear text.** Use **LDAPS (636)**
> or **StartTLS on 389**, and check which your application is actually doing — a great many
> default to plaintext.

## NTP — 123

**Chapter 36 §36.3 explained why it uses UDP.** Here is what it does and why it matters more
than it appears to.

### The mechanism

**Four timestamps per exchange:**

```
   t1: client sends
   t2: server receives
   t3: server sends
   t4: client receives

   offset = ((t2 - t1) + (t3 - t4)) / 2
   delay  =  (t4 - t1) - (t3 - t2)
```

**The offset calculation assumes the path is symmetric** — that the outbound and return
delays are equal. **When they are not** — an asymmetric route (Chapter 32 §32.2), a
congested direction — **the offset is wrong by half the asymmetry**, and NTP has no way to
detect it.

**This is NTP's fundamental limitation**, and it is why NTP achieves milliseconds rather
than microseconds. **PTP (IEEE 1588)** achieves sub-microsecond by having the network
hardware timestamp packets and account for switch residence time — which requires switch
support and is why it is used in finance, broadcast and industrial control rather than
generally.

### Strata

```
   Stratum 0:  a reference clock — GPS, atomic, radio       (not on the network)
   Stratum 1:  a server directly attached to stratum 0
   Stratum 2:  synchronised to a stratum 1 server
   Stratum 3:  synchronised to a stratum 2 server
   ...
   Stratum 16: unsynchronised
```

**Each level adds error.** In practice **stratum 2 or 3 is entirely adequate** for anything
that is not a measurement instrument.

**And NTP consults several servers deliberately** — it discards outliers and averages the
rest, because **a single server that is confidently wrong is worse than no server.** Mills's
remark applies: *a man with one clock knows what time it is; a man with two is never sure.*

### Why time matters more than it looks

**A list worth having**, because "the clock is a few minutes out" sounds harmless:

| Depends on accurate time | Failure when wrong |
|---|---|
| **TLS certificates** | **valid certificates rejected**, or expired ones accepted |
| **Kerberos** | **authentication fails entirely** — the default tolerance is **5 minutes** |
| **DNSSEC** | signatures appear expired or not yet valid |
| **Log correlation** | events cannot be ordered across systems |
| Multi-factor tokens (TOTP) | codes rejected |
| Scheduled jobs | run at the wrong time, or twice, or not at all |
| Distributed databases | conflict resolution corrupted |
| Billing and audit | disputes that cannot be resolved |

**The Kerberos row is the one that produces the most dramatic failure:** **a domain member
whose clock drifts more than five minutes cannot authenticate at all**, and the error
message rarely mentions time.

> **"Nobody can log in" is a time problem surprisingly often**, and it is worth checking
> early because it takes one command.

### Configuring it

```bash
# chrony — the modern choice on Linux
chronyc sources -v
chronyc tracking

# systemd-timesyncd — simpler, adequate for clients
timedatectl status

# ntpd — traditional
ntpq -p
```

**`chronyc sources` output:**

```
MS Name/IP address     Stratum Poll Reach LastRx Last sample
^* ntp1.example.com          2   10   377    45   -1.2ms
^+ ntp2.example.com          2   10   377   123   +0.8ms
^- ntp3.example.com          3   10   377    89   +9.4ms
```

**`^*` is the selected source; `^+` is a usable alternative; `^-` is excluded** as an
outlier. **`Reach 377` is octal — eight consecutive successful polls**, which is what you
want to see.

**Design guidance:**

- **Internal servers synchronise from a small number of trusted internal sources**, which
  synchronise externally — a hierarchy, not everything reaching the Internet
- **Use at least three upstream sources**, so an outlier can be identified
- **`pool.ntp.org`** for general use; **vendor pools** (`time.google.com`,
  `time.cloudflare.com`) are also good and use leap smearing
- **Firewall NTP** — Chapter 36 §36.4's amplification, and **`monlist` was a 557×
  amplifier** until it was removed
- **Monitor clock offset** as a first-class metric

**And NTP has authentication** — symmetric keys, or **NTS (Network Time Security, RFC 8915)**
which is TLS-based and is the modern answer. **Largely undeployed**, and the risk is real:
**an attacker who can move your clock can make an expired certificate valid.**

## What breaks here

**Mail rejected as spam despite being legitimate.** SPF, DKIM or DMARC misconfigured — or
DMARC alignment failing.

**Mail failing after a forwarding arrangement was added.** SPF breaks on forwarding; DKIM
does not.

**A client unable to send mail.** Port 25 blocked by the ISP. Use 587.

**An open relay.** Authentication not required on 25. It will be abused within hours.

**LDAP authentication working and sending the password in clear.** Port 389 without
StartTLS.

**Nobody can log in to the domain.** Clock skew beyond Kerberos's 5-minute tolerance.

**Certificate errors on a whole fleet at once.** The clock, not the certificates.

**NTP appearing synchronised and being wrong.** Asymmetric path, or a single bad source
with nothing to compare against.

> **Network+ note.** Objective 1.4 expects these ports, and **they are examined**: **SMTP
> 25/587/465, POP3 110/995, IMAP 143/993, LDAP 389, LDAPS 636, NTP 123.** Over-learn: **SMTP
> sends, POP3 and IMAP retrieve**; **IMAP keeps mail on the server and POP3 downloads it**;
> **SPF, DKIM and DMARC are DNS TXT records**; and **NTP uses UDP 123 and stratum counts
> distance from the reference clock.**
