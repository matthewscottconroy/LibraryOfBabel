# 61.3 TLS VPNs and WireGuard

**Two alternatives to IPsec, chosen for opposite reasons: one because it gets through anything,
the other because it does almost nothing.**

## TLS VPNs

**Also called SSL VPNs, and the name persists although SSL does not** (Chapter 58 §58.4).

**The argument for them is deployability rather than cryptography.**

| | IPsec | **TLS VPN** |
|---|---|---|
| Transport | **ESP (protocol 50) or UDP 4500** | **TCP 443, or UDP 443** |
| **Through a hotel or airport network** | **frequently blocked** | **works — it looks like HTTPS** |
| **Through a restrictive corporate guest network** | blocked | **works** |
| Client | **often needs installing** | **may be browser-only** |
| Granularity | **network access** | **network or per-application** |

> **The entire argument is that port 443 outbound is permitted everywhere**, because blocking it
> breaks the web. **A VPN that looks like HTTPS traverses networks that block everything else** —
> which is why remote access moved to TLS and site-to-site did not.

**And there are two distinct things called a TLS VPN**, which are frequently conflated:

**Clientless / portal mode.** **A web page proxying specific applications** — a file share, a web
application, an RDP session in a browser. **No client software.** **Its limitations are real:
only applications the gateway supports, and the gateway must understand each one.**

**Full tunnel client.** **A client that creates a virtual interface and tunnels IP over TLS.**
**Functionally equivalent to IPsec** with a different transport, and it is what most "SSL VPN"
products actually deploy.

**The honest technical criticism, and it is worth knowing:**

> **Tunnelling TCP inside TCP is a bad idea.** **Both layers retransmit, and their timers
> interact** — **the inner TCP retransmits because the outer TCP is already retransmitting**,
> producing **TCP meltdown**: throughput collapses under loss rather than degrading.

**Which is why serious TLS VPNs use DTLS or QUIC over UDP** and fall back to TCP only when UDP
is blocked. **If your TLS VPN is running over TCP on a lossy link, that is the explanation for
its performance**, and switching it to UDP is frequently a single configuration change.

**And TLS VPN concentrators are a recurring source of critical vulnerabilities.** **They are
Internet-facing, they terminate TLS, they parse complex input, and they are attractive** —
**several have had pre-authentication remote code execution vulnerabilities exploited at scale**,
and **they are on Chapter 55 §55.3's emergency patching track by definition.**

## WireGuard

**The interesting one, and its argument is about what it does not do.**

| | **IPsec** | **WireGuard** |
|---|---|---|
| **Lines of code** | **hundreds of thousands** | **about 4,000** |
| Cipher suites | **negotiated, dozens of combinations** | **one, fixed** |
| Key exchange | negotiated | **fixed (Noise protocol framework)** |
| Configuration | **phase 1, phase 2, selectors, lifetimes, modes** | **a key, an endpoint, and allowed IPs** |
| Transport | ESP or UDP 4500 | **UDP, any port** |
| **In the kernel** | yes | **yes, since Linux 5.6** |

**The design decisions, and each is deliberate:**

**No cryptographic agility.** **ChaCha20-Poly1305, Curve25519, BLAKE2s, HKDF** — **fixed.**
**If one is broken, the protocol version changes and everyone upgrades.**

> **This is a genuine trade, argued both ways.** **Agility permits migration without a protocol
> change and is the mechanism by which downgrade attacks work** (Chapter 58 §58.4's TLS 1.3
> reduced its suites for exactly this reason). **WireGuard chose the other side: no negotiation,
> no downgrade, no mismatched-proposal failure.**

**Silence on the wire.** **A WireGuard endpoint does not respond to unauthenticated packets at
all.** **Port scanning finds nothing.** **There is no handshake to start without a valid key**,
which removes an entire class of pre-authentication attack surface (Chapter 59 §59.4's
broker-mediated argument, in a VPN).

**Cryptokey routing.** **The configuration is a mapping from public key to permitted source
addresses:**

```
   [Peer]
   PublicKey  = xTIB…
   AllowedIPs = 10.2.0.0/16, 192.168.5.0/24
   Endpoint   = 198.51.100.9:51820
```

> **`AllowedIPs` does two things at once**: **it is the routing table entry — packets for those
> ranges go to this peer — and it is the ingress filter — packets from this peer are accepted
> only if their source is in that range.** **Routing and authorisation are the same
> configuration line**, which eliminates a class of misconfiguration in which they disagree.

