# Chapter 66 — Exercises

## A. Recall

**A1.** Name the three independent performance quantities, how each is measured, and what fixes
each.

**A2.** Give the four components of delay and say which can be reduced.

**A3.** State the single-stream throughput bound and explain why adding bandwidth may not help.

**A4.** State the Mathis relationship and give its constant.

**A5.** Why is the average latency the wrong statistic for voice?

**A6.** What does `curl -w` decompose a request into, and what does a large `ttfb` with a small
`tcp_connect` indicate?

**A7.** Distinguish an error from a discard, and give the fix for each.

**A8.** Why does the presence of late collisions prove a duplex mismatch?

**A9.** Explain precisely how a forced/auto configuration produces a mismatch.

**A10.** Why does a duplex mismatch produce a performance symptom rather than a failure?

**A11.** Why is 802.3x flow control usually disabled, and what is the correct alternative where
it is needed?

**A12.** Why is a counter's absolute value nearly useless?

**A13.** State the one-line symptom that identifies an MTU problem.

**A14.** What is the MSS for a 1400-byte MTU over IPv4? Over IPv6?

**A15.** What does MSS clamping fix, and what does it not?

**A16.** Why is PMTUD mandatory rather than optional in IPv6?

**A17.** State the bufferbloat measurement in one sentence.

**A18.** Why does a speed test not detect bufferbloat?

**A19.** What does CoDel measure, and why is that different from what a tail-drop queue
measures?

**A20.** Why does AQM configured without a shaper achieve nothing?

## B. Apply

**B1.** For each report, state which of bandwidth, latency, loss or jitter is most likely, and
the first measurement:

(a) "Video calls break up"
(b) "Copying files to the server takes all afternoon"
(c) "The application takes ten seconds to open and then works fine"
(d) "Everything is fine until about 4 p.m."
(e) "It's slow from home and fine in the office"
(f) "The website takes three seconds to load"

**B2.** Using the Mathis relationship with MSS 1460 and $C = \sqrt{3/2}$, compute the maximum
single-stream throughput for:

(a) RTT 20 ms, loss 0.01%
(b) RTT 20 ms, loss 0.1%
(c) RTT 80 ms, loss 0.01%
(d) RTT 80 ms, loss 1%

Then state what upgrading a 100 Mb/s circuit to 1 Gb/s would achieve in each case.

**B3.** Interpret this `curl -w` output and state where the time is going:

```
   dns_lookup:     5.012
   tcp_connect:    5.096
   tls_handshake:  5.271
   ttfb:           5.402
   total:          5.680
```

**B4.** For each counter reading, give the diagnosis:

(a) 42,000 CRC errors, 0 collisions, 0 late collisions
(b) 3,100 CRC errors on one side; 2,800 late collisions on the other
(c) 0 errors, 180,000 output discards, average utilisation 28%
(d) 0 errors, 4,000 input discards, utilisation 4%
(e) 91,000 giants on one interface
(f) Pause frames incrementing on an access switch uplink

**B5.** A path has an unknown MTU. `ping -M do -s 1472` fails and `-s 1272` succeeds.

(a) Give the bisection sequence to find the exact value in at most five further tests.
(b) The answer is 1420. What encapsulation is most likely?
(c) Give the MSS clamp value you would configure.
(d) State what would still be broken after clamping.

**B6.** A branch has a 50 Mb/s circuit. Idle latency to the data centre is 14 ms. During a file
upload it rises to 680 ms.

(a) Name the fault.
(b) Estimate the buffer size in bytes that would produce this.
(c) Give the configuration you would apply and the value you would use.
(d) State what you would measure afterwards to confirm.

**B7.** For each, state whether the fault is bandwidth, an MTU problem, a duplex mismatch or
bufferbloat, and give the confirming test:

(a) `ping` works; `ssh` connects and freezes on `ls -l` of a large directory
(b) A speed test reports 940 Mb/s; video calls are unusable during backups
(c) A file server transfer runs at 3 Mb/s on a Gigabit LAN with everything else normal
(d) A 1 Gb/s WAN link is saturated between 09:00 and 10:00 every weekday
(e) A web page's HTML renders and no images appear, over the VPN only

**B8.** A monitoring system pings the WAN gateway every 60 seconds and reports 12 ms average
with no alerts. Users report severe interactive problems each afternoon.

(a) Explain how both can be true.
(b) Design the measurement that would detect the fault.
(c) Design the alert, in Chapter 54 §54.4's terms.

## C. Analyse

**C1.** The chapter argues the three quantities are independent and that adding bandwidth
addresses only one. Analyse why bandwidth is nonetheless the reflexive response, and what a
network team should do to change the conversation.

**C2.** Analyse the Mathis relationship's implications for a wide-area file transfer service.
What would you change about the application, the transport and the network, in that order?

