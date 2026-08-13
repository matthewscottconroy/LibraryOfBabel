# 61.1 The Tunnel Idea

**Take a packet, encrypt it, and put it inside another packet addressed to a device that will
decrypt it and forward the original.**

**That is tunnelling**, and **a virtual private network is tunnelling plus cryptography.**

## The shape

```
   Original packet:
   ┌──────────────┬─────────────────────────────┐
   │ IP 10.1.0.5  │  payload                    │
   │ → 10.2.0.9   │                             │
   └──────────────┴─────────────────────────────┘

   Encapsulated:
   ┌──────────────┬──────────┬───────────────────────────────────┐
   │ IP 203.0.113 │ tunnel   │  ENCRYPTED(original packet)       │
   │ → 198.51.100 │ header   │                                   │
   └──────────────┴──────────┴───────────────────────────────────┘
    the outer header the        the inner packet, which the
    Internet routes on          Internet never sees
```

**Two addresses per packet.** **The outer pair are routable and public; the inner pair are
whatever you like** — private addresses, a different address family, or not IP at all.

## The three things a tunnel buys

**Worth separating, because different deployments want different ones and the argument for each
is different.**

### Confidentiality and integrity across an untrusted path

**The point most people think of.** **Traffic crossing the public Internet is readable by every
network it traverses** (Chapter 57 §57.2); **encryption makes it useless to them.**

**And note that this is the only one of the three that requires cryptography.** **The other two
are properties of encapsulation alone.**

### Connectivity the underlying network would not provide

| Want to carry | Over | Because |
|---|---|---|
| **RFC 1918 addresses** | the Internet | **which would drop them** (Chapter 27 §27.1) |
| **IPv6** | an IPv4-only path | **transition mechanisms** (Chapter 28 §28.4) |
| **Ethernet frames** | **a routed network** | **Chapter 67's overlays** |
| **A routing protocol adjacency** | a network that does not run it | **GRE over IPsec, and it is why GRE persists** |
| Multicast | a path that does not forward it | |

> **This is frequently the real reason a tunnel exists, and it has nothing to do with
> security.** **A GRE tunnel with no encryption is a perfectly sensible thing to build**, and
> asking "what is this tunnel for?" distinguishes the cases.

### A shared trust domain

**Machines at both ends behave as though on one network.**

**Convenient**, and — as §61.4 argues — **increasingly regarded as a liability rather than a
feature**, because **it is exactly the "being on the network confers privilege" assumption
Chapter 59 §59.4 rejects.**

## The tunnel taxonomy

| Protocol | Encapsulates | Encrypts | Typical use |
|---|---|---|---|
| **GRE** | **almost anything** | **no** | routing adjacencies, multicast, overlays |
| **IPsec** | **IP** | **yes** | **site-to-site; the standard** |
| **IPsec + GRE** | **anything, encrypted** | yes | **routing protocols over IPsec** |
| **WireGuard** | IP | **yes** | **modern site-to-site and remote access** |
| **TLS VPN (SSL VPN)** | IP or application | yes | **remote access through restrictive networks** |
| **VXLAN / GENEVE** | **Ethernet** | **no** (usually) | **data centre overlays** (Chapter 67 §67.2) |
| **L2TP** | Ethernet/PPP | **no — paired with IPsec** | legacy remote access |
| **PPTP** | PPP | **broken** | **do not use** |

**Two entries need a note.**

**GRE plus IPsec is a common and slightly awkward pairing.** **IPsec in tunnel mode carries IP
and does not carry multicast or non-IP traffic**, so **a routing protocol adjacency — OSPF's
multicast hellos (Chapter 31) — cannot run directly over it.** **The fix is to build a GRE
tunnel and encrypt it with IPsec**, which works, adds overhead, and is why the combination
appears in every enterprise WAN design.

**Modern IPsec implementations offer "virtual tunnel interfaces" (VTI)** which present the IPsec
tunnel as a routable interface and remove the need for GRE — **and where the platform supports
it, use it.**

**PPTP should be stated plainly:** **its authentication (MS-CHAPv2) is broken and its encryption
(MPPE) is weak.** **It is trivially decrypted**, and its presence in a configuration is a
finding.

## Split tunnelling

**The decision that determines what the tunnel is for.**

```
   Full tunnel:                       Split tunnel:

   Client ──▶ everything ──▶ VPN      Client ──▶ corporate ──▶ VPN
              through the tunnel               ──▶ Internet ──▶ direct
```

| | **Full tunnel** | **Split tunnel** |
|---|---|---|
| Corporate traffic | through the tunnel | through the tunnel |
| **Internet traffic** | **through the tunnel, then out** | **direct** |
| **Inspection and policy** | **applies to everything** | **applies to corporate traffic only** |
| **Concentrator capacity** | **must carry all Internet use** | **much less** |
| **User experience** | **tromboning** (Chapter 51 §51.1) | **good** |
| **Client as a bridge** | **less exposure** | **the client is on two networks at once** |

