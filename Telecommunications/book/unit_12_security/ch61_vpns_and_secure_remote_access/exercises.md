# Chapter 61 — Exercises

## A. Recall

**A1.** State the tunnel idea in one sentence, and say what distinguishes a VPN from a tunnel.

**A2.** Give the three things a tunnel buys, and say which requires cryptography.

**A3.** Why does a GRE tunnel with no encryption make sense? Give two reasons.

**A4.** Why can AH never traverse NAT?

**A5.** Distinguish transport mode from tunnel mode, and say which "site-to-site VPN" means.

**A6.** Name four parameters that must match in IKE phase 1 and four in phase 2.

**A7.** What is NAT-T, why is it needed, and what does it require in addition to encapsulation?

**A8.** Why does a tunnel that drops every hour usually indicate rekeying, and how do you
confirm it?

**A9.** Give three differences between policy-based and route-based IPsec, and say which to
prefer.

**A10.** What is the entire argument for a TLS VPN?

**A11.** Explain TCP meltdown and say what avoids it.

**A12.** Name three deliberate WireGuard design decisions and what each removes.

**A13.** What two things does `AllowedIPs` do?

**A14.** State WireGuard's honest limitation.

**A15.** Why is a full-tunnel deployment approximately six times the bandwidth of a split-tunnel
one?

**A16.** What is the single commonest initial access route into organisations with a VPN?

## B. Apply

**B1.** Compute the effective MTU for each, starting from 1500:

(a) GRE
(b) IPsec ESP tunnel mode with AES-GCM
(c) IPsec ESP with NAT-T
(d) GRE over IPsec
(e) WireGuard
(f) IPsec over a PPPoE link with a 1492 MTU

For each, give the TCP MSS clamp value you would configure.

**B2.** A site-to-site tunnel is established and passes no traffic. Give the diagnostic sequence
in order, stating what each step rules out.

**B3.** Two IKEv1 tunnels drop briefly at intervals of exactly 28,800 seconds and 3,600 seconds
respectively.

(a) Diagnose each.
(b) State what configuration you would compare.
(c) Explain why IKEv2 largely removes this problem.

**B4.** An organisation has 34 branch sites, all connecting to two data centres.

(a) How many tunnels with pre-shared keys, and how many keys?
(b) Repeat if the branches must also reach each other directly.
(c) At what point does certificate authentication become the correct choice, and why?

**B5.** Size a VPN concentrator deployment for 2,500 staff.

(a) Compute concurrent users at normal (30%) and full-remote (85%) rates.
(b) Compute aggregate bandwidth for full tunnel at 3 Mb/s per user and split tunnel at
0.5 Mb/s.
(c) The available concentrator is rated at 4 Gb/s encrypted throughput and 2,000 concurrent
sessions. Assess both scenarios.
(d) State three things besides the concentrator that must also carry the load.

**B6.** For each requirement, choose IPsec, TLS VPN or WireGuard and justify in one sentence:

(a) Site-to-site between a Cisco router and a Fortinet firewall
(b) Remote access for consultants working from client sites with restrictive networks
(c) A mesh of 40 cloud instances across three providers
(d) Remote access with a requirement for FIPS-validated cryptography
(e) A tunnel that must carry OSPF between two sites
(f) Remote access for staff who move between mobile and Wi-Fi mid-session

**B7.** Write the WireGuard configuration for a hub and two spokes, where each spoke may reach
the hub's 10.0.0.0/16 and the spokes may reach each other. State what `AllowedIPs` must contain
at each peer and explain why.

**B8.** An organisation currently full-tunnels all remote traffic for inspection.

(a) State the argument for split tunnelling.
(b) State what inspection capability is lost.
(c) Design the replacement inspection arrangement.
(d) State the one circumstance under which you would keep full tunnelling.

## C. Analyse

**C1.** The chapter separates the three things a tunnel buys and observes that only one requires
cryptography. Analyse how often each is the actual motivation, and what asking "what is this
tunnel for?" would change in a typical enterprise design.

**C2.** Analyse IPsec's reputation for complexity. Is the complexity in the cryptography, the
specification, the number of choices, or the implementations? Support your answer with specific
examples from §61.2.

**C3.** WireGuard's fixed cipher suite is a deliberate rejection of cryptographic agility.
Analyse both sides: what agility buys, what it costs, and what happens to each design when a
primitive is broken.

**C4.** Analyse the claim that "WireGuard is simpler than IPsec" as a category error. What is a
fair comparison, and does WireGuard still win?

**C5.** Analyse the split-tunnel security argument. Is a split-tunnelled client genuinely a
bridge between two networks in a way a full-tunnelled one is not? Be precise about the threat.

