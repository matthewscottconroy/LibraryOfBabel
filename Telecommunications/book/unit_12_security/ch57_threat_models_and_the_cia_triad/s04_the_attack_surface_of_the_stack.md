# 57.4 The Attack Surface of the Stack

This section walks the stack we built and enumerates what is exposed at each level. It
doubles as a review of Units II through X, and it is the map for Chapter 62.

## The table

| Layer | Exposure | Built in |
|---|---|---|
| **Physical** | **cable tapping, unlocked rooms, unattended ports, RF eavesdropping** | 10, 42 |
| **Data link** | **MAC flooding, ARP spoofing, VLAN hopping, rogue DHCP, STP attacks** | 17–20 |
| **Network** | **IP spoofing, route injection, BGP hijack, ICMP abuse, fragmentation** | 24–34 |
| **Transport** | **SYN flood, session hijack, port scanning** | 35–38 |
| **Application** | **injection, credential attacks, protocol abuse, DNS poisoning** | 39–41 |
| **Human** | **phishing, pretexting, physical social engineering** | — |

**Two observations before the detail.**

> **The last row is not a joke and is not an afterthought.** The overwhelming majority of
> successful compromises begin with a human being persuaded to do something, not with a
> protocol weakness. Every technical control in this unit protects a system whose most
> reliable entry point is an email.

And the physical row is the one most often ignored by network engineers, on the assumption
that it is facilities' problem. An unlocked comms room with a spare switch port is a complete
bypass of every control in Chapters 58 through 62.

## Physical

What an adversary with physical access can do, and it is nearly everything.

| | |
|---|---|
| **A spare port** | **a device on your network, inside every perimeter control** |
| **A tap on copper or fibre** | **passive, undetectable without optical power monitoring** |
| **Console access** | **password recovery procedures are published by every vendor** |
| **Removing a device** | availability, and the configuration goes with it |
| **A rogue access point** | **an extension of your network into the car park** |
| **RF reception** | Chapter 42 — **radio does not stop at the property boundary** |

**The console point deserves emphasis:** every vendor publishes a documented procedure for
recovering a device whose password is unknown, requiring only physical access and a reboot.
This is a feature, and it means physical access to a device is administrative access to it
unless the recovery procedure has been deliberately disabled — which is itself a trade, since it
also means a genuinely forgotten password bricks the device.

**Controls:** locked rooms with logged access (Chapter 56 §56.3), port security and 802.1X
on access ports (Chapter 59 §59.2), **disabling unused ports**, optical power monitoring on
critical fibre, and encryption, which makes a tap useless.

## Data link

Chapter 17 through 20's mechanisms, each of which trusts something.

| Attack | Mechanism abused | Control |
|---|---|---|
| **MAC flooding** | **the CAM table is finite** — fill it and the switch floods (Chapter 17 §17.2) | **port security** |
| **ARP spoofing** | **ARP has no authentication at all** (Chapter 18 §18.3) | **DAI, DHCP snooping** |
| **Rogue DHCP** | **a client believes the first answer** (Chapter 40 §40.4) | **DHCP snooping** |
| **VLAN hopping** | **double tagging, or negotiating a trunk** (Chapter 20 §20.2) | **disable DTP; never use VLAN 1 as native** |
| **STP manipulation** | **lowest bridge ID wins, and anyone may claim it** (Chapter 19 §19.3) | **root guard, BPDU guard** |
| **CDP/LLDP disclosure** | **devices announce model, version and port** | **disable towards users** |

> The pattern is uniform: every Layer 2 protocol in this book was designed for a cooperative
> environment and authenticates nothing. ARP, DHCP, STP and the discovery protocols all
> believe whatever they are told, and the controls are all bolt-ons added later.

Which is why Layer 2 attacks are so effective and so under-defended: they require access
to the segment, and once an attacker has that, the protocols offer no resistance of their own.

## Network

| Attack | Mechanism | Control |
|---|---|---|
| **IP spoofing** | **the source address is asserted, never verified** (Chapter 24 §24.2) | **BCP 38 / uRPF filtering** |
| **Route injection** | **an IGP that accepts an unauthenticated neighbour** (Chapter 31) | **protocol authentication** |
| **BGP hijack** | **announcements are believed** (Chapter 32 §32.4) | **RPKI, prefix filters** |
| **ICMP abuse** | redirects, amplification, tunnelling (Chapter 34) | **filter selectively, not entirely** |
| **Fragmentation attacks** | **overlapping fragments defeat inspection** (Chapter 24 §24.3) | **reassemble before inspecting** |
| **DDoS amplification** | **UDP services that answer more than they are asked** (Chapter 36) | **do not run open resolvers or NTP monlist** |

BCP 38 deserves a paragraph because it is the clearest case of a solved problem that is not
solved.

> Source address filtering at the network edge — dropping packets whose source address could
> not legitimately come from that direction — eliminates spoofing, and therefore eliminates
> reflection and amplification attacks. It was specified in 2000. It is still not
> universally deployed, because the network that deploys it protects everyone except
> itself.

