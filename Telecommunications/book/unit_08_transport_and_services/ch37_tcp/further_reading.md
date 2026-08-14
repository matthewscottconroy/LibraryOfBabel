# Chapter 37 — Further Reading

## Primary sources

**RFC 9293 — Eddy, W. (2022). *Transmission Control Protocol (TCP).***
**The current specification**, consolidating RFC 793 and forty years of errata and
extensions. **Read §3.3 (sequence numbers), §3.5 (establishing and closing) and §3.8
(state machine).** Note how little has changed.

**RFC 793 — Postel, J. (1981).**
The original, for the state diagram and the prose. Worth reading alongside 9293 to see what
forty-one years did and did not alter.

**Jacobson, V. (1988). "Congestion Avoidance and Control." *ACM SIGCOMM*.**
**Read this.** The RTT estimator of §37.3 is in the appendix; the congestion control is
Chapter 38's. Among the most consequential papers in computing, and it is readable.

**Karn, P. & Partridge, C. (1987). "Improving Round-Trip Time Estimates in Reliable
Transport Protocols." *ACM SIGCOMM*.**
Karn's algorithm, and a good short example of a paper whose contribution is a *refusal* —
declining to take an ambiguous measurement.

**RFC 7323 — Borman, D., Braden, B., Jacobson, V. & Scheffenegger, R. (2014). *TCP
Extensions for High Performance.***
**Window scaling, timestamps and PAWS** — §37.2 and §37.4. The fix for a field sized in
1981.

**RFC 2018 — Mathis, M., Mahdavi, J., Floyd, S. & Romanow, A. (1996). *TCP Selective
Acknowledgment Options.***
SACK. Short, and the mechanism is obvious once stated — which is the mark of a good
extension.

RFC 5681 — Allman, M., Paxson, V. & Blanton, E. (2009). *TCP Congestion Control.*
Fast retransmit and fast recovery, specified. (The congestion control proper is Chapter 38.)

**RFC 8985 — Cheng, Y., Cardwell, N., Dukkipati, N. & Jha, P. (2021). *The RACK-TLP Loss
Detection Algorithm for TCP.***
**The modern replacement for duplicate-ACK counting.** Read §2 for the motivation — the
measured impact of tail loss is the interesting part.

RFC 6528 — Gont, F. & Bellovin, S. (2012). *Defending against Sequence Number Attacks.*
The ISN generation of §37.1, **twenty-three years after Bellovin described the
vulnerability.**

**RFC 6298 — Paxson, V., Allman, M., Chu, J. & Sargent, M. (2011). *Computing TCP's
Retransmission Timer.***
Jacobson's estimator, specified normatively, including the 1-second minimum.

RFC 896 — Nagle, J. (1984). *Congestion Control in IP/TCP Internetworks.*
Nagle's algorithm, and the congestion problem as it appeared before Jacobson.

RFC 813 — Clark, D. (1982). *Window and Acknowledgement Strategy in TCP.*
Silly window syndrome and the receiver-side fix.

## Books

**Stevens, W. R. (1994). *TCP/IP Illustrated, Volume 1*, chapters 17–24.**
The reference for this chapter, and probably the best technical exposition in the field.
Every mechanism, with real captures, explained by someone who clearly enjoyed it. Chapters
21 (timeout and retransmission) and 22 (persist timer) are the ones to read twice.

**Fall, K. & Stevens, W. R. (2011). *TCP/IP Illustrated, Volume 1*, 2nd ed.,
chapters 12–17.**
The revision. Covers SACK, window scaling, modern loss recovery and the state of things
after another seventeen years.

**Wright, G. & Stevens, W. R. (1995). *TCP/IP Illustrated, Volume 2: The
Implementation*.**
The 4.4BSD source, annotated line by line. If you want to know what actually happens,
this is it — and much of the code is recognisable in every derivative stack since.

**Stevens, W. R., Fenner, B. & Rudoff, A. (2003). *UNIX Network Programming, Volume 1*,
3rd ed.**
The application side: `TCP_NODELAY`, `SO_LINGER`, `SO_REUSEADDR`, and the socket options
that produce §37.4 and §37.5's behaviours.

