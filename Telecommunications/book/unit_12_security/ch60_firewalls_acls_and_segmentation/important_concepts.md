# Chapter 60 — Important Concepts

**A firewall is a list of rules evaluated in order, each matching a property and specifying an
action** *(§60.1)* — **That has been the entire concept since 1988**, and the simplicity is what
lets you reason about it.

**Evaluation stops at the first match, so a permit below a covering deny is dead** *(§60.1)* —
**The symptom is a rule that is present, correct and has no effect**, and **the diagnosis is to
look upward, not at the rule itself.** **Hit counters make it immediate**; shadowing is not
reliably found by reading.

**There is an implicit deny whether or not you write it** *(§60.1)* — **The correct default**, and
**it means adding a service requires adding a rule.** **Write it explicitly with rate-limited
logging anyway**, because the implicit deny does not log and **denied traffic is exactly what you
want to see.**

**Rules are directional and traffic is bidirectional** *(§60.1)* — A stateless filter permitting
outbound HTTP must permit the return, and **that return rule lets an attacker walk through by
sourcing from port 80.** **`established` helps and is not sufficient — the flag is in the packet
and the attacker sets it.** **That weakness is exactly what stateful inspection was invented to
fix.**

**Blocking all ICMPv6 does not harden an IPv6 network; it stops it working** *(§60.1)* —
Neighbour discovery is IPv6's ARP, router advertisements are how hosts get addresses, and
**IPv6 routers do not fragment so Packet Too Big is mandatory.** **RFC 4890 names what must
pass.** **And in IPv4, filtering Type 3 Code 4 creates a PMTUD black hole** — small packets work,
large ones vanish, and the cause is a rule written years ago by someone being careful.

**A stateful firewall needs a rule only for the direction of initiation** *(§60.2)* — **The
return is permitted by the table, not by a rule**, which is what makes a modern policy readable
and closes the port-80 window.

**A UDP "connection" has no end, so the firewall invents one** *(§60.2)* — **An idle timeout of
30–180 seconds**, after which a peer sending unsolicited traffic finds the entry gone. **This is
why VoIP, VPN and IoT protocols specify keepalives shorter than typical firewall timeouts**, and
**a device that goes silent for five minutes and then expects a response is a support case
waiting to happen.**

**A SYN flood is an attack on the state table, not on the server** *(§60.2)* — **The firewall
protecting the server fails before the server does.** **250,000 SYN/s fills a two-million-entry
table in eight seconds.** SYN proxying, per-source limits and short half-open timeouts — **none
default on every platform.**

**A stateful firewall must see both directions** *(§60.2)* — **Asymmetric routing makes it drop
entirely legitimate traffic**, and the symptom is "some connections work and some do not."
**It appears during failover, which is to say during an incident** — Chapter 51 §51.3's
direct-connect failover breaks for exactly this reason.

**A stateful device in a stateless network reintroduces the constraint packet switching
removed** *(§60.2)* — **IP was designed so any packet can take any path.** Firewalls, load
balancers and NAT devices all require flow affinity and all break in the same circumstances.

**Encryption defeats ALGs exactly as the protocol becomes secure** *(§60.2)* — **The firewall
cannot parse what it cannot read**, so SIP over TLS needs STUN/TURN/ICE instead. **And "disable
the SIP ALG" is the first line of almost every VoIP troubleshooting guide**, because small-
business implementations rewrite headers wrongly.

**Connections per second is the figure that actually limits you** *(§60.2)* — **Not throughput.**
**A firewall passing 10 Gb/s of long flows may collapse under 40,000 new connections per second
at a fraction of that bandwidth.** **And "20 Gb/s firewall throughput" and "2.5 Gb/s threat
prevention throughput" appear on the same data sheet** — the second is the number that applies
to how you will configure it.

**Applications migrated to 443 because it was the port firewalls permitted** *(§60.3)* — **Which
destroyed the port as a classifier**, and the arms race between filtering and evasion has run
ever since. **Application identification is now approximate and its accuracy is falling**, as
Encrypted Client Hello and encrypted DNS close the remaining plaintext identifiers.

**TLS inspection is a man-in-the-middle attack you perform against your own users** *(§60.3)* —
Calling it anything else obscures what must be reasoned about. **It costs performance, a private
CA on every client whose compromise is total, broken certificate pinning, a single point of
plaintext for the whole organisation, and legal exposure.**

**A TLS-inspecting middlebox frequently negotiates weaker parameters than the client would
have** *(§60.3)* — **A documented finding, not a theoretical concern.** **The client sees a green
padlock signed by a CA it trusts and cannot know what was negotiated on its behalf** — **the
security indicator is measuring the wrong connection.** **Endpoint agents see plaintext without
breaking TLS**, which is why the market moved that way.

**The IDS/IPS false-positive asymmetry decides the deployment** *(§60.3)* — **An IDS at 1% false
positives is noisy; an IPS at 1% blocks legitimate traffic 1% of the time.** **The resulting
pressure is to tune it until it blocks almost nothing**, at which point it is an expensive IDS.
**Deploy in detection mode, tune for weeks, promote signatures individually.**

**A proxy terminates the connection, so the server only receives requests the proxy
constructed** *(§60.3)* — **A stronger property than inspecting packets in flight.** **Forward
proxies died with HTTPS and returned as cloud security services** — the architecture came back
and the location changed.

**Segmentation's value is quantifiable, which is unusual for a security control** *(§60.4)* —
**A flat 2,000-host network exposes 1,999 hosts to one compromise; twenty segments exposes 99.**
**It does not prevent the compromise; it changes the outcome from "the organisation" to "one
department."** **And crossing a boundary is a detection opportunity that lateral movement within
a flat network does not provide.**

**A DMZ is a zone from which a compromise cannot reach the internal network** *(§60.4)* — **Not
"where the public servers go."** **If the compromised web server can reach the internal database
with full credentials, the DMZ is decorative** — and that is the commonest DMZ failure, because
narrowing the rule took effort.

**A VLAN is not a security control; it is a broadcast domain** *(§60.4)* — **Two VLANs on one
switch are separated only until something routes between them, and in almost every network
something does.** **The control is the policy at the routing point.** **VRFs are stronger and
under-used** — no path exists unless one is deliberately created, which is a much better default
than "routed by default, filtered by policy."

**Nobody writes twelve million rules** *(§60.4)* — **500 workloads have 124,750 possible pairwise
flows.** **Microsegmentation is tractable only with label-based policy** generated from the
orchestration system — **which is why it succeeded in cloud and container environments first,
where the labels already exist.** **A project that begins by enumerating flows will not
finish.**

**In-band management fails exactly when you need it** *(§60.4)* — **During the outage.** A
misconfigured ACL that removes your own access, a routing failure, a switch that will not boot.
**A console server plus a cellular modem is inexpensive relative to a site visit**, which is the
comparison that wins the argument.

**The first three segments deliver most of the benefit** *(§60.4)* — **Management plane, backups,
and servers from workstations.** **Segmenting backups alone changes ransomware outcomes**
(Chapter 57 §57.1 step 5). **Starting with microsegmentation is how segmentation projects
fail.**

**Segmentation for scope reduction funds itself** *(§60.4)* — **A flat network puts everything in
PCI scope**, and **the assessment saving frequently exceeds the cost of segmenting** — a rare
case of a security control arguing for itself in terms a finance director accepts without
dispute.