**In March 2020 this decision broke a great many organisations at once**: **concentrators sized
for 10% of staff met 100% of them**, and **full tunnelling meant every Teams call and every
Windows update crossed the corporate link twice.**

**The honest position:**

> **Split tunnelling is correct for most organisations and it dilutes the inspection point.**
> **The mitigation is that inspection should not depend on the tunnel** — an endpoint agent or a
> cloud security service (Chapter 51 §51.2's SASE) **sees the traffic regardless of the path.**

**And the "client as a bridge" concern is real and frequently overstated.** **A split-tunnelled
client is simultaneously on the home network and the corporate one**, so **a compromised device
on the home network has a path** — **which is true, and is equally true of a full-tunnelled
client, because the compromise is on the client itself.** **The genuine mitigation is host
firewalling and endpoint control, not tunnel mode.**

## MTU, which is where tunnels actually go wrong

**Every tunnel adds a header, and the added header reduces the space available for the payload.**

| Encapsulation | Overhead | **Resulting MTU from 1500** |
|---|---|---|
| **GRE** | 24 | **1476** |
| **IPsec ESP tunnel (AES-GCM)** | ~58 | **~1442** |
| **IPsec ESP + NAT-T** (UDP 4500) | ~66 | **~1434** |
| **WireGuard** | 60 | **1440** (1420 is the usual configured value) |
| **GRE + IPsec** | ~82 | **~1418** |
| **Tunnel over PPPoE (1492)** | | **subtract again** |

**And the failure mode is Chapter 24 §24.3's and Chapter 66 §66.3's:**

> **Small packets work. Large ones vanish.** **Pings succeed, SSH works, and a file transfer or
> an HTTPS page hangs after the first few packets.**

**The mechanism:** **the sender's packet is too large for the tunnel; the tunnel endpoint should
send an ICMP "Fragmentation Needed"; and something filters ICMP** (Chapter 60 §60.1) **so the
sender never learns.** **This is the PMTUD black hole, and it is the single most common tunnel
problem.**

**The remedies, in order of preference:**

**Set the tunnel interface MTU correctly.** **The tunnel knows its own overhead; configure it.**

**Clamp the TCP MSS.** **`ip tcp adjust-mss 1360`** — the tunnel endpoint rewrites the MSS option
in passing SYN packets, **so both ends negotiate a segment size that fits.** **This fixes TCP
and only TCP**, and it is deployed almost universally because it is reliable and requires nothing
of the endpoints.

**Permit ICMP Type 3 Code 4.** **Which should be done anyway** and cannot be relied upon,
because the filtering is frequently in a network you do not control.

**And check both directions.** **An asymmetric MTU problem — working one way and not the other —
is common** and is diagnosed with `ping -M do -s <size>` (Linux) or `ping -f -l <size>`
(Windows), bisecting on size.

## Routing, which is the other place they go wrong

**A tunnel is an interface, and an interface participates in routing.**

**Three recurring problems:**

**Recursive routing.** **The route to the tunnel's own endpoint points through the tunnel.**
**The tunnel comes up, the routing protocol learns a better path to the far endpoint via the
tunnel, and the tunnel drops** — then repeats. **The symptom is a tunnel that flaps
rhythmically**, and **the fix is to ensure the endpoint's route is learned outside the tunnel**
(a static route, or route filtering).

**Asymmetric paths.** Chapter 60 §60.2 — **a stateful firewall on one side and not the other.**

**Overlapping address space.** **Two sites both using 10.0.0.0/24 cannot be joined by a tunnel
without NAT** (Chapter 33), **and this is discovered during a merger.** **It is Chapter 27's
address planning argument, arriving as a project.**

## What breaks here

**A tunnel that establishes and passes no traffic.** **Check routing before cryptography** — the
tunnel being up says nothing about whether packets are directed into it.

**Pings work and transfers hang.** **MTU.** Clamp MSS, set the interface MTU, and check ICMP is
not filtered.

**A tunnel that flaps at regular intervals.** **Recursive routing**, or a keepalive timeout, or
rekeying (§61.2).

**Two sites with the same private range.** **Overlapping address space.** NAT, or renumber.

**A routing protocol that will not form an adjacency over IPsec.** **Multicast does not traverse
plain IPsec tunnel mode.** GRE over IPsec, or a VTI.

**Everything slow through the VPN and fine outside it.** **Full tunnelling and tromboning**, or
concentrator capacity, or MTU-induced retransmission.

**PPTP found in production.** **A finding.** Its cryptography is broken.

**A split-tunnel policy that fails to update when a SaaS provider changes address ranges.**
**Common**, and it is Chapter 51 §51.2's application-definition staleness in another guise.

> **Network+ note.** Objective 4.4 and 1.8 cover VPNs. Over-learn: **a VPN creates an encrypted
> tunnel over an untrusted network**; **site-to-site connects networks and client-to-site
> connects users**; **split tunnelling sends only corporate traffic through the tunnel**;
> **GRE encapsulates but does not encrypt**; and **tunnelling adds overhead and reduces the
> effective MTU.** The split-tunnel definition and the site-to-site/client-to-site distinction
> are examined regularly.
