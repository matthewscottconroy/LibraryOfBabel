# Chapter 23 — Exercises

## A. Recall

**A1.** State Kahn's problem in one sentence, including the clause that makes it hard.

**A2.** List the four principles of the Cerf–Kahn design.

**A3.** Why was TCP split into TCP and IP in 1978? Name two application types that
motivated it.

**A4.** Name the four TCP/IP layers and give two protocols at each.

**A5.** Give the PDU name at each of the five levels of the trace in §23.3.

**A6.** State the end-to-end argument in both of its clauses.

**A7.** Give the EtherType for IPv4, IPv6 and ARP, and the IP protocol number for TCP,
UDP and ICMP.

## B. Apply

**B1.** Map every OSI layer onto its TCP/IP equivalent and state which OSI layers are
collapsed and why each collapse is justified.

**B2.** A host sends a 500-byte HTTP POST body over TCP/IPv4/Ethernet.

(a) Give the size at each PDU level.
(b) Give the total byte-times on the wire including preamble and interframe gap.
(c) Compute the efficiency.

**B3.** Repeat B2 for a 20-byte DNS query over UDP. Comment on the difference.

**B4.** A packet crosses four routers between source and destination. State how many
times each of the following changes: destination MAC, source MAC, destination IP,
source IP, TTL, IP header checksum, TCP checksum.

**B5.** Trace the demultiplexing chain for an incoming frame containing an IPv6 packet
carrying UDP to port 53. Give the value checked at each layer.

**B6.** For each function, state whether the end-to-end argument places it at the
endpoints, in the network, or both — and justify:

(a) file integrity  (b) congestion control  (c) link error recovery on radio
(d) encryption  (e) duplicate suppression  (f) traffic policing

## C. Analyse

**C1.** Explain why gateways were designed to be stateless, and what property this
gives the network. Name the principle.

**C2.** "The specification's silence is its most valuable feature." Defend this using
the table of link technologies in §23.2, then give one cost of the silence.

**C3.** The 1983 flag day succeeded and can never be repeated. Explain what made it
possible, and give three current protocol transitions that are constrained by its
impossibility.

**C4.** The end-to-end argument's application to security was "correct in principle and
catastrophic in practice". Explain both halves, and identify the specific gap in the
reasoning.

**C5.** Explain the hourglass shape as a consequence of the end-to-end argument.
Then explain why the same property that makes it powerful makes IPv6 hard.

**C6.** Give the three places where the end-to-end argument is weakest, and for each
give a real mechanism that exists because of the weakness.

**C7.** QUIC is a transport protocol carried inside another transport protocol.
Explain why, tracing the causal chain back to a decision made in the 1990s.

## D. Design

**D1.** You are designing a protocol for a new application: real-time telemetry from
5,000 sensors, where the most recent reading matters and old readings are worthless.
Choose a transport, justify it with the end-to-end argument, and state what you must
implement yourself.

**D2.** Design an incremental deployment plan for a hypothetical new IP option. State
what happens when only 5%, 50% and 95% of the Internet supports it, and identify the
benefit an early adopter receives.

**D3.** Your organisation proposes putting a caching proxy in the middle of every
connection to improve performance. Write the analysis: what it optimises, what
endpoint obligation it does not remove, and what it breaks.

**D4.** Write the one-page explanation of encapsulation you would give to a new hire,
using the §23.3 trace. It must be understandable by someone who has read Units I–IV.

## E. Troubleshoot

**E1.** A capture at a client shows a frame destined for `rr:rr:rr:rr:rr:rr` with an IP
destination of `203.0.113.10`. A capture at the server shows the same IP destination
with a different destination MAC. Explain, and state what this proves about the path.

**E2.** A user reports that a website is unreachable. `ping 203.0.113.10` succeeds.
Identify which steps of the §23.3 trace are proven working and which remain.

**E3.** Traceroute shows no MAC addresses at any hop. Explain why, in terms of
encapsulation.

**E4.** A capture at the client and a capture at the server show different numbers of
packets for the same transfer. Give two mechanisms that could cause this.

**E5.** A newly-deployed application using SCTP works on the LAN and fails across the
Internet. Explain, and name the general phenomenon.

**E6.** DNS resolution succeeds but the HTTP connection times out with no response at
all. Using the trace, state which steps completed and give the two most likely causes.

## F. Extend

**F1.** Read Cerf and Kahn (1974) and identify every place the paper describes
something that later became a separate protocol. Note what they got wrong.

**F2.** Read Saltzer, Reed and Clark (1984) in full. Identify one function in modern
networks that they would say is misplaced, and defend or refute their position.

**F3.** Capture a single HTTP request with Wireshark and expand every layer. Account for
every byte, and compare with §23.3's arithmetic.

**F4.** Read Clark's later work on tussle in cyberspace and explain how his framing
differs from the 1984 paper. What changed?

**F5.** Find a protocol standardised in the last five years and determine what it did to
be incrementally deployable. If it did nothing, predict its adoption.