**C6.** TLS VPN concentrators have repeatedly had pre-authentication remote code execution
vulnerabilities. Analyse why this class of device is particularly exposed, and compare with
WireGuard's silence-on-the-wire property.

**C7.** Analyse the argument that the VPN is being replaced. What exactly is being replaced,
what is not, and what would an organisation actually have at the end of a successful
transition?

**C8.** March 2020 exposed VPN sizing assumptions across the industry simultaneously. Analyse
this as a risk assessment failure (Chapter 57 §57.3): what was the ARO assumed for "everyone
works remotely", and what should it have been?

## D. Design

**D1.** Design the site-to-site connectivity for 34 branches and two data centres: topology,
protocol, authentication, redundancy, routing over the tunnels, MTU handling, and monitoring.
Justify each choice, and state what you would do differently at 200 branches.

**D2.** Design remote access for 2,500 staff with 300 contractors: authentication, tunnel mode,
what each population may reach, device posture requirements, redundancy, sizing, and the
inspection arrangement. Include the failure modes you would rehearse.

**D3.** Design a WireGuard-based mesh for a 60-instance multi-cloud estate: key distribution,
addressing, `AllowedIPs` policy, how a new instance joins, how a compromised instance is
removed, and what management plane you would use or build.

**D4.** An organisation wants to move from a general-purpose VPN to per-application brokered
access over three years. Design the sequence: what moves first, what stays, how the residual
VPN is reduced and segmented, and what you would tell the board about the end state.

**D5.** Design the MTU strategy for a network with IPsec over broadband, IPsec over PPPoE, and
GRE over IPsec in different places. Specify what is configured where, how it is verified, and
what monitoring would detect a regression.

## E. Troubleshoot

**E1.** A tunnel establishes, the SA shows encrypted packets outbound and zero inbound.
Diagnose.

**E2.** SSH through a tunnel works and file transfers hang after a few packets. Give the cause,
the confirming test with exact command syntax, and two fixes.

**E3.** A tunnel comes up, drops after 30 seconds, comes up, and repeats. Give three possible
causes and how to distinguish them.

**E4.** A tunnel between two sites works and OSPF will not form an adjacency across it.
Diagnose and give two remedies.

**E5.** After a merger, two sites both using 192.168.1.0/24 must be connected. Explain the
problem and give three approaches with their consequences.

**E6.** A TLS VPN performs acceptably on office broadband and collapses on a mobile connection.
Explain the mechanism and give the fix.

**E7.** A WireGuard peer shows no handshake for two hours. State whether this is a fault and how
you would determine it.

**E8.** During a company-wide remote working day, users report they cannot connect at all, with
an unhelpful error. The concentrator's CPU and bandwidth are both low. Diagnose.

**E9.** A VPN failover to the standby concentrator succeeds and users cannot authenticate.
Give two likely causes.

**E10.** A contractor's VPN session is found to have reached a finance database. Analyse where
the failure is, in the terms of Chapter 59.

## F. Extend

**F1.** Build an IPsec site-to-site tunnel in a lab between two different implementations
(strongSwan and a virtual router, or two different vendors' virtual appliances). Record every
parameter that had to match, and the log message produced by deliberately mismatching each of
three of them.

**F2.** Build the same connectivity with WireGuard. Compare the configuration length, the time
taken, and the log output on failure. Write a paragraph on what the comparison does and does not
demonstrate.

**F3.** Measure the MTU problem: build a tunnel, set the MTU deliberately wrong, and observe the
failure. Then find the correct MTU experimentally with `ping -M do` bisection, and confirm it
matches the arithmetic in B1.

**F4.** Measure throughput of IPsec with AES-GCM and WireGuard with ChaCha20 on the same
hardware, with and without AES-NI available (`openssl speed` will tell you, and AES-NI can be
disabled in some hypervisors). Report the four figures and explain the pattern.

**F5.** Capture an IKEv2 exchange in Wireshark and identify each message. Determine what is
visible to an observer before the tunnel is established, and compare with a WireGuard
handshake.

**F6.** Scan a WireGuard endpoint and an IPsec endpoint with `nmap` from a machine you own.
Report what each discloses, and relate the result to §61.3's silence-on-the-wire argument.

**F7.** Read the WireGuard whitepaper (Donenfeld, 2017). Summarise the design goals in one page
and assess how many are achieved by removing options rather than by adding mechanisms.

**F8.** Investigate a published VPN concentrator vulnerability that was exploited at scale.
Determine what was exposed, how long between disclosure and mass exploitation, and what a
defender's realistic window was. Relate the result to Chapter 55 §55.3's patching tracks.
