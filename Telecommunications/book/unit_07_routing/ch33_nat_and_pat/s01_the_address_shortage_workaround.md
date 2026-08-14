# 33.1 The Address Shortage Workaround

In May 1994, Kjeld Egevang and Paul Francis published RFC 1631. It is nine pages, it
proposes a mechanism its authors describe as a **short-term measure**, it catalogues the
architectural damage the mechanism causes with unusual candour, and it recommends it
anyway.

Thirty-two years later it is on essentially every network on Earth, and IPv6 — the
permanent solution it was meant to buy time for — is at roughly half of Internet traffic.

## The problem it solved

Chapter 28 §28.1 gave the arithmetic. In 1994:

- **4.3 billion addresses**, with about 14% unusable before allocation
- Growth that was clearly exponential
- **Classful waste** already burning most of the space (Chapter 25 §25.4)
- CIDR deployed in 1993 and buying perhaps a decade
- IPv6 not yet specified

**The projection was exhaustion in the late 1990s.**

**The observation that produced NAT** — Chapter 27 §27.1's — is that **most connected hosts
never need to be reachable from the Internet.** A workstation, a printer, a till, a
building sensor: each initiates connections outward and accepts none.

**Giving each of those a globally unique address wastes one.** At the scale of billions of
devices, that waste is the whole problem.

## The mechanism

**Translate at the boundary.** Inside, use addresses that need only be locally unique
(RFC 1918). Outside, use one public address — or a few — shared by everyone inside.

```
   Inside                      NAT router                     Outside
   192.168.1.10  ─────────────▶  │  ─────────────▶  203.0.113.5
   192.168.1.11  ─────────────▶  │  ─────────────▶  203.0.113.5
   192.168.1.12  ─────────────▶  │  ─────────────▶  203.0.113.5
                                 │
                     rewrites the source address on the way out,
                     and the destination on the way back
```

**The router keeps a table** of which internal conversation corresponds to which external
one, so that returning traffic can be sent to the right host. §33.2 is that table.

**The saving is enormous.** An organisation of 10,000 employees needs **one** public
address rather than 10,000. Across the Internet, NAT plus RFC 1918 removed demand for
billions of addresses.

**The measured effect:** exhaustion arrived at IANA in **February 2011** rather than the
late 1990s. NAT and CIDR together bought roughly fifteen years.

## The terminology

Cisco's four terms, which are examined and which confuse everyone once:

| Term | Meaning |
|---|---|
| **Inside local** | The host's **real** address, on the inside — `192.168.1.10` |
| **Inside global** | How the inside host **appears** to the outside — `203.0.113.5` |
| **Outside global** | The external host's **real** address — `93.184.216.34` |
| **Outside local** | How the external host appears **to the inside** — usually the same |

**The pattern:** *inside/outside* says **whose address it is**; *local/global* says **which
side of the NAT you are standing on.**

"Inside local" and "inside global" are the pair that matters — they are the two ends of
the translation, and the other two are usually identical to each other because most NAT
only translates one side.

## What NAT is not

Three claims that are made constantly and are wrong, and they matter enough to state
before §33.2's mechanics.

### NAT is not a firewall

**NAT is address translation. A firewall is policy enforcement.** They are different jobs,
and NAT devices happen to do both because the same box sits in the same place.

**What NAT actually provides:** an inbound packet arriving at the public address with no
matching translation entry **has nowhere to go**, so it is dropped. That is not a policy
decision — it is an **absence of information**. The router does not know which internal
host to send it to, so it cannot.

**This is real protection, and it is a side effect.** Compare Chapter 17 §17.1: the
difficulty of capturing traffic on a switched network is likewise a side effect of a
performance improvement rather than a designed security property. **Side-effect security
is fragile**, because it disappears the moment the side effect changes.

**What it does not protect against:**

| Attack | Stopped by NAT? |
|---|---|
| Inbound scanning | ✓ (incidentally) |
| **Malware phoning home** | × — outbound is unrestricted |
| **A user visiting a malicious site** | × |
| **A compromised host attacking others inside** | × |
| **Anything that establishes an outbound connection** | × |
| Data exfiltration | × |

