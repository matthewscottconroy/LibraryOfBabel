# 37.5 Teardown and the State Machine

Closing a TCP connection is harder than opening one, and the reason is worth stating: **a
connection is two independent byte streams**, and either may finish while the other
continues.

## The four-way close

```
   Client                                          Server
     │                                               │
     │  ── FIN, seq=u ──────────────────────────────▶│   "I have no more to send"
     │◀───────────────────────────── ACK, ack=u+1 ───│   "acknowledged"
     │                                               │
     │        ... the server may still send data ... │
     │                                               │
     │◀───────────────────────────── FIN, seq=v ─────│   "I have no more either"
     │  ── ACK, ack=v+1 ────────────────────────────▶│
     │                                               │
```

**Four packets, not three.** The handshake could combine the server's SYN with its ACK
because both were ready at once. **The close cannot combine the server's ACK with its FIN**,
because the server may have data still to send.

**This is the half-close**, and it is a real feature:

```bash
# The classic use
ssh host 'cat > /tmp/file' < local_file
# The client sends the file, then FIN. The server keeps its
# direction open to report success or an error.
```

**In practice the two are often combined** — a server with nothing more to send responds
with `FIN, ACK` in one packet, giving a three-packet close.

**Like SYN, FIN consumes a sequence number** (§37.1), hence the `+1`.

## The state machine

**RFC 793's diagram, reduced to what you will read in `ss` output:**

```
                       ┌──────────┐
                       │  CLOSED  │
                       └────┬─────┘
              passive open  │  active open (connect)
                 (listen)   │  send SYN
                    ┌───────┴────────┐
                    ▼                ▼
              ┌──────────┐     ┌───────────┐
              │  LISTEN  │     │ SYN-SENT  │
              └────┬─────┘     └─────┬─────┘
         recv SYN  │                 │ recv SYN-ACK, send ACK
         send      │                 │
         SYN-ACK   ▼                 │
              ┌──────────┐           │
              │ SYN-RECV │           │
              └────┬─────┘           │
                   │ recv ACK        │
                   └────────┬────────┘
                            ▼
                     ┌─────────────┐
                     │ ESTABLISHED │
                     └──┬───────┬──┘
          we close first│       │peer closes first
          send FIN      │       │recv FIN, send ACK
                        ▼       ▼
              ┌────────────┐  ┌────────────┐
              │ FIN-WAIT-1 │  │ CLOSE-WAIT │  ← the app must call close()
              └─────┬──────┘  └─────┬──────┘
              recv  │               │ app closes, send FIN
              ACK   ▼               ▼
              ┌────────────┐  ┌────────────┐
              │ FIN-WAIT-2 │  │  LAST-ACK  │
              └─────┬──────┘  └─────┬──────┘
              recv  │               │ recv ACK
              FIN   ▼               ▼
              ┌────────────┐     ┌────────┐
              │ TIME-WAIT  │     │ CLOSED │
              └─────┬──────┘     └────────┘
                    │ 2×MSL
                    ▼
                ┌────────┐
                │ CLOSED │
                └────────┘
```

**The left branch is "we closed first"; the right is "they did".** Which side closes first
determines who pays the costs below.

## TIME-WAIT

**The state everyone tries to eliminate, and it is doing something necessary.**

**The side that closes first** — the one that sent the first FIN — **holds the connection's
tuple for 2×MSL**, where MSL is the Maximum Segment Lifetime, conventionally 60 seconds.

**On Linux the wait is 60 seconds** (a fixed `TCP_TIMEWAIT_LEN`, not literally 2×MSL).

### Why it exists — two reasons

**1. Late segments must not corrupt a new connection.**

A segment from the old connection, delayed in the network, could arrive after a **new**
connection using **the same five-tuple** has been established — and its sequence numbers
could fall within the new connection's window.

**The data would be accepted as belonging to the new connection.** Holding the tuple for
2×MSL guarantees every stray segment has expired before it can be reused.

**2. The final ACK may be lost.**

```
   A ── FIN ──▶ B
   A ◀── ACK ── B
   A ◀── FIN ── B
   A ── ACK ──▶ B      ← if this is lost...
```

**B retransmits its FIN.** If A had closed completely, A would respond with **RST** — and B
would record an error on a connection that closed cleanly.

**By staying in TIME-WAIT, A can retransmit the ACK.** The state exists partly to be
polite to the peer.

### The cost, and where it falls

**TIME-WAIT holds a tuple, and therefore a local port.**

**On a server:** the server usually does **not** close first — a well-behaved client
closes, so the client accumulates TIME-WAIT and the server does not. **A server with many
TIME-WAIT sockets is closing first**, which is worth investigating.

**On a client:** ephemeral port exhaustion (Chapter 35 §35.3). ~28,000 ports ÷ 60 s ≈ **470
new connections per second, sustained.**

### The remedies

