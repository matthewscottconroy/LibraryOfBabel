# Chapter 35 — Exercises

## A. Recall

**A1.** State the problem ports solve, in one sentence, and why IP alone cannot solve it.

**A2.** How many bits is a port? How many values? Give the three ranges with their
boundaries.

**A3.** List the five fields of the five-tuple and say where in the packet each comes
from.

**A4.** Give the port and protocol for: FTP, SSH, Telnet, SMTP, DNS, DHCP, TFTP, HTTP,
POP3, NTP, IMAP, SNMP, LDAP, HTTPS, SMB, syslog.

**A5.** What does a socket bound to `0.0.0.0` accept that one bound to `127.0.0.1` does
not?

**A6.** What does the `CLOSE-WAIT` state indicate, and whose fault is it?

**A7.** On a listening socket, what do `Recv-Q` and `Send-Q` mean?

## B. Apply

**B1.** For each pair, state whether they can coexist as bindings, and why:

(a) `0.0.0.0:8080` and `127.0.0.1:8080`
(b) `127.0.0.1:8080` and `10.0.0.5:8080`
(c) TCP `0.0.0.0:53` and UDP `0.0.0.0:53`
(d) `0.0.0.0:443` and `0.0.0.0:443`

**B2.** Four connections arrive at a server at `203.0.113.10:443`. Write the complete
five-tuple for each and confirm all are distinguishable:

(a) from `198.51.100.7:51234`  (b) from `198.51.100.7:51235`
(c) from `192.0.2.9:51234`     (d) from `198.51.100.7:51234` over IPv6

**B3.** A machine has an ephemeral range of 32768–60999 and a 60-second `TIME-WAIT`.

(a) What sustained rate of new outbound connections can it support?
(b) Recompute for a range of 10240–65535.
(c) It needs 2,000 connections per second to one server. Give three ways to achieve it.

**B4.** Read this output and answer the questions:

```
State      Recv-Q Send-Q Local Address:Port   Peer Address:Port
LISTEN     0      511    0.0.0.0:443          0.0.0.0:*
LISTEN     0      4096   127.0.0.1:5432       0.0.0.0:*
LISTEN     128    128    0.0.0.0:8080         0.0.0.0:*
ESTAB      0      82340  10.0.0.5:443         198.51.100.7:51234
ESTAB      65536  0      10.0.0.5:8080        198.51.100.9:44100
SYN-SENT   0      1      10.0.0.5:51299       203.0.113.50:443
CLOSE-WAIT 0      0      10.0.0.5:8080        198.51.100.3:39221
```

(a) Which services are reachable from the network?
(b) Which listener is in trouble, and what kind?
(c) What does the `Send-Q` of 82340 indicate, and whose problem is it?
(d) What does the `Recv-Q` of 65536 indicate, and whose problem is it?
(e) What does the `SYN-SENT` line mean?
(f) What does the `CLOSE-WAIT` line mean, and what happens if there are 5,000 of them?

**B5.** For each capture, identify which end is the client and which is the server, and
justify:

(a) `10.0.0.5:51234 → 203.0.113.10:443`
(b) `203.0.113.10:443 → 10.0.0.5:51234`
(c) `10.0.0.5:33001 → 10.0.0.9:3306`
(d) `10.0.0.5:60123 → 10.0.0.9:52001`

**B6.** Write the `ss` command for each:

(a) All listening TCP sockets with process names
(b) All established connections to or from port 443
(c) Count of sockets in `TIME-WAIT`
(d) All UDP sockets
(e) Connections to the `203.0.113.0/24` network

## C. Analyse

**C1.** Derive the requirement for ports from first principles, as §35.1 does. Then
explain why some must be fixed by convention and others need not be.

**C2.** Explain why TCP port 53 and UDP port 53 are different, using the demultiplexing
chain.

**C3.** "Five numbers, and half of Unit VIII follows from them." Show how the five-tuple
explains: many clients on one server port, NAT, stateful firewalls, and ECMP hashing.

**C4.** Explain why "the server ran out of ports" is almost always wrong, and what the
correct diagnosis usually is.

**C5.** Explain the privilege rule for ports below 1024: its original reasoning, why the
reasoning was social rather than technical, and why its security value is now minimal.

**C6.** Explain why `Recv-Q` and `Send-Q` on an established socket assign blame to
different parties. Give a scenario for each.

**C7.** Explain why running SSH on port 2222 has genuine operational value and no security
value.

**C8.** Explain why port-based firewall rules are weak, and what next-generation firewalls
do instead.

**C9.** `tcp_tw_recycle` was removed from Linux. Explain what it did, why it broke clients
behind NAT, and what the safe alternative is.

## D. Design

**D1.** Write the port-exposure policy for a web application server: what listens on what
address, what is reachable from where, and the justification for each.

**D2.** An API gateway must sustain 5,000 new outbound connections per second to a backend
pool. Design the solution, considering ephemeral range, connection reuse, and the tuple
arithmetic.

**D3.** For the semester project's network, list every service, its port and protocol, its
bind address, and which zones may reach it.

**D4.** Design a monitoring check that detects each of: a service that stopped listening,
a service that started listening on the wrong address, a backlog overflow, and a
`CLOSE-WAIT` leak.

## E. Troubleshoot

**E1.** A web application works when tested from the server itself and is unreachable from
the network. Give the one command that diagnoses it and the two most likely causes.

**E2.** A service fails to start with "Address already in use". `ss -tlnp | grep :8080`
shows nothing. Explain and give the command that finds it.

**E3.** A server's file-descriptor count grows steadily over hours until it stops accepting
connections. Give the socket state you would look for and what it means.

**E4.** Clients report intermittent connection timeouts to a busy service. The server's CPU
and memory are fine. `netstat -s` shows a growing "listen queue overflowed" counter.
Diagnose.

**E5.** A load generator fails with `EADDRNOTAVAIL` at about 450 connections per second.
Explain and give three fixes in order of preference.

**E6.** DNSSEC validation fails for one zone while ordinary DNS works. The firewall permits
UDP/53. Diagnose.

**E7.** Browsers report a site as working but slightly slow. Investigation shows HTTP/3 is
never used. Give the likely firewall cause.

**E8.** A security scan reports port 6379 open on a database server's public interface.
Explain the severity and the fix.

**E9.** Many sockets are in `SYN-SENT` to one destination. Everything else works. What
layer is the problem at, and what is your next command?

## F. Extend

**F1.** Run `ss -tlnp` on every machine you have access to. For each listener, decide
whether its bind address is correct. Document anything that surprises you.

**F2.** Write a script that reports: listener count, connections by state, `CLOSE-WAIT`
count, and listen-queue overflows. Run it as a monitoring check for a week.

**F3.** Deliberately exhaust the ephemeral range with a connection loop, observe the error,
then fix it three different ways and measure the achievable rate for each.

**F4.** Use `ss -tni` on a long-running connection and interpret every field: RTT,
congestion window, retransmissions and the algorithm. Compare against Chapter 38 once you
have read it.

**F5.** Write a small TCP server and client. Demonstrate: the five-tuple in `ss`, the
`accept()`-returns-a-new-socket behaviour, a `CLOSE-WAIT` leak by omitting `close()`, and
its fix.
