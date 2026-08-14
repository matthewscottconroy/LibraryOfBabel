# 41.4 Voice, Video and Management

Two families that share a property: both were designed for a network the designers
controlled, and both had to be retrofitted for one they did not.

## Voice: the signalling/media split

**The architectural decision that shapes everything:**

| | Signalling | Media |
|---|---|---|
| **Protocol** | **SIP** | **RTP** |
| Port | 5060 / 5061 | dynamic, even-numbered UDP |
| Transport | UDP or TCP | **UDP** |
| Carries | who is calling whom, and how | **the audio** |
| Path | via servers | **directly between endpoints, where possible** |

> Signalling sets up the call; media flows separately, and usually by a different path.

This is why voice troubleshooting has a characteristic split: *the call connects and
there is no audio* means signalling worked and media did not — and those are different
protocols, different ports, and often different paths.

### SIP — 5060 and 5061

Deliberately modelled on HTTP (§41.1), and it shows:

```
INVITE sip:bob@example.com SIP/2.0
Via: SIP/2.0/UDP 10.0.0.5:5060;branch=z9hG4bK776
From: <sip:alice@example.org>;tag=1928301774
To: <sip:bob@example.com>
Call-ID: a84b4c76e66710
CSeq: 314159 INVITE
Contact: <sip:alice@10.0.0.5:5060>
Content-Type: application/sdp
Content-Length: 142

v=0
o=alice 2890844526 IN IP4 10.0.0.5
c=IN IP4 10.0.0.5                      ← where to send the audio
m=audio 49170 RTP/AVP 0 8 97           ← the port, and which codecs
```

Text, headers, methods, status codes — `INVITE`, `ACK`, `BYE`, `REGISTER`, `OPTIONS`,
and responses like `180 Ringing`, `200 OK`, `486 Busy Here`, borrowed directly from HTTP's
scheme.

**The SDP body is the important part.** It carries the address and port where audio should
be sent, and which codecs the endpoint supports — and the two sides negotiate a common
codec from their lists.

And `c=IN IP4 10.0.0.5` is a private address, which is Chapter 33 §33.3's problem in one
line: SIP embeds addresses in its payload, so NAT breaks it, and the whole STUN/TURN/ICE
family exists to work around that.

### RTP — the media

**RFC 3550** (Chapter 36 §36.3). UDP, with a small header carrying what the application
needs to reconstruct a stream:

| Field | Purpose |
|---|---|
| **Sequence number** | detect loss and reordering |
| **Timestamp** | **reconstruct the timing** — the sampling instant |
| Payload type | which codec |
| SSRC | which source, when several are mixed |

**RTCP** on the adjacent odd port carries loss, jitter and round-trip reports back to the
sender, which uses them to adapt the codec rate — congestion response implemented in the
application (Chapter 36 §36.4).

### The jitter buffer

The mechanism that makes voice work over a network that does not guarantee timing.

