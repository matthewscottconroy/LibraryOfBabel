# 35.2 The Socket

§35.1 ended with a problem: a thousand clients connected to port 443, all bearing the same
destination port, all needing to be told apart. This section is the answer, and it is one
of the most useful facts in the book.

## The five-tuple

**A connection is identified by five things:**

$$(\underbrace{\text{protocol}}_{\text{TCP}},\ \underbrace{\text{src IP}}_{\text{198.51.100.7}},\ \underbrace{\text{src port}}_{\text{51234}},\ \underbrace{\text{dst IP}}_{\text{203.0.113.10}},\ \underbrace{\text{dst port}}_{\text{443}})$$

**Every field is in the packet.** The protocol comes from the IP header; the addresses from
the IP header; the ports from the transport header. **A receiving host can therefore
identify the connection from the packet alone**, with no state lookup beyond matching the
tuple.

**A thousand clients on port 443:**

| Protocol | Src IP | Src port | Dst IP | Dst port |
|---|---|---|---|---|
| TCP | `198.51.100.7` | 51234 | `203.0.113.10` | **443** |
| TCP | `198.51.100.9` | 44821 | `203.0.113.10` | **443** |
| TCP | `192.0.2.55` | 51234 | `203.0.113.10` | **443** |
| TCP | `198.51.100.7` | 51235 | `203.0.113.10` | **443** |

**All four are distinct connections**, and every one is distinguishable.

**Rows 1 and 3** share a source port and differ in source address. **Rows 1 and 4** share a
source address and differ in source port. **The tuple is unique as long as any one field
differs**, which is what makes the arrangement work.

## Why this changes everything

**Once you hold the five-tuple in your head, several things become obvious at once:**

**How a server handles many clients.** It does not need a port per client. One listening
port, and the client's address and port distinguish them.

**Why NAT works** (Chapter 33 §33.2). The router rewrites the source address and, when
necessary, the source port — **two of the five fields** — and records the mapping. The
tuple remains unique on both sides.

**Why a stateful firewall is possible** (Chapter 60 §60.2). It records the tuples it has
seen going out and permits the matching return traffic. **The tuple is the state.**

**Why one client can open many connections to one server.** Different source ports, so
different tuples. This is exactly what a browser does — several connections to one site,
each with its own ephemeral port.

**Why load balancer hashing works** (Chapter 19 §19.4, Chapter 29 §29.3). Hash the tuple,
or part of it, and every packet of one conversation lands on the same path or the same
backend. **The tuple is the flow identifier.**

**Why `conntrack` and `show ip nat translations` look the way they do.** They are tables of
tuples.

> **Five numbers, and half of Unit VIII follows from them.**

## The socket

**A socket is one endpoint of a connection** — the operating system's handle on it.

**A listening socket** is bound to a local address and port and has no peer yet:

$$(\text{TCP},\ \texttt{0.0.0.0},\ \texttt{443},\ *,\ *)$$

**A connected socket** has all five fields filled in.

**The distinction shows in the tools:**

```
$ ss -tln
State    Local Address:Port    Peer Address:Port
LISTEN   0.0.0.0:443           0.0.0.0:*          ← listening, no peer
LISTEN   127.0.0.1:5432        0.0.0.0:*          ← listening, LOCAL ONLY

$ ss -tn
State    Local Address:Port      Peer Address:Port
ESTAB    203.0.113.10:443        198.51.100.7:51234    ← a connection
ESTAB    203.0.113.10:443        198.51.100.9:44821    ← another
```

**The two `LISTEN` lines differ in a way that matters.** `0.0.0.0:443` accepts connections
on **every interface**; `127.0.0.1:5432` accepts only from the local machine (Chapter 27
§27.2).

**That difference is a security control**, and it is the first thing to check when a
service is unexpectedly reachable — or unexpectedly not.

## Binding

**The operating system enforces one rule:**

> **Two sockets may not bind the same (protocol, address, port) triple.**

```bash
# First program:
bind(0.0.0.0:8080)   → succeeds

# Second program:
bind(0.0.0.0:8080)   → EADDRINUSE: Address already in use
```

**"Address already in use" means exactly this** — something already holds that triple.

**And the address part matters.** These do **not** conflict:

```
   0.0.0.0:8080        ← wait, this DOES conflict with the others
   127.0.0.1:8080
   10.0.0.5:8080
```

**`127.0.0.1:8080` and `10.0.0.5:8080` can coexist** — different addresses. But
**`0.0.0.0:8080` conflicts with both**, because it claims every address including those
two.

