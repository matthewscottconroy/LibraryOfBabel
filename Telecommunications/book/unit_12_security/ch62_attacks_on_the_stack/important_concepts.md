# Chapter 62 — Important Concepts

The switch's behaviour under MAC flooding is correct *(§62.1)* — Flooding unknown
destinations is what a switch is specified to do, and it is what makes the network work when a
device is silent. The attack exhausts the state that prevents it. A 32,000-entry table
fills in a third of a second at 100,000 frames/s, and the tool has existed since 1999.

Port security's maximum is the practical difficulty *(§62.1)* — Too low produces support
calls; too high does nothing. `restrict` is usually the right violation action — `protect`
hides the attack and `shutdown` is a denial of service against the user.

DTP negotiates trunks with whoever asks *(§62.1)* — A port at `dynamic auto` becomes a
trunk if the attacker's device requests it, and then they receive every VLAN. `switchport
mode access` and `switchport nonegotiate` is one line each and frequently absent. There is
no legitimate reason for an access port to negotiate anything.

Double tagging is one-way and that is sufficient *(§62.1)* — The reply cannot be
double-tagged back, so it is injection rather than conversation — and injection is enough
for several attacks. Never use VLAN 1; set the native VLAN to an unused one; tag the native
VLAN where supported, which removes the mechanism.

Most rogue DHCP servers are accidents *(§62.1)* — A home router in a wall socket, or a
hypervisor's virtual network bridged to production, and it produces identical symptoms far
more often than an attack does. DHCP snooping stops both.

DHCP snooping builds the binding table everything else depends on *(§62.1)* — MAC, IP, VLAN,
port, lease. Dynamic ARP inspection uses it, which is why snooping must be deployed first.

ARP has no authentication at all *(§62.1)* — A host that receives a reply believes it,
and many accept unsolicited gratuitous ARP. `ettercap` makes the classic on-path attack a
single command. DAI checks every ARP on an untrusted port against the binding table.

Deploying IPv6 without RA Guard is deploying rogue DHCP with no snooping *(§62.1)* — A
Router Advertisement on a user port is an attack or an accident, and any Linux machine with
forwarding enabled produces one.

BPDU Guard plus PortFast is the most valuable Layer 2 hardening measure *(§62.1)* —
The lowest bridge ID wins and anyone may claim it, so an attacker becomes root — and so does
a small unmanaged switch someone plugged into two wall sockets. It stops the attack and the
far more common accident.

Every hardening line is one command, and the reason estates are unhardened is that nothing
breaks when they are absent *(§62.1, §62.4)* — Chapter 55 §55.1's invisibility argument.
Which is the argument for automating it: a standard applied by a tool is deployed; a standard
in a wiki is a document.

Strict uRPF is correct at an access edge and wrong where routing is asymmetric *(§62.2)* —
Chapter 60 §60.2. Loose mode or an ACL instead.

What an on-path position buys depends entirely on whether the traffic is encrypted *(§62.2)*
— Against plaintext, everything; against properly validated TLS, dropping and metadata.
The whole of Chapter 58 exists to make the second sentence true, which is why the attacks
that matter now defeat validation rather than cryptography.

SSL stripping does not break TLS; it ensures TLS is never used *(§62.2)* — The user sees no
certificate error because there is no certificate, and the only visible difference is a missing
padlock. HSTS fixes it after the first visit; preloading fixes the first visit.

FREAK and Logjam exploited cryptography that was deliberately weakened by law *(§62.2)* —
Export-grade code stayed in implementations for twenty years and attackers found they could
force its use. A deliberately weakened option that nobody removed became a vulnerability two
decades later, which is Chapter 55 §55.1's accumulation with a legal cause.

Kaminsky's insight was that a forged reply could carry an authority record *(§62.2)* —
poisoning the entire domain rather than one name. Source port randomisation was the emergency
fix; DNSSEC is the actual answer and its deployment is partial.

The DNS attacks that actually happen require no protocol weakness *(§62.2)* — Registrar
compromise, resolver substitution, domain shadowing, and subdomain takeover of a CNAME pointing
at a deleted cloud resource. Registry lock is under-used for high-value domains, and
dangling records accumulate because nobody removes them.

Session token theft defeats MFA because the authentication already happened *(§62.2)* —
Which is why it has become the dominant attack against organisations with good
authentication. Short lifetimes, token binding, and re-authentication for sensitive actions.

Every successful mitigation in §62.2 makes the attack impossible rather than visible
*(§62.2)* — Authenticate what was not authenticated; remove the option that can be downgraded
to; verify against an out-of-band source of truth. None of them is "detect the attack", and
detection-based approaches to this class have been consistently disappointing.

An application-layer attack may need a few hundred requests per second *(§62.3)* — The
bandwidth is trivial and the service is down. "We have a 100 Gb/s scrubbing contract"
answers one third of the problem.

Amplification requires spoofing and a service that over-answers *(§62.3)* — NTP `monlist`
at 556× needs 180 Mb/s for a 100 Gb/s attack; memcached at ~51,000× needs 2 Mb/s. The
1.35 Tb/s GitHub attack in 2018 used memcached, and the remedy — disable UDP by default,
filter the port — closed the vector in weeks. Amplification vectors are closed by fixing
the reflector, and nobody has an incentive to prioritise it until it is used.

Do not operate a reflector *(§62.3)* — An open resolver, NTP with `monlist`, exposed
memcached, an SSDP-responding device on a public address. Chapter 57 §57.4's externality: the
cost falls elsewhere. And BCP 38 would eliminate the whole category, because spoofing is
step 1.

SYN cookies hold no state for an incomplete handshake *(§62.3)* — The state is encoded in
the sequence number and allocated only when the ACK returns carrying it. Frequently enabled
only under attack, which means "if configured."

Every protocol denial-of-service exploits an asymmetry in work done *(§62.3)* — Slowloris,
slow POST, TLS renegotiation, HTTP/2 rapid reset. The general remedy is to make the client
prove effort or state before the server commits any.

Argon2id makes the login endpoint an amplifier *(§62.3)* — Deliberately slow is correct for
storage and dangerous for an endpoint. Rate limit per account as well as per source.

Mirai used 62 default credential pairs and found hundreds of thousands of devices *(§62.3)*
— Chapter 57 §57.1's opportunist, at scale. And the Dyn attack demonstrated dependency
concentration: the target was a DNS provider and the casualties were hundreds of unrelated
services.

RTBH completes the denial of service against yourself, deliberately *(§62.3)* — To protect
everything else. A real and appropriate tool that must be understood as what it is.

Prepare before you need it *(§62.3)* — Know your baseline, know your circuit's capacity,
have the scrubbing arrangement pre-arranged and rehearsed, know the out-of-hours number, and
check that your origin is not directly reachable — the address is frequently discoverable
through certificate transparency, DNS history or an unprotected mail server.

Defence in depth means no single control failure results in compromise *(§62.4)* — A
statement about independence, and Chapter 56 §56.2's shared fate applied to security.
Three firewalls from one vendor with one vulnerability are one firewall. Two controls managed
by the same compromised account are one control. For each control, ask what single event
defeats it, and count how many answers are the same event.

Assume breach, because the highest-value controls are not preventive *(§62.4)* —
Segmentation and least privilege contain; logging and baselines detect; offline backups and
tested recovery determine the outcome. You are not going to catch the intrusion; you are
limiting and detecting what follows.

Items 1 to 4 change outcomes more than three products *(§62.4)* — MFA on remote access and
administration; no defaults and no exposed management; patch what is known to be exploited;
offline credential-separated backups. Cheap, certain, unglamorous, and not what the security
market sells.

Almost nothing in this unit is new *(§62.4)* — Bellovin enumerated the protocol attacks in
1989; Saltzer and Schroeder gave the design principles in 1975. The gap between what is
known and what is deployed is where security work actually happens, and it is organisational and
economic rather than technical.
