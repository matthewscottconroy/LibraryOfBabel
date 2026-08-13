# 21.2 Interfaces and Service Primitives

The saving in §21.1 depends entirely on one thing: **the interface between layers must
be stable and complete**. If applications need to know what medium they are running
over, nothing was factored out and the *m*+*n* arrangement is *m*×*n* wearing a
disguise.

So the interface is the whole design. This section is about what an interface must
specify, and what it must refuse to.

## Service versus protocol

The distinction that makes layering work, and the one most often blurred.

**A service** is what a layer offers to the layer above. It is defined by
*operations* — what you can ask for and what you get back. It says nothing about how.

**A protocol** is how peer entities at the same layer communicate to implement the
service. Message formats, state machines, timers.

```
        Host A                              Host B

    ┌───────────┐                       ┌───────────┐
    │  Layer 4  │◀────  protocol  ─────▶│  Layer 4  │
    └───────────┘                       └───────────┘
          ▲                                   ▲
       service                             service
     (an interface)                      (an interface)
          ▼                                   ▼
    ┌───────────┐                       ┌───────────┐
    │  Layer 3  │◀────  protocol  ─────▶│  Layer 3  │
    └───────────┘                       └───────────┘
```

**Services are vertical; protocols are horizontal.** The layer above sees only the
service. It has no visibility into the protocol, and — this is the point — **the
protocol can be replaced entirely without the layer above noticing**, provided the
service is unchanged.

TCP's congestion control has been rewritten many times: Tahoe, Reno, NewReno, CUBIC,
BBR (Chapter 38 §38.3). No application changed. The service — a reliable ordered byte
stream — was constant while the protocol implementing it was rebuilt repeatedly.

## Service primitives

The formal vocabulary, from the OSI work. Four primitives:

| Primitive | Meaning |
|---|---|
| **Request** | The user asks the layer to do something |
| **Indication** | The layer tells its user something happened |
| **Response** | The user replies to an indication |
| **Confirm** | The layer reports the outcome of a request |

A confirmed service uses all four:

```
   Host A                                       Host B
   user      layer                    layer      user
     │  CONNECT.request                            │
     │────────▶│                                   │
     │         │────── protocol message ──────▶│   │
     │         │                               │───▶  CONNECT.indication
     │         │                               │◀───  CONNECT.response
     │         │◀───── protocol message ───────│   │
     │◀────────│                                   │
     │  CONNECT.confirm                            │
```

This is TCP's three-way handshake described without reference to TCP. `connect()`,
`accept()` and the return of `connect()` are the same four primitives with different
names — which is the point of having the abstract vocabulary at all.

The four names are OSI's, and they are worth knowing because they let you compare
protocols that share no other terminology.

## The socket interface — the one that won

Formal service definitions in the OSI style were specified extensively and implemented
rarely. **The interface that actually organised the industry** came from Berkeley in
1983, written for 4.2BSD by Bill Joy's group, and it is almost aggressively informal:

```c
   int  socket(int domain, int type, int protocol);
   int  bind(int s, struct sockaddr *addr, socklen_t len);
   int  listen(int s, int backlog);
   int  accept(int s, struct sockaddr *addr, socklen_t *len);
   int  connect(int s, struct sockaddr *addr, socklen_t len);
   ssize_t send(int s, const void *buf, size_t len, int flags);
   ssize_t recv(int s, void *buf, size_t len, int flags);
   int  close(int s);
```

Eight calls. That is the boundary between every application and every network, on
every operating system, for forty years.

Its properties are worth reading off deliberately, because they explain the
durability:

**It looks like file I/O.** `read`, `write`, `close` work on sockets. Existing
programs and existing programmers needed almost nothing new.

**It hides the protocol.** `socket(AF_INET, SOCK_STREAM, 0)` gives TCP;
`SOCK_DGRAM` gives UDP; `AF_INET6` gives IPv6. **The same eight calls**, and an
application written in 1985 can use IPv6 with a change to one argument.

