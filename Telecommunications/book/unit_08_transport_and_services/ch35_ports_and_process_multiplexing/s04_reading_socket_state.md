# 35.4 Reading Socket State

The socket table is the most under-read diagnostic surface on any machine. This section is
about reading it fluently, because a great many faults that present as "the application is
broken" are visible in one command.

## The one command

```bash
ss -tlnp
```

| Flag | Meaning |
|---|---|
| `-t` | TCP |
| `-l` | listening sockets only |
| `-n` | numeric — **do not resolve names**, which is faster and avoids DNS delays |
| `-p` | show the **process** |

```
$ ss -tlnp
State  Recv-Q Send-Q  Local Address:Port  Peer Address:Port  Process
LISTEN 0      4096    0.0.0.0:22          0.0.0.0:*          users:(("sshd",pid=812,fd=3))
LISTEN 0      511     0.0.0.0:443         0.0.0.0:*          users:(("nginx",pid=1204,fd=6))
LISTEN 0      4096    127.0.0.1:5432      0.0.0.0:*          users:(("postgres",pid=980,fd=5))
LISTEN 0      4096    127.0.0.1:6379      0.0.0.0:*          users:(("redis",pid=1101,fd=6))
```

**Read it as four facts per line:** what port, on which addresses, which process, and how
deep the accept queue may go.

**The address column is the security review.** `0.0.0.0` means every interface —
reachable from the network. `127.0.0.1` means local only. **In the output above, PostgreSQL
and Redis are correctly local-only and nginx and sshd are correctly public**, and a single
glance confirms it.

**Always use `-n`.** Without it, `ss` and `netstat` resolve every address, which on a busy
machine takes minutes and, if DNS is the thing that is broken, hangs.

## The states

TCP's state machine is Chapter 37 §37.5's subject. **The states you will actually read**,
and what each tells you:

| State | Meaning | What it indicates |
|---|---|---|
| **LISTEN** | waiting for connections | the service is up |
| **SYN-SENT** | we sent SYN, nothing back | **the peer is unreachable or filtered** |
| SYN-RECV | we received SYN, sent SYN-ACK | mid-handshake; **many of these means a SYN flood** |
| **ESTABLISHED** | connected | normal |
| FIN-WAIT-1/2 | we are closing | normal, briefly |
| CLOSE-WAIT | **the peer closed; we have not** | **an application bug** |
| **TIME-WAIT** | we closed, waiting out stragglers | **normal; large numbers are usually fine** |
| CLOSED | — | transient |

**Three of those are diagnostic and worth learning properly.**

### SYN-SENT — the peer is not answering

```
SYN-SENT  10.0.0.5:51234  203.0.113.10:443
```

**We sent a SYN and received nothing.** Not a refusal — silence.

**This is the "silence versus RST" distinction** of Chapter 22 §22.4, seen from the
client's socket table. **Silence means a firewall is dropping**, or the route is
asymmetric, or the host is down. **A RST would have produced no socket at all** — the
connection would have failed immediately with "connection refused".

**Sockets accumulating in SYN-SENT is a network problem, not an application problem.**

### CLOSE-WAIT — an application bug

**The one that indicates a defect in code rather than in the network.**

```
CLOSE-WAIT  10.0.0.5:8080  198.51.100.7:44821
```

**The peer sent FIN. Our application has not called `close()`.**

The kernel is waiting for the application to notice the connection has ended and release
it. **Until it does, the socket and its file descriptor stay allocated.**

**A steadily growing count of `CLOSE-WAIT` sockets is a resource leak**, and it ends with
the process hitting its file-descriptor limit and refusing new connections — at which point
the symptom is "the service stopped accepting connections" and the cause is code that has
been leaking for hours.

```bash
ss -tan state close-wait | wc -l          # count them
ss -tanp state close-wait                 # find the guilty process
```

**Hundreds of these, growing, is a bug report.**

### TIME-WAIT — usually fine

**The most misunderstood state**, and the one people try hardest to eliminate.

After closing a connection, the side that closed first holds the tuple for **2×MSL** —
conventionally **60 seconds** on Linux — so that late-arriving segments from the old
connection cannot be mistaken for part of a new one using the same tuple (Chapter 37
§37.5).

**Thousands of `TIME-WAIT` sockets on a busy server is normal**, costs a small amount of
memory each, and is not a problem.

**It becomes a problem only on the *client* side**, where it consumes ephemeral ports
(§35.3) and limits the outbound connection rate.

```bash
ss -tan state time-wait | wc -l
```

**The safe remedy is `net.ipv4.tcp_tw_reuse=1`**, which lets outbound connections reuse a
`TIME-WAIT` socket when it is safe to do so.

**`tcp_tw_recycle` was the unsafe one** — it broke connections from any client behind NAT,
because it made assumptions about timestamps that NAT violates. **It was removed from Linux
in kernel 4.12.** If you find advice recommending it, the advice is old and was always
wrong.

## The queues

**Two columns most people ignore**, and they answer a question nothing else does.

