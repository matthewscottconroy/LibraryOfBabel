# Chapter 61 — Further Reading

## Read these first

Ferguson, N. & Schneier, B. (1999). "A Cryptographic Evaluation of IPsec."
Twenty pages, free, and blunt. **Read the conclusions**, then read IKEv2 and WireGuard and
see which recommendations each adopted.

Donenfeld, J. (2017). "WireGuard: Next Generation Kernel Network Tunnel." NDSS.
**F7 uses it.** Short, and the design-goals section is the argument for removing options
rather than adding mechanisms.

Bellovin, S. (1996). "Problem Areas for the IP Security Protocols." USENIX Security.
Why encryption without integrity is dangerous, established before the protocol was finished
and ignored for years.

The Noise Protocol Framework specification (noiseprotocol.org).
Read the introduction and one handshake pattern. It is the clearest demonstration
available that handshake design is a solved problem you should not attempt yourself.

## Specifications

RFC 4301 — Security Architecture for IP, RFC 4303 — ESP, RFC 7296 — IKEv2.
RFC 4301's architecture overview is the readable one. RFC 7296 replaced a shelf of IKEv1
documents with one, which is itself informative.

RFC 3948 — UDP Encapsulation of ESP (NAT-T), and **RFC 8221** for the current algorithm
recommendations. Read 8221 before configuring anything — it names what to use and what to
stop using.

RFC 2784 — GRE, and **RFC 2890** for the key and sequence extensions.

RFC 8446 — TLS 1.3 and RFC 9147 — DTLS 1.3. DTLS is what a competent TLS VPN uses
underneath, and the differences from TLS are the interesting part.

RFC 2637 — PPTP, if you must. And read Schneier and Mudge's cryptanalysis alongside it.

IANA's IKEv2 parameters registry — for what the numbers in a proposal actually mean when
you are reading a log.

## Books and long-form

Frankel, S. & Krishnan, S. — RFC 6071, "IPsec and IKE Document Roadmap."
Not a book, and it is the map through forty RFCs. Consult it when a document references
three others.

Doraswamy, N. & Harkins, D. — *IPSec: The New Security Standard*.
Dated, and **Harkins co-wrote IKE**, so the design reasoning is first-hand.

Grigorik, I. — *High Performance Browser Networking* (Chapter 58's reading) — the TLS and
transport chapters bear directly on why a TLS VPN behaves as it does.

Vendor VPN design guides — Cisco's, Fortinet's, Palo Alto's and strongSwan's documentation.
strongSwan's is the best free reference for IPsec configuration in general, because it
explains the parameters rather than listing them.

## Analysis and critique

Schneier, B. & Mudge (1998). "Cryptanalysis of Microsoft's Point-to-Point Tunneling
Protocol."
And the 2012 MS-CHAPv2 work by Marlinspike and Hulton. Together they are a complete case
study in a protocol being broken, patched and broken again.

Krawczyk, H. (2003). "SIGMA: The 'SIGn-and-MAc' Approach to Authenticated Diffie-Hellman."
The design pattern under IKEv2 and TLS 1.3. Mathematical, and the introduction states the
problem — authenticating a key exchange without revealing identities — very clearly.

Honda, O. et al., and the wider literature on "TCP meltdown."
**The TCP-over-TCP problem**, quantified. Search "why TCP over TCP is a bad idea" for the
readable summary — it has been the standard reference for twenty years.

Formal verification work on WireGuard — Donenfeld and Milner's symbolic analysis, and the
independent Tamarin and CryptoVerif treatments.
Worth knowing they exist: this is a protocol whose security properties were proved rather
than argued.

Published CVE analyses of TLS VPN concentrators — **F8 uses one.** The Fortinet, Pulse
Secure, Citrix and SonicWall pre-authentication vulnerabilities each have detailed public
write-ups, and reading one changes how you think about that class of device.

## Tools

**strongSwan** or **Libreswan** — **F1 uses one.** The best way to learn IPsec is to configure
it between two implementations and deliberately mismatch parameters. `swanctl --list-sas` and
the log at level 2 are where the learning happens.

**WireGuard** — **F2.** `wg-quick` gets a tunnel up in two minutes, which is the whole
demonstration; `wg show` gives the last handshake time, which is the only status there is.

Tailscale, Netbird, Headscale, Firezone — WireGuard plus the key distribution it declines
to provide. F3 uses one or builds the equivalent. Headscale is the open control server
and is instructive to run.

**OpenVPN** — still widely deployed, userspace, and a useful comparison point for §61.3's
kernel-versus-userspace performance argument.

`ping -M do -s` (Linux) / **`ping -f -l`** (Windows) — **F3.** MTU bisection, and it is the
single most useful tunnel diagnostic there is.

**`ip tcp adjust-mss`** or `iptables -t mangle --clamp-mss-to-pmtu` — **the fix.**

**Wireshark** with the ISAKMP and WireGuard dissectors — **F5.** Watching an IKEv2 exchange and
a WireGuard handshake side by side makes §61.3's argument visible in one screen.

**`nmap`** against your own endpoints — **F6.** The difference between what an IPsec endpoint
and a WireGuard endpoint disclose is the whole silence-on-the-wire point.

`iperf3` through a tunnel, with and without correct MTU — **F4**, and it will convince anyone
that MTU matters more than cipher choice.

## Following the field

The WireGuard mailing list and Donenfeld's talks — for where it is going, including the
post-quantum discussion.

The IETF `ipsecme` working group — IPsec is still being developed, and the post-quantum
key exchange work is where the next change will come from.

Cloud providers' VPN gateway documentation — AWS, Azure and Google all publish their
supported IKE parameters explicitly, and matching them is a common practical task that
teaches the parameter list faster than any specification.

CISA and NCSC advisories on remote access appliances — **subscribe.** This is the device
class most likely to require an emergency change (Chapter 55 §55.3).

## Where to look next

**Chapter 62** attacks the deployments in this chapter; **Chapter 59 §59.4** is the model
replacing the general-purpose VPN; **Chapter 51 §51.2** covers SD-WAN, which is this chapter's
tunnels with central policy; and **Chapter 67 §67.2** covers the unencrypted overlays — VXLAN
and GENEVE — that use the same encapsulation idea for a different purpose.
