# Chapter 35 — Ports and Process Multiplexing

Do not look up the port table yet. Feel the problem first.

Your laptop has one IP address. Right now it is very likely running: a browser with
nine tabs, each holding several connections; a mail client polling a server; a chat
application maintaining a websocket; a package manager checking for updates; a
time daemon; a printing service listening for discovery; and something you installed
in 2022 and forgot about that is talking to a host you would not recognise.

Dozens of simultaneous conversations. **One address.**

A packet arrives. The IP header says it is for this machine. Which of those thirty
conversations does it belong to? The IP header cannot say — its destination field
identifies the host and there is nothing else in it that could distinguish one
process from another. The kernel is holding a packet that is correctly delivered and
completely ambiguous.

The answer is to add a number. Sixteen bits in the transport header naming the
sending process's endpoint, and sixteen more naming the receiving one. That is a
**port**, and it exists for exactly this reason and no other.

## The five-tuple

A port alone is not quite enough, and seeing why sharpens the concept.

Suppose your browser opens four connections to the same web server. All four have
your address, the server's address, protocol TCP, and destination port 443.
Identical in four of five respects — and yet they must be kept separate, because
each carries a different request.

They are distinguished by the **source port**, which the client chooses at random
from the ephemeral range. So the identifier of a connection is not a port but a
**five-tuple**:

```
( protocol, source address, source port, destination address, destination port )
```

TCP: 192.168.10.70 : 51234 → 198.51.100.10 : 443
TCP: 192.168.10.70 : 51235 → 198.51.100.10 : 443
TCP: 192.168.10.70 : 51236 → 198.51.100.10 : 443

Three distinct connections, unambiguous. The operating system maintains a table
keyed by this tuple, and every arriving segment is matched against it.

This is the object usually called a **socket**, and once you see connections as
five-tuples several other things become obvious at once: why NAT (Chapter 33) must
rewrite ports as well as addresses; why a stateful firewall (Chapter 60) keeps a
table of exactly these; why ECMP load balancing hashes the five-tuple to keep a
flow on one path; and why a connection survives a change of route but not a change
of address, which is the problem QUIC's connection IDs were invented to solve.

## The ranges, and now the numbers

Only now are port numbers worth memorising, because now they have somewhere to live.

**Well-known ports, 0–1023.** Assigned by IANA. On Unix systems, binding to one
historically required root privilege — a convention from an era when the assumption
was that a privileged process was a trustworthy one, which has not aged well but is
still enforced.

**Registered ports, 1024–49151.** Assigned by IANA on request to specific
applications. Less strictly observed.

**Dynamic / ephemeral ports, 49152–65535.** For clients to choose from. In practice
operating systems use different ranges — Linux defaults to 32768–60999, which is
visible in `/proc/sys/net/ipv4/ip_local_port_range` — and the discrepancy
occasionally matters when writing firewall rules.

The ports themselves are grouped by purpose in Chapter 41 and tabulated in
Appendix B, arranged by *what the protocol does* rather than numerically, which is
the arrangement that makes them learnable.

Two observations worth carrying now. **A port number is a convention, not a
constraint** — nothing prevents a web server on port 8443 or SSH on port 2222, and
scanning for services by port number therefore gives an incomplete picture.
And **a listening port is an attack surface**: every open port is a program willing
to accept input from strangers, which is why Chapter 62's hardening begins with
enumerating and closing them.

## Reading socket state

§35.4 is a practical section and it is one of the highest-value pages in the unit,
because the single question "what is this machine actually talking to?" is answered
in one command.

```
$ ss -tunap
Netid State   Local Address:Port    Peer Address:Port   Process
tcp   LISTEN  0.0.0.0:22            0.0.0.0:*           sshd
tcp   LISTEN  127.0.0.1:5432        0.0.0.0:*           postgres
tcp   ESTAB   192.168.10.70:51234   198.51.100.10:443   firefox
tcp   TIME-WAIT 192.168.10.70:51201 198.51.100.10:443   -
```

Four lines, each telling you something a diagram cannot:

- `sshd` listening on `0.0.0.0:22` — reachable from **any** interface, which is a
  decision someone made and may not have intended.
- `postgres` listening on `127.0.0.1:5432` — loopback only, therefore not reachable
  from the network at all. The difference between these two lines is the difference
  between an exposed database and a safe one, and it is visible nowhere else.
- An established connection with its five-tuple and owning process.
- A socket in `TIME-WAIT`, which is a normal part of TCP teardown (Chapter 37 §37.5)
  and which alarms people who have not met it.

The skill of reading this output — knowing what should be listening, noticing what
should not be, and recognising the states — is used in troubleshooting, in security
review, and in capacity work. §35.4 develops it, and Chapter 64 returns to it.

## By the end you will be able to

- Explain why ports must exist, without reference to any specific port number.
- State the five-tuple and use it to explain how multiple connections to one server
  are distinguished.
- Explain the three port ranges and their conventions.
- Read `ss` or `netstat` output and identify listening services, established
  connections, their owning processes, and their exposure.
- Explain why "the port is a convention" matters for security scanning and for
  firewall design.
