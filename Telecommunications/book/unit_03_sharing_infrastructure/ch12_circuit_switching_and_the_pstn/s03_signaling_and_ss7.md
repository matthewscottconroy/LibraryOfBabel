# 12.3 Signalling and SS7

The telephone network's control plane became a packet network in the 1970s —
twenty years before its data plane did. This section is about why, and about the
security assumption it embedded that is still being exploited.

## In-band signalling, and its consequences

Strowger's dial (§12.1) put the signalling on the same wires as the conversation.
That is **in-band signalling**, and it was the obvious design: there is one pair of
wires, so everything travels on it.

As the network grew, in-band signalling moved from pulses to **tones**. Between
exchanges, a set of multi-frequency tones conveyed supervision and digits, and one
tone in particular — **2,600 Hz** — indicated that a trunk was idle.

The design was economical and it had a structural flaw that took two decades to
become obvious: **the signalling was accessible to anyone who could reach the
audio path.** Which, on a telephone call, is both subscribers.

## Phreaking, and the security lesson

In the late 1950s a number of people independently discovered that whistling
2,600 Hz into a long-distance call caused the far exchange to believe the trunk had
been released, while the local exchange — which had already completed billing setup
— kept the connection. The caller could then send multi-frequency digits to route a
new call, free.

The **blue box** automated this, and the subculture that grew around it produced,
among others, Steve Jobs and Steve Wozniak, who built and sold them before founding
Apple. John Draper acquired the name **Captain Crunch** because a toy whistle in a
cereal packet produced 2,600 Hz cleanly.

The lesson generalises far beyond telephony and is worth stating in its general
form:

> **If control information travels on the same channel as user data, users can
> generate control information.**

That is the same class of vulnerability as SQL injection (user data interpreted as
control), as cross-site scripting, as format-string attacks, and as every buffer
overflow that overwrites a return address. The mixing of a control plane and a data
plane is a structural hazard, and separating them is a structural defence.

AT&T's response was **common channel signalling** — moving the control information
onto a physically separate network that subscribers cannot reach.

## SS7

**Signalling System No. 7**, developed from 1975 and standardised by the ITU-T,
carries call setup, routing and management on a **separate packet-switched network**
running in parallel with the circuit-switched voice network.

The architecture:

```
   ┌──────────┐              ┌──────────┐
   │   SSP    │══════════════│   SSP    │     ═══ voice circuits (TDM)
   │(exchange)│              │(exchange)│
   └────┬─────┘              └─────┬────┘
        │                          │
        └───── STP ─── STP ────────┘         ─── SS7 signalling (packets)
                │
              ┌─┴──┐
              │SCP │  (databases: routing, 800-number translation, roaming)
              └────┘
```

- **SSP** — Service Switching Point: an exchange that originates and terminates
  calls.
- **STP** — Signal Transfer Point: a packet router for signalling messages.
- **SCP** — Service Control Point: a database, queried during call setup for
  toll-free number translation, portability lookups, calling-card validation and
  roaming.

**What it bought:**

- **Security against phreaking**, immediately and completely. The control channel is
  unreachable from a subscriber line.
- **Much faster call setup.** Signalling travels at packet speed on a separate
  network rather than being clocked through each exchange in turn. Setup fell from
  seconds to a fraction of a second.
- **Trunks are not held during setup.** In-band signalling required the voice path
  to be built hop by hop before the far end even rang; SS7 establishes the path only
  after the callee answers, so a ringing-no-answer consumes no trunk capacity.
- **Services became possible.** Toll-free numbers, calling cards, number
  portability, caller ID and roaming all require a database lookup during setup, and
  SS7's SCP is what makes that possible. The entire "intelligent network" of the
  1980s and 1990s is built on it.

## The parallel with SIP

The architecture SS7 established is the one modern voice uses, and the resemblance
is not a coincidence.

| | PSTN | VoIP |
|---|---|---|
| Signalling | SS7 | **SIP** |
| Media | TDM circuits | **RTP** |
| Path | Different networks | Different paths, often different transports |
| Requirement | Reliable | Timely |
| Transport | Reliable, sequenced | **UDP**, because late is useless |

**SIP negotiates; RTP carries.** They take different paths, have different
requirements, and use different transports for exactly the reasons Chapter 38 §38.2
gives — signalling must arrive, media must arrive *on time*, and those are
different objectives.

Chapter 41 §41.4 develops it. The point here is that the engineers who designed SIP
in the 1990s were reimplementing a separation that SS7 had established twenty years
earlier, and they knew it.

## The security assumption, and what it costs now

SS7 was designed for a world of a few dozen national monopoly carriers, all of whom
knew each other, all of whom were licensed, and all of whom had regulators and
lawyers and reputations.

In that world, **authentication between carriers was unnecessary**. A message
arriving on the signalling network came from a peer carrier, by construction,
because nobody else could reach the network.

Deregulation, liberalisation and the growth of interconnection multiplied the
participants from dozens to thousands without changing the assumption. Access to
SS7 became purchasable — legitimately, by small carriers and aggregators; and
illegitimately, from insiders.

The consequences are live:

**SMS interception.** SS7 messages can request that a subscriber's messages be
delivered to a different location, as roaming requires. An attacker with SS7 access
can therefore intercept SMS — which defeats every SMS-based two-factor
authentication scheme in existence. Demonstrated publicly since 2014 and exploited
in documented bank account takeovers.

**Location tracking.** The network must know which cell a subscriber is in to route
calls; querying that is a normal operation.

**Call interception and denial of service** by similar means.

The mitigations — SS7 firewalls, home routing of SMS, and the SIGTRAN-era
authentication work — are partial and unevenly deployed. Diameter, the signalling
protocol for LTE and 5G, was designed with authentication, and its early deployments
had their own comparable flaws.

**The general lesson, which recurs throughout Unit XII:**

> A protocol's security depends on assumptions about who can participate, and those
> assumptions have a shelf life. ARP (1982), BGP (1989), DNS (1983), SMTP (1982)
> and SS7 (1975) were all designed for a small population of mutually trusting
> participants, and every one has required retrofitted defences that remain
> incompletely deployed.

Chapter 57's introduction makes this argument in general; SS7 is the oldest and
clearest instance.

## What breaks here

**A signalling network reachable by parties it was not designed for.** The whole of
the above.

**Control and data sharing a channel.** Phreaking's lesson, and it generalises to
every injection vulnerability in computing.

**An SS7 or Diameter interconnect without a signalling firewall.** Standard practice
now, absent in many networks, and the reason SMS remains a weak second factor
(Chapter 59 §59.1).

**Assuming SMS-based two-factor authentication is secure.** It is better than
nothing and it is defeated by both SIM swapping and SS7 interception, which is why
Chapter 59 ranks it below authenticator apps and far below hardware keys.

> **Network+ note.** SS7 is not examined directly. Two things from this section
> are: the **separation of signalling from media**, which reappears as SIP and RTP
> in objective 1.4; and the **weakness of SMS as an authentication factor**, which
> objective 4.1 expects. The historical mechanism is what makes both memorable.
