# Chapter 21 — Exercises

## A. Recall

**A1.** State the combinatorial argument for layering in one sentence, with the two
expressions.

**A2.** Distinguish a *service* from a *protocol*. Which is vertical and which is
horizontal?

**A3.** Name the four OSI service primitives and say what each does.

**A4.** List four costs of layering.

**A5.** What does TSO stand for, what does it do, and which layer boundary does it
violate?

**A6.** Why does TCP's checksum include a pseudo-header, and what does that
pseudo-header contain?

## B. Apply

**B1.** An organisation supports 12 applications over 8 media types.

(a) How many implementations under the direct approach?
(b) Under the layered approach?
(c) A ninth medium is added. How many new implementations under each?
(d) At what point does the ratio exceed 10×, if both sets grow equally?

**B2.** Compute the on-the-wire bandwidth for a G.711 VoIP call: 160-byte payload
every 20 ms, over RTP/UDP/IP/Ethernet. Compare with the codec's nominal 64 kb/s and
state the overhead ratio.

**B3.** Repeat B2 for G.729 (20-byte payload every 20 ms) and explain why the overhead
ratio is so much worse.

**B4.** A 1460-byte payload is sent over TCP/IPv4/Ethernet with an 802.1Q tag and
through a VXLAN tunnel. Compute total bytes on the wire and the efficiency.

**B5.** For each of the following, name the layer that nominally owns the device and
the layer(s) it actually inspects:

(a) a NAT router  (b) a stateful firewall  (c) an L7 load balancer
(d) an FTP application-layer gateway  (e) a NIC performing TSO

**B6.** An application calls `send()` with 64 KB on a host with TSO enabled, over a
1500-byte-MTU Ethernet link. Describe what `tcpdump` on that host shows, what a switch
port mirror shows, and why they differ.

## C. Analyse

**C1.** The socket interface has eight core calls and has organised the industry for
forty years; OSI's formal service definitions are more complete and were barely
implemented. Explain what this says about interface design.

**C2.** Explain precisely why TCP performs poorly on a lossy wireless link. Your answer
must identify the inference TCP makes, why it is correct on wired networks, and what
information exists but cannot cross the layer boundary.

**C3.** Three mitigations for C2 are given in §21.3. For each, state which layer
boundary it works around and what new problem it introduces.

**C4.** "QUIC is the largest layer violation in modern networking, and it exists
because the boundary cost more than it was worth." Defend this, naming three specific
costs QUIC was designed to avoid.

**C5.** Explain the causal chain from *middleboxes violate layering* to *TCP cannot be
changed* to *QUIC encrypts its headers*. Identify who is defending against whom at
each stage.

**C6.** The narrow waist of the hourglass is described as "both the achievement and the
constraint". Explain both halves, using IPv6 as the evidence for the second.

**C7.** A colleague argues that the OSI model should be abandoned because it is
violated everywhere. Give the strongest version of their case, then the response.

## D. Design

**D1.** You are designing a protocol for industrial sensors that report 8 bytes every
second over Wi-Fi, battery-powered, for ten years. Compute the header overhead under a
conventional stack, then propose what you would change and what you would give up.

**D2.** Design the interface (in the §21.2 sense) between a video streaming
application and a transport. Specify what it must expose that sockets do not, and
justify each addition against a failure it prevents.

**D3.** An organisation is deploying VoIP for 400 concurrent calls. Compute the
bandwidth required at the WAN edge, showing the header overhead explicitly, and state
what you would do to reduce it and what it would cost.

**D4.** Write a one-page briefing for a manager explaining why a "simple" change to
IP is not simple, using the hourglass argument. No jargon.

## E. Troubleshoot

**E1.** A packet capture on a web server shows TCP segments of 42,000 bytes. The MTU
is 1500. Explain, and give the command that makes the capture reflect reality.

**E2.** An FTP transfer works from inside the network and fails from outside, though
the control connection establishes fine. Explain what is happening and at which layer.

**E3.** A SIP call sets up correctly and has no audio in one direction. What layer
violation is failing, and what family of protocols exists to solve it?

**E4.** A newly-deployed application uses a TCP option not seen before. It works on
90% of client networks and fails silently on the rest. Diagnose, and explain why this
is very difficult to fix.

**E5.** A file transfer over a satellite link achieves 2 Mb/s on a 50 Mb/s circuit
with 0.3% loss. Explain the mechanism, and give two mitigations at different layers.

**E6.** After enabling a NIC feature for performance, a server begins sending packets
with bad checksums according to a capture — but connections work fine. Explain.

## F. Extend

**F1.** Disable all offloads on a host (`ethtool -K`), capture the same transfer with
and without, and account for every difference in the two captures.

**F2.** Measure the actual on-the-wire bandwidth of a VoIP call with a capture and
compare with your calculation in B2. Account for any discrepancy.

**F3.** Read Saltzer, Reed and Clark's end-to-end paper (1984) and identify which of
§21.4's violations they would have anticipated and which would have surprised them.

**F4.** Find a protocol in current use that carries an address in its payload, other
than FTP and SIP. Explain what it costs and what it would take to fix.

**F5.** Argue the case that NAT was a mistake, then argue that it was the correct
engineering decision. Decide which you believe and say why.