That incentive structure is the whole problem, and it recurs in Chapter 32's RPKI story and
in Chapter 48's governance discussion: the Internet's remaining security problems are
overwhelmingly the ones where the cost falls on a different party from the benefit.

## Transport

| Attack | Mechanism | Control |
|---|---|---|
| **SYN flood** | **the handshake allocates state before authentication** (Chapter 37 §37.2) | **SYN cookies** |
| **Session hijack** | **predictable sequence numbers, or a readable session** | **randomised ISN, and encryption** |
| **Port scanning** | **a closed port answers differently from a filtered one** (Chapter 35) | **drop rather than reject** |
| **RST injection** | **an off-path attacker who can guess the tuple and sequence** | **encryption; and it is why QUIC exists** |
| **Connection exhaustion** | **state tables are finite** | rate limiting |

The SYN flood is worth remembering as a design lesson rather than an attack: the server
allocates memory in response to an unauthenticated packet. Any protocol that does work
before authenticating is vulnerable to the same shape of attack, and SYN cookies' answer —
encode the state in the sequence number and hold none — is the general remedy.

## Application

Where most exploitation actually happens, and it is largely outside this book's scope —
with three exceptions that are squarely networking.

**DNS** (Chapter 39). Cache poisoning, hijacked registrar accounts, and DNS as an
exfiltration channel. DNS is the most attractive target in the stack, because
controlling name resolution controls where traffic goes, without touching routing.

**Credential attacks against network services.** SSH, RDP, VPN portals and management
interfaces facing the Internet, attacked continuously by §57.1's opportunists. MFA is the
most effective control available, and it is the one that most reliably prevents the
initial access step of §57.1's ransomware chain.

**Protocol abuse of management planes.** SNMP with default communities (Chapter 54 §54.2),
unauthenticated NETCONF, vendor management protocols that were never meant to be exposed.

## Human

**The row that decides most incidents.**

| Vector | |
|---|---|
| **Phishing** | **credential harvesting, or a payload** |
| **Pretexting** | **"this is IT, I need to verify your password"** — and the modern version calls the service desk to reset MFA |
| **Physical social engineering** | **tailgating, a high-visibility jacket, a delivery** |
| **Supply chain** | **the trusted supplier's compromised update** |

**What a network engineer can actually contribute:**

- **Assume the human control fails.** Design so that a compromised laptop is contained
  (§57.1's step 4), rather than relying on nobody clicking.
- **MFA everywhere**, particularly on remote access and management.
- Make the safe path the easy path. A password policy that forces reuse, or a VPN so
  slow that people work around it, produces the behaviour it was meant to prevent.
- **Egress monitoring** (Chapter 54 §54.4). You will not see the click; you may see what
  happens next.

> The service desk MFA reset is the current weak point in most organisations, and it is a
> process problem rather than a technical one. An attacker who can persuade a helpdesk to
> reset a factor has defeated the factor.

## Reading the table as a design tool

The value of the enumeration is that it turns "is the network secure?" into a set of
answerable questions.

**For each row, three questions:**

1. Is this exposure present here? (An air-gapped network has no BGP exposure.)
2. What control addresses it, and is it deployed?
3. How would I know if it were being exploited? — and this third question is the one
   usually unanswered.

The third question is where Chapter 54 meets Chapter 57. A control with no
detection is a control you are trusting, and most attack surfaces in the table above have a
detection signature — MAC table churn, ARP anomalies, unexpected DHCP offers, BGP announcement
changes, SYN backlog depth, DNS query volume.

> **Enumerate the surface, deploy the control, and then instrument the control.** The third
> step is the one that is skipped, and it is why compromises are detected by third parties
> (§57.1).

## What breaks here

Every technical control in place and a compromise via phishing. **Expected.** Design for
containment rather than prevention.

**An unlocked comms room.** Every control above it is bypassed. This is not facilities'
problem.

Layer 2 controls absent because "it is the internal network". The internal network is
where the compromised laptop is.

**BCP 38 not deployed.** You are contributing to everyone else's problem, and it is
inexpensive.

**Management interfaces on public addresses.** The largest opportunistic exposure,
and it is found within minutes.

**A control deployed with no detection.** You are trusting it. Instrument it.

**MFA defeated by a helpdesk reset.** A process gap, not a technical one, and it is
currently the most productive route into well-defended organisations.

> **Network+ note.** Objective 4.2 covers attack types directly, and this section is the map
> for them. Over-learn: on-path (formerly man-in-the-middle), DoS/DDoS, VLAN hopping, MAC
> flooding, ARP spoofing, rogue DHCP, DNS poisoning, and social engineering; and the
> corresponding hardening controls — port security, DAI, DHCP snooping, BPDU/root guard,
> disabling unused ports and changing default credentials. This section and Chapter 62 cover
> the whole of that objective.
