# 28.4 Transition and Coexistence

IPv6 is not backward compatible with IPv4 (§28.1). An IPv6-only host cannot speak to an
IPv4-only host, and no amount of cleverness at the endpoints changes that.

So the transition requires mechanisms, there have been many, most have failed, and the
few that work are worth knowing precisely.

## Dual-stack

**Run both protocols on everything.** The recommended approach, and the one most
enterprises use.

```
   Host:  192.0.2.50/24  and  2001:db8:1:1::50/64
          IPv4 default gw     IPv6 default gw (from the RA)
```

**How a connection is chosen:** the application resolves a name, gets both an **A**
record and an **AAAA** record, and — per RFC 6724 and RFC 8305 — **prefers IPv6**.

**Happy Eyeballs (RFC 8305)** is the refinement that made this workable. Rather than
trying IPv6, waiting for it to time out, and then trying IPv4 — which produced a
multi-second delay whenever IPv6 was broken — the client:

1. Starts the IPv6 connection
2. Starts the IPv4 connection **~250 ms later**
3. **Uses whichever completes first**, and abandons the other

**This is why broken IPv6 stopped being catastrophic for users.** Before Happy Eyeballs,
a site with a black-holed AAAA record was unreachable for anyone with IPv6; afterwards it
was 250 ms slower. The mechanism that made IPv6 deployment safe to attempt — and
therefore, arguably, the mechanism that unblocked adoption.

**Dual-stack's costs**, stated plainly:

| Cost | Detail |
|---|---|
| Two of everything | addresses, routes, firewall rules, monitoring, DNS |
| **Twice the security surface** | and the classic failure is a thorough IPv4 firewall with IPv6 wide open |
| Twice the troubleshooting | *"is it broken for both, or just one?"* |
| **You still need IPv4 addresses** | so it does not solve exhaustion at all |

That last row is the reason dual-stack is a transition strategy and not a destination.
It buys nothing in address terms — which is precisely why mobile carriers skipped it.

## NAT64 and DNS64

**The mechanism that lets an IPv6-only host reach IPv4-only servers.** This is what
carriers actually deploy.

**DNS64** (RFC 6147) — the resolver, asked for a AAAA record and finding only an A
record, **synthesises** a AAAA by embedding the IPv4 address in a special prefix:

```
   Real:        198.51.100.5
   Synthesised: 64:ff9b::198.51.100.5   =   64:ff9b::c633:6405
                └───┬───┘
        the well-known prefix (RFC 6052)
```

**NAT64** (RFC 6146) — a gateway that recognises traffic to `64:ff9b::/96`, extracts the
embedded IPv4 address, and translates:

```
   IPv6-only host                NAT64 gateway            IPv4-only server
   2001:db8::1                                            198.51.100.5
        │                             │                          │
        │──▶ to 64:ff9b::c633:6405 ──▶│                          │
        │                             │──▶ from a public IPv4 ──▶│
        │                             │◀────────────────────────│
        │◀────────────────────────────│                          │
```

**The host believes it is speaking IPv6 throughout.** It has no IPv4 stack and does not
need one.

**The limitation:** DNS64 requires DNS. **An application using a literal IPv4 address —
`http://198.51.100.5/` — bypasses the resolver entirely and fails.** Which is not
hypothetical; a significant amount of software embeds addresses.

## 464XLAT — how mobile actually works

The fix for NAT64's literal-address problem, and the mechanism running on hundreds of
millions of phones.

```
   ┌── the phone ──────────────┐
   │  App speaks IPv4          │
   │        ↓                  │
   │  CLAT: translates         │        the carrier's IPv6-only network
   │  IPv4 → IPv6              │───────────────────────────────┐
   └───────────────────────────┘                               │
                                                        ┌──────▼──────┐
                                                        │    PLAT     │
                                                        │  (NAT64)    │
                                                        └──────┬──────┘
                                                               │ IPv4
                                                          the IPv4 Internet
```

**CLAT** on the phone presents a **fake IPv4 interface** to applications and translates
their traffic to IPv6. **PLAT** in the carrier network is a NAT64 translating back to
IPv4 at the edge.

**So:**

- Applications see IPv4 and work unmodified, including ones with literal addresses
- The carrier's network carries **only IPv6** — one protocol to operate, no
  carrier-grade NAT for IPv6 traffic
- IPv4 exists only at the two ends

**T-Mobile US has run this since 2014.** Verizon, Jio, and many others followed.
**It is the largest IPv6 deployment in the world and most of its users have never heard
of IPv6** — which is the correct outcome for infrastructure.

Android supports CLAT natively; iOS requires apps to be IPv6-capable, which Apple
enforced by **requiring IPv6-only compatibility for App Store submissions from June
2016** — a single policy decision that did more for IPv6 application readiness than a
decade of advocacy.

## Tunnelling — mostly historical

Carrying IPv6 inside IPv4, for islands of IPv6 separated by IPv4-only networks.

