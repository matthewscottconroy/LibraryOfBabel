# 23.4 The End-to-End Argument

In 1984 Jerome Saltzer, David Reed and David Clark published a paper that answers the
question §21.1 left open. Layering tells you *to* decompose; it does not tell you
**where to put a function**. The end-to-end argument does.

It is the most important paper in network architecture, it is fourteen pages, and it is
freely available. Read it.

## The argument

Their statement, paraphrased:

> **A function can only be completely and correctly implemented with the knowledge and
> help of the application standing at the endpoints. Therefore providing that function
> as a feature of the communication system itself is not possible — and a version
> provided there may be useful as a performance enhancement.**

Two clauses. The first is a claim about correctness; the second is a concession about
performance. Both matter, and people who quote only the first get the argument wrong.

## The canonical example

**Reliable file transfer.** A file must move from disk A to disk B, intact.

**The proposal:** make the network reliable. Every link checks and retransmits, so
nothing is ever lost.

**Why it fails.** Enumerate what can corrupt the file:

| Failure | Caught by a reliable network? |
|---|---|
| A bit flips on the wire | ✓ |
| A router drops a packet | ✓ |
| A bit flips **in the sending host's memory** | ✗ |
| The **disk read** returns bad data | ✗ |
| The **sending application** has a bug | ✗ |
| The **receiving host's memory** corrupts it | ✗ |
| The **disk write** fails silently | ✗ |

A reliable network catches some failures. **The application must check anyway** —
compute a checksum of the file it read, compare against a checksum of the file
written — because the failures the network cannot see are exactly as fatal as the ones
it can.

**And once the application performs that check, the network's guarantee is redundant
for correctness.** The file is verified end to end regardless.

**Therefore:** the function belongs at the endpoints. Implementing it in the network
does not remove the endpoint's obligation, so it cannot be *necessary* — it can only be
an optimisation.

## The performance clause

Here is where the argument is most often misread. It does **not** say the network
should never help.

If a link loses 10% of packets, end-to-end retransmission means retransmitting a whole
file repeatedly. Link-level retransmission is enormously more efficient — it recovers
locally without involving the endpoints.

**802.11 does exactly this** (Chapter 44 §44.2), and it is right to. Radio is lossy;
recovering at the link is far cheaper than at the endpoint.

The test is:

> **Does this function eliminate the endpoint's obligation? If not, it is an
> optimisation, and it must justify its cost.**

Link-level retransmission does not eliminate TCP's need to retransmit. It just makes
TCP need to do it less often, and that is a legitimate reason to have it.

## What IP does not do, and why

Every omission from IP is an application of this argument:

| IP does not provide | Because |
|---|---|
| **Reliability** | endpoints must check anyway; and some applications do not want it |
| **Ordering** | endpoints must reorder anyway; and some do not care |
| **Duplicate suppression** | endpoints must detect duplicates anyway |
| **Congestion control** | the endpoint controls its own sending rate |
| **Encryption** | the endpoint must trust the network otherwise — see below |
| **Quality of service** | the application knows what it needs; the network does not |

**IP is thin by design, and every thing it does not do was argued about.**

## The security exception

The row worth dwelling on, because the argument's application to security was correct
in principle and catastrophic in practice.

**The end-to-end reasoning:** encryption must be end to end, because an endpoint cannot
trust any intermediate encryption — the intermediate could be compromised, and the
endpoint has no way to verify otherwise. Therefore encrypt at the endpoints, and the
network need not.

**Logically impeccable.** And what happened is that **the network did not encrypt, and
the endpoints did not either**, for about twenty-five years:

- Telnet sent passwords in plaintext.
- FTP sent credentials in plaintext.
- HTTP sent everything in plaintext.
- SMTP sent mail in plaintext.
- SNMPv1 sent its community string in plaintext.

Every one of those protocols could have been designed with encryption. None was, partly
because encryption was expensive in 1980, partly because it was export-restricted, and
partly because **the architecture said security was somebody else's layer, and nobody
was that layer.**