| Remedy | Verdict |
|---|---|
| **Connection reuse / keep-alive** | **the real answer** — do not open so many |
| **`net.ipv4.tcp_tw_reuse=1`** | **safe.** Reuse a TIME-WAIT socket for a *new outbound* connection when timestamps prove it is safe |
| More destination addresses | the tuple includes them, so the space multiplies |
| **`SO_REUSEADDR`** | lets a server rebind a listening port while old connections linger |
| **`tcp_tw_recycle`** | **removed from Linux 4.12. It broke every client behind NAT.** Advice recommending it is old and was always wrong |
| Reducing MSL | **do not** — it reintroduces the hazard TIME-WAIT prevents |

**Thousands of TIME-WAIT sockets on a busy machine are normal and are not a problem.** They
cost a small amount of memory each. **They matter only when they exhaust ephemeral ports on
a client.**

## CLOSE-WAIT — the application bug

**Chapter 35 §35.4 introduced it; here is why it happens.**

**The peer sent FIN. The kernel acknowledged it and moved to CLOSE-WAIT. Now it is waiting
for the local application to call `close()`.**

**Until the application does, the socket and its file descriptor remain allocated** — and
the kernel cannot help, because the application may still want to send data (the half-close
above).

**A growing CLOSE-WAIT count is a resource leak in application code**, and its endgame is
file-descriptor exhaustion and a service that stops accepting connections hours after the
bug ran.

```bash
ss -tanp state close-wait | head        # find the process
lsof -p PID | wc -l                     # how many descriptors it holds
cat /proc/PID/limits | grep files       # the ceiling it is approaching
```

> **CLOSE-WAIT accumulating is never a network problem.** It is a missing `close()`, an
> exception path that skips cleanup, or a connection pool that never reaps.

## RST — the abrupt close

**Not part of the graceful sequence. A refusal or an abort.**

**When a RST is sent:**

| Cause | Meaning |
|---|---|
| **SYN to a port with nothing listening** | **"connection refused"** |
| Data arriving for a connection that does not exist | e.g. after a reboot |
| **`SO_LINGER` with timeout 0**, then `close()` | deliberate abort — skips TIME-WAIT |
| An application crashing | the OS resets its connections |
| **A firewall or middlebox injecting one** | to terminate a session |

**RST is immediate and unacknowledged.** Data in flight is discarded, and **the peer
receives "connection reset by peer"** — one of the least informative error messages in
common use, because all six causes produce it.

**Reading a capture is how you distinguish them:**

| Pattern | Diagnosis |
|---|---|
| **RST immediately after SYN** | nothing listening |
| RST after ESTABLISHED, no FIN | an abort — crash, or deliberate |
| **RST from a third party's address** | **injected** — a middlebox, a firewall, or an attacker |
| RST after a period of idleness | a middlebox timed the session out and reset it |

**The last row is common and worth recognising.** A stateful firewall that has forgotten a
long-idle connection may reset it — which is why **TCP keepalives** exist:

```bash
sysctl net.ipv4.tcp_keepalive_time      # 7200 — two hours before the first probe
sysctl net.ipv4.tcp_keepalive_intvl     # 75
sysctl net.ipv4.tcp_keepalive_probes    # 9
```

**The two-hour default is far too long for modern middleboxes**, which commonly forget
after 5–30 minutes. **Applications that hold idle connections should enable keepalives and
set the interval well below the shortest middlebox timeout on the path.**

## Reading states in `ss`

```bash
ss -tan | awk '{print $1}' | sort | uniq -c | sort -rn
```

```
   4821 ESTAB
   2103 TIME-WAIT
     18 LISTEN
      7 CLOSE-WAIT
      3 SYN-SENT
```

**The interpretation:**

| State | Many of them means |
|---|---|
| **ESTAB** | normal |
| **TIME-WAIT** | **normal on a client**; on a server, it is closing first |
| **CLOSE-WAIT** | **an application bug** |
| **SYN-SENT** | **outbound connections unanswered — a network or firewall problem** |
| **SYN-RECV** | half-open — possibly a SYN flood (§37.1) |
| **FIN-WAIT-2** | the peer is not closing its side — often the same bug as CLOSE-WAIT, at the other end |
| **LAST-ACK** | our FIN is unacknowledged — the peer is gone |

**One command, and the distribution tells you which of five different problems you have.**

## What breaks here

**Growing CLOSE-WAIT.** A missing `close()`. Find the process; it is a code defect.

**"Connection reset by peer" with no other information.** Capture and look at *when* the
RST arrives — the pattern distinguishes the causes.

**Idle connections dying after a few minutes.** A middlebox forgot the session. Enable
keepalives with a shorter interval.

**Ephemeral exhaustion from TIME-WAIT.** Reuse connections; `tcp_tw_reuse` as a
supplement.

**A server unable to restart because its port is "in use".** Lingering connections. Use
`SO_REUSEADDR`.

**Advice recommending `tcp_tw_recycle`.** It was removed and it was always dangerous.

> **Network+ note.** Objective 1.4 expects the TCP state machine and connection teardown.
> Over-learn: **the four-way close — FIN, ACK, FIN, ACK**; **FIN consumes a sequence
> number**; **TIME-WAIT is entered by whichever side closes first and lasts 2×MSL**;
> **RST is an abrupt abort and means "refused" when it follows a SYN**; and **ESTABLISHED,
> LISTEN, TIME-WAIT and CLOSE-WAIT** as the states you will actually see.