```
State  Recv-Q Send-Q  Local Address:Port
LISTEN 0      511     0.0.0.0:443
ESTAB  4096   0       203.0.113.10:443
```

**On a `LISTEN` socket:**

- **`Recv-Q`** = connections **waiting to be `accept()`ed**
- **`Send-Q`** = the **backlog limit**

**A non-zero `Recv-Q` on a listening socket means the application is not accepting fast
enough.** If it reaches `Send-Q`, **new connections are dropped** — and the client sees a
timeout, not a refusal, which sends everyone looking at the network.

```bash
# Is the backlog overflowing?
ss -tln
netstat -s | grep -i listen        # "times the listen queue of a socket overflowed"
```

**That counter is the direct evidence**, and it is the difference between "the network is
slow" and "the application cannot keep up".

**On an `ESTABLISHED` socket:**

- **`Recv-Q`** = bytes received and **not yet read by the application**
- **`Send-Q`** = bytes written by the application and **not yet acknowledged by the peer**

**A large, persistent `Recv-Q`** means the application is not reading — it is slow,
blocked, or stuck.

**A large, persistent `Send-Q`** means the peer is not acknowledging — the network is
congested, the peer's receive window is closed, or the peer is not reading (Chapter 37
§37.4).

> **`Recv-Q` blames the local application. `Send-Q` blames the network or the peer.**
> Two columns, and they separate "our fault" from "their fault" in one glance.

## The command set

```bash
# What is listening
ss -tlnp
ss -ulnp                          # UDP
ss -tulnp                         # both

# What is connected
ss -tnp
ss -tan                           # all states
ss -s                             # summary counts by state

# Filtering — ss's query language is worth knowing
ss -tn state established
ss -tn state time-wait
ss -tn '( dport = :443 or sport = :443 )'
ss -tn dst 203.0.113.0/24
ss -tnp '( sport >= :32768 )'     # outbound ephemeral connections

# Which process holds a port
lsof -i :443
fuser -n tcp 443

# Per-socket detail, including congestion window and RTT
ss -tni
```

**`ss -tni` deserves a mention.** It reports, per connection, the RTT estimate, the
congestion window, retransmission counts and the congestion-control algorithm in use:

```
ESTAB 0 0 10.0.0.5:44312 203.0.113.10:443
    cubic wscale:7,7 rto:236 rtt:35.5/2.1 cwnd:24 bytes_sent:1420000
    bytes_acked:1419500 retrans:0/3 rcv_space:14600
```

**`retrans:0/3` means three retransmissions total, none currently outstanding.** For
diagnosing a slow connection, this is Chapter 38's material available live, per connection,
without a capture.

## Other platforms

```bash
netstat -tlnp        # older Linux; ss is faster and better
netstat -an          # macOS, BSD, Windows — universally available
netstat -anb         # Windows, with the process (requires admin)
lsof -i -P -n        # macOS, very readable
Get-NetTCPConnection -State Listen        # PowerShell
```

**`netstat -an` is on everything**, which makes it the fallback when you are on an
unfamiliar system.

## The diagnostic sequence

**When a service is reported down**, in order:

```bash
# 1. Is the process running?
systemctl status nginx

# 2. Is it LISTENING, and where?
ss -tlnp | grep :443

# 3. Can it be reached locally?
curl -v https://localhost/

# 4. Can it be reached from the network?
#    (from another machine)
nc -zv 203.0.113.10 443

# 5. If not, is it a firewall?
#    RST = closed;  silence = filtered  (Chapter 22 §22.4)
```

**Step 2 resolves a large fraction of these**, and the two most common answers are *nothing
is listening* and *it is listening on `127.0.0.1` only*.

**Steps 3 and 4 together are the key separation:** if it works locally and not remotely,
**the application is fine and the problem is the bind address or the network.** That
eliminates the entire application from the search in two commands.

## What breaks here

**Nothing listening.** The service is not running, or failed to bind.

**Listening on `127.0.0.1` when it should be public.** Very common, one configuration
line.

**Listening on `0.0.0.0` when it should be local.** A security finding.

**Growing `CLOSE-WAIT` count.** An application is not closing sockets. A bug, and it ends
in file-descriptor exhaustion.

**Non-zero `Recv-Q` on a listener.** The application is not accepting fast enough; the
backlog may be overflowing and clients are seeing timeouts.

**Large `Send-Q` on established connections.** The peer or the network, not you.

**Many `SYN-SENT`.** Outbound connections going unanswered — a network or firewall
problem.

**Many `SYN-RECV`.** Half-open connections — possibly a SYN flood (Chapter 62).

**Advice recommending `tcp_tw_recycle`.** It is old, it was dangerous, and it has been
removed.

> **Network+ note.** Objective 5.5 expects `netstat` and its equivalents. Over-learn:
> **`ss -tlnp` shows what is listening and which process**; **`0.0.0.0` is all interfaces
> and `127.0.0.1` is local only**; **ESTABLISHED, LISTEN and TIME-WAIT** as states; and
> **a service reachable locally and not remotely is a bind-address or firewall problem,
> not an application problem.**
