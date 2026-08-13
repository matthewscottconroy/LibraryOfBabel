# Chapter 38 — Further Reading

## Primary sources

**Jacobson, V. (1988). "Congestion Avoidance and Control." *ACM SIGCOMM*.**
**Read this one properly.** Sixteen pages, and among the most consequential papers in
computing. The measurements and plots are as instructive as the mechanisms — **it is a
demonstration of how to diagnose a system failing in a way nobody has a name for.**

**Chiu, D.-M. & Jain, R. (1989). "Analysis of the Increase and Decrease Algorithms for
Congestion Avoidance in Computer Networks." *Computer Networks and ISDN Systems*.**
**The proof that AIMD converges** and the other three combinations do not. The vector
diagram of §38.2 is theirs, and the argument is accessible without heavy mathematics.

**Mathis, M., Semke, J., Mahdavi, J. & Ott, T. (1997). "The Macroscopic Behavior of the TCP
Congestion Avoidance Algorithm." *ACM CCR*.**
The equation. Short, and **it is the paper that turns "loss hurts" into a number you can
put in a ticket.**

**RFC 5681 — Allman, M., Paxson, V. & Blanton, E. (2009). *TCP Congestion Control.***
Slow start, congestion avoidance, fast retransmit and fast recovery, specified normatively.
**The document to cite when someone claims a stack is misbehaving.**

**RFC 8312 — Rhee, I., Xu, L., Ha, S. et al. (2018). *CUBIC for Fast Long-Distance
Networks.***
CUBIC, standardised twelve years after it became Linux's default — which is itself a
comment on how deployment and standardisation relate.

**Cardwell, N., Cheng, Y., Gunn, C. S., Yeganeh, S. H. & Jacobson, V. (2016). "BBR:
Congestion-Based Congestion Control." *ACM Queue*.**
**The reframing**, with Google's production measurements. Read §2 for the argument that
loss-based algorithms operate at maximum queueing delay by construction.

**RFC 3168 — Ramakrishnan, K., Floyd, S. & Black, D. (2001). *The Addition of Explicit
Congestion Notification to IP.***
ECN. Worth reading alongside **RFC 8311** (2018), which added the fallback behaviour that
finally made deployment safe — **seventeen years later.**

**RFC 8257 — Bensley, S. et al. (2017). *Data Center TCP (DCTCP).***
Proportional response to ECN marking, and the argument for why a data centre can do things
the Internet cannot.

**RFC 9000 — Iyengar, J. & Thomson, M. (2021). *QUIC: A UDP-Based Multiplexed and Secure
Transport.***
The specification. **Read §1–3 for the design rationale** — it states the ossification
argument in the authors' own words. §14 on datagram size implements Chapter 36 §36.4's rule
as a protocol requirement.

**RFC 9114 — Bishop, M. (2022). *HTTP/3.***
HTTP over QUIC, and **RFC 9204** for QPACK, which exists because HPACK assumed ordered
delivery.

**Honda, M. et al. (2011). "Is it Still Possible to Extend TCP?" *ACM IMC*.**
**The measurement study behind QUIC's existence.** Without it, "TCP cannot be extended" is
an assertion; with it, it is a measurement.

**Nichols, K. & Jacobson, V. (2012). "Controlling Queue Delay." *ACM Queue*.**
CoDel, and the argument that **queue length is the wrong metric** — persistent delay is
what matters.

**Gettys, J. & Nichols, K. (2011). "Bufferbloat: Dark Buffers in the Internet." *ACM
Queue*.**
The problem named.

## Books

**Fall, K. & Stevens, W. R. (2011). *TCP/IP Illustrated, Volume 1*, 2nd ed.,
chapter 16.**
Congestion control with captures, covering the modern algorithms. The best textbook
treatment.

**Peterson, L. & Davie, B. *Computer Networks: A Systems Approach*, chapter 6.**
Congestion control and resource allocation built up from principles, with the AIMD
convergence argument worked. Freely available online.

