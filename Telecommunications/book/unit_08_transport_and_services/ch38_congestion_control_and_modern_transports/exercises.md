# Chapter 38 — Exercises

## A. Recall

**A1.** Describe congestion collapse in five steps, and state why it is stable rather than
self-correcting.

**A2.** What did TCP have in 1981 that protected the receiver, and what did it lack that
protected the network?

**A3.** State the conservation-of-packets principle and explain what makes TCP
self-clocking.

**A4.** Give the growth rule for slow start and for congestion avoidance.

**A5.** State AIMD and explain, in one sentence each, what additive increase and
multiplicative decrease do to the difference between two flows' windows.

**A6.** Give the Mathis equation and name the two counter-intuitive consequences.

**A7.** What two quantities does BBR measure, and what does it aim for?

**A8.** What do the four ECN codepoints mean, and which is set by the router?

**A9.** Give three reasons TCP could not be changed, and how QUIC answers each.

## B. Apply

**B1.** A connection starts with an initial window of 10 segments on a path with 40 ms RTT.
Give `cwnd` after each of the first 8 round trips, and the elapsed time to reach 1 MB in
flight.

**B2.** Using the Mathis equation with MSS 1460 and C = 1.22, compute the single-stream
ceiling for:

(a) RTT 20 ms, loss 0.01%   (b) RTT 20 ms, loss 1%
(c) RTT 200 ms, loss 0.01%  (d) RTT 200 ms, loss 1%

State what the four numbers show about the two variables.

**B3.** A 40 Gb/s path has 60 ms RTT. Compute the loss rate classic TCP would need to fill
it, and express it as "one loss in *n* packets".

**B4.** Two flows share a bottleneck. Flow A has 10 ms RTT, flow B has 100 ms. Both use
Reno. Predict the ratio of their throughputs and explain the mechanism.

**B5.** A link carries a single TCP flow whose window oscillates between 800 KB and
400 KB. Compute average utilisation and explain why it is not 100%.

**B6.** For each scenario, choose a congestion-control algorithm and justify:

(a) A CDN serving mobile users on lossy cellular links
(b) A data centre with ECN-capable switches throughout
(c) A general-purpose Linux server on a corporate LAN
(d) A backup job between two data centres, 10 Gb/s, 80 ms apart, 0.001% loss

**B7.** Compare round trips to first application byte for: HTTP/1.1 over TCP+TLS 1.2,
HTTP/2 over TCP+TLS 1.3, HTTP/3 first connection, HTTP/3 resumed. Then compute the
wall-clock difference on a 150 ms path.

## C. Analyse

**C1.** Explain why congestion collapse does not resolve itself when offered load falls
slightly.

**C2.** Explain the conservation-of-packets principle and why self-clocking makes a sender
transmit at the right rate without knowing what the right rate is.

**C3.** Prove informally that AIMD converges to fairness and that MIAD does not. Use the
two-flow diagram.

**C4.** Explain why a single TCP flow averages about 75% utilisation, and why this is a
feature rather than a defect.

**C5.** "TCP needs loss." Explain, and connect it to Chapter 24 §24.1's claim that a
network with no loss is underutilised.

**C6.** Explain why CUBIC's time-based growth removes TCP's bias against long paths, with
a worked comparison against Reno on a 200 ms path.

**C7.** "Loss is not congestion; loss is what happens after congestion has already filled a
queue." Explain what this implies about where loss-based algorithms operate, and what BBR
does differently.

**C8.** Explain why BtlBw and RTprop cannot be measured at the same time, and what BBR
does about it.

**C9.** ECN was specified in 2001 and deployed after 2015. Give three reasons for the
delay and identify the general phenomenon.

**C10.** Explain head-of-line blocking in HTTP/2 over TCP, why HTTP/2 was measurably worse
than HTTP/1.1 on lossy paths, and how QUIC fixes it.

**C11.** QUIC encrypts nearly all of its header. Give the security reason and the
architectural reason, and state what operators lose.

**C12.** QUIC reached 25% of web traffic in five years; IPv6 took thirty to reach 50%.
Explain the difference in terms of who benefits from deployment.

## D. Design

**D1.** A video-streaming service serves users on mobile networks with 1–3% random loss.
Specify the transport and congestion control, and justify with the Mathis arithmetic.

**D2.** Design the congestion-control and queueing strategy for a data centre fabric.
Specify the algorithm, the ECN configuration, and what problem each addresses.

**D3.** An organisation is considering enabling HTTP/3. Write the assessment: what improves,
what costs more, what monitoring must change, and what could break.

**D4.** For the semester project's network, specify the firewall rules needed to permit
HTTP/3 and explain what happens if they are omitted.

**D5.** Write the tuning plan for a server that must sustain 10 Gb/s to clients 100 ms
away. Include window sizes, congestion control, and what you would measure.

## E. Troubleshoot

**E1.** A 10 Gb/s inter-site link carries backups at 90 Mb/s. Utilisation is 4% and
measured loss is 0.05%. Explain quantitatively and give two remedies.

**E2.** Users report high latency during large downloads on an otherwise idle link.
Diagnose, name the phenomenon, and give two fixes at different layers.

**E3.** After switching a server to BBR, throughput improved and a colleague reports their
CUBIC transfers on the same link got slower. Explain.

**E4.** A transfer over a satellite link achieves 2 Mb/s despite a 50 Mb/s circuit. Loss is
0.3%, RTT 600 ms. Compute the expected ceiling and state whether the observation is
consistent.

**E5.** ECN was enabled on a fleet of servers and a small percentage of destinations became
unreachable. Explain and give the modern mitigation.

**E6.** HTTP/3 is enabled on a website and analytics show almost no HTTP/3 usage from one
large customer's network. Diagnose.

**E7.** After a QUIC migration, the security team reports they can no longer classify
traffic. Explain what changed and what the options are.

**E8.** An application using 0-RTT reports duplicate database records. Explain the
mechanism and whose fault it is.

## F. Extend

**F1.** Use `tc netem` to build a 100 ms path with 0.1% loss on loopback. Measure
single-stream throughput with Reno, CUBIC and BBR. Compare with the Mathis prediction and
explain any divergence.

**F2.** Record `cwnd` every 100 ms during a long transfer with `ss -tni` and plot it.
Identify slow start, congestion avoidance, and each loss event.

**F3.** Repeat F2 with BBR and compare the shapes. Explain the difference in terms of what
each algorithm is optimising.

**F4.** Measure the queueing delay under load: `ping` a host while saturating the link,
first with the default queue and then with `fq_codel`. Quantify bufferbloat.

**F5.** Capture a QUIC connection. Identify what is visible and what is encrypted. Then
enable `SSLKEYLOGFILE` and decrypt it in Wireshark, and compare what you can see.

**F6.** Read Jacobson (1988) in full. Identify which of its seven mechanisms are still
present unchanged in a modern stack.

**F7.** Find the current HTTP/3 adoption figures and compare with IPv6's trajectory over an
equivalent period. Support or challenge §38.4's explanation.