Packets arrive with variable delay (Chapter 3 §3.3's jitter). Audio must play at a
constant rate. So the receiver buffers:

```
   Arrivals:  ▓  ▓▓   ▓    ▓▓▓  ▓   ▓▓      ← irregular
                    │
              [ jitter buffer ]
                    │
   Playout:   ▓ ▓ ▓ ▓ ▓ ▓ ▓ ▓ ▓ ▓ ▓ ▓       ← regular
```

**The trade is direct and unavoidable:**

| Buffer | Effect |
|---|---|
| **Too small** | packets arriving late are **discarded** — audible gaps |
| **Too large** | **added latency**, and conversation becomes difficult |

Adaptive buffers adjust continuously, growing when jitter rises and shrinking when it
falls.

> **Jitter is worse than latency for voice.** A consistent 150 ms is workable; a mean of
> 80 ms that varies between 20 and 200 is not — because the buffer must be sized for the
> worst case, so **the jitter sets the latency.**

This is why Chapter 3 §3.3's `mdev` field matters and why the average alone is
misleading.

### The requirements

| Metric | Target | Beyond this |
|---|---|---|
| **One-way latency** | **< 150 ms** | 150–400 ms tolerable; **> 400 ms unusable** |
| **Jitter** | **< 30 ms** | buffers cannot compensate |
| **Loss** | **< 1%** | audible; **> 3% unintelligible** |

The 150 ms figure is ITU G.114, and it is about **conversational dynamics** rather than
audio quality — beyond it, people begin talking over each other because the turn-taking
cues arrive too late.

And bandwidth is dominated by headers (Chapter 21 §21.3): a G.729 codec produces
8 kb/s of audio and consumes about 39 kb/s on the wire. Capacity planning must use
the on-the-wire figure, and using the codec rate underestimates by a factor of five.

### Why voice is a QoS problem

Voice is low-bandwidth and utterly intolerant of delay, while bulk transfer is the
opposite. Without prioritisation, a single large download adds hundreds of milliseconds of
queueing delay (Chapter 13's bufferbloat) and voice becomes unusable.

The marking is DSCP EF (46) (Chapter 24 §24.2), and Chapter 52 covers what must be
configured for it to mean anything. The marking alone does nothing.

## Video

Two different problems, and conflating them causes bad decisions.

### Conferencing — interactive

The same constraints as voice, plus bandwidth. **RTP over UDP**, WebRTC in browsers,
and **latency matters** because it is a conversation.

**Bandwidth adapts** — the codec reduces its rate when RTCP reports loss, so a conference
degrades in quality rather than failing.

### Streaming — one-way

**Entirely different**, and it uses **TCP or QUIC**, not UDP.

Because there is no conversation, latency does not matter — a few seconds of buffer is
imperceptible — so reliability is worth more than timeliness, which reverses Chapter 36
§36.2's criterion.

**HLS and DASH** work by cutting the video into segments and fetching them over HTTP:

```
   video_1080p_0001.ts   video_720p_0001.ts   video_480p_0001.ts
   video_1080p_0002.ts   video_720p_0002.ts   video_480p_0002.ts
```

The client measures its own throughput and picks the next segment's quality — adaptive
bitrate, decided by the client, over ordinary HTTP.

> Streaming video is not a networking problem. It is HTTP file transfer with a clever
> client, which is why it works through every firewall and CDN without special handling.

## Management: SNMP

Simple Network Management Protocol — ports **161** (queries) and **162** (traps).

**The model:** an **agent** on each device exposes a tree of values; a **manager** polls it.

**The OID tree:**

```
   1.3.6.1.2.1.2.2.1.10.3
   │ │ │ │ │ │ │ │ │ │  └── interface index 3
   │ │ │ │ │ │ │ │ │  └───── ifInOctets
   │ │ │ │ │ │ │ │  └──────── ifEntry
   │ │ │ │ │ │ │ └─────────── ifTable
   │ │ │ │ │ │ └───────────── interfaces
   │ │ │ │ │ └─────────────── mib-2
   │ │ │ │ └───────────────── mgmt
   │ │ │ └─────────────────── internet
   │ │ └───────────────────── dod
   │ └─────────────────────── org
   └───────────────────────── iso
```

A MIB is the map from readable names to OIDs, and `ifInOctets.3` is far easier to
remember than the number.

**The operations:** `GET`, `GETNEXT`, `GETBULK`, `SET`, and `TRAP`/`INFORM`.

### The versions, and the security problem

| Version | Authentication | Encryption |
|---|---|---|
| **v1** | **a community string in clear text** | none |
| **v2c** | **the same** | none |
| **v3** | **users, with authentication** | **yes** |

"Community string" is a password sent in plaintext, and the defaults — `public` for
read, `private` for write — are still found on production equipment.

> SNMPv1/v2c with `public` is equivalent to no authentication at all, and an SNMP
> `SET` with a writable community can reconfigure the device.

**Use v3.** Where v2c is unavoidable — and it often is, on older equipment — restrict it
by source address, make it read-only, and never use a default string.

And SNMP is a UDP amplifier (Chapter 36 §36.4): a `GETBULK` produces a large response
from a small request, and an SNMP agent reachable from the Internet is a reflector.

### Traps and INFORMs

A `TRAP` is fire-and-forget — the device sends it and never knows whether it arrived.
Which fails precisely when it matters, because a device in trouble sends a trap onto a
network that may be congested.

`INFORM` (SNMPv2 onward) is acknowledged, and should be preferred for anything
important.

**And polling remains necessary.** Traps report events; polling establishes that the
device is alive, which a silent device cannot tell you.

## Management: syslog — 514

Chapter 36 §36.3 covered why UDP syslog loses messages. Here is the format.

```
<134>Mar 15 14:23:01 switch01 %LINK-3-UPDOWN: Interface Gi0/5, changed state to down
 └┬┘  └────┬────┘ └───┬──┘ └──────────────────┬─────────────────────────────────┘
 PRI    timestamp   hostname                message
```

**The PRI value encodes facility and severity:**

$$\text{PRI} = \text{facility} \times 8 + \text{severity}$$

134 = 16×8 + 6 = local0, informational.

The severities, which are examined and are worth knowing in order:

| Level | Name | Meaning |
|---|---|---|
| **0** | **Emergency** | system unusable |
| **1** | **Alert** | act immediately |
| **2** | **Critical** | critical condition |
| **3** | **Error** | error condition |
| **4** | **Warning** | warning |
| **5** | Notice | normal but significant |
| **6** | **Informational** | informational |
| **7** | **Debug** | debugging |

Lower is more severe, which is worth stating because the numbering feels inverted.

A device configured to log level 6 sends everything from 0 to 6 — and level 7 on a
busy device generates an overwhelming volume.

**The operational guidance:**

- **Centralise.** Logs on the device are lost when the device fails, which is exactly when
  you need them.
- Use TCP or TLS syslog (RFC 6587, RFC 5425) for anything security-relevant — UDP
  drops silently under load (Chapter 36 §36.3).
- **Synchronise clocks** (§41.3), or correlation across devices is impossible.
- **Set severity deliberately.** Level 6 for most, 7 only while debugging.
- Alert on the absence of logs, not only on their content — a device that stops logging
  has either failed or been compromised.

## NetFlow, sFlow, IPFIX

Different from SNMP: not counters, but *who talked to whom*.

| | Character |
|---|---|
| **NetFlow** | Cisco's; **exports flow records** — five-tuple, bytes, packets, timestamps |
| **IPFIX** | the IETF standardisation of NetFlow v9 |
| **sFlow** | **packet sampling** — a statistical subset, cheaper at high rates |

A flow record is one line per conversation, and the aggregate answers questions SNMP
cannot: *what is consuming this link?*, *who talked to that address?*, *when did this
traffic pattern start?*

And it is Chapter 64's most valuable troubleshooting input after packet capture —
because it is always on, whereas a capture must be started before the problem occurs.

## What breaks here

**A call that connects with no audio.** Signalling worked, media did not. NAT, a firewall,
or a routing asymmetry on the media path.

**Audio in one direction only.** One side's media path is blocked, or one side is behind a
symmetric NAT.

Voice quality degrading when someone downloads a file. No QoS. The marking alone does
nothing.

**Jitter within limits and audio still poor.** The average hides the peaks; look at the
distribution.

**SNMP working with `public`.** That is not authentication. It is also probably reachable
from more places than you think.

**A trap that was never received.** Fire-and-forget. Use INFORM, and poll as well.

**Logs missing for exactly the incident window.** UDP syslog dropped under load.

**Log timestamps that cannot be correlated.** Clock skew (§41.3).

> **Network+ note.** Objective 1.4 expects **SIP 5060/5061, SNMP 161/162, syslog 514**;
> objective 3.1 expects SNMP and syslog as monitoring tools. Over-learn: **SIP signals and
> RTP carries media**; SNMP v1/v2c use plaintext community strings and v3 adds
> authentication and encryption; **syslog severities 0–7 with 0 most severe**; and
> **NetFlow reports flows while SNMP reports counters.** The severity ordering and the
> SNMP version differences are examined directly.
