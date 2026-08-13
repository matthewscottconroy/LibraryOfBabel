# 36.4 Datagram Hazards

UDP's simplicity moves work to the application, and the work that gets skipped causes
problems that extend beyond the application that skipped it. This section is the
obligations that come with choosing UDP.

## Hazard 1 — Amplification

**The most consequential**, because the damage falls on a third party.

**The mechanism, in three steps:**

1. **UDP has no handshake**, so **the source address is never verified.** A server
   receiving a query has no way to know whether the claimed sender actually sent it.
2. **An attacker sends a small query with a spoofed source** — the victim's address.
3. **The server sends a large response to the victim.**

```
   Attacker ──── 60-byte query, src = victim ────▶ Server
                                                     │
   Victim   ◀──────── 4,000-byte response ───────────┘

                     amplification factor ≈ 67×
```

**The attacker's bandwidth is multiplied**, and — crucially — **the traffic arriving at the
victim comes from legitimate servers**, so it cannot simply be blocked by source.

**The measured amplification factors:**

| Protocol | Factor |
|---|---|
| **DNS** (with EDNS0) | **28–54×** |
| **NTP `monlist`** | **~557×** |
| **memcached** | **10,000–51,000×** |
| SSDP | 30× |
| CharGen | 358× |
| SNMP v2 | 6× |

**The memcached figure is not a typo.** In February 2018 it produced a **1.35 Tb/s** attack
on GitHub — at the time the largest ever recorded — from a protocol that should never have
been on the Internet at all.

**NTP `monlist`** — a diagnostic command returning the last 600 clients — drove a wave of
attacks in 2013–14 until the command was removed from implementations and operators patched.

### The obligations

**If you run a UDP service**, three things:

**Rate-limit responses.** DNS servers implement **Response Rate Limiting**; it is standard
and it should be on.

**Do not run open resolvers or open reflectors.** An open DNS resolver, an unauthenticated
NTP server, a memcached instance on a public address — each is an attack weapon pointed at
strangers.

**Prefer designs where the response is not much larger than the request.** Amplification is
proportional to the ratio.

**If you run a network**, one thing:

**Implement BCP 38** (Chapter 27 §27.2). **Source-address spoofing is what makes every
amplification attack possible**, and ingress filtering is what stops it.

> **BCP 38 is twenty-five years old, universally recommended, and incompletely deployed.**
> Its non-deployment is the enabling condition for this entire class of attack, and the
> economics are Chapter 32 §32.4's: the cost falls on one party and the benefit on
> everyone else.

## Hazard 2 — No congestion control

**The obligation people most often skip**, and it is a matter of network citizenship.

**TCP slows down when the network is congested** (Chapter 38 §38.2). **UDP does not**, and
nothing in the protocol makes it.

**So a UDP application that sends at a fixed rate regardless of conditions:**

- Does not reduce its rate when queues fill
- **Takes bandwidth from TCP flows that do reduce theirs**
- Can drive a link into sustained congestion collapse

**The unfairness is structural.** Put a TCP flow and an unresponsive UDP flow on a
congested link: TCP backs off on loss, UDP does not, **so UDP's share grows and TCP's
shrinks** until TCP has almost nothing. **The well-behaved flow is punished for behaving
well.**

**This is not hypothetical.** It is why Chapter 38's 1986 congestion collapse is a
recurring risk rather than a historical curiosity, and why RFC 8085 exists.

### RFC 8085 — the UDP usage guidelines

**BCP 145**, and if you write a UDP application you are obliged to read it. Its
requirements, in brief:

| Requirement | Detail |
|---|---|
| **Implement congestion control** | or restrict the sending rate to a low, fixed value |
| **Back off on loss** | reduce the rate when loss is detected |
| **Do not send faster than the path allows** | measure, do not assume |
| **Use a large enough retransmission timer** | at least 1 second initially |
| **Avoid fragmentation** | keep datagrams under the path MTU |

**The alternative is a circuit-breaker:** if loss exceeds a threshold, **stop sending
entirely** rather than continuing to contribute to the collapse.

**How real applications comply:**

**RTP/RTCP** — receivers report loss and jitter, and the sender **reduces the codec rate**.
Congestion response, implemented in the application, because the transport does not provide
it (§36.3).

**QUIC** — implements full congestion control, using the same algorithms as TCP (Chapter 38
§38.3). **QUIC is better-behaved than a naive TCP implementation**, not worse.

**WebRTC** — Google Congestion Control, which uses delay as well as loss to infer
congestion earlier.

> **"We used UDP so we do not have to deal with congestion control" is not a design
> decision. It is an externality imposed on everyone sharing the path.**

## Hazard 3 — Fragmentation

**Chapter 24 §24.3's problems, arriving through UDP's front door.**

**A UDP datagram larger than the path MTU is fragmented by IP** — and every problem of
fragmentation follows:

- **Losing one fragment loses the whole datagram**, so effective loss is multiplied by the
  fragment count
- **Only the first fragment has the UDP header**, so port-based firewalls cannot classify
  the rest
- **Many firewalls drop fragments entirely**
- Reassembly is state held on behalf of a possibly hostile sender

**And UDP applications hit this more than TCP ones**, for a specific reason:

