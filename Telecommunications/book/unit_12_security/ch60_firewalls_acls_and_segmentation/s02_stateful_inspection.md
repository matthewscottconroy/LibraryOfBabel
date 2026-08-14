# 60.2 Stateful Inspection

**A stateful firewall remembers connections.** When an internal host initiates one, the
firewall records the five-tuple (Chapter 35 §35.3) in a connection table; return traffic
matching an existing entry is permitted automatically, and anything else is not.

Strictly better than §60.1's stateless filter, and it costs things worth understanding.

## The connection table

```
   ┌──────────────────────────────────────────────────────────────────────┐
   │ proto  src            dst              state        idle    bytes    │
   ├──────────────────────────────────────────────────────────────────────┤
   │ tcp    10.20.5.14:51234 → 203.0.113.10:443  ESTABLISHED  0:12   84K  │
   │ tcp    10.20.5.14:51235 → 203.0.113.10:443  SYN_SENT     0:01     0  │
   │ udp    10.20.5.90:53001 → 10.9.0.5:53       —            0:04   120  │
   │ icmp   10.20.5.14 → 8.8.8.8  echo id 4471   —            0:02    64  │
   └──────────────────────────────────────────────────────────────────────┘
```

**And the rule that follows:**

> A stateless filter needs a rule for each direction. A stateful one needs a rule for the
> direction in which the connection is initiated, and the return traffic is permitted by
> the table rather than by a rule.

Which is what makes a modern policy readable. `permit tcp inside any outside any eq 443`
is the whole rule — no companion rule for the return, and no window through which an
attacker can source traffic from port 443.

## Tracking a stateless protocol

TCP has a state machine; UDP and ICMP do not, and the firewall must invent one.

| Protocol | State inferred from | Entry removed |
|---|---|---|
| **TCP** | **the actual flags** — SYN, SYN-ACK, ACK, FIN, RST | **on FIN/RST, or on timeout** |
| **UDP** | **the first packet creates a pseudo-connection** | **on idle timeout only** |
| **ICMP** | **an echo request creates an entry keyed on the identifier** | on timeout |

The UDP timeout is the interesting one and it produces real problems.

> **A UDP "connection" has no end.** The firewall keeps the entry for an idle timeout —
> typically 30 to 180 seconds — and then removes it. A protocol whose peer sends
> unsolicited traffic after a long silence will find the entry gone.

Which is why VoIP, VPN keepalives and IoT protocols specify keepalive intervals shorter than
typical firewall timeouts — and why a device that sends nothing for five minutes and then
expects a response is a support case waiting to happen (Chapter 47's LPWAN devices, and any
long-polling application).

TCP has the same problem at longer scale. An idle TCP connection whose entry is timed out
produces the classic symptom: the session appears alive at both ends and no traffic passes,
until one side eventually times out. TCP keepalives, or an application-layer heartbeat, are
the remedy — and the firewall's idle timeout should be known and documented, because
applications are designed against it.

## What state costs

Three costs, and each is a failure mode.

### Memory

One entry per connection, typically **200–400 bytes**, so a million connections is a few
hundred megabytes — **finite, and exhaustible.**

And exhaustion is the attack (Chapter 57 §57.4):

| SYN rate | **Fills a 1,000,000-entry table in** |
|---|---|
| 10,000/s | **100 s** |
| 100,000/s | **10 s** |
| 1,000,000/s | **1 s** |

> A SYN flood is not an attack on the server. It is an attack on the state table, and the
> firewall protecting the server fails before the server does.

**The defences:** **SYN cookies** (Chapter 37 §37.2) or SYN proxying at the firewall — complete
the handshake on the client's behalf and only then create state; **per-source connection
limits**; and aggressive half-open timeouts. All three should be configured; none is
default on every platform.

### Asymmetric routing breaks it

The failure that appears only during a failover, which is to say during an incident.

```
   Outbound: host ──▶ Firewall A ──▶ Internet
   Return:   Internet ──▶ Firewall B ──▶ host

   Firewall B has no entry for this connection.
   It drops the packet, correctly, according to its policy.
```

> **A stateful firewall must see both directions of a connection.** If it does not, it drops
> traffic that is entirely legitimate, and the symptom is "some connections work and some do
> not", varying by flow.

**The remedies, in order of preference:** ensure symmetry by design (Chapter 56 §56.2's
active/standby, or a routing policy that keeps flows on one path); state synchronisation
between the firewall pair, which every enterprise firewall supports and which must be
configured and tested; or clustering, where the pair behaves as one device.

