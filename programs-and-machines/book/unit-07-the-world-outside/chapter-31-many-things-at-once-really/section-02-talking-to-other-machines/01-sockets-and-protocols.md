# Sockets and Protocols

A **socket** is one end of a connection between two programs. Once it is open, it
is a pair of streams — Chapter 29's abstraction, unchanged — and everything you
know about reading and writing applies.

That is the useful thing about sockets: the hard part is not the API.

## A server and a client

```java
try (ServerSocket server = new ServerSocket(0)) {          // 0 = any free port
    int port = server.getLocalPort();
    Socket connection = server.accept();                   // blocks until someone connects
    ...
}
```

```java
try (Socket c = new Socket("127.0.0.1", port);
     BufferedReader in  = new BufferedReader(new InputStreamReader(c.getInputStream()));
     PrintWriter    out = new PrintWriter(c.getOutputStream(), true)) {
    out.println("hello");
    System.out.println(in.readLine());
}
```

Verified, with a small echo server:

```
connected to port 34979
sent hello -> received ECHO hello
sent world -> received ECHO world
sent BYE   -> received OK bye
```

Two programs, on two threads, exchanging text. `accept()` blocks until a client
arrives and returns a `Socket` for that conversation; the server socket goes back
to listening.

Note the try-with-resources on everything. A socket is a scarce resource in
exactly Chapter 28's sense, and a server that leaks them runs out of file
descriptors and stops accepting connections.

## Addresses and ports

A connection is identified by four things: source address, source port,
destination address, destination port.

An **IP address** identifies a machine. A **port** identifies a program on it —
0 to 65535, with the low 1024 conventionally reserved. 80 is HTTP, 443 is HTTPS,
22 is SSH, 5432 is PostgreSQL.

`127.0.0.1` is **localhost**, the machine itself, which is what the demonstration
used. Passing 0 as the port asks the operating system for any free one, which is
the right thing in a test — hard-coding a port makes a test that fails when
something else is already listening.

## TCP and UDP

Two transport protocols, and the choice is a real one.

**TCP** — `Socket` and `ServerSocket` — provides a *connection*: bytes arrive in
order, exactly once, or the connection fails and you are told. It does this with
sequence numbers, acknowledgements and retransmission, all invisible to you.

**UDP** — `DatagramSocket` — sends independent packets. They may arrive out of
order, more than once, or not at all, and nothing tells you which. In exchange
there is no connection setup and no head-of-line blocking.

Use TCP unless you have a reason. The reasons are real but narrow: live video and
audio, where a late packet is worthless and retransmitting it wastes bandwidth;
games, for the same reason; DNS, where the exchange is one small packet each way
and a connection would cost more than the query.

TCP gives you a **stream of bytes**, and this is the detail that produces most
socket bugs. It does not preserve message boundaries. Ten `write` calls may arrive
as one `read`, or one `write` may arrive as three reads. There is no `readMessage`,
because TCP does not know what a message is.

Which is why you need the next section.

## A protocol is an agreement

Two programs exchanging bytes must agree on what the bytes mean. That agreement is
a **protocol**, and designing one is the same activity as designing a file format
in Chapter 29 — with one addition: it is a *conversation*, so it has states and an
order.

The echo server's protocol, stated properly:

> Messages are lines of UTF-8 text terminated by `\n`. The client sends a line.
> The server replies with a single line: `ECHO ` followed by the text, or `OK bye`
> if the line was exactly `BYE`, after which the server closes the connection.

Four things that specification contains, and every protocol needs all four.

**Framing** — how a receiver knows where one message ends. Here it is the newline.
The three general answers are a **delimiter** (a newline, as here, or HTTP's blank
line), a **length prefix** (four bytes saying how many follow, which is what most
binary protocols use), or **close the connection** (which permits only one
message).

Framing is the thing beginners omit, and the symptom is a program that works on
localhost and fails over a real network, because localhost happens to deliver each
write as one read and a network does not.

**Encoding** — UTF-8, stated. Chapter 29's argument, and over a network the two
ends may be different operating systems with different defaults, so the failure is
likelier.

**Grammar** — what a valid message looks like. Chapter 24's material, and for a
protocol of any complexity you should write the grammar down and generate or hand-
write a parser from it.

**States and sequence** — who speaks first, what may follow what, when the
connection ends. This is the part a file format does not need, and it is where
protocols get genuinely difficult.

## Text or binary

Text protocols — HTTP, SMTP, IRC, Redis — can be read by a human with `telnet` or
`nc`, debugged by eye, and logged legibly. They are larger and slower to parse.

Binary protocols — TLS, gRPC, most database wire formats — are compact and fast and
require a tool to inspect.

The same rule as Chapter 29: **text unless you have measured a reason.** HTTP is
text and runs most of the internet, which is a strong argument that the cost is
affordable. Note that HTTP/2 and HTTP/3 are binary, which is a measured reason
arriving at sufficient scale.

## What is underneath

Worth knowing, briefly, so that the abstraction's edges are visible.

Sockets sit on **TCP/IP**, which is layered: your bytes go into TCP segments, into
IP packets, into Ethernet or wireless frames, and are reassembled at the other
end. Routers along the way know nothing about your connection — they forward
packets toward an address, and the connection is a fiction maintained by the two
endpoints.

That design — intelligence at the edges, a dumb network in the middle — is the
end-to-end principle, and it is why the internet could grow without anyone
redesigning it. A new protocol needs no permission from any router.

Two consequences you will meet:

**Packets have a maximum size**, around 1500 bytes on most networks. Larger writes
are split, which is another reason message boundaries are not preserved.

**Nagle's algorithm** batches small writes to avoid sending a packet per byte,
which adds latency. `socket.setTcpNoDelay(true)` disables it, and it matters for
interactive protocols where a small message must go now.

Next: the two facts about networks that shape everything built on them.