> **TCP segments to fit the path automatically. UDP does not — the application chooses the
> datagram size, and if it chooses badly, IP fragments.**

**A TCP application that writes 8 KB gets it split into MTU-sized segments by the stack. A
UDP application that sends an 8 KB datagram gets six IP fragments**, with all of the above.

**The rule:**

> **Keep UDP datagrams under the path MTU.** In practice: **under 1,400 bytes** for
> anything crossing the Internet, and **1,200 bytes** if it must work everywhere including
> IPv6's 1,280 minimum with tunnel overhead.

**DNS learned this the hard way.** The original 512-byte limit was chosen so a response
would fit any path. EDNS0 raised it, DNSSEC responses exceeded even that, **and
fragmentation-related failures became common enough that RFC 9715 now recommends limiting
DNS responses to around 1,232 bytes** — a retreat back toward the original reasoning after
twenty years.

**QUIC mandates it:** a QUIC packet must fit in the path MTU, and QUIC does its own
discovery (Chapter 34 §34.4) rather than relying on ICMP.

## Hazard 4 — No delivery confirmation

**Obvious, and the consequences are subtle.**

**The sender does not know whether anything arrived.** Which means:

**Silent failure.** Syslog messages, SNMP traps and telemetry can be dropped at any point
with **no indication to anyone** (§36.3).

**And it fails worst under load**, which is when you need it most: queues fill, UDP is
dropped, **and the monitoring that would have told you is the thing being dropped.**

**The application must decide** whether it needs confirmation, and if so implement it —
which is §36.2's warning about reimplementing TCP badly.

## Hazard 5 — NAT and firewall timeouts

**Chapter 33 §33.2's problem, from the application's side.**

**UDP has no connection**, so no teardown, so **a NAT or stateful firewall must guess when
a flow has ended** — typically after 30 to 300 seconds of silence.

**When it guesses wrong**, the translation entry is removed and **inbound packets have
nowhere to go.** The application's peer keeps sending; nothing arrives; **and no error is
generated at either end.**

**Which is why long-lived UDP applications send keepalives** — a packet every 25 or 30
seconds whose only purpose is to keep a NAT entry alive:

| Application | Keepalive |
|---|---|
| WireGuard | `PersistentKeepalive = 25` |
| IPsec NAT-T | RFC 3948, every 20 s |
| SIP | OPTIONS or a null packet |
| QUIC | PING frames |

**A protocol carrying otherwise-pointless traffic because a middlebox might forget it
exists.** Chapter 21 §21.4's layer violation, paid for by every UDP application
permanently.

## Hazard 6 — Spoofing and reflection generally

**No handshake means no proof the sender is who it claims.**

**TCP's handshake provides weak authentication as a side effect:** an attacker who cannot
see the return traffic cannot complete the three-way handshake, because they cannot guess
the server's initial sequence number (Chapter 37 §37.1).

**UDP has no equivalent.** A single spoofed packet is accepted and acted upon.

**The consequences beyond amplification:**

- **DNS cache poisoning** — a forged response accepted before the real one (Chapter 39
  §39.4)
- **Forged syslog entries**
- **Forged SNMP sets**, if the community string is known or guessed
- **Spoofed NTP**, which can move a clock and thereby break certificate validation

**The mitigations are all application-level:** randomised query IDs and source ports (DNS),
authentication (SNMPv3, NTS for NTP), or **encryption with authentication** (DoT, DoH,
DTLS, QUIC).

## The obligations, summarised

**If you choose UDP, you take on:**

| Obligation | Because |
|---|---|
| **Congestion control, or a strict rate limit** | RFC 8085; otherwise you harm others |
| **Datagrams under the path MTU** | fragmentation is worse for UDP |
| **Retransmission, if you need it** | at your own granularity |
| **Ordering and duplicate detection, if you need them** | nothing else provides them |
| **Authentication** | there is no handshake to make spoofing hard |
| **Keepalives** | or NAT will forget you |
| **Rate limiting on any service you run** | or you become an amplifier |

**And the network operator takes on one:**

| **BCP 38 ingress filtering** | it is what makes amplification possible |

> **UDP is not simpler. It is simpler *in the protocol* and correspondingly harder *in the
> application*.** The work does not disappear; it moves, and it moves to somewhere with
> less scrutiny and fewer decades of accumulated fixes.

## What breaks here

**Your server used in an amplification attack.** It is an open reflector. Rate-limit it and
restrict access.

**A UDP application starving TCP on a shared link.** No congestion control. RFC 8085.

**A UDP datagram over ~1,400 bytes failing on some paths.** Fragmentation, and something
is dropping fragments. Reduce the size.

**A UDP flow dying after 30 seconds of silence.** NAT timeout. Send keepalives.

**Log messages missing exactly when the incident happened.** UDP syslog under load. Use
TCP.

**A DNS response that fails intermittently.** Fragmentation of a large response. Limit the
response size or use TCP.

> **Network+ note.** Objective 4.2 expects amplification and reflection attacks; objective
> 2.2 touches congestion. Over-learn: **UDP amplification works because the source address
> is not verified and the response is larger than the request**; **DNS, NTP and memcached
> are the classic amplifiers**; **BCP 38 ingress filtering is the network-side defence**;
> and **UDP has no congestion control, so the application must provide it.**
