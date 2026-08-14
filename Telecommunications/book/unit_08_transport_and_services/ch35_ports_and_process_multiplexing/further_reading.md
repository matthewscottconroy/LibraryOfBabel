# Chapter 35 — Further Reading

## Primary sources

RFC 6335 — Cotton, M., Eggert, L., Touch, J., Westerlund, M. & Cheshire, S. (2011).
*Internet Assigned Numbers Authority (IANA) Procedures for the Management of the Service
Name and Transport Protocol Port Number Registry.*
The three ranges of §35.3, stated normatively — thirty years after they came into use.
Also the assignment procedures, which are worth skimming to see how a namespace is
actually governed.

The IANA Service Name and Transport Protocol Port Number Registry
(iana.org/assignments/service-names-port-numbers).
**The authoritative list.** Consult it rather than a blog post when you need to know what
a port is.

RFC 793 (TCP) §3.2 and RFC 768 (UDP).
Where the port fields are specified. RFC 768 is three pages and the port fields are
most of it.

RFC 1700 — Postel, J. & Reynolds, J. (1994). *Assigned Numbers.*
Historical, superseded by the online registries, and worth looking at once to see the
scale of what one person was maintaining.

RFC 6056 — Larsen, M. & Gont, F. (2011). *Recommendations for Transport-Protocol Port
Randomization.*
Why ephemeral ports should be unpredictable, and the attacks that predictable ones enable.
Relevant to Chapter 39's DNS cache poisoning.

## Books

Stevens, W. R., Fenner, B. & Rudoff, A. (2003). *UNIX Network Programming, Volume 1*,
3rd ed.
The reference for §35.2. Sockets, binding, `accept()`, the states, and every corner
case. If you will write network code, this is the book; if you will only operate networks,
chapters 2–4 still repay reading because they explain *why* the socket table looks as it
does.

Stevens, W. R. (1994). *TCP/IP Illustrated, Volume 1*, chapters 10–11 and 17–18.
Ports and the transport headers, with captures.

Kerrisk, M. (2010). *The Linux Programming Interface*, chapters 56–61.
Sockets on Linux specifically, including `SO_REUSEADDR`, `SO_REUSEPORT`, and the
file-descriptor accounting that makes `CLOSE-WAIT` leaks fatal.

Gregg, B. (2020). *Systems Performance*, 2nd ed., chapter 10.
Network performance from the host's side. The `ss -tni` fields of §35.4, interpreted
properly, and the methodology for deciding whether a problem is the application, the
kernel, or the network.

## Applied

`ss -tlnp` on every machine you can reach. Exercise F1, and the most valuable thing in
this list. For each listener, ask whether its bind address is right. Most people find
something they did not expect — a service listening publicly that should be local, or a
process nobody remembers starting.

**`ss -tni` on a long-running connection.** RTT, congestion window, retransmissions, and
the algorithm. Come back to it after Chapter 38 and read it again; it will mean more.

**`lsof -i :PORT`** — *"what has this port open?"* Portable, and the answer when `ss -p`
is unavailable or you are on a system you do not know.

**`ss -s`** for a summary, and `watch -n1 'ss -s'` during a load test. Watching
`TIME-WAIT` climb and plateau makes §35.4's arithmetic concrete.

Exhaust the ephemeral range deliberately (exercise F3):

```bash
# A tight connection loop against a local service
for i in $(seq 1 100000); do curl -s -o /dev/null http://localhost:8080/ & done
# then watch
watch -n1 "ss -tan state time-wait | wc -l"
sysctl net.ipv4.ip_local_port_range
```

Then fix it three ways — keep-alive, a wider range, `tcp_tw_reuse` — and measure the
achievable rate for each. The numbers you produce will stay with you.

Write the CLOSE-WAIT leak (exercise F5). A ten-line server that accepts and never
closes, a client that connects and disconnects in a loop, and `ss -tan state close-wait |
wc -l` climbing. Then add the `close()` and watch it stop. Fifteen minutes, and you
will recognise the pattern in production for the rest of your career.

**`nmap -sV`** against a host you own, and compare with `nmap -sT`. The first probes; the
second infers from the port number. The difference is §35.3's point about convention not
being evidence.

**Lab 24** in this book's [labs/](../../../labs/) directory builds a small server, exposes
it on the wrong bind address, diagnoses it with `ss`, then reproduces a backlog overflow
and a `CLOSE-WAIT` leak and diagnoses each from the socket table alone.

## For the certification-minded

Objective 1.4 is ports and protocols, and it is among the most heavily examined
objectives on the test. Objective 5.5 expects `netstat`.

The port table is the single largest piece of pure memorisation in the certification.
Learn it cold:

| | | | |
|---|---|---|---|
| **20/21** FTP | **22** SSH | **23** Telnet | **25** SMTP |
| **53** DNS (TCP+UDP) | **67/68** DHCP | **69** TFTP | **80** HTTP |
| **110** POP3 | **123** NTP | **143** IMAP | **161/162** SNMP |
| **389** LDAP | **443** HTTPS | **445** SMB | **514** syslog |
| **636** LDAPS | **993** IMAPS | **995** POP3S | **3389** RDP |

**Appendix B has the extended table.**

Five more things worth over-learning:

1. **The three ranges**: 0–1023 well-known, 1024–49151 registered, 49152–65535 ephemeral.
2. An IP address identifies a host; a port identifies a process.
3. DNS uses both TCP and UDP on 53.
4. TCP and UDP port spaces are separate.
5. `ss -tlnp` / `netstat -tlnp` shows what is listening.

And the two operational habits worth more than the memorisation:

When a service works locally and not remotely, check the bind address before anything
else. `127.0.0.1` versus `0.0.0.0` accounts for an enormous share of these, and it is one
line of configuration.

When sockets accumulate in `CLOSE-WAIT`, it is a code defect, not a network problem.
Knowing this saves you from debugging the network for a day.
