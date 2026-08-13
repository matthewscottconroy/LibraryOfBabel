# Chapter 3 — Exercises

## A. Recall

**A1.** A link is advertised as 250 Mb/s. Express its capacity in megabytes per
second. How long, at capacity, to transfer a 4 GB file? (State which GB you mean.)

**A2.** For each, state which of the four latency components dominates:
(a) a 40,000 km GEO satellite hop; (b) a 1,500-byte frame on a 56 kb/s modem;
(c) a packet crossing a router whose CPU is at 98%; (d) a packet crossing a
data-centre switch at 3 a.m.; (e) a packet crossing the same switch during a
backup window.

**A3.** Define jitter without using the word "latency." Then explain why a jitter
buffer cannot reduce jitter to zero without cost.

**A4.** A `ping` reports `rtt min/avg/max/mdev = 12.1/12.4/13.0/0.2 ms`. A second
reports `min/avg/max/mdev = 12.1/68.9/540.2/97.3 ms`. Both have the same minimum.
What differs, and which would you rather run a voice call over?

## B. Apply

**B1.** Compute goodput efficiency for an Ethernet frame carrying a 1,460-byte TCP
payload over IPv4, including the interframe gap and preamble. Repeat for IPv6
(40-byte header). Repeat for a 9,000-byte jumbo payload. Tabulate the three.

**B2.** A VoIP call uses the G.711 codec at 64 kb/s with 20 ms packetisation, giving
160 bytes of payload per packet. Compute (a) packets per second, (b) total bits per
second on an Ethernet link including all headers and the interframe gap, (c) the
overhead ratio, and (d) how many simultaneous calls fit on a 10 Mb/s link at 70%
target utilisation.

**B3.** A 1,500-byte frame crosses six 100 Mb/s links totalling 2,400 km of fibre,
with 20 µs of processing per hop and negligible queueing. Compute each latency
component and the one-way total. Then recompute with the links upgraded to
10 Gb/s. What percentage improvement did the upgrade deliver?

**B4.** Using the M/M/1 table in §3.2, compute the factor by which queueing delay
increases when utilisation rises from 0.6 to 0.85. A manager proposes running links
at 90% "to get value from the investment." Write the two-sentence reply.

**B5.** A path has 80 ms RTT. Compute the maximum single-stream TCP throughput for
window sizes of 64 KB, 256 KB, 1 MB, and 8 MB. Then compute the window required to
saturate a 2.5 Gb/s link on that path.

**B6.** Using the Mathis relation with MSS = 1,460 bytes and RTT = 60 ms, estimate
maximum single-stream throughput at loss rates of 0.01%, 0.1%, and 2%. A link is
1 Gb/s and 25% utilised, and users report slow transfers; measured loss is 0.4%.
Explain the situation quantitatively.

**B7.** A traceroute shows 35% loss at hop 4, 0% at hops 5–12, and the destination
responds normally. A colleague opens a ticket with the transit provider about
hop 4. Explain what is actually happening and what evidence would indicate a real
problem at hop 4 instead.

## C. Analyse

**C1.** Derive the bandwidth–delay product from first principles: start from "the
sender may have at most *W* bytes unacknowledged" and "an acknowledgement takes
one RTT to return," and show that steady-state throughput is *W*/RTT. Then extend
the derivation to *N* parallel connections and state the assumption that makes the
extension valid — and the circumstance in which the assumption fails.

**C2.** A satellite operator claims their new service has "the same bandwidth as
fibre." Assume 500 Mb/s capacity and a GEO orbit at 35,786 km. Compute the minimum
possible RTT. Then compute the single-stream TCP throughput at that RTT with a
64 KB window, with a 2 MB window, and with a 32 MB window. At what point does the
window become impractical, and what other technique would you reach for instead?
(You are reinventing performance-enhancing proxies; Chapter 49 names them.)

**C3.** Show that for a fixed amount of data *D* to be transferred, the total time
is approximately RTT × (number of round trips) + *D*/rate. Use this to compute the
crossover file size below which latency dominates and above which bandwidth
dominates, for a path of 1 Gb/s and 80 ms RTT assuming a protocol that needs 4
round trips of setup. Comment on what this implies for a web page composed of 60
small objects.

**C4.** Bursty loss is worse than uniform loss for voice but *better* for TCP
throughput at the same average rate. Argue both halves. (Hint: consider what a
codec's concealment algorithm does, and what TCP's congestion response does per
loss *event* rather than per lost packet.)

## D. Design

**D1.** A company with 400 staff in Sydney needs to work against an application
server in Frankfurt. The application performs 22 request/response round trips to
load its main screen, and each response averages 40 KB. Available links: 100 Mb/s
at 280 ms RTT, or 20 Mb/s at 265 ms RTT, at similar cost.

(a) Compute the screen-load time on each link. (b) Recommend one, with reasoning.
(c) Propose two changes that would improve the user experience more than either
link would, and quantify the expected improvement of each. (d) State what
measurement you would take before spending anything.

## E. Troubleshoot

**E1.** Users at a branch office report that "the network is slow in the
afternoons." You gather:

- Circuit: 200 Mb/s. Five-minute average utilisation peaks at 71% at 14:00.
- `ping` to the head office at 09:00: `min/avg/max/mdev = 18.2/18.9/21.0/0.6 ms`
- `ping` to the head office at 14:00: `min/avg/max/mdev = 18.2/94.7/610.3/88.1 ms`, 0.8% loss
- `iperf3` single stream at 14:00: 11 Mb/s. With `-P 16`: 138 Mb/s.
- The head office reports no complaints.

Identify the dominant problem, the secondary problem, and the piece of evidence
that rules out a physical-layer fault. Then explain why upgrading the circuit to
500 Mb/s would help less than the utilisation graph suggests, and name the two
mechanisms (from Chapters 52 and 66) you would investigate instead.