**C3.** Duplex mismatch produces a fault where small transfers work perfectly. Analyse why this
delays diagnosis, and identify two other faults in this book with the same property.

**C4.** Analyse 802.3x flow control as a design. What problem was it solving, why does it
propagate congestion, and what does priority flow control change?

**C5.** MSS clamping fixes TCP and not UDP, and UDP-based protocols are increasingly dominant
(Chapter 38 §38.4). Analyse what this means for MTU management over the next decade.

**C6.** Analyse bufferbloat as an instance of a general pattern: a locally reasonable decision
producing a globally harmful outcome. Identify the incentives that produced it and two other
examples in this book.

**C7.** The chapter notes that bufferbloat is not examinable and is the fault you are most likely
to find and fix. Analyse this gap between certification content and practice. What causes it,
and what should a practitioner do about it?

**C8.** FQ-CoDel requires no classification and frequently outperforms a hand-built DSCP policy
(Chapter 52 §52.2). Analyse what a classification-based policy still offers, and whether it is
worth the effort at an edge link.

## D. Design

**D1.** Design the performance monitoring for a 30-site organisation: what is measured, at what
interval, from where, what percentiles are recorded, and what alerts exist. It must be capable
of detecting bufferbloat, microbursts and MTU problems before users report them.

**D2.** Design the standard performance investigation procedure: given "it's slow", the sequence
of questions and measurements that determines which of the three quantities is at fault within
ten minutes, executable by someone who has not read this chapter.

**D3.** An organisation's branch links all exhibit severe bufferbloat. Design the remediation
programme: what you would deploy, in what order, how you would measure the improvement, and how
you would present the result to management given that "throughput is unchanged".

**D4.** Design the MTU strategy for a network with IPsec over broadband, IPsec over PPPoE, and
VXLAN in the data centre. Specify the values, where each is configured, how MSS clamping is
applied, what is done about UDP, and the monitoring that would detect a regression.

**D5.** Write the one-page explanation of bufferbloat you would give to a non-technical manager
who has been told the solution is a faster circuit. It must be persuasive without being
condescending and must include a measurement they could repeat.

## E. Troubleshoot

**E1.** A 1 Gb/s WAN circuit was upgraded from 100 Mb/s and transfer times are unchanged.
Diagnose, and give the two measurements that would settle it.

**E2.** SSH to a server works and hangs the moment output exceeds a screen. Diagnose in one
command and state the fix.

**E3.** A file server delivers 4 Mb/s to one workstation and 900 Mb/s to another on the same
switch. Diagnose.

**E4.** A speed test shows full rate and users say the network is unusable during the working
day. Diagnose.

**E5.** A web application is slow for users at one site only. `ping` shows 9 ms and `iperf3`
shows 940 Mb/s. Give your next step and what it would tell you.

**E6.** After enabling jumbo frames on a storage network, some transfers fail entirely and others
work. Diagnose.

**E7.** A VPN carries interactive traffic acceptably and file transfers fail. Diagnose, and state
which chapter's material applies.

**E8.** Output discards are incrementing on a link whose five-minute average utilisation is 22%.
Explain and give the two possible responses.

**E9.** A remote worker's video calls fail whenever their household is uploading. They have a
100/20 Mb/s connection. Diagnose and give a fix they could apply themselves.

**E10.** CAKE is configured on a branch router and loaded latency is unchanged. Give three
reasons.

## F. Extend

**F1.** Measure bufferbloat on your own connection: idle latency, then latency while saturating
the uplink, then the downlink. Record the maximum in each case and grade it against §66.4's
table.

**F2.** Apply CAKE or FQ-CoDel with an appropriate shaper on a Linux router or OpenWrt device and
repeat F1. Report the before-and-after figures and the shaper value you used, and explain how you
chose it.

**F3.** Create a duplex mismatch deliberately in a lab, measure the throughput, and record the
counters on both sides. Then fix it and measure again. Report the ratio.

**F4.** Create an MTU black hole (a tunnel with ICMP Type 3 Code 4 filtered), reproduce each
symptom in §66.3's catalogue, and capture the retransmission signature. Then apply MSS clamping
and confirm.

**F5.** Verify the Mathis relationship experimentally: use `tc netem` to introduce known loss
rates on a link with a known RTT, measure single-stream TCP throughput at each, and plot the
result against the formula.

**F6.** Instrument a link with one-second and five-minute utilisation sampling simultaneously for
a week. Identify every microburst visible in the first and invisible in the second, and correlate
with the discard counter.

**F7.** Take a real "it's slow" report and work it through §66.1's method, documenting each
measurement and what it eliminated. Report how long the diagnosis took and which measurement was
decisive.

**F8.** Read the CAKE paper (Høiland-Jørgensen et al., 2018) and identify the three things CAKE
does that FQ-CoDel does not. Determine which of them apply to a link you administer.
