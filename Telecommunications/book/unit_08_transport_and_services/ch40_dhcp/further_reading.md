# Chapter 40 — Further Reading

## Primary sources

**RFC 2131 — Droms, R. (1997). *Dynamic Host Configuration Protocol.***
**The specification.** Forty-five pages, and it is readable. **Read §3 (the protocol) and
§4.4 (the client state machine)** — the renewal timers of §40.2 are stated there with the
reasoning, and the state machine explains several behaviours that look arbitrary otherwise.

**RFC 2132 — Alexander, S. & Droms, R. (1997). *DHCP Options and BOOTP Vendor
Extensions.***
**The option catalogue.** Not for reading through; for looking things up in. Bookmark it.

**RFC 951 — Croft, B. & Gilmore, J. (1985). *Bootstrap Protocol.***
BOOTP. Worth reading to see how much of DHCP is inherited — the packet format is
essentially unchanged.

**RFC 3046 — Patrick, M. (2001). *DHCP Relay Agent Information Option.***
Option 82, and the foundation of DHCP snooping's binding table.

**RFC 3442 — Lemon, T. & Cheshire, S. (2003). *The Classless Static Route Option for
DHCPv4.***
Option 121 — **and §5, which states that option 3 must be ignored when 121 is present.**
This is the trap of §40.3, stated normatively.

**RFC 8415 — Mrugalski, T. et al. (2018). *Dynamic Host Configuration Protocol for IPv6.***
DHCPv6, consolidated. Note what it does not provide, and read it against Chapter 28 §28.3.

**RFC 3074 — Volz, B. et al. — the DHCP failover protocol drafts.**
The failover mechanism of §40.3. The standardisation history is messy; the vendor
implementations converged anyway.

## Books and guides

**Droms, R. & Lemon, T. (2002). *The DHCP Handbook*, 2nd ed. Sams.**
**By the protocol's author and its principal implementer.** Dated on specifics and
authoritative on everything conceptual — particularly the failover and relay chapters, which
nothing else covers as well.

**ISC Kea Administrator Reference Manual** (kea.readthedocs.io).
**The current reference for anyone deploying DHCP today**, and the migration guidance from
ISC DHCP is worth reading even if you are not migrating, because it explains what the old
server's assumptions were.

**Microsoft's DHCP documentation** and the Windows Server DHCP failover guide.
Whatever your platform preferences, Windows DHCP is what a great many enterprises run, and
its failover implementation is well documented.

**Cisco's DHCP snooping and DAI configuration guides.**
**Read the snooping guide before enabling DAI** — Chapter 18 §18.3's dependency is spelled
out there, and the failure mode of getting it wrong is a total outage.

## Applied

**Capture a full DORA exchange.** Exercise F1, and it takes two minutes:

```bash
tcpdump -i eth0 -nn -v port 67 or port 68
# in another terminal:
sudo dhclient -r eth0 && sudo dhclient eth0
```

**In Wireshark, filter `dhcp`** and expand the options. **Find option 53** in each packet —
it is what distinguishes DISCOVER from REQUEST, and seeing it makes the four-message
structure concrete.

**`ipconfig getpacket en0`** on macOS prints every option the client received, decoded. **It
is the fastest way to answer "did the server actually send option 43?"** and there is no
equally direct equivalent on other platforms.

**Compare what different clients request** (exercise F2):

```bash
# capture DISCOVER from a Windows machine, a Linux machine and a phone
# then compare option 55 in each
```

**The differences are substantial**, and they explain why an option that works for one
device type does not reach another.

**Run a server.** `dnsmasq` is the easiest for a lab — a single configuration file — and
**ISC Kea** is the one to learn if you will run this in production:

```
# dnsmasq.conf — a complete lab DHCP server
interface=eth1
dhcp-range=10.1.5.100,10.1.5.200,12h
dhcp-option=3,10.1.5.1
dhcp-option=6,10.1.1.53,10.1.1.54
dhcp-host=aa:bb:cc:dd:ee:ff,10.1.5.50
log-dhcp
```

**`log-dhcp` is the important line.** The server's log is the most under-used diagnostic in
this chapter (§40.4), and it says explicitly what it did and why.

**Build a relay** (exercise F3). Two subnets, a router between them, `ip helper-address` or
`dhcrelay`, and **verify `giaddr` in a capture.** Watching the server pick the right scope
from `giaddr` is what makes §40.4's mechanism click.

**Run a rogue server on an isolated segment** (exercise F4) and watch which clients it
captures. **Then enable DHCP snooping and repeat.** The before-and-after is the most
persuasive security demonstration in this unit, and it takes fifteen minutes.

**Set a very short lease and watch the timers** (exercise F5):

```
dhcp-range=10.1.5.100,10.1.5.110,4m
```

**Capture for ten minutes** and confirm renewal happens at 2 minutes (T1) and, if you block
the server, a broadcast at 3.5 minutes (T2). **The percentages become memorable once
observed.**

**Check your own network:**

```bash
# What did I get, and from whom?
ipconfig /all                    # Windows: server address and lease dates
nmcli device show                # Linux
ipconfig getpacket en0           # macOS

# Is there more than one server answering?
sudo nmap --script broadcast-dhcp-discover
```

**That last command is worth running once on any network you are responsible for.** If it
reports two servers and you expected one, you have found something.

**Lab 29** in this book's [labs/](../../../labs/) directory builds a two-subnet topology
with a relay, captures DORA on both sides to show the `giaddr` mechanism, then introduces a
rogue server, demonstrates the man-in-the-middle, and defeats it with snooping — and finally
enables DAI to show the binding-table dependency.

## For the certification-minded

Objective 1.6 expects DHCP's purpose; objective 2.3 expects its operation, scopes,
reservations and relays; objective 4.2 expects rogue DHCP servers.

**Seven things worth over-learning:**

1. **DORA** — Discover, Offer, Request, Acknowledge.
2. **UDP ports 67 (server) and 68 (client).**
3. **The client broadcasts from `0.0.0.0` to `255.255.255.255`.**
4. **T1 at 50% of the lease, T2 at 87.5%.**
5. **A relay agent (`ip helper-address`) forwards broadcasts to a central server** — needed
   because broadcasts do not cross routers.
6. **Scope, exclusion, reservation** — and that exclusions keep static addresses out of the
   pool.
7. **Options 3 (gateway), 6 (DNS), 51 (lease), 66/67 (TFTP boot).**

**`169.254.x.x` is examined**, and the expected answer is that the client could not reach a
DHCP server.

And the three operational facts worth more than the objective:

**When one host fails DHCP, check its VLAN. When a subnet fails, check the relay. When
everything fails, check the server.** One question — how wide is it? — narrows it to one of
three causes.

**"It works if I release and renew" means PortFast is missing.** The port was still
listening when the client asked.

**Enable DHCP snooping.** It is one line, it costs nothing, and it prevents both the
accidental home-router incident and the deliberate man-in-the-middle. **And Dynamic ARP
Inspection cannot work without it.**
