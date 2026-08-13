# Chapter 33 — Exercises

## A. Recall

**A1.** State the observation that motivated NAT, and the arithmetic saving it produces.

**A2.** Define inside local, inside global, outside global and outside local.

**A3.** Distinguish static NAT, dynamic NAT and PAT in one sentence each.

**A4.** What five fields identify a conversation, and which one does PAT manipulate?

**A5.** Give typical NAT translation timeouts for established TCP, TCP after FIN, and UDP.

**A6.** Name three protocols that break across NAT and say why in one phrase each.

**A7.** What are STUN, TURN and ICE, and what problem do all three exist to solve?

**A8.** What range is CGNAT space, and what does its presence on a WAN interface tell you?

## B. Apply

**B1.** Complete this translation table for four internal hosts, two of which have chosen
the same ephemeral port and the same destination:

| Inside local | Inside global | Outside global |
|---|---|---|
| `10.0.0.5:51000` | ? | `93.184.216.34:443` |
| `10.0.0.6:51000` | ? | `93.184.216.34:443` |
| `10.0.0.7:49200` | ? | `1.1.1.1:53` |
| `10.0.0.5:51001` | ? | `142.250.180.14:443` |

Public address is `203.0.113.9`.

**B2.** Trace a packet from `192.168.5.20:44000` to `198.51.100.10:80` and back through a
PAT router with public address `203.0.113.2`. Give every field that changes in each
direction, including which checksums are recomputed and why.

**B3.** Write the configuration for each, on Cisco IOS:

(a) Static NAT for a web server at `10.1.1.50` to `203.0.113.50`
(b) PAT for `10.1.0.0/16` using the outside interface
(c) Port forwarding of external TCP 443 to `10.1.1.60:8443`
(d) Dynamic NAT from a pool of `203.0.113.100–110`

**B4.** An organisation has one public address and 800 hosts. Each host averages 40
concurrent connections during business hours.

(a) How many translation entries are required?
(b) Is one public address sufficient? Show the arithmetic.
(c) What happens at the limit, and what would you monitor?

**B5.** For each symptom, state whether it is caused by NAT, and if so which aspect:

(a) FTP directory listing hangs  (b) A VoIP call has no audio  (c) A large file transfer
hangs while small ones work  (d) A service is reachable externally and not internally
(e) A game works at home and not on mobile data  (f) Every connection drops when the
router reboots

## C. Analyse

**C1.** "NAT is not a firewall." Explain what protection NAT actually provides, why it is
a side effect, and list four attacks it does not stop.

**C2.** Explain the FTP active-mode failure across NAT completely, then explain why
passive mode helps a client and not a server, then explain what an ALG must do and why
FTPS defeats it.

**C3.** Explain why a NAT router must recompute the TCP checksum, and connect it to a
design decision made in 1981.

**C4.** Explain NAT hole punching, then explain precisely why symmetric NAT defeats it.

**C5.** "Nearly every successful peer-to-peer application is not peer-to-peer." Defend
this and give three examples, then state what architectural property was lost.

**C6.** Compare the security posture of (a) IPv4 with NAT and no firewall, (b) IPv4 with
NAT and a firewall, (c) IPv6 with a stateful firewall. Rank them and justify.

**C7.** Explain why CGNAT produces shared-reputation problems and why the affected user
has no recourse.

**C8.** Work through the economics that make CGNAT rational for a provider, then the
economics that make IPv6 more rational still.

**C9.** "NAT is the most successful temporary measure in the history of computing, and its
success is precisely what made it permanent." Argue for and against, then state your own
position.

## D. Design

**D1.** Design the NAT configuration for a site with: 200 workstations, a public web
server, a mail server, an inbound VPN concentrator, and a block of five public addresses.
Specify which kind of NAT for each and why.

**D2.** For the semester project's network, write the complete NAT configuration and
identify every application that will need special handling.

**D3.** An organisation is deploying IPv6 alongside IPv4. Write the firewall policy that
provides equivalent protection on both, and identify what must be configured on IPv6 that
NAT was doing implicitly on IPv4.

**D4.** Design the logging strategy for a CGNAT deployment serving 50,000 subscribers such
that an abuse report can be traced to a subscriber. Estimate the storage.

**D5.** A VoIP deployment behind NAT has intermittent one-way audio. Design the
remediation, considering ALG, STUN/TURN, static translations, and SIP-aware firewall
options, with the trade-offs of each.

## E. Troubleshoot

**E1.** A user reports that a web application works from home and not from the office. The
office uses NAT with a single public address. Give three candidate causes.

**E2.** `show ip nat translations` shows thousands of entries for one internal host. What
is happening, and what do you check?

**E3.** A UDP-based monitoring application loses connectivity every five minutes and
recovers when it sends data. Explain and give two fixes.

**E4.** Internal users cannot reach the company website by name; external users can. DNS
resolves to the public address in both cases. Diagnose and give the standard fix.

**E5.** After a firewall replacement, IPsec VPNs from remote workers fail while everything
else works. Give the most likely cause.

**E6.** An abuse report names your public address at a specific time. What do you need in
order to identify the host, and what if you do not have it?

**E7.** A customer complains that they cannot host a game server. Their router's WAN
address is `100.71.5.19`. Explain the situation and their options.

**E8.** After enabling IPv6, a security audit finds internal hosts reachable from the
Internet. Explain what the team assumed and what they should have configured.

## F. Extend

**F1.** Capture an FTP session in active mode across a NAT, with and without the ALG
enabled. Identify the `PORT` command in the payload and observe the rewriting.

**F2.** Use a STUN client to discover your own public address and port, then determine
your NAT's type. Explain what your result implies for peer-to-peer applications.

**F3.** Build a double-NAT lab (host → NAT → NAT → server) and document which applications
break. Compare with single NAT.

**F4.** Read RFC 1631 in full. List every consequence its authors predicted and mark
which came true.

**F5.** Instrument a busy NAT router and plot translation-table size over a day. Identify
the peak, the timer effects, and how close you are to exhaustion.

**F6.** Configure IPv6 alongside IPv4 on a lab network and verify that an inbound
connection is blocked by the firewall rather than by translation. Then permit one and
confirm end-to-end connectivity works without any translation at all.