**It hides the medium completely.** Nothing in the interface mentions Ethernet, Wi-Fi,
fibre or satellite. This is §21.1's *n* side made invisible, and it is why the Web
worked over Wi-Fi on the day Wi-Fi shipped.

**It is small.** Eight calls that can be learned in an afternoon. Compare the OSI
service definitions, which are correct, complete, and were implemented by almost
nobody.

> **A specified-but-unimplemented interface organises nothing. Sockets won by being
> available, adequate and free** — the same combination that decided Chapter 23's
> larger contest.

## What an interface must not expose

The failures are more instructive than the successes.

**It must not expose the medium.** Once an application knows it is on Wi-Fi, it will
be written to depend on Wi-Fi, and the *m*+*n* saving evaporates.

**It must not expose addressing details unnecessarily.** `sockaddr` is a generic
structure precisely so that IPv4, IPv6 and Unix-domain sockets share one interface.
The applications that broke on IPv6 are overwhelmingly the ones that assumed a 32-bit
address — that stored it in an `int`, or allocated 16 bytes for a text form. **The
interface was general and the applications were not**, and that is a large part of why
IPv6 took thirty years.

**It must not expose timing.** An application that depends on a particular latency
works on the network it was tested on and fails elsewhere.

And, symmetrically:

**It must not hide what the user genuinely needs.** This is the harder failure, and
§21.3 is about it.

## What the interface hides that it should not

Three examples where the abstraction is too opaque, and each is a real, ongoing
operational problem.

**Path MTU.** An application writes 64 KB; the stack fragments or discovers the path
MTU or fails, and the application cannot tell which. When path MTU discovery breaks —
which it does regularly, because ICMP is filtered (Chapter 34 §34.1) — the symptom is
a connection that establishes and then hangs on large transfers, and **the application
has no way to see the cause**.

**Connection quality.** `send()` returns success when the data is in the kernel's
buffer, not when it has been delivered. An application cannot easily tell a fast
network from a slow one, or a lossy one from a clean one, without measuring itself.

**Which interface was used.** A host with Wi-Fi and Ethernet and a cellular modem
makes a routing decision the application does not see. For a mobile device deciding
whether to download a large file over a metered link, this matters — which is why every
mobile platform has added a non-standard API to expose exactly this, outside the
socket interface.

The pattern is consistent: **where the abstraction hides something the user genuinely
needs, the abstraction is bypassed rather than fixed.** §21.4 catalogues the results.

## Encapsulation as the mechanism

The interface is a contract; **encapsulation** is how it is honoured on the wire.

Each layer takes the unit handed down, treats it as **opaque data**, and adds its own
header:

```
   Application:                              [ data ]
   Transport:                     [ TCP hdr | data ]
   Internet:            [ IP hdr | TCP hdr | data ]
   Link:      [ Eth hdr | IP hdr | TCP hdr | data | FCS ]
```

The word *opaque* is doing the work. **IP does not parse the TCP header.** Ethernet
does not parse the IP header beyond the EtherType that says what it is. Each layer
treats what it carries as bytes it must deliver and must not interpret.

That discipline is what makes independent evolution possible — and it is exactly what
middleboxes violate (§21.4), with consequences that have shaped the last twenty years
of protocol design. Chapter 23 §23.3 traces a real request through every layer.

## What breaks here

**An application that knows too much about the network.** It works in the environment
it was written for.

**An application that assumes a 32-bit address.** It does not work on IPv6, and the
fix is often a rewrite rather than a recompile.

**An abstraction leaking a failure it cannot explain.** A hung transfer with no error
is usually path MTU. The interface has no way to say so.

**Confusing service and protocol.** "TCP guarantees delivery" is a statement about the
service. "TCP retransmits after three duplicate ACKs" is a statement about the
protocol. Mixing them produces confident wrong answers.

> **Network+ note.** Not examined directly, and the **service/protocol distinction**
> makes several examined topics easier: it is why a layer's implementation can change
> without affecting anything above, and it is the precise sense in which the OSI model
> is useful.