**This is why "address already in use" sometimes appears when `ss` shows nothing obvious
on that port** — something is bound to a wildcard, or to a specific address you did not
check.

**`SO_REUSEADDR` and `SO_REUSEPORT`** relax the rule deliberately: the first allows binding
while an old connection lingers in `TIME-WAIT`, and the second allows several processes to
share a listening socket for load distribution — which is how modern servers use multiple
CPU cores.

## The connection lifecycle

**A client's side:**

```
   socket()                  create an endpoint
   [connect() implicitly binds an ephemeral source port]
   connect(203.0.113.10:443) the OS picks a free source port, say 51234
                             → the five-tuple now exists
```

**The client does not choose its port** — the kernel picks one from the ephemeral range
(§35.3), guaranteeing it is free.

**A server's side:**

```
   socket()
   bind(0.0.0.0:443)         claim the port
   listen()                  accept incoming connections
   accept()                  → returns a NEW socket with the full five-tuple
```

**`accept()` returning a new socket is the part worth understanding.** The listening socket
remains listening; the new socket represents one connection.

**So a server with 1,000 clients has 1,001 sockets** — one listener plus one per
connection — all sharing the same local port, distinguished by the peer's address and
port.

**This is why "the server ran out of ports" is almost always wrong.** A server does not
consume a port per client. **A client** making thousands of outbound connections may
exhaust its ephemeral range; a server accepting them does not.

## The exception: UDP

**UDP has no connections**, so the five-tuple is not held as state by the protocol.

A UDP socket may be:

**Unconnected** — bound to a local port, receiving from anyone, and using `recvfrom()` to
learn who each datagram came from. **This is what a DNS server or a DHCP server does.**

**"Connected"** — the application calls `connect()` on a UDP socket, which does not send
anything but tells the kernel to accept datagrams only from that peer and to allow
`send()` without specifying a destination each time. **A convenience, not a connection.**

**The consequence for NAT and firewalls** (Chapter 33 §33.2): with no connection, there is
no teardown, so **a NAT or firewall must guess when a UDP flow has ended** — hence the
timeout, and hence the keepalives that long-lived UDP applications send.

## Reading socket state during an incident

**The commands, and what each answers:**

```bash
ss -tlnp          # what is LISTENING, and which process
ss -tnp           # established TCP connections
ss -unp           # UDP sockets
ss -tan           # everything, including TIME-WAIT
ss -s             # summary counts
ss -tn state established '( dport = :443 or sport = :443 )'
```

**On other systems:**

```bash
netstat -tlnp             # older Linux, same idea
netstat -an               # Windows, macOS, BSD
lsof -i :443              # which process holds a port — very useful
Get-NetTCPConnection      # PowerShell
```

**The questions they answer, in the order you will ask them:**

| Question | Command |
|---|---|
| **Is anything listening on that port?** | `ss -tlnp \| grep :443` |
| **Which process?** | the same, `-p` |
| **Is it listening everywhere or only locally?** | look at the address — `0.0.0.0` vs `127.0.0.1` |
| Are connections being established? | `ss -tnp` |
| How many, and in what states? | `ss -s`, `ss -tan` |
| Is something exhausting a resource? | count `TIME-WAIT` (Chapter 37 §37.5) |

**`ss -tlnp` is the single most useful command in this chapter.** *"Is anything actually
listening?"* is the question that resolves a large share of "the service is down" reports,
and it takes one line.

## What breaks here

**"Address already in use" with nothing visible on the port.** Something is bound to a
wildcard address, or to a specific address you did not check. `ss -tlnp` with no filter.

**A service running and unreachable from the network.** Bound to `127.0.0.1` rather than
`0.0.0.0`. **This is extremely common** and the fix is one line of the service's
configuration.

**A service unexpectedly reachable from the Internet.** The opposite — bound to `0.0.0.0`
when it should be local. **A finding, in any security review.**

**"The server ran out of ports."** Almost certainly wrong. Servers do not consume a port
per client; check ephemeral exhaustion on the **client** side, or `TIME-WAIT` accumulation.

**A UDP application losing connectivity after idle time.** No connection means no teardown
means a NAT or firewall timeout guessed wrong.

> **Network+ note.** Objective 1.4 expects ports; objective 5.5 expects `netstat`/`ss`.
> Over-learn: **a connection is identified by the five-tuple**; **one listening port serves
> many clients because the client's address and port distinguish them**; **`0.0.0.0` means
> all interfaces and `127.0.0.1` means local only**; and **`ss -tlnp` shows what is
> listening and which process owns it.**
