# Chapter 37 — Exercises

## A. Recall

**A1.** Give the three packets of the handshake with their flags, and say what each
establishes.

**A2.** Why is the acknowledgement `x+1` rather than `x`?

**A3.** Name the four options negotiated in the handshake and what each does.

**A4.** What does the acknowledgement number mean, precisely?

**A5.** How many duplicate ACKs trigger fast retransmit, and why that number?

**A6.** Give Jacobson's RTO formula and explain what each term contributes.

**A7.** State Karn's algorithm and the addition that makes it work.

**A8.** What is the maximum unscaled TCP window, and what throughput does it permit at
100 ms RTT?

**A9.** Which side enters TIME-WAIT, and for how long, and give the two reasons it exists.

**A10.** What does CLOSE-WAIT indicate, and whose fault is it?

## B. Apply

**B1.** A client sends SYN with `seq=1000000`. The server replies with SYN-ACK `seq=500000`.
Give the acknowledgement numbers in the SYN-ACK and in the third packet.

**B2.** Compute the maximum throughput of an unscaled TCP connection at RTT 5 ms, 25 ms,
80 ms and 250 ms.

**B3.** Compute the bandwidth-delay product, and hence the window needed, for:

(a) 1 Gb/s, 15 ms  (b) 10 Gb/s, 40 ms  (c) 100 Mb/s, 500 ms  (d) 40 Gb/s, 1 ms

**B4.** For each, give the window-scale factor needed and whether it is achievable:

(a) 1.25 MB  (b) 50 MB  (c) 200 MB  (d) 2 GB

**B5.** Given SRTT and RTTVAR, compute the RTO, then state whether Linux's 200 ms floor
would apply:

(a) 12 ms / 1 ms  (b) 90 ms / 30 ms  (c) 300 ms / 100 ms  (d) 2 ms / 0.5 ms

**B6.** A connection loses one segment. Compute the recovery time under: (a) fast
retransmit, (b) RTO with SRTT 40 ms and RTTVAR 5 ms on Linux, (c) the third successive
timeout.

**B7.** Decode this handshake and state everything you can conclude:

```
A > B: Flags [S], seq 100, win 64240, options [mss 1460,sackOK,TS val 1,nop,wscale 7]
B > A: Flags [S.], seq 900, ack 101, win 65160, options [mss 1380,sackOK,TS val 5,nop]
A > B: Flags [.], ack 901, win 502
```

**B8.** A receiver has received bytes 1–1000 and 2001–3000, and is missing 1001–2000.

(a) What acknowledgement number does it send?
(b) What does a SACK option add?
(c) Without SACK, what might the sender retransmit?

## C. Analyse

**C1.** Explain why the handshake needs exactly three packets — why two is insufficient and
four unnecessary.

**C2.** Explain the Mitnick attack step by step, why predictable ISNs enabled it, and how
RFC 6528 prevents it while preserving the old-duplicate protection.

**C3.** Explain why TCP numbers bytes rather than packets, and give a concrete case where
packet numbering would fail.

**C4.** Explain why cumulative acknowledgement cannot express a gap, what that costs, and
how SACK fixes it.

**C5.** Explain why three duplicate ACKs and not one. What phenomenon would one duplicate
confuse loss with?

**C6.** Explain why the RFC 793 RTT estimator was inadequate, using a path with SRTT 20 ms
and high variance. Show what Jacobson's version does differently.

**C7.** Explain Karn's ambiguity problem and why *both* halves of his solution are needed.

**C8.** Explain tail loss: why fast retransmit cannot help, why short transfers are mostly
tail, and what TLP and RACK each do about it.

**C9.** Derive the 64 KB wall and explain why a connection can be perfectly healthy and
achieve 5 Mb/s on a gigabit path.

**C10.** Explain why a zero window is definitively not a network problem.

**C11.** Explain the Nagle/delayed-ACK interaction completely, with a timeline, and give
two fixes ranked.

**C12.** Explain both reasons TIME-WAIT exists, and why reducing MSL is the wrong remedy.

**C13.** Explain why a server accumulating TIME-WAIT is worth investigating.

## D. Design

**D1.** A data-transfer service moves large files between two data centres, 10 Gb/s,
80 ms apart. Specify every TCP setting you would tune and justify each with arithmetic.

**D2.** An RPC service has 1 ms RTT and reports intermittent 200 ms latencies. Diagnose
from the description and write the fix, including what you would change in the application.

**D3.** Design the connection-management strategy for an API gateway making 5,000 outbound
requests per second to a backend pool. Address TIME-WAIT, ephemeral ports and keepalives.

**D4.** For the semester project's network, specify the TCP tuning you would apply to the
file server and justify it against the site's bandwidth and latency.

**D5.** Write the monitoring checks that would detect: a middlebox stripping window scale,
a CLOSE-WAIT leak, ephemeral exhaustion, and persistent zero windows.

## E. Troubleshoot

**E1.** A transfer between two sites achieves 5 Mb/s. Both have gigabit links, the path has
100 ms RTT, and there is no loss. Diagnose, and give the exact command that confirms it.

**E2.** A server's file-descriptor count grows by about 40 per hour until it stops
accepting connections after two days. Diagnose.

**E3.** An application makes one small write for headers and one for the body, and every
request takes 201 ms on a 1 ms LAN. Diagnose and give two fixes.

**E4.** A capture shows repeated identical segments with no duplicate ACKs preceding them.
What is happening, and what does it suggest?

**E5.** A long-lived connection dies after eight minutes of idleness, every time. Diagnose
and give the fix with specific values.

**E6.** `ss -tan` shows 15,000 TIME-WAIT on a web server and 200 on its clients. What is
unusual, and what would you check?

**E7.** Connections to a service fail immediately with "connection refused" from one
network and time out from another. Explain both.

**E8.** A file transfer stalls for exactly one second at a time, several times per
transfer. The path has 0.1% loss. Diagnose.

**E9.** Wireshark reports many "spurious retransmissions". Explain what this means and what
it suggests about the connection.

**E10.** A capture taken mid-connection shows windows of about 500 bytes and the connection
is fast. Explain.

## F. Extend

**F1.** Capture a complete TCP connection — handshake, data, teardown. Annotate every
packet with its flags, sequence and acknowledgement numbers, and verify the `+1` rules.

**F2.** Use `tc netem` to add 100 ms delay to loopback, then transfer a large file with
window scaling enabled and disabled (`sysctl net.ipv4.tcp_window_scaling`). Measure both
and compare with your answer to B2.

**F3.** Reproduce the Nagle/delayed-ACK stall: write a client that does two small writes
per request, measure the latency, then set `TCP_NODELAY` and measure again.

**F4.** Write a server that accepts connections and never closes them. Watch CLOSE-WAIT
grow with `ss`, find the descriptor limit, and observe the failure. Then fix it.

**F5.** Use `ss -tni` on a long-running transfer and record `cwnd`, `rtt` and `retrans`
every second. Plot them. Return to this after Chapter 38.

**F6.** Introduce 1% loss with `tc netem` and measure throughput with SACK enabled and
disabled (`sysctl net.ipv4.tcp_sack`). Quantify SACK's benefit.

**F7.** Read RFC 793's state diagram and RFC 9293's revision. Identify what changed in
thirty-nine years and why.
