# Chapter 40 — Important Concepts

Four values, or nothing works *(§40.1)* — address, mask, gateway, resolver. Three
of the four produce distinctive failures when wrong, which is why Chapter 22 §22.4's method
checks them in order.

The count is not the problem; the rate of change is *(§40.1)* — As with static routing.
Mobility alone made manual assignment untenable once portable computers existed, because
every move is a different subnet.

Manual assignment's characteristic failure *(§40.1)* — **Duplicate addresses**, whose
symptom is intermittent, host-dependent connectivity that changes as ARP caches expire.
Partial, intermittent and host-specific — the most confusing kind.

RARP → BOOTP → DHCP *(§40.1)* — RARP returned **only an address** and could not cross
a router. BOOTP ran over UDP/IP so a relay could forward it, and returned mask,
gateway and a boot file — but its assignments were static. BOOTP solved "how does a
host learn its settings"; DHCP solved "who decides what they are".

The inheritance is visible *(§40.1)* — DHCP uses BOOTP's **ports 67/68**, its packet
format and its relay mechanism. A DHCP packet is a BOOTP packet with the vendor field
repurposed, which is why `tcpdump` labels everything "BOOTP/DHCP".

The lease is the distinguishing idea *(§40.1)* — An address is granted for a bounded
time and reclaimed if not renewed. Necessary because **addresses are finite**, hosts
vanish without saying goodbye, and a lease expiry is a natural point at which a host
re-asks — so a configuration change propagates within a lease time with no host touched.

The lease time is two things at once *(§40.1)* — the period over which a change reaches
every host, and the period a host survives without the server. They pull in opposite
directions, and the choice is which matters more.

DHCPv6 never supplies a gateway *(§40.1)* — It always comes from the Router
Advertisement, and Android does not implement DHCPv6 at all, so a stateful-only IPv6
network silently fails for phones.

**DORA** *(§40.2)* — Discover, Offer, Request, Acknowledge.

The client broadcasts because it must *(§40.2)* — Source **`0.0.0.0`** because it has no
address; destination **`255.255.255.255`** because it does not know a server exists. The
bootstrap problem, and broadcast is the only mechanism that solves it.

The server broadcasts too *(§40.2)* — Because the client does not yet hold the offered
address, so a unicast would require ARPing for an address nobody answers for.

The REQUEST is broadcast to decline the other offers *(§40.2)* — It carries the chosen
server identifier (option 54), so every other server that offered hears it and
**releases its reservation.** The detail the acronym hides, and the reason the third
message is not a unicast confirmation.

Several servers may offer, and there is no arbitration *(§40.2)* — The client takes the
first. This is how redundancy works and how a rogue server does damage — there is no
authentication and no way to tell a real server from a laptop.

Option 55 explains a common confusion *(§40.2)* — The client lists what it wants and the
server returns **only those**. So an option configured on the server may never reach a
device because it never asked.

NAK makes mobility work *(§40.2)* — A laptop requesting yesterday's office address on a
home network is refused, discards its configuration, and starts afresh. **Normal, not
alarming.**

Renewal: T1 at 50%, T2 at 87.5% *(§40.2)* — At **T1** the client **unicasts** to its own
server (two messages, no broadcast); at **T2** it broadcasts to any server; only at expiry
does it give up. A client with a 24-hour lease survives a twelve-hour outage without
noticing.

Which is why a DHCP outage is discovered hours later *(§40.2, §40.4)* — Running hosts
renew quietly; only booting hosts fail, so the first report comes from whoever restarts a
machine the next morning. DHCP needs monitoring rather than complaint-driven discovery.

**INFORM** *(§40.2)* — A statically-addressed host asking for the **other** options — DNS,
domain, NTP, proxy — with no address and no lease.

A pool must not overlap anything assigned by hand *(§40.3)* — The most common
DHCP misconfiguration, and the failure is delayed: a pool configured in January produces a
duplicate in June when it reaches that address.

Reservations beat static configuration *(§40.3)* — The address lives **on the server**
rather than the device, so it is centrally visible, centrally changeable, and survives a
device rebuild. Use them for printers, cameras, APs and phones; use true statics only for
things that must work when DHCP does not.

MAC randomisation breaks reservations *(§40.3)* — Modern phones and laptops randomise
per network, so a reservation keyed on MAC may stop matching after an update.

Options 60 and 43 are how devices self-provision *(§40.3)* — Option 60 says what kind
of device this is; option 43 returns vendor-specific data for that kind — the
controller's address for an access point, the call manager's for a phone. This is why an
AP unboxed and plugged in finds its controller, and why "the new AP will not join" is
usually a DHCP question.

Option 121 has a trap *(§40.3)* — If it is present, option 3 is ignored, so the
default route must be listed explicitly or the client has none.

**Redundancy** *(§40.3)* — **Split scopes** (conventionally 80/20) are simple and waste pool
capacity; **DHCP failover** shares the pool and synchronises lease state. Kea stores
leases in a database, making redundancy a database problem — worth knowing, since ISC DHCP
reached end of life in 2022.

Pool utilisation deserves an alert at 80% *(§40.3)* — Because the remedy takes planning
and the failure is total for new clients.

**The relay agent** *(§40.4)* — A router forwarding client broadcasts as **unicast** to a
central server. Without it, sixty VLANs need sixty servers. **`ip helper-address`**.

`giaddr` is the key field *(§40.4)* — The relay inserts its own address on the
client's subnet, and that is how the server knows which scope to use. A server with no
scope matching a `giaddr` silently ignores the request — and says so in its log.

**Option 82** *(§40.4)* — The relay adding **which switch port** the client is on. Used for
location-based assignment, and it is what **DHCP snooping** builds its binding table from.

`ip helper-address` forwards eight services, not one *(§40.4)* — Including DNS, TFTP and
NetBIOS. Restrict it with `no ip forward-protocol udp ...`.

`169.254.x.x` — the causes in order *(§40.4)* — Wrong VLAN on the port (the commonest
in enterprises), missing relay, missing scope for that `giaddr`, pool exhausted, server
down, **missing PortFast**, cable.

"It works if I release and renew" means PortFast *(§40.4)* — The port took 30 seconds to
forward and DHCP timed out at boot; by the time you retry, it is forwarding.

One host, one subnet, or everything *(§40.4)* — The first branch in the diagnosis.
One host means its port or cable; one subnet means the relay or the scope; everything means
the server. This question eliminates most of the search space.

**Rogue DHCP servers** *(§40.4)* — No authentication, and the first answer wins. The
accidental cases — a home router used as a switch, a hypervisor's NAT network, connection
sharing — are more common than the malicious. The malicious version supplies its own
address as the gateway and is a man-in-the-middle established by answering a broadcast.

The symptoms vary between hosts *(§40.4)* — Because it depends on which server answered
first. Addresses from an unexpected range — `192.168.x.x` on a `10.x.x.x` network — is the
giveaway.

**DHCP snooping** *(§40.4)* — The switch drops server messages on untrusted ports. One
line per switch plus one trusted port, and it eliminates an entire class of accident and
attack. And it builds the binding table that Dynamic ARP Inspection needs — which is why
DAI without snooping drops everything.

**DHCP starvation** *(§40.4)* — Exhaust the pool with spoofed MACs so that legitimate
clients fall back to the attacker's rogue server. Bounded by snooping rate limits and port
security.

The server's log is the most under-used diagnostic *(§40.4)* — It says "no free leases",
"unknown network segment for giaddr", and every DISCOVER it declined, in plain text.