And it is the reason Chapter 51 §51.3's direct-connect failover breaks things: the failover
changes the path and the return traffic arrives somewhere with no state.

### It is a stateful device in a stateless network

**A philosophical cost with practical consequences.** IP was designed so that any packet can
take any path (Chapter 24 §24.1). A stateful firewall requires that a flow's packets take
one path — which reintroduces exactly the constraint that packet switching removed, and
which is why firewalls, load balancers and NAT devices all impose the same requirement and all
break in the same circumstances.

## Application layer gateways

Protocols that carry addresses in their payload need help, and the help is a layer
violation (Chapter 21 §21.4).

FTP in active mode is the canonical case:

```
   Client ──▶ Server:21   "PORT 10,20,5,14,200,50"     ← "call me back on 10.20.5.14:51250"
   Server ──▶ Client:51250                              ← an INBOUND connection
```

Which a stateful firewall would deny, because nothing initiated it from inside.

An ALG reads the control channel, understands the PORT command, and opens a temporary pinhole
for the expected data connection. The same applies to SIP (Chapter 41 §41.4), H.323,
TFTP and several others.

**Three things to know about ALGs:**

They are a layer violation and they are necessary. Chapter 33 §33.3 makes the same point
about NAT — and both break for the same reason.

**Encryption defeats them.** The firewall cannot parse what it cannot read, so SIP over TLS
needs STUN, TURN and ICE instead, and the ALG becomes useless exactly as the protocol becomes
secure.

**And SIP ALGs are notorious.** Almost every VoIP troubleshooting guide begins "disable the SIP
ALG on the router", because consumer and small-business implementations rewrite headers
incorrectly and break calls that would otherwise work. The correct advice for a well-designed
VoIP deployment is to disable it and let the endpoints handle traversal.

## Sizing and tuning

Four numbers that determine whether a firewall works under load, and only one of them is on
the data sheet.

| | |
|---|---|
| **Throughput** | **the quoted figure, usually with large packets and no inspection enabled** |
| **Connections per second** | **the number that actually limits you** — establishing state is expensive |
| **Concurrent connections** | table size |
| **Throughput with inspection enabled** | **frequently a third to a tenth of the headline** |

> **Read the data sheet's footnotes.** "20 Gb/s firewall throughput" and "2.5 Gb/s threat
> prevention throughput" appear on the same page, and the second is the number that applies to
> how you will configure it.

And connections per second is the under-appreciated one. A network with many short-lived
connections — a busy web proxy, an API gateway, a NAT device serving thousands of users — is
limited by state establishment rather than by bandwidth, and a firewall that passes 10 Gb/s
of long flows may collapse under 40,000 new connections per second at a fraction of that
bandwidth.

**Timeouts deserve tuning:**

| Timeout | Default | Consideration |
|---|---|---|
| TCP established | **1 hour typical** | **too long wastes table; too short breaks idle sessions** |
| **TCP half-open** | 30 s | **shorten under attack** |
| **UDP** | **30–180 s** | **applications are designed against this** |
| ICMP | 2–10 s | |

## What breaks here

Some connections work and some do not, varying by flow. **Asymmetric routing.** The stateful
device sees one direction.

**Everything breaks during a firewall failover.** State was not synchronised, or was
synchronised and not tested (Chapter 56 §56.2).

A firewall fails before the server it protects, under attack. **State table exhaustion.**
SYN proxying and per-source limits.

**A long-idle session that silently stops working.** The idle timeout removed the entry.
Keepalives, or a longer timeout, and document which.

A VoIP call that sets up with no audio. The SIP ALG rewrote something, or NAT traversal
failed (Chapter 33 §33.3). Disabling the ALG is the usual first step.

FTP working in one mode and not the other. Active mode needs an inbound connection.
Passive mode, or an ALG.

A firewall at 30% of its rated throughput and dropping traffic. Inspection is enabled and
the rating was not. Read the footnotes.

**A NAT/proxy device failing at low bandwidth.** Connections per second, not throughput.

An IoT device that works for a week and then stops until rebooted. A UDP state entry timed
out and the device does not re-establish. Common, and it is the device's design error and
your support call.

> **Network+ note.** Objective 4.3 covers firewall types. Over-learn: a stateful firewall
> tracks connection state and permits return traffic automatically; a stateless firewall
> filters each packet independently against rules; **stateful is the modern default**; and
> **firewalls may be network-based or host-based.** The stateful/stateless distinction is
> examined constantly, and the asymmetric-routing consequence is what you will actually
> troubleshoot.
