# Appendix B — Ports, Protocols, and Standards

Arranged by **purpose** rather than by number, because that is the arrangement in which
they are learnable and the arrangement in which you will need them. Chapter 41 develops
each; Chapter 35 explains why ports exist at all.

A reminder from Chapter 35 §35.3: **a port number is a convention, not a constraint.**
Nothing prevents a service from running elsewhere, and security assessment by port
number alone is incomplete.

---

## B.1 Naming and Addressing

| Protocol | Port | Transport | Notes |
|---|---|---|---|
| DNS | 53 | UDP, TCP | UDP for queries; TCP for zone transfers and responses > 512 B |
| DNS over TLS (DoT) | 853 | TCP | |
| DNS over HTTPS (DoH) | 443 | TCP | Indistinguishable from web traffic by design |
| mDNS | 5353 | UDP | Link-local service discovery (Bonjour/Avahi) |
| LLMNR | 5355 | UDP | Windows link-local resolution; a known attack vector |
| DHCP (server / client) | 67 / 68 | UDP | Broadcast until an address exists |
| DHCPv6 | 547 / 546 | UDP | |
| NetBIOS name service | 137 | UDP | Legacy; disable where possible |

---

## B.2 The Web

| Protocol | Port | Transport | Notes |
|---|---|---|---|
| HTTP | 80 | TCP | |
| HTTPS | 443 | TCP | HTTP/1.1 and HTTP/2 over TLS |
| HTTP/3 (QUIC) | 443 | **UDP** | Chapter 38 §38.4 |
| HTTP proxy (common) | 3128, 8080 | TCP | Convention only |

---

## B.3 Remote Access and File Transfer

| Protocol | Port | Transport | Notes |
|---|---|---|---|
| SSH | 22 | TCP | |
| SFTP | 22 | TCP | An SSH subsystem — **not** FTPS |
| SCP | 22 | TCP | Also over SSH |
| Telnet | 23 | TCP | Cleartext. Use only as a diagnostic client |
| FTP (control / data) | 21 / 20 | TCP | Two connections; breaks through NAT (Ch 33 §33.3) |
| FTPS | 990 (implicit), 21 (explicit) | TCP | FTP over TLS — unrelated to SFTP |
| TFTP | 69 | UDP | No authentication; firmware and boot images |
| RDP | 3389 | TCP | Never expose to the Internet |
| VNC | 5900+ | TCP | |
| SMB / CIFS | 445 | TCP | Must not cross a perimeter |
| NFS | 2049 | TCP, UDP | |

---

## B.4 Mail, Directory, and Time

| Protocol | Port | Transport | Notes |
|---|---|---|---|
| SMTP (relay) | 25 | TCP | Server-to-server |
| SMTP submission | 587 | TCP | Client-to-server, authenticated. Use this |
| SMTPS | 465 | TCP | Implicit TLS submission |
| POP3 / POP3S | 110 / 995 | TCP | Download-and-delete model |
| IMAP / IMAPS | 143 / 993 | TCP | Server-side mailbox model |
| LDAP / LDAPS | 389 / 636 | TCP | |
| Global Catalog (AD) | 3268 / 3269 | TCP | |
| Kerberos | 88 | TCP, UDP | Clock-skew sensitive |
| NTP | 123 | UDP | Skew breaks Kerberos, TLS, MFA and log correlation |

Mail authentication records live in DNS as `TXT` records: **SPF** (which hosts may
send), **DKIM** (cryptographic signature), **DMARC** (policy and reporting).

---

## B.5 Voice, Video, and Real-Time

| Protocol | Port | Transport | Notes |
|---|---|---|---|
| SIP | 5060 | UDP, TCP | Signalling |
| SIP over TLS | 5061 | TCP | |
| H.323 | 1720 | TCP | Legacy |
| RTP / RTCP | dynamic, 16384–32767 typical | UDP | Media. Chapter 36 explains why UDP |
| STUN | 3478 | UDP | NAT traversal |
| TURN | 3478, 5349 | UDP, TCP | Relayed NAT traversal |

---

## B.6 Management and Monitoring

| Protocol | Port | Transport | Notes |
|---|---|---|---|
| SNMP | 161 | UDP | v1/v2c cleartext community strings — use v3 |
| SNMP trap | 162 | UDP | |
| Syslog | 514 | UDP | 6514 for syslog over TLS |
| NetFlow / IPFIX | 2055, 4739 | UDP | Collector-dependent |
| RADIUS (auth / acct) | 1812 / 1813 | UDP | 1645/1646 on older equipment |
| TACACS+ | 49 | TCP | Encrypts the whole payload |
| gNMI | 9339 | TCP | Streaming telemetry |
| NETCONF | 830 | TCP | Over SSH |

