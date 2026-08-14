# 22.3 Layers Four to Seven

Layers 1–3 describe reality accurately. Layers 4–7 are where the model and the world
diverge, and being honest about how far is more useful than pretending otherwise.

## Layer 4 — Transport

**What it does:** provide end-to-end communication **between processes**, with
whatever guarantees the application asked for.

**Unit:** the **segment** (TCP) or **datagram** (UDP).

This layer is real, is implemented, and is Unit VIII's subject. Its concerns:

| Concern | Chapter |
|---|---|
| Port numbers — identifying processes | 35 |
| Connection establishment and teardown | 37 |
| Reliability — acknowledgements and retransmission | 37 |
| Ordering | 37 |
| Flow control — protecting the receiver | 37 |
| Congestion control — protecting the network | 38 |
| Segmentation to fit the path MTU | 24, 37 |

**The two protocols:**

| | TCP | UDP |
|---|---|---|
| Connection | yes | no |
| Reliable | yes | no |
| Ordered | yes | no |
| Flow control | yes | no |
| Congestion control | yes | **no** |
| Header | 20 bytes min | **8 bytes** |
| Used by | web, mail, file transfer, SSH | DNS, DHCP, voice, video, games |

**The distinction that matters:** Layer 3 gets a packet to a **host**. Layer 4 gets it
to a **process on that host**. Without ports, a machine could run exactly one network
application.

Layer 4 is where the model still holds cleanly. Below it, unambiguous. Above it, the
trouble starts.

## Layer 5 — Session

**What it is supposed to do:** establish, manage and terminate dialogues between
applications — dialogue control (who may transmit), synchronisation points for
recovery, and session re-establishment after a failure.

**What actually implements it:** almost nothing, as a separate layer.

The functions are real and they are done — inside applications, or by libraries, or by
TCP:

| Session function | Where it actually lives |
|---|---|
| Session establishment | TCP's connection (Layer 4), or application logic |
| Dialogue control | application protocol |
| Checkpointing and recovery | application, e.g. HTTP range requests |
| Session identity | HTTP cookies, session tokens — Layer 7 |
| Named sessions | NetBIOS, RPC, SMB — the cited examples |

**The conventional exam answers** are NetBIOS, RPC, SQL sessions, PPTP and SMB — and
each of them is really an application protocol with session-like features.

The honest statement: **Layer 5 describes a real category of function that was never
separated into a real layer.** Chapter 23's model has no session layer at all, and
nothing is missing as a result.

## Layer 6 — Presentation

**What it is supposed to do:** translate between the application's data
representation and a common network representation. Character encoding, byte order,
data structure serialisation, compression, encryption.

**The historical problem was genuine.** In 1980, connecting an EBCDIC IBM mainframe to
an ASCII DEC minicomputer required character translation, and big-endian to
little-endian conversion was a real hazard. ASN.1 and BER were OSI's answer, and they
are still used — SNMP, LDAP, X.509 certificates, 3G/4G/5G signalling.

**Why it faded:** the world converged. ASCII, then Unicode. IEEE 754 for floating point.
Network byte order fixed as big-endian by convention. **The translation problem largely
solved itself by standardisation of the endpoints rather than by translation in the
middle**, which is a more general lesson than it looks.

**The conventional exam answers:** TLS/SSL, JPEG, GIF, MPEG, ASCII, EBCDIC, MIME.

**The awkwardness of TLS.** It is the standard exam answer for Layer 6, and it is
genuinely difficult to place:

- It sits above TCP and below HTTP, which is Layer 5/6 territory
- It performs encryption, which is a presentation-layer function
- It negotiates a session, which is session-layer
- It authenticates, which OSI did not really place anywhere
- **QUIC merges it into the transport entirely** (Chapter 38)

The exam answer is Layer 6. The accurate answer is that TLS spans 4–7 and the model has
no slot for it. **Answer 6 on the test and understand why the question is unanswerable.**

## Layer 7 — Application

**What it does:** provide network services to the user's program.

**A precise statement, because this is the most-confused layer:** Layer 7 is **not the
application**. Your web browser is not Layer 7. **HTTP** is Layer 7. The browser is a
program that uses a Layer 7 protocol.

The distinction matters: Layer 7 is the *protocol* that lets two applications
communicate, not the software with the buttons.

| Protocol | Purpose | Chapter |
|---|---|---|
| HTTP/HTTPS | web | 41 |
| DNS | name resolution | 39 |
| DHCP | address configuration | 40 |
| SMTP, IMAP, POP3 | mail | 41 |
| FTP, SFTP, TFTP | file transfer | 41 |
| SSH, Telnet | remote access | 41 |
| SNMP | management | 54 |
| NTP | time | 41 |
| LDAP | directory | 41 |
| SIP, RTP | voice and video | 41 |

## The honest assessment

**Layers 1–4: real.** Distinct implementations, distinct headers on the wire, distinct
devices, clean boundaries.

**Layers 5–7: one layer in practice.** An application performs its own session
management, its own encoding, and its own protocol, usually within a single program
using libraries. There is no boundary you can point to, no header that marks the
transition, and no device that operates at "layer 6".

Which is exactly what Chapter 23's four-layer model says: it has one **application
layer** covering OSI 5, 6 and 7, and it is the more accurate description of what runs.

```
        OSI                      TCP/IP
   ┌───────────────┐        ┌───────────────┐
   │ 7 Application │        │               │
   ├───────────────┤        │  Application  │
   │ 6 Presentation│  ────▶ │               │
   ├───────────────┤        │               │
   │ 5 Session     │        │               │
   ├───────────────┤        ├───────────────┤
   │ 4 Transport   │  ────▶ │  Transport    │
   ├───────────────┤        ├───────────────┤
   │ 3 Network     │  ────▶ │  Internet     │
   ├───────────────┤        ├───────────────┤
   │ 2 Data Link   │  ────▶ │               │
   ├───────────────┤        │  Link         │
   │ 1 Physical    │  ────▶ │               │
   └───────────────┘        └───────────────┘
```

## Why the upper layers still earn their keep

Two reasons, and they are practical rather than architectural.

**"Layer 7" is useful vocabulary.** A "Layer 7 firewall" or "Layer 7 load
balancer" means something precise and important: a device that reads and acts on
**application protocol content**, not merely addresses and ports. There is no better
short way to say that.

**"Layer 8" is the field's most-used joke and most-accurate diagnosis.** The user.
Extended informally to Layer 9 (organisational policy) and Layer 10 (money or
politics). The joke persists because a large proportion of reported network problems
are resolved at layer 8, and pretending otherwise helps nobody.

## What breaks here

**Insisting every protocol has exactly one layer.** TLS does not. QUIC does not. Say
so, then give the exam answer if an exam is asking.

**Confusing the application with the application layer.** The browser is a program;
HTTP is the protocol.

**Assuming Layer 5 and 6 exist as separate implementations.** They do not, and nothing
is lost.

**Dismissing Layer 7 as vague.** It is the least architecturally clean layer and the
most operationally useful term in the model.

> **Network+ note.** Objective 1.1 examines all seven. The exam wants: **TLS = 6**,
> **HTTP/DNS/DHCP/SMTP = 7**, **TCP/UDP = 4**, **NetBIOS/RPC/SMB = 5**. Give those
> answers. Know that layers 5 and 6 are not separately implemented, and that this is
> why the TCP/IP model collapses them. Objective 1.1 also expects **the PDU names** —
> bit, frame, packet, segment/datagram — which Chapter 23 §23.3 covers.
