# Chapter 36 — Exercises

## A. Recall

**A1.** Give UDP's header size, its four fields, and the width of each.

**A2.** In which IP version is the UDP checksum optional, and in which is it mandatory?
Why the difference?

**A3.** List eight things TCP provides that UDP does not.

**A4.** Name the one property UDP provides that TCP does not.

**A5.** State the four criteria for choosing UDP.

**A6.** Give the protocol, port and reason-for-UDP for: DNS, DHCP, TFTP, NTP, SNMP,
syslog, RTP.

**A7.** What is the amplification factor of a UDP reflection attack, in words, and what
makes it possible?

**A8.** State the maximum UDP datagram size you should send across the Internet, and why.

## B. Apply

**B1.** Compute the total on-the-wire bytes for a DNS query (29-byte payload) and its
response (45-byte payload) over UDP/IPv4/Ethernet. Then compute the same transaction over
TCP including handshake and teardown. State the ratio.

**B2.** A path has 80 ms RTT. Compute the time to complete a one-request/one-response
transaction over (a) UDP, (b) TCP, (c) TCP with TLS 1.2 (two extra round trips), (d) QUIC
with 0-RTT.

**B3.** For each application, choose TCP or UDP and justify against §36.2's criteria:

(a) A stock price feed to 5,000 subscribers
(b) A database replication stream
(c) A temperature sensor reporting every 5 minutes
(d) A video call
(e) Downloading a 4 GB file
(f) A DNS resolver
(g) An online multiplayer game's position updates
(h) The same game's chat messages
(i) A firmware image pushed to a switch

**B4.** An attacker with 1 Gb/s of upstream bandwidth uses a reflector with an
amplification factor of 50. Compute the attack volume delivered to the victim. Repeat for
memcached at 10,000×.

**B5.** A UDP application sends 4 KB datagrams over a path with MTU 1500.

(a) How many IP fragments per datagram?
(b) With 1% per-packet loss, what fraction of datagrams arrive complete?
(c) What datagram size would you recommend, and why?

**B6.** A WireGuard tunnel is idle. The NAT's UDP timeout is 30 seconds. Explain what
happens without `PersistentKeepalive` and compute the minimum keepalive interval you would
configure.

## C. Analyse

**C1.** Explain head-of-line blocking with a diagram, then explain precisely why it is
correct behaviour for a file transfer and harmful for live audio.

**C2.** "TCP's retransmission does not merely fail to help real-time media; it actively
harms it." Defend this.

**C3.** Explain §36.2's third criterion — that an application may do reliability better
than TCP — using RTP's keyframe handling as the example.

**C4.** Explain why NTP cannot use TCP. Be specific about what TCP would corrupt.

**C5.** Explain why DHCP cannot use TCP, in terms of what TCP requires that the client does
not have.

**C6.** UDP's message-boundary preservation is described as an under-appreciated feature.
Explain what an application must do without it, and give an example of a bug it prevents.

**C7.** Explain UDP amplification completely: the three steps, why the response cannot be
blocked by source, and the two defences at different points in the network.

**C8.** Explain the structural unfairness between a TCP flow and an unresponsive UDP flow
on a congested link. Why does the well-behaved flow lose?

**C9.** "UDP is not simpler; it is simpler in the protocol and harder in the application."
Defend this using RFC 8085's obligations.

**C10.** QUIC runs over UDP and is described as more reliable than TCP in the way that
matters. Explain, and state what this does to the traditional TCP-versus-UDP framing.

## D. Design

**D1.** Design the transport for a telemetry system: 50,000 sensors, one 60-byte reading
each per minute, loss of individual readings acceptable, gaps must be detectable. Justify
every choice against §36.4's obligations.

**D2.** Design the logging architecture for an organisation with both high-volume
operational logs and security audit logs. Specify the transport for each and justify the
difference.

**D3.** You are writing a new UDP-based protocol. Write the section of its specification
that addresses RFC 8085's requirements.

**D4.** For the semester project's network, identify every UDP service, and for each state
its amplification risk and the control you would apply.

**D5.** An organisation's DNS servers are being used in amplification attacks. Write the
remediation plan, including what you can fix and what requires others to act.

## E. Troubleshoot

**E1.** A DNS query for a DNSSEC-signed zone fails while ordinary queries succeed. The
firewall permits UDP/53. Give two candidate causes and how to distinguish them.

**E2.** A VPN tunnel works while traffic flows and dies after about a minute of idleness.
Diagnose and give the one-line fix.

**E3.** Log messages from a busy server are missing for the exact ten-minute window of an
incident. Explain, and state what should have been configured.

**E4.** A UDP-based application achieves 25 KB/s over a gigabit link with 20 ms RTT.
Diagnose.

**E5.** After a network upgrade, a video conferencing application has worse quality than
before, though bandwidth has increased. Give a candidate explanation involving a
well-intentioned change.

**E6.** A monitoring system shows a device as down; the device's logs show it sent a trap.
Explain.

**E7.** An organisation's Internet link is saturated with inbound UDP from thousands of
legitimate DNS servers. Explain what is happening and what can and cannot be done.

**E8.** A UDP application works on the LAN and loses about 30% of datagrams across a VPN.
Datagram size is 3 KB. Diagnose.

## F. Extend

**F1.** Capture a DNS query and response, and the same lookup forced over TCP
(`dig +tcp`). Count the packets and bytes for each and compare with your answer to B1.

**F2.** Write a UDP client and server. Send 10,000 datagrams as fast as possible across a
lossy link (`tc netem loss 2%`) and measure how many arrive and in what order. Then
implement sequence numbers and detect the loss and reordering yourself.

**F3.** Extend F2 with retransmission and a congestion-control scheme of your choice.
Compare its behaviour with TCP's over the same link. Reflect on §36.2's warning.

**F4.** Measure the amplification factor of a DNS server you control, using
`dig ANY` and comparing request and response sizes. Then configure Response Rate Limiting
and measure again.

**F5.** Read RFC 8085 in full and audit a UDP application you use against its
requirements. Report which it meets.

**F6.** Capture QUIC traffic (visit any Google property with a modern browser). Confirm it
is UDP/443, and observe how much of the header is encrypted. Relate to Chapter 21 §21.4.