---

## B.7 Databases and Infrastructure

| Service | Port | Transport |
|---|---|---|
| MySQL / MariaDB | 3306 | TCP |
| PostgreSQL | 5432 | TCP |
| Microsoft SQL Server | 1433 | TCP |
| Oracle | 1521 | TCP |
| Redis | 6379 | TCP |
| MongoDB | 27017 | TCP |
| memcached | 11211 | TCP, UDP |

Every one of these has appeared in a breach caused by exposure to the Internet without
authentication. memcached over UDP is the amplification vector of Chapter 62 §62.3.

---

## B.8 Tunnels and VPNs

| Protocol | Port / number | Notes |
|---|---|---|
| IPsec ESP | IP protocol 50 | Not a port; some NAT devices cannot track it |
| IPsec AH | IP protocol 51 | Cannot traverse NAT at all |
| IKE | 500/UDP | Negotiation |
| IPsec NAT-T | 4500/UDP | ESP encapsulated in UDP |
| L2TP | 1701/UDP | Usually with IPsec |
| OpenVPN | 1194/UDP (default) | Often moved to 443 to traverse filters |
| WireGuard | 51820/UDP (default) | Configurable; silent to unauthenticated packets |
| GRE | IP protocol 47 | No encryption |

---

## B.9 Port Ranges

| Range | Name | Assignment |
|---|---|---|
| 0–1023 | Well-known | IANA; historically root-only to bind on Unix |
| 1024–49151 | Registered | IANA on request |
| 49152–65535 | Dynamic / ephemeral | Client-chosen |

Operating systems differ: Linux defaults to 32768–60999
(`/proc/sys/net/ipv4/ip_local_port_range`), which matters when writing firewall rules.

---

## B.10 EtherTypes and IP Protocol Numbers

**EtherType** (Ethernet frame, bytes 12–13 — Chapter 15 §15.3):

| Value | Payload |
|---|---|
| 0x0800 | IPv4 |
| 0x0806 | ARP |
| 0x8100 | 802.1Q VLAN tag |
| 0x86DD | IPv6 |
| 0x8847 | MPLS unicast |
| 0x888E | 802.1X EAPOL |

**IP Protocol** (IPv4 header — Chapter 24 §24.2):

| Value | Protocol |
|---|---|
| 1 | ICMP |
| 2 | IGMP |
| 6 | TCP |
| 17 | UDP |
| 41 | IPv6 encapsulation |
| 47 | GRE |
| 50 | ESP |
| 51 | AH |
| 58 | ICMPv6 |
| 89 | OSPF |
| 112 | VRRP |

---

## B.11 ICMP Types Worth Knowing

| Type | Code | Meaning |
|---|---|---|
| 0 | 0 | Echo Reply |
| 3 | 0 | Destination Network Unreachable |
| 3 | 1 | Destination Host Unreachable |
| 3 | 3 | Port Unreachable |
| 3 | 4 | **Fragmentation Needed, DF set** — PMTUD depends on this |
| 3 | 13 | Administratively Prohibited |
| 5 | 0 | Redirect |
| 8 | 0 | Echo Request |
| 11 | 0 | **Time Exceeded (TTL)** — traceroute depends on this |

Blocking ICMP indiscriminately breaks Path MTU Discovery (Chapter 34 §34.4). RFC 4890
specifies which ICMPv6 types **must** be permitted; for IPv6 this is not optional.

---

## B.12 Ethernet Standards

Read the name: `10GBASE-SR` = 10 Gb/s, BASEband, Short Reach multimode.

