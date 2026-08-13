# Chapter 40 — DHCP

A host cannot participate in an IP network until it has four things: an address, a
subnet mask, a default gateway, and at least one DNS resolver. Chapter 25 §25.3
showed that without the first two it cannot even decide whether a destination is
local; without the third it cannot reach anything remote; without the fourth it can
reach things only by number.

For a long time, the way a host got these was that somebody typed them in.

Consider what that means at scale. Two hundred workstations, each configured by hand
from a spreadsheet. Every new machine requires a visit and a free address someone has
to find. Every decommissioned machine leaves an entry that may or may not get
released. Change the DNS server and you touch two hundred machines. And the failure
mode of a mistake — two hosts assigned the same address — produces intermittent,
baffling connectivity for both, with no error message anywhere and nothing in either
machine's configuration that looks wrong.

Now add laptops, which move between buildings. Add phones. Add the visitor in
reception. Manual addressing does not scale down gracefully into the mobile world;
it simply stops being possible.

**DHCP** — RFC 2131, March 1997, building on the earlier BOOTP — automates it, and
it does considerably more than hand out addresses.

## DORA

Four messages, and the acronym is worth learning because the packet names are what
you will see in a capture.

**DISCOVER.** The client has no address, so it broadcasts from `0.0.0.0` to
`255.255.255.255`, saying in effect: *is there a DHCP server?* Note the bootstrapping
problem this solves — a host with no address cannot send a unicast packet, so the
protocol must work entirely in broadcast until an address exists.

**OFFER.** A server responds with a proposed address and configuration. If several
servers exist, several offers arrive.

**REQUEST.** The client broadcasts its acceptance of one offer — broadcast rather
than unicast so that the *other* servers learn their offers were declined and can
release the addresses they had tentatively reserved.

**ACK.** The chosen server confirms, and the client may now use the address.

Two details that matter operationally. Before using the address, a well-behaved
client sends a gratuitous ARP (Chapter 18 §18.3) to check that nobody else is using
it — belt and braces against a server whose lease database is wrong. And the client
begins renewing at 50% of the lease duration, retrying at 87.5%, so a brief server
outage is invisible.

## The options, which are the real content

The address is the least interesting thing DHCP delivers. The **options** field
carries a catalogue of configuration that quietly runs the enterprise, and knowing
that it exists is what separates someone who can deploy a phone system from someone
who cannot.

| Option | Carries |
|---|---|
| 1 | Subnet mask |
| 3 | Default gateway |
| 6 | DNS servers |
| 15 | Domain name |
| 42 | NTP servers |
| 43 / 60 | Vendor-specific — how wireless APs find their controller |
| 51 | Lease time |
| 66 / 67 | TFTP server and boot filename — how a device network-boots |
| 82 | Relay agent information — which switch port the request came from |
| 119 | DNS search list |
| 150 | TFTP servers (Cisco) — how IP phones find their configuration |

Options 66, 67, 43 and 150 are the ones people meet unexpectedly. A phone or access
point that boots, gets an address, and then does nothing is almost always missing the
option that tells it where its controller or configuration lives.

Option 82 is worth a special note: it lets the *relay agent* record which physical
switch port a request arrived on, which makes port-based address assignment and
location tracking possible, and which is the foundation of DHCP snooping's security
model.

## Relays, and the broadcast problem

DHCP works by broadcast. Routers do not forward broadcasts (Chapter 17 §17.3). So a
DHCP server on one subnet cannot hear clients on another, and the naive conclusion is
that every subnet needs its own server.

The actual answer is the **relay agent** (`ip helper-address` in Cisco syntax): the
router listens for DHCP broadcasts, converts them to unicast, and forwards them to a
central server, inserting the receiving interface's address so the server knows which
scope to allocate from.

This is one of the most useful things to understand about DHCP, because the symptom
when it is missing is characteristic and confusing: **clients on the subnet with the
server work perfectly; clients on every other subnet get APIPA addresses.** Someone
troubleshooting the failing subnet finds nothing wrong with it, because nothing is
wrong with it — the router simply is not relaying.

## The failure modes

§40.4 catalogues them, and they are among the most recognisable symptoms in the book.

**A 169.254.x.x address** (Chapter 27 §27.2) means the client asked and nobody
answered. The client is fine. Look at the relay, the server, and the path between
them — in that order.

**Pool exhaustion.** All addresses leased. New clients fail; existing ones are fine.
Common causes: too-long leases on a network with high turnover (a conference space
with an 8-day lease is a classic), or a subnet that has simply outgrown its /24.

**A rogue DHCP server.** Someone plugs in a home router backwards, and its LAN port —
which runs a DHCP server — now serves your network. Clients get addresses from the
wrong subnet with the wrong gateway, and the ones that get them depend on which
server replied first, so the failure is *random per client*. This is one of the
purest examples of an intermittent, non-reproducible fault with a completely
deterministic cause. The mitigation, **DHCP snooping**, designates trusted ports and
drops server messages from all others, and it is one of the highest-value access-layer
security features available.

**Duplicate addresses**, from a static address inside the dynamic pool. The fix is
the discipline from Chapter 27 §27.4: reserve an explicit static range outside the
pool, and write it in the plan.

## By the end you will be able to

- Explain why manual addressing fails and what specifically DHCP replaces.
- Trace DORA in a capture, identifying source and destination addresses at each step
  and explaining why each is broadcast or unicast.
- Explain lease renewal timing and predict client behaviour during a server outage.
- Configure a scope with reservations, exclusions and the options a described
  deployment requires.
- Explain relay agents and diagnose the missing-relay symptom immediately.
- Recognise and diagnose APIPA, pool exhaustion, rogue servers and duplicate
  addresses from their symptoms.
