# Chapter 28 — Further Reading

## Primary sources

**RFC 8200 — Deering, S. & Hinden, R. (2017). *Internet Protocol, Version 6 (IPv6)
Specification.***
The current specification, replacing RFC 2460. **Read §3 and compare the header with
IPv4's** (Chapter 24 §24.2): the IPv6 header is *simpler* despite four times the address
size, and understanding why is worth the twenty minutes.

**RFC 4291 — Hinden, R. & Deering, S. (2006). *IP Version 6 Addressing Architecture.***
The address types of §28.2, in full.

**RFC 5952 — Kawamura, S. & Kawashima, M. (2010). *A Recommendation for IPv6 Address Text
Representation.***
Eight pages on how to write an address. Sounds trivial; it is the difference between
your ACLs matching and not.

**RFC 4862 — Thomson, S., Narten, T. & Jinmei, T. (2007). *IPv6 Stateless Address
Autoconfiguration.***
SLAAC, and the DAD procedure.

**RFC 4861 — Narten, T. et al. (2007). *Neighbor Discovery for IP version 6.***
NDP. Chapter 18 §18.4 covers it; §7 is the core.

**RFC 4941 / RFC 7217 / RFC 8064 — privacy and stable-privacy addresses.**
The response to EUI-64 tracking. RFC 8064 makes stable-privacy the recommended default.

**RFC 8415 — Mrugalski, T. et al. (2018). *DHCP for IPv6.***
DHCPv6, consolidated. Note what it does not provide.

**RFC 8106 — Jeong, J. et al. (2017). *IPv6 Router Advertisement Options for DNS
Configuration.***
RDNSS. The four pages that closed SLAAC's decade-long gap.

**RFC 8305 — Schinazi, D. & Pauly, T. (2017). *Happy Eyeballs Version 2.***
**Read this.** Short, and it is the mechanism that made IPv6 deployment safe to attempt.

**RFC 6146 (NAT64), RFC 6147 (DNS64), RFC 6877 (464XLAT).**
The transition mechanisms that actually work. RFC 6877 is what runs on your phone.

**RFC 7526 — Troan, O. & Carpenter, B. (2015). *Deprecating 6to4.***
Why an automatic zero-configuration mechanism failed. **A mechanism with no accountable
operator has no one to fix it.**

**RFC 4890 — Davies, E. & Mohacsi, J. (2007). *Recommendations for Filtering ICMPv6
Messages in Firewalls.***
**Read before writing any IPv6 firewall rule.** Blocking ICMPv6 breaks IPv6 entirely.

## Books

**Hagen, S. (2014). *IPv6 Essentials*, 3rd ed. O'Reilly.**
**The best single book on IPv6.** Thorough, practical, and good on the operational
differences rather than only the protocol.

**Horley, E. (2013). *Practical IPv6 for Windows Administrators.* Apress.**
Narrower than the title suggests and unusually good on the deployment decisions of §28.3
— the M/O flag matrix and its consequences are explained better here than anywhere.

**Coffeen, T. (2014). *IPv6 Address Planning.* O'Reilly.**
Short, and directly useful: how to allocate a /48 or /32 sensibly, with worked plans.
The IPv6 counterpart to Chapter 26 §26.4.

**Chown, T. et al. — the RIPE and NIST IPv6 deployment guides.**
Free, practical, and written by people who have done it. RIPE-554 (procurement
requirements) is genuinely useful when buying equipment.

**Gont, F. — the IPv6 security work.**
His SI6 Networks toolkit documentation and his RFCs are the systematic treatment of IPv6
attack surface. **Read this before deciding IPv6 is "just IPv4 with bigger addresses".**

## Applied

**Get IPv6 and use it.** This is not optional if you want to understand it.

- Check what you have: **`test-ipv6.com`** or `curl -6 https://ifconfig.co`
- If your ISP provides it, enable it and see what happens
- If not, **Hurricane Electric's tunnelbroker.net** gives you a routed /48 in about ten
  minutes, free

**`ip -6 addr`, `ip -6 route`, `ip -6 neigh`.** Look at a working host and learn what
normal looks like — particularly the several global addresses of §28.3, which surprise
people the first time.

**`rdisc6 eth0`** (the `ndisc6` package). **The key diagnostic tool.** Dumps Router
Advertisements in full including the M and O flags, so you can predict what a host will do
and then verify it.

**`ping6 ff02::1%eth0`** and **`ping6 ff02::2%eth0`** — all nodes, all routers. Do this on
any network with IPv6 and see what answers.

**`tcpdump -i eth0 icmp6`** while bringing an interface up, and watch the full SLAAC
sequence: RS, RA, neighbour solicitations for DAD, and the addresses appearing. **This is
exercise F2 and it makes §28.3 permanent.**

**Python's `ipaddress` module** for compression practice:

```python
import ipaddress
ipaddress.IPv6Address('2001:0db8:0000:0000:0000:ff00:0042:8329').compressed
ipaddress.IPv6Address('2001:db8::1').exploded
list(ipaddress.IPv6Network('2001:db8::/48').subnets(new_prefix=64))[:4]
```

**Audit your own network for unintended IPv6** — exercise F6. Try to ping a neighbour by
link-local address; run `rdisc6`; check whether your firewall has IPv6 rules. **Most
people are surprised.**

**Lab 15** in this book's [labs/](../../../labs/) directory builds a dual-stack segment,
captures the SLAAC exchange, demonstrates a rogue RA and defeats it with RA Guard, and
shows the ICMPv6-filtering failure deliberately.

## Measurement

**Google IPv6 statistics** (google.com/intl/en/ipv6/statistics.html) — the canonical
adoption figure, with a per-country breakdown that repays study.

**APNIC's measurements** (stats.labs.apnic.net) — Huston's data, more detailed and
broken down by network.

**Facebook, Akamai and Cloudflare IPv6 dashboards** — different measurement
methodologies, useful for triangulating.

## For the certification-minded

Objective 1.8 is IPv6 addressing and objective 2.3 covers SLAAC. **Compression is
examined directly.**

Eight things worth over-learning:

1. **The compression rules**, and that **`::` may appear only once**.
2. **`2000::/3` global, `fe80::/10` link-local, `fd00::/8` unique local, `ff00::/8`
   multicast, `::1` loopback, `::` unspecified.**
3. **Every subnet is a /64.**
4. **EUI-64 inserts `ff:fe` and flips the 7th bit.**
5. **IPv6 has no broadcast** — `ff02::1` replaces it.
6. **DHCPv6 uses ports 546/547** and **never provides a default gateway** — it always
   comes from the RA. **The single most examined IPv6 operational fact.**
7. **Dual-stack, tunnelling and NAT64/DNS64** as the three transition approaches, and
   **dual-stack does not solve address exhaustion.**
8. **`fe80::` is normal in IPv6**, unlike IPv4's `169.254.x.x`.

And the operational point worth more than the whole objective: **IPv6 is enabled by
default on every modern operating system and is preferred over IPv4.** If you have not
deployed it, you have not avoided it — you have an unmonitored, unfiltered path through
your network, and the only question is whether you know about it.