Peterson, L. & Davie, B. *Computer Networks: A Systems Approach*, chapter 5.
Good on *why* the mechanisms are as they are, with the sliding window built up from first
principles. Freely available online.

## Applied

**Capture a complete connection** (exercise F1). Handshake, data, teardown. **Annotate every
packet by hand and verify the `+1` rules for SYN and FIN. This is the most
valuable exercise in the chapter**, and it takes twenty minutes.

```bash
tcpdump -nn -S -i any 'host example.com and port 443' -w /tmp/cap.pcap
```

**Use `-S`** for absolute sequence numbers; relative ones are friendlier and hide what you
are trying to learn.

**Wireshark: Statistics → TCP Stream Graphs → Time Sequence (tcptrace).** The clearest
visualisation of a connection's behaviour that exists — window, in-flight data,
retransmissions and stalls, all on one plot. **Learn to read it.**

**`ss -tni`**, repeatedly, during a transfer:

```bash
watch -n1 "ss -tni 'dst 203.0.113.10'"
```

**`cwnd`, `rtt`, `retrans` and `rto` changing in real time.** Come back to this after
Chapter 38.

**Reproduce the 64 KB wall** (exercise F2):

```bash
tc qdisc add dev lo root netem delay 50ms
sysctl -w net.ipv4.tcp_window_scaling=0
# transfer a large file over loopback, measure
sysctl -w net.ipv4.tcp_window_scaling=1
# measure again
tc qdisc del dev lo root
```

The two numbers you produce are §37.4's argument, and having produced them yourself you
will never again mistake this fault for a network problem.

**Reproduce the Nagle stall** (exercise F3). Two small writes per request, measure the
latency, set `TCP_NODELAY`, measure again. **The 200 ms is unmistakable.**

**Reproduce the CLOSE-WAIT leak** (exercise F4). A server that accepts and never closes,
`ss -tan state close-wait | wc -l` climbing, `ulimit -n` eventually reached. **Fifteen
minutes, and you will recognise it in production forever.**

**`tc netem` generally** — loss, delay, reordering, duplication. Everything in this chapter
can be provoked on a single machine:

```bash
tc qdisc add dev lo root netem loss 1% delay 100ms 10ms reorder 5%
```

**Lab 26** in this book's [labs/](../../../labs/) directory works through a captured
connection field by field, then uses `netem` to provoke fast retransmit, an RTO, a zero
window and a tail-loss stall, requiring each to be identified from the capture alone.

## For the certification-minded

Objective 1.4 expects TCP, the three-way handshake, flow control and the comparison with
UDP. The handshake is examined directly and frequently.

Eight things worth over-learning:

1. **SYN, SYN-ACK, ACK** — and that **SYN consumes a sequence number**, hence the `+1`.
2. The acknowledgement number is the next byte expected, not the last byte received.
3. **Three duplicate ACKs trigger fast retransmit.**
4. **Flow control protects the receiver; congestion control protects the network.**
5. The receiver advertises a window in every ACK, and **throughput ≤ window ÷ RTT**.
6. **Teardown is four packets** — FIN, ACK, FIN, ACK — and **FIN also consumes a sequence
   number**.
7. **RST means abort**; after a SYN it means **nothing is listening**.
8. **The states you will see**: LISTEN, SYN-SENT, SYN-RECV, ESTABLISHED, FIN-WAIT,
   CLOSE-WAIT, TIME-WAIT.

Expect a packet-sequence question requiring you to fill in acknowledgement numbers, and a
"what does this state mean" item.

And the four operational facts worth more than the objective:

5 Mb/s on a fast long path with no loss is window scaling missing. Capture the SYN.

**A zero window is the receiving application, not the network.**

**CLOSE-WAIT accumulating is a missing `close()` in code.**

A consistent 200 ms per transaction is Nagle meeting delayed ACK. Set `TCP_NODELAY`
or combine the writes.