Nearly every modern attack begins with an outbound connection, which NAT permits
without inspection.

### NAT is not security

Following directly: an organisation that treats "we are behind NAT" as a security posture
has no security posture. **The firewall provides the security.** Chapter 58 covers what a
firewall actually does, and the distinction is the same one Chapter 20 §20.1 made about
VLANs: NAT creates a boundary; it does not enforce anything at it.

### NAT is not required for privacy

Public addresses do not identify individuals. The privacy argument for NAT confuses
"addressable" with "identified", and IPv6's privacy addresses (Chapter 28 §28.3) provide
far better privacy than NAT does while keeping every host reachable.

## The cost

RFC 1631 listed these, and thirty years proved every one.

**The end-to-end principle is broken** (Chapter 23 §23.4). A host behind NAT **does not
know its own address** as the world sees it. The address in its IP header is not the
address its peer sees, which breaks every protocol that assumes otherwise.

**Inbound connections require configuration.** A host behind NAT is not addressable, so
running a server, accepting a call, or receiving a file transfer needs explicit port
forwarding or a workaround protocol. §33.3 is the catalogue.

**The Internet became client-server.** This is the largest consequence and it is easy to
miss because it happened gradually. The original design made every host a peer; NAT made
most hosts **clients only**, and the applications that adapted are the ones that route
everything through a central server.

> **Peer-to-peer applications did not fail because peer-to-peer is a bad idea. They failed
> because most hosts stopped being addressable**, and the successful ones are those that
> relay through a server — which is a centralisation that the architecture did not intend
> and that has consequences well beyond networking.

**Protocols that embed addresses break** (Chapter 21 §21.4). FTP's `PORT` command sends an
IP address as text inside the data stream; SIP does the same for media. **A NAT must parse
the application layer to fix them** — which is what an application-layer gateway is, and
which encryption defeats entirely.

**Troubleshooting is harder.** Logs show the translated address, so identifying which
internal host did something requires correlating the NAT table at the time. **With CGNAT
(§33.4) this becomes a serious problem** for abuse handling and for law enforcement.

**Stateful, so it is a single point of failure.** The translation table is state
(Chapter 23 §23.1's fate-sharing, violated). If the NAT device restarts, **every
connection through it dies**, because the table that mapped them is gone.

## The honest assessment

**Was NAT a mistake?** The argument runs both ways and both sides are serious.

**Against NAT:**

- It broke the architecture, permanently
- It made IPv6's transition harder by removing the urgency (Chapter 28 §28.1)
- It centralised the Internet
- It cost thirty years of application complexity — STUN, TURN, ICE, relays
- The "temporary measure" outlived the permanent solution's deployment schedule

**For NAT:**

- The alternative was running out of addresses in 1998, before IPv6 existed
- The Internet grew by three orders of magnitude anyway
- Billions of devices connected that otherwise could not have
- It was deployable **immediately, unilaterally, by anyone**, with no coordination —
  which is precisely the property Chapter 23 §23.1 says every successful Internet change
  must have

**The fairest verdict:** it was the **correct engineering decision under the constraints of
1994**, made by people who understood exactly what they were trading and said so in
writing. The mistake was not deploying it; **the mistake was that its success removed the
pressure to finish the real fix.**

That pattern — a workaround so effective that it prevents the solution — recurs, and it is
worth recognising when you are building one.

## What breaks here

**Treating NAT as security.** It is not. The firewall is.

**Expecting inbound connections to work.** They do not, without configuration.

**A protocol that works internally and fails across NAT.** It embeds addresses.

**Losing every connection when the NAT device restarts.** The table was the state.

**Being unable to identify which host generated logged traffic.** Correlate against the
translation table, and keep the logs to make that possible.

> **Network+ note.** Objective 2.2 expects NAT and PAT. Over-learn: **NAT translates
> addresses to conserve public address space**; **inside local is the private address and
> inside global is the public one**; and — the one that appears in security questions —
> **NAT is not a security control.** Expect the four Cisco terms as a matching question.