| Standard | Rate | Medium | Max distance |
|---|---|---|---|
| 10BASE-T | 10 Mb/s | Cat3+ UTP | 100 m |
| 100BASE-TX | 100 Mb/s | Cat5+ | 100 m |
| 1000BASE-T | 1 Gb/s | Cat5e+ | 100 m |
| 1000BASE-SX | 1 Gb/s | MMF | 550 m |
| 1000BASE-LX | 1 Gb/s | SMF | 5 km |
| 2.5G/5GBASE-T | 2.5/5 Gb/s | Cat5e/Cat6 | 100 m |
| 10GBASE-T | 10 Gb/s | Cat6a | 100 m |
| 10GBASE-SR | 10 Gb/s | MMF (OM3/OM4) | 300/400 m |
| 10GBASE-LR | 10 Gb/s | SMF | 10 km |
| 40GBASE-SR4 | 40 Gb/s | MMF, 4 lanes | 100/150 m |
| 100GBASE-LR4 | 100 Gb/s | SMF, 4 λ | 10 km |
| 400GBASE-DR4 | 400 Gb/s | SMF, 4 lanes | 500 m |

**Power over Ethernet:**

| Standard | Name | Power at PSE | Power at PD |
|---|---|---|---|
| 802.3af | PoE | 15.4 W | 12.95 W |
| 802.3at | PoE+ | 30 W | 25.5 W |
| 802.3bt Type 3 | PoE++ | 60 W | 51 W |
| 802.3bt Type 4 | PoE++ | 90 W | 71.3 W |

---

## B.13 IEEE 802 and Wi-Fi

| Standard | Subject |
|---|---|
| 802.1Q | VLAN tagging |
| 802.1X | Port-based network access control |
| 802.1D / w / s | Spanning Tree / Rapid STP / Multiple STP |
| 802.1AB | LLDP |
| 802.1AX | Link aggregation (LACP) |
| 802.3 | Ethernet |
| 802.11 | Wireless LAN |
| 802.15.1 / .4 | Bluetooth / LR-WPAN (Zigbee, Thread) |

| Wi-Fi generation | Standard | Bands | Max PHY rate |
|---|---|---|---|
| Wi-Fi 4 | 802.11n | 2.4, 5 | 600 Mb/s |
| Wi-Fi 5 | 802.11ac | 5 | 6.9 Gb/s |
| Wi-Fi 6 | 802.11ax | 2.4, 5 | 9.6 Gb/s |
| Wi-Fi 6E | 802.11ax | + 6 | 9.6 Gb/s |
| Wi-Fi 7 | 802.11be | 2.4, 5, 6 | 46+ Gb/s |

**2.4 GHz non-overlapping channels: 1, 6, 11** (derived in Chapter 43 §43.2).

---

## B.14 Cable Categories

| Category | Bandwidth | Supports | Max distance |
|---|---|---|---|
| Cat5e | 100 MHz | 1GBASE-T; 2.5GBASE-T | 100 m |
| Cat6 | 250 MHz | 1G at 100 m; 10G at 55 m; 5GBASE-T | 100 m / 55 m |
| Cat6a | 500 MHz | 10GBASE-T | 100 m |
| Cat7 | 600 MHz | 10G, shielded | 100 m |
| Cat8 | 2000 MHz | 25/40GBASE-T | 30 m |

**T568B pinout** (the common choice; be consistent within a site):

| Pin | Pair colour |
|---|---|
| 1 | White/Orange |
| 2 | Orange |
| 3 | White/Green |
| 4 | Blue |
| 5 | White/Blue |
| 6 | Green |
| 7 | White/Brown |
| 8 | Brown |

T568A swaps the orange and green pairs. A cable with A at one end and B at the other is
a **crossover**; modern equipment with Auto-MDI/MDI-X makes this irrelevant, which is
why crossover cables have largely disappeared.

---

## B.15 Key RFCs

| RFC | Subject |
|---|---|
| 768 | UDP |
| 791 | IPv4 |
| 792 | ICMP |
| 793 / 9293 | TCP |
| 826 | ARP |
| 1034 / 1035 | DNS |
| 1122 / 1123 | Requirements for Internet Hosts |
| 1191 | Path MTU Discovery |
| 1918 | Private address space |
| 1918 / 4632 | CIDR |
| 2131 | DHCP |
| 2328 | OSPFv2 |
| 2474 | DiffServ / DSCP |
| 3021 | /31 on point-to-point links |
| 3550 | RTP |
| 4271 | BGP-4 |
| 4861 / 4862 | IPv6 Neighbor Discovery / SLAAC |
| 5952 | IPv6 address text representation |
| 7323 | TCP extensions for high performance |
| 8200 | IPv6 |
| 8446 | TLS 1.3 |
| 9000 | QUIC |
| BCP 38 (RFC 2827) | Source address validation |

Not every RFC is a standard. Check a document's status before citing it — the series
also contains Informational, Experimental, Historic, and the annual April Fools
publications.
