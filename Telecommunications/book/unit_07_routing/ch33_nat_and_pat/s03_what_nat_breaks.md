# 33.3 What NAT Breaks

RFC 1631 predicted most of this in 1994. This section is the thirty-year record, and it is
worth reading as a case study in how far a single architectural compromise propagates.

## The root cause

**Everything below follows from one violated assumption:**

> A host knows its own address, and that address is what its peers see.

The Internet's original design guaranteed this. NAT removes it, and every protocol
written before 1994 that depended on it broke.

## Protocols that carry addresses in their payload

**The direct casualties**, and the ones that needed application-layer gateways.

### FTP

**The clearest example**, and it is worth working through because it shows exactly what
"embedding an address" costs.

FTP uses **two connections**: a control connection on port 21, and a separate data
connection for each transfer.

**Active mode** — the original design:

```
   Client → Server (control, port 21):
        PORT 192,168,1,10,20,100

   meaning "connect back to me at 192.168.1.10 port 5220"
                                   ↑              ↑
                        the client's own address, as ASCII text
                        inside the data stream
```

The server then opens a connection *back* to the client.

**Behind NAT this fails twice.** The address `192.168.1.10` is private and unroutable, so
the server cannot reach it — and even with the right address, the inbound connection has
no translation entry, so NAT drops it.

**Passive mode** was the workaround: the client sends `PASV`, the server replies with an
address and port, and **the client** opens the data connection outbound. Which works for a
client behind NAT, and fails for a **server** behind NAT, because now the *server's*
address is the embedded one.

The real fix is an application-layer gateway: the NAT device parses the FTP control
stream, finds the `PORT` and `227` responses, and rewrites the addresses inside them —
adjusting the TCP sequence numbers when the text length changes, because the rewritten
address may be a different number of characters.

A Layer 3 device parsing and editing Layer 7 text, and fixing up Layer 4 sequence
numbers to compensate. Chapter 21 §21.4's layer violation, in its most concrete form.

And FTPS — FTP over TLS — breaks the gateway entirely, because the control stream is
encrypted and cannot be parsed. Which is why SFTP (over SSH, one connection, no embedded
addresses) is the right answer and FTPS is not.

### SIP and RTP

Voice over IP has the same problem, at scale.

SIP negotiates a call and the media description — **SDP** — carries the IP address and
port where RTP audio should be sent. That address is the phone's private address.

The result is the classic VoIP symptom: the call sets up and there is no audio, or
audio in one direction only. Signalling works because it is a normal outbound connection;
media fails because it needs a peer-to-peer flow to an address that does not exist
externally.

The workarounds are an entire protocol family, and they exist solely because of NAT:

| Protocol | What it does |
|---|---|
| **STUN** (RFC 8489) | *"What is my public address and port?"* — a server tells you what it sees |
| **TURN** (RFC 8656) | **Relay everything through a server** when direct fails |
| **ICE** (RFC 8445) | Try every candidate path — direct, STUN-discovered, TURN relay — and use the first that works |

ICE is what WebRTC uses, and it is why a browser video call works at all. It gathers
candidate addresses, tries them in parallel, and settles on whatever succeeds.

**TURN's cost is worth stating.** A relayed call consumes the relay's bandwidth in both
directions for the whole call, and the media crosses a third party. A meaningful
fraction of calls — commonly 10–20% — require relaying, which is a permanent, ongoing
infrastructure cost imposed entirely by NAT.

### Others

**IPsec AH** is broken outright: it authenticates the IP header including the addresses,
and NAT changes them, so the authentication fails by design. **NAT-Traversal** (RFC 3948)
wraps ESP in UDP port 4500 to get around it.

**SNMP** traps, **H.323**, **NFS**, older peer-to-peer protocols, and various games all
embed addresses and all needed gateways or workarounds.

## Peer-to-peer, and the shape of the Internet

**The largest consequence, and the least visible.**

If both endpoints are behind NAT, neither can initiate to the other. Neither has an
address the other can reach.

**The techniques:**

**NAT hole punching.** Both hosts connect outward to a rendezvous server, which tells each
the other's public address and port. Both then send to each other simultaneously —
each outbound packet creates a translation entry, and the incoming packet from the other
side arrives just after and finds one waiting.