**Stateless and connectionless.** **There is no tunnel "up" or "down"**, only whether a recent
handshake exists. **Roaming is free: a peer whose address changes is recognised by its key and
the endpoint is updated.** **Which makes it excellent for mobile clients** and is why it
underpins most modern mesh VPN products.

**And no built-in key distribution.** **This is the honest limitation:**

> **WireGuard has no equivalent of IKE's certificate authentication.** **Keys must be distributed
> and configured out of band**, and **at any scale that means a management system.** **The
> protocol is deliberately incomplete**, and the products built on it — Tailscale, Netbird,
> Firezone and others — **are supplying the key distribution and identity layer that WireGuard
> declines to specify.**

**Which is a reasonable design and it must be stated when comparing them:** **"WireGuard is
simpler than IPsec" compares a transport protocol with a complete system.** **Comparing
WireGuard-plus-a-management-plane with IPsec is the fair comparison, and it still favours
WireGuard on operational simplicity — by less.**

## Choosing

| Requirement | Choose |
|---|---|
| **Site-to-site between vendors' equipment** | **IPsec** — it is the interoperable standard |
| **Remote access through restrictive networks** | **TLS VPN**, over UDP where possible |
| **Remote access with best performance and roaming** | **WireGuard** |
| **A routing protocol over the tunnel** | **IPsec with a VTI**, or GRE over IPsec |
| **Mesh between many endpoints** | **WireGuard**, with a management plane |
| **Compliance requiring a specific validated implementation** | **IPsec, usually — check what is certified** |
| **Traffic that must be inspected in the middle** | **terminate it; no tunnel offers this** |

**And a note on FIPS and equivalent validation**, because it decides procurement in some sectors:
**WireGuard's fixed primitives are not all FIPS-approved** — **ChaCha20-Poly1305 is not** — **so
regulated environments frequently cannot use it regardless of its merits.** **This is a
compliance constraint rather than a security judgement** (Chapter 57 §57.3), and it should be
identified early rather than at the end of an evaluation.

## Performance

**Three factors dominate, and cipher choice is rarely one of them.**

**Where the crypto runs.** **Kernel implementations (IPsec, WireGuard) substantially outperform
userspace ones (most TLS VPN clients, OpenVPN)** — **the copies between kernel and userspace
cost more than the encryption.**

**Hardware acceleration.** **AES-NI makes AES-GCM essentially free on modern CPUs**; **ChaCha20
is fast without it** (Chapter 58 §58.1). **On a device with AES acceleration, IPsec with AES-GCM
is very fast; on one without, WireGuard's ChaCha20 wins.**

**MTU and MSS.** §61.1 — **and a misconfigured MTU costs far more than any cipher choice**,
because it produces retransmission.

> **A VPN that is slow is almost never slow because of the cipher.** **Check MTU, then check
> whether the crypto is in the kernel, then check the CPU** — in that order.

## What breaks here

**A TLS VPN with collapsing throughput on a lossy link.** **TCP over TCP.** Switch to UDP/DTLS.

**A TLS VPN gateway with a critical vulnerability.** **Expected, and it is why they are on the
emergency patching track.** Chapter 55 §55.3.

**WireGuard configured and traffic not passing to one subnet.** **`AllowedIPs` does not include
it** — and remember it is both the route and the filter.

**Two WireGuard peers both behind NAT and unable to connect.** **Neither can reach the other's
endpoint.** **One must be reachable, or a relay is required** — which is what the commercial
mesh products supply.

**A WireGuard tunnel that appears down.** **There is no "up".** Check the last handshake time;
**a peer with no traffic and no keepalive has simply not handshaken recently, which is normal.**

**WireGuard rejected on compliance grounds.** **The primitives are not FIPS-approved.** Identify
this before evaluating.

**A VPN that is slow.** **MTU first.** Almost always MTU.

**"WireGuard is simpler" used to justify replacing an interoperable IPsec deployment.** **Compare
like with like** — the key distribution has to come from somewhere.

> **Network+ note.** Objective 4.4. Over-learn: **SSL/TLS VPNs operate over TCP or UDP 443 and
> traverse restrictive networks**; **clientless VPNs are browser-based**; **IPsec operates at
> Layer 3 and TLS VPNs at the session layer**; and **VPN concentrators terminate remote access
> tunnels.** WireGuard is not yet examined and is what you are increasingly likely to deploy.