The correction took decades and is not finished: TLS (Chapter 58), HTTPS Everywhere,
Let's Encrypt in 2016 making certificates free, and the resulting shift from about 30%
of web traffic encrypted in 2014 to over 95% now.

**The end-to-end argument said where security belonged. It did not cause anyone to put
it there.** Chapter 57 §57.1 develops the point: an architectural principle that
assigns responsibility without assigning it to a *specific party* assigns it to nobody.

## The hourglass

The end-to-end argument produces a characteristic shape:

```
        ╲  HTTP  SMTP  DNS  SSH  RTP  BGP  DHCP  … ╱
         ╲   FTP  IMAP  NTP  SNMP  SIP  QUIC   ╱
          ╲      TCP    UDP    SCTP  DCCP    ╱
           ╲                              ╱
            ╲──────────  IP  ───────────╱      ← the narrow waist
           ╱                              ╲
          ╱   Ethernet   Wi-Fi   PPP      ╲
         ╱   DOCSIS  LTE  5G  DSL  fibre   ╲
        ╱  satellite  Bluetooth  Zigbee  …  ╲
```

**Many protocols above. Many technologies below. One protocol in the middle.**

The waist is narrow because of the end-to-end argument: IP does the minimum, so it
constrains as little as possible above and requires as little as possible below.

**What the narrow waist buys:**

- **Any application over any medium**, with no *m*×*n* problem (§21.1)
- **Independent innovation** above and below
- **A single thing to implement** to join the Internet

**What it costs** — and this is the underappreciated half:

- **IP is nearly impossible to change.** Everything depends on it. IPv6 was specified
  in 1998 and reached 50% of traffic to major providers around 2024. **Twenty-six
  years, and not finished.**
- **Anything IP does not do must be done everywhere else.** Security, mobility, quality
  of service, multicast — each is retrofitted separately into dozens of protocols.
- **The waist can be pinched.** Middleboxes that inspect above IP (§21.4) constrain
  what can pass, and the ossification that follows is why QUIC hides inside UDP.

> **The narrow waist is the Internet's greatest achievement and its permanent
> constraint.** Both statements are true, and neither is a criticism.

## Where the argument is weakest

Three places, and honest engagement with them is more useful than reverence.

**Trust.** The argument assumes endpoints can be trusted to behave. They cannot. An
endpoint that ignores congestion control harms everyone, and only the network can stop
it. This is why routers do policing, shaping and fair queueing — network functions that
exist because endpoints are not trustworthy.

**Performance at scale.** CDNs (Chapter 52 §52.4) are pure middle-of-the-network
optimisation, and they carry a majority of Internet traffic. The end-to-end argument
permits them as optimisations; it does not predict that the optimisation would become
the dominant architecture.

**Constrained devices.** A battery-powered sensor with 32 KB of memory cannot implement
a full end-to-end stack. Gateways that terminate protocols on behalf of such devices
violate the principle deliberately, and Chapter 46 covers the consequences.

Clark himself has written extensively on where the argument's assumptions no longer
hold — his later work on "tussle" in cyberspace treats the network as a place where
parties with conflicting interests negotiate, which is a very different framing from
1984's.

## What breaks here

**Quoting only the first clause.** The performance concession is half the argument, and
without it you cannot explain 802.11 retransmission or CDNs.

**Treating it as prohibiting network functions.** It prohibits network functions that
claim to *replace* endpoint functions. Optimisations are permitted and must justify
their cost.

**Assuming the endpoints will do their job.** They frequently do not, and that gap is
where most of Unit XII lives.

**Believing IP can be changed when needed.** It cannot, on any timescale shorter than
decades. Every design decision at that layer is effectively permanent.

> **Network+ note.** Not examined directly. Its consequences are examined constantly:
> **IP is connectionless and best-effort**, **TCP provides reliability and UDP does
> not**, and **the application chooses which transport it wants**. All three are the
> end-to-end argument in operational form, and understanding the reasoning makes them
> impossible to forget.