**It works, unreliably**, and its success depends on the NAT's behaviour:

| NAT type | Hole punching |
|---|---|
| **Full cone** | works easily |
| Restricted cone | usually works |
| Port-restricted cone | usually works |
| **Symmetric** | **usually fails** — a different external port per destination |

**Symmetric NAT** allocates a new external port for every distinct destination, so the
port the rendezvous server observed is not the port the peer will reach. This is common
in carrier-grade NAT (§33.4), which is why peer-to-peer works worse on mobile networks.

When hole punching fails, you relay — TURN, or a proprietary equivalent.

**And the architectural consequence:**

> **Nearly every successful "peer-to-peer" application is not peer-to-peer.** It routes
> through servers, because most hosts stopped being addressable.

Video calls, file sharing, messaging, multiplayer games — all maintain server
infrastructure whose primary purpose is to relay traffic between hosts that could, in the
original architecture, have talked directly.

This is a centralisation the architecture did not intend, and it has consequences well
outside networking: the relay operator sees the metadata, can be compelled to produce it,
can be blocked, and must be paid for. Chapter 23 §23.4's end-to-end argument describes a
network that NAT quietly stopped being.

## Diagnosis becomes harder

**Logs record the translated address.** An abuse report naming `203.0.113.5` at a given
time identifies a network, not a host.

Finding the host requires the NAT log for that moment — source address, source port,
destination, and timestamp — retained and searchable. Most small networks do not keep
it, and with CGNAT (§33.4) the problem becomes severe: thousands of subscribers share
one address, and identifying one requires the provider's logs including the **port range**,
which many did not log at all until regulators required it.

**Traceroute and MTU discovery are affected too.** ICMP messages generated for a
translated packet must themselves be translated — the embedded copy of the original header
contains the translated addresses, and NAT must rewrite *inside the ICMP payload* to make
the message meaningful to the original sender. Most implementations do this; some do
not, and the failure mode is a path MTU black hole (Chapter 24 §24.3) that appears only
across the NAT.

## The mitigations, and their character

Notice what every entry in this table has in common:

| Problem | Mitigation | Nature |
|---|---|---|
| Embedded addresses | **ALG** | a layer violation |
| VoIP media | **STUN/TURN/ICE** | a protocol family that exists only for this |
| Peer-to-peer | **hole punching, relays** | a workaround, plus infrastructure |
| IPsec | **NAT-T** | encapsulation to hide from the NAT |
| Inbound services | **port forwarding, UPnP** | manual configuration, or a security hole |
| Logging | **retain translation records** | storage and process |
| **All of it** | **IPv6** | **the actual fix** |

Every row but the last is a workaround, and collectively they represent an enormous
amount of engineering effort — thousands of person-years across the industry — spent
recovering a property the network used to have for free.

This is the real cost of NAT, and it is not the CPU cycles or the translation table.
It is that a generation of protocol designers had to assume their protocol would be
broken by the network, and design around it.

## What breaks here

FTP working in one mode and not the other. Active versus passive, and which side is
behind NAT.

**FTPS failing where FTP works.** The ALG cannot parse an encrypted control stream.

A VoIP call with signalling and no audio. SDP carries a private address. STUN/TURN.

**Audio in one direction only.** One side is behind a symmetric NAT.

Peer-to-peer working on some networks and not others. Symmetric NAT, commonly
carrier-grade.

**IPsec failing across NAT.** AH cannot work; enable NAT-T for ESP.

**Large transfers hanging across a NAT.** ICMP not being translated correctly — a path MTU
black hole confined to that path.

**Being unable to answer an abuse report.** No translation logs.

> **Network+ note.** Objective 2.2 covers NAT; objective 5.5 touches on the diagnostic
> consequences. Over-learn: NAT breaks protocols that embed addresses in their payload,
> FTP and SIP being the standard examples; an application-layer gateway rewrites them,
> and encryption defeats it; and STUN, TURN and ICE exist to work around NAT for
> real-time media. The FTP active-versus-passive distinction is examined.