| Mechanism | Status |
|---|---|
| **6in4** (protocol 41) | Manual tunnels. Still used deliberately. |
| **6to4** (`2002::/16`) | **Deprecated** (RFC 7526). Automatic, and unreliable — asymmetric paths through anonymous relays. |
| **Teredo** | IPv6 through NAT via UDP. Microsoft's; deprecated in practice. |
| **ISATAP** | Intra-site tunnelling. Largely gone. |
| **6rd** | 6to4 fixed: the ISP's own prefix and relays. Used by some providers, notably Free in France. |
| **GRE / IPsec** | General tunnels carrying IPv6. Fine, and current. |

**6to4's failure is instructive.** It was automatic and required no configuration, which
sounds ideal. But it relied on **anonymous public relays** at `192.88.99.1`, so traffic
went through whichever relay was nearest — with no accountability, frequently
asymmetric paths, and no way to diagnose a failure. RFC 7526 deprecated it in 2015,
and the lesson is that a mechanism with no accountable operator has no one to fix it.

**Tunnel brokers** (Hurricane Electric's `tunnelbroker.net` remains the best known) are
still genuinely useful for **learning** — they give you real routable IPv6 in about ten
minutes on a network that has none, which is the fastest way to get hands-on.

## What does not exist: NAT66

There is no IPv6 NAT in the IPv4 sense, and this is deliberate.

The two reasons for IPv4 NAT were address conservation — irrelevant with 2¹²⁸ — and
accidental security, which was never NAT's job (Chapter 33 §33.3).

**NPTv6** (RFC 6296) exists: a stateless one-to-one prefix translation for multihoming
without provider-independent addresses. It preserves the end-to-end model in a way NAT
does not, and it is rarely used.

**Do not deploy IPv6 with NAT because IPv4 had it.** The firewall provides the security;
the addresses are free; and the end-to-end restoration is one of the main things IPv6 is
for.

## The security gap

**The most important operational point in this chapter.**

> **Every modern operating system has IPv6 enabled by default and prefers it.** If your
> network provides IPv6 — deliberately or otherwise — hosts will use it.

**The failure mode:** an organisation with a meticulous IPv4 firewall policy, IPv6 never
considered, and IPv6 connectivity present anyway. Every IPv4 rule is bypassed by an IPv6
path nobody is filtering.

**Where the unintended IPv6 comes from:**

- **Rogue RAs.** A Windows machine with connection sharing, a hypervisor, a misconfigured
  device — any of them can advertise itself as an IPv6 router and hosts will believe it.
- **Tunnels.** Teredo and 6to4 activate automatically on some systems.
- **Link-local.** Always present, always working, on every segment. **Nothing can turn it
  off**, and it is enough for lateral movement between hosts on a segment.
- **The provider.** Many now supply IPv6 by default whether or not you asked.

**Three positions, and only two are defensible:**

1. **Deploy IPv6 properly**, with equivalent security policy. Best.
2. **Block it deliberately** at every boundary, and monitor for it. Acceptable.
3. **Ignore it.** You have an unmonitored, unfiltered network path. Not acceptable,
   and it is where most organisations are.

**The audit is short:** does your firewall have IPv6 rules matching its IPv4 rules? Does
your IDS see IPv6? Do your logs record IPv6 addresses? Is **RA Guard** on your access
ports? If any answer is no, position 3 describes you.

## Where things stand

| Metric | Roughly |
|---|---|
| Google users reaching it over IPv6 | **45–50%** |
| Alexa/Tranco top-1000 sites with AAAA | ~40% |
| Mobile networks | **majority IPv6**, often IPv6-only with 464XLAT |
| Enterprise internal networks | **still largely IPv4** |
| Cloud providers | full support; some charge for IPv4 |

**The pattern is consistent.** IPv6 wins where **one organisation controls both ends** —
a mobile carrier, a hyperscaler's internal fabric, a content provider's edge — and lags
where **many parties must coordinate**, which is the enterprise LAN.

That is §28.1's incentive argument, visible in the deployment statistics.

AWS began charging for public IPv4 addresses in February 2024 — around $43 per
address per year. That single pricing change moved more workloads to IPv6 in a year than
the preceding decade of advocacy, and it is the clearest evidence for §28.1's claim that
**cost is the argument that works.**

## What breaks here

**A dual-stack host preferring broken IPv6.** Happy Eyeballs handles it, at a 250 ms
cost. Without Happy Eyeballs, total failure.

**A AAAA record pointing at an unreachable address.** The classic self-inflicted IPv6
outage; Happy Eyeballs hides it from users and not from your monitoring.

**IPv4 literals failing on an IPv6-only network.** DNS64 cannot help. 464XLAT can.

**A firewall with IPv4 rules only.** Everything is bypassable over IPv6.

**Rogue RAs.** RA Guard.

**Deploying NAT66 out of habit.** Do not.

**6to4 anywhere.** Deprecated. Use a tunnel broker or native connectivity.

> **Network+ note.** Objective 1.8 expects dual-stack and tunnelling. Over-learn:
> **dual-stack runs both protocols and is the recommended approach**; **tunnelling carries
> IPv6 inside IPv4**; **NAT64/DNS64 lets IPv6-only hosts reach IPv4 servers**; and
> **IPv6 is enabled by default on modern systems**, which is the security point. Expect a
> question on why dual-stack does not solve address exhaustion.
