# Chapter 62 — Attacks on the Stack

This chapter walks back down through everything we built and breaks it.

It is the payoff of the unit and, in a sense, of the book, because you can only attack
a mechanism you understand. Every attack here is the inverse of a chapter you have
already read: ARP spoofing is Chapter 18 with the trust assumption removed; VLAN
hopping is Chapter 20's native VLAN used as intended by someone with different
intentions; BGP hijacking is Chapter 32's rough consensus applied by a party who does
not share it.

That structure is deliberate. Learning attacks as a list is memorisation. Learning
them as *the natural consequence of a design assumption that no longer holds* is
understanding, and it generalises to the attacks that have not been invented yet.

## The layer two attacks

§62.1 covers the ones that require local access and are therefore the ones an insider,
a compromised device, or someone in a meeting room can perform.

**MAC flooding.** Chapter 17 §17.4 noted that a switch's address table is finite. Fill
it with fabricated addresses and the switch, unable to learn, floods every frame to
every port — degrading itself to a hub and restoring the eavesdropping that switching
removed. *Mitigation:* port security, limiting learned addresses per port.

**ARP spoofing.** Chapter 18 §18.3 noted that ARP has no authentication. Send
unsolicited replies claiming to be the default gateway and every host on the segment
sends you their traffic; forward it onward and neither party notices anything but a
small delay. This is the classic on-path attack and it takes one command with freely
available tools. *Mitigation:* Dynamic ARP Inspection, which validates ARP against the
DHCP snooping database.

**VLAN hopping.** Chapter 20 §20.3's double-tagging attack, exploiting the native
VLAN's untagged frames. *Mitigation:* set the native VLAN to an unused ID, never use
VLAN 1, disable dynamic trunking on access ports.

**Rogue DHCP.** Chapter 40 §40.3. Answer faster than the real server and you are the
default gateway. *Mitigation:* DHCP snooping with trusted ports.

**STP attacks.** Chapter 19 §19.2 elects the lowest bridge ID as root. Claim a lower
one and traffic reroutes through you — or simply collapses. *Mitigation:* BPDU Guard
and Root Guard.

Notice the shape common to all five: each exploits a protocol that was designed to
be helpful and trusting on a local segment. And notice that every mitigation is a
switch feature that exists, is free, is off by default, and is configured in one or
two lines. Access-layer hardening is among the highest-value security work available,
and it is skipped constantly.

## Spoofing, poisoning, and being in the middle

§62.2 covers the attacks that manipulate the mechanisms that turn one kind of
identifier into another.

**IP spoofing** — forging a source address, which nothing in IP prevents. It underlies
the reflection attacks of §62.3, and the mitigation is **BCP 38** source address
validation at the network edge: a provider should not accept, from a customer, packets
whose source address the customer does not own. It is twenty-six years old, it would
eliminate an entire attack class, and its deployment remains incomplete — which is a
useful case study in why security improvements requiring universal cooperation do not
happen.

**DNS cache poisoning** — inserting false records into a resolver's cache. Dan
Kaminsky's 2008 discovery made this dramatically more practical than had been believed
and prompted a coordinated multi-vendor patch, and the deeper fix — DNSSEC — remains
partially deployed.

**On-path attacks** (the term the industry now uses in preference to
"man-in-the-middle") in their various forms, and the general defence: authenticated
encryption end to end. An attacker who can read and modify traffic can do neither
usefully if the traffic is authenticated, which is why the answer to "is public Wi-Fi
safe" is now "yes, for anything using TLS properly."

**Downgrade attacks**, which are worth a section of their own because they defeat
otherwise-sound cryptography by attacking the *negotiation* rather than the algorithm —
forcing both parties to agree on something weak. Chapter 58's account of TLS 1.3's
removals is the story of this class being systematically eliminated, and WireGuard's
refusal to negotiate at all (Chapter 61 §61.3) is the alternative approach.

## Denial of service

§62.3 covers the attacks against availability, and the arithmetic is what makes them
comprehensible.

**Volumetric** attacks exhaust bandwidth. The key technique is **amplification**: send
a small forged request to a service that produces a large response, so the victim
receives the response and the attacker spends almost nothing. Chapter 38 §38.3
introduced this; the factors are worth tabulating because they explain why particular
protocols get abused:

| Service | Approximate amplification |
|---|---|
| DNS | 28–54× |
| NTP monlist | up to 557× |
| memcached | up to 51,000× |

The 1.35 Tb/s attack on GitHub in February 2018 used memcached servers that should
never have been reachable from the Internet at all. Every one of these attacks
requires source address spoofing, and every one would be impossible with universal BCP
38 deployment.

**Protocol** attacks exhaust state rather than bandwidth. The SYN flood is the classic:
send SYNs, never complete the handshake, and fill the server's half-open connection
table (Chapter 37 §37.1). *Mitigation:* SYN cookies, which encode the necessary state
into the sequence number so that no server-side state is allocated until the handshake
completes — an elegant piece of engineering worth studying.

**Application** attacks exhaust processing, using few, entirely legitimate-looking
requests that happen to be expensive to serve. The hardest to defend, because there is
nothing anomalous about any individual request.

The defence, honestly stated: you cannot absorb a large volumetric attack on your
own connection. If the attack exceeds your circuit capacity, the circuit is full
before your equipment sees anything, and no firewall helps. Mitigation must happen
upstream, which means a scrubbing service or a provider with capacity — and that is a
commercial arrangement made in advance, not a technical response made during an
incident.

## Putting it back together

§62.4 assembles the defensive posture, layer by layer, and gives the hardening
checklist that Chapter 72's design chapter will expect you to apply.

The framing worth keeping: every mitigation in this chapter is cheap, and almost
none of them are enabled by default. Port security, DHCP snooping, DAI, BPDU Guard,
disabling unused ports, changing default credentials, disabling unused services, SNMPv3
instead of v2c, management on a separate VLAN, and BCP 38 filtering — that list would
eliminate most of the attacks in this chapter, costs nothing but configuration time,
and is absent from a large proportion of production networks.

Doing the ordinary things well, as Unit XII's introduction argued, beats doing the
exotic things at all.

## By the end you will be able to

- Explain each layer-two attack mechanistically and state the specific feature that
  mitigates it.
- Explain IP spoofing's role in reflection attacks and what BCP 38 would prevent.
- Compute amplification factors and identify which services are dangerous to expose.
- Explain SYN flooding and how SYN cookies defeat it without state.
- Explain why volumetric attacks cannot be mitigated at the victim's edge.
- Produce a hardening checklist for an access switch and justify each item by the
  attack it prevents.