**Kurose, J. & Ross, K. *Computer Networking*, chapter 3.**
The clearest introductory treatment, with the sawtooth and fairness developed carefully.

**Grigorik, I. (2013). *High Performance Browser Networking.* O'Reilly.**
**Freely available at hpbn.co.** The application-facing view: how slow start, the initial
window and head-of-line blocking actually affect page-load time. **The best explanation of
why these mechanisms matter to anyone building on top of them**, and the HTTP/2 chapters
explain the problem QUIC was built to solve.

## Applied

**Watch the sawtooth.** Exercise F2, and it makes the whole chapter concrete:

```bash
# In one terminal, a long transfer
iperf3 -c <host> -t 60
# In another
while true; do ss -tni "dst <host>" | grep -o 'cwnd:[0-9]*'; sleep 0.1; done
```

**Plot it.** The exponential rise, the linear phase, and the sharp halvings are all visible.

**Then repeat with BBR** (exercise F3) and compare the shapes. **BBR does not produce a
sawtooth**, and seeing that difference explains §38.3 better than any description.

**Build a path with `tc netem`** and measure the Mathis prediction against reality:

```bash
tc qdisc add dev lo root netem delay 50ms 5ms loss 0.1%
iperf3 -c 127.0.0.1 -t 30
# then compare with:
python3 tools/perfcalc.py loss --rtt 100 --loss 0.001
```

**Compare algorithms on the same impaired path:**

```bash
for cc in reno cubic bbr; do
  sysctl -w net.ipv4.tcp_congestion_control=$cc
  iperf3 -c <host> -t 20 | tail -3
done
```

**Measure bufferbloat** (exercise F4) — the most visceral demonstration in the chapter:

```bash
ping <gateway>                      # baseline latency
# now saturate the uplink in another terminal
iperf3 -c <host> -t 60
# watch the ping latency
```

**On a domestic connection the latency often rises from 15 ms to several hundred**, and
enabling `fq_codel` on the router brings it back. **Nothing else in this book produces such
a large improvement from one configuration change.**

**`tools/perfcalc.py`** in this book computes the Mathis relation, the BDP and the window
requirement — use `perfcalc.py loss` and `perfcalc.py bdp` to check the arithmetic in your
own network's terms.

**Capture QUIC** (exercise F5). Visit any Google or Cloudflare property with a modern
browser and filter on `quic` in Wireshark. **Notice how little is readable.** Then set
`SSLKEYLOGFILE` and decrypt, and compare.

**`qlog`** — QUIC's structured endpoint logging, and the replacement for packet capture as
a debugging tool. **qvis** (qvis.quictools.info) visualises it.

**Lab 27** in this book's [labs/](../../../labs/) directory builds an impaired path,
measures Reno, CUBIC and BBR against the Mathis prediction, plots `cwnd` over time for
each, and then demonstrates bufferbloat and its remedy.

## For the certification-minded

Objective 2.2 touches congestion control; objective 5.4 covers performance problems.
**The mechanisms are not examined in depth** — the concepts appear in troubleshooting
scenarios.

Six things worth over-learning:

1. **TCP infers congestion from packet loss.**
2. **Slow start is exponential; congestion avoidance is linear.**
3. **AIMD — additive increase, multiplicative decrease.**
4. **Flow control protects the receiver; congestion control protects the network.**
5. **QUIC runs over UDP/443 and HTTP/3 is HTTP over QUIC.**
6. **Blocking UDP/443 blocks HTTP/3**, and the fallback to TCP hides it.

And the two things worth far more than the objective for real work:

**The Mathis relation.** When someone says "the link is only 30% utilised so the network is
fine", the answer is an arithmetic one, and having it ready ends the discussion. **0.01%
loss caps a 100 ms flow at 14 Mb/s regardless of link capacity.**

**Bufferbloat.** High latency under load with plenty of bandwidth is a queue, not a
capacity problem, and `fq_codel` fixes it. Measure it on your own connection once; you will
find it.
