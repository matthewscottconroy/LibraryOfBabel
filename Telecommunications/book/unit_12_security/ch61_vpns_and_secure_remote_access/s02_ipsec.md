# 61.2 IPsec

The standard for site-to-site connectivity, and it has a reputation for complexity that is
largely deserved and almost entirely attributable to the number of choices it presents.

## The pieces

Three things must be understood separately: what protects the packet, how it is wrapped, and
how the keys were agreed.

### AH and ESP — what protects the packet

| | **AH** — Authentication Header | **ESP** — Encapsulating Security Payload |
|---|---|---|
| Integrity | **yes** | **yes** |
| Authentication | **yes** | **yes** |
| **Encryption** | **no** | **yes** |
| Protects the outer IP header | **yes** | **no** |
| **Traverses NAT** | **no — never** | **yes, with NAT-T** |
| Used | **almost never** | **always** |

> AH authenticates the outer IP header, which means AH cannot traverse NAT at all — **NAT
> modifies exactly what AH is protecting** (Chapter 33 §33.3). **This single property made it
> irrelevant**, since almost every real deployment crosses NAT somewhere.

Use ESP. There is no practical reason to deploy AH, and its presence in a configuration
usually means someone was following a very old document.

### Transport and tunnel — how it is wrapped

```
   Transport mode (host to host):
   ┌────────┬─────┬──────────────────┬───────┐
   │ orig IP│ ESP │  ENCRYPTED payload│ ESP tr│
   └────────┴─────┴──────────────────┴───────┘
    original header kept — the endpoints are the crypto endpoints

   Tunnel mode (gateway to gateway):
   ┌────────┬─────┬────────────────────────────────┬───────┐
   │ new IP │ ESP │ ENCRYPTED(orig IP + payload)   │ ESP tr│
   └────────┴─────┴────────────────────────────────┴───────┘
    new header — the gateways are the crypto endpoints, and the
    inner addresses are hidden
```

Tunnel mode is what "site-to-site VPN" means, and transport mode is used host-to-host and
inside other encapsulations (L2TP/IPsec).

### IKE — how the keys were agreed

Internet Key Exchange, and IKEv2 is what you should be deploying.

**Its job:** authenticate the endpoints to each other, perform a Diffie–Hellman exchange
(Chapter 58 §58.2), and derive the keys — which is §58.2's universal pattern, with IPsec's
particular choices.

| | **IKEv1** | **IKEv2** |
|---|---|---|
| Exchanges to establish | **6 or 9** (main / aggressive mode) | **4** |
| Modes | **main, aggressive** — and aggressive leaks identity | **one** |
| **NAT traversal** | **an extension** | **built in** |
| **Dead peer detection** | an extension | **built in** |
| **Rekeying** | awkward; a common failure source | **clean** |
| **MOBIKE** — endpoint address change | no | **yes** |
| Complexity | **substantial** | **substantially less** |

> **IKEv1 is deprecated.** Aggressive mode in particular sends the identity in the clear and
> is subject to offline dictionary attack against a pre-shared key, and it exists because
> main mode with pre-shared keys cannot support a peer with a dynamic address. **IKEv2 solves
> that properly.**

## The negotiation, and where it fails

Two phases, each with its own set of parameters that must match at both ends.

```
   Phase 1 (IKE SA)      ── authenticate; establish a secure channel for phase 2
     encryption          AES-256-GCM
     integrity/PRF       SHA-384
     DH group            19 (256-bit ECP) / 14 (2048-bit MODP)
     authentication      certificate | PSK
     lifetime            8 h

   Phase 2 (Child SA)    ── the actual data protection
     protocol            ESP
     encryption          AES-256-GCM
     PFS group           19            ← a fresh exchange per rekey
     lifetime            1 h / 100 GB
     traffic selectors   10.1.0.0/16 ↔ 10.2.0.0/16
```

> Every one of those must match, and a mismatch anywhere produces a tunnel that will not
> establish, with a log message that names the phase and — if you are fortunate — the
> parameter.

This is IPsec's actual complexity: not the cryptography, but the number of things two
independently-configured devices must agree on. WireGuard's argument (§61.3) is precisely
that most of these choices should not exist.

And the traffic selectors are the ones that catch people:

Traffic selectors define which traffic the tunnel carries. They must match — or be
compatible — at both ends, and a mismatch produces a tunnel that comes up and drops the
traffic you wanted, or that comes up and immediately renegotiates.

Policy-based versus route-based is the same distinction, made at the configuration level:

| | **Policy-based** | **Route-based (VTI)** |
|---|---|---|
| Traffic enters the tunnel because | **it matches a selector** | **the routing table says so** |
| Multiple subnets | **multiple SAs, or a summarised selector** | **one tunnel, routes point at it** |
| **Routing protocols** | **cannot run over it** | **can — it is an interface** |
| **Failover** | awkward | **routing does it** |
| **Recommended** | **legacy** | **yes** |

> **Use route-based tunnels where the platform supports them.** A VTI is an interface; it can
> carry a routing protocol, it can be monitored like an interface, and failover is a routing
> problem rather than a policy problem. Policy-based IPsec is a large source of the
> protocol's reputation.

## NAT traversal

Because ESP is protocol 50, not TCP or UDP, and has no ports.

> A NAT device translating a connection needs a port to translate. ESP has none, so a NAT
> device cannot map two internal hosts' ESP flows to one external address, and many simply
> drop it.

NAT-T's answer: encapsulate ESP in UDP port 4500.

```
   ┌────────┬──────────┬─────┬────────────────────┐
   │ new IP │ UDP 4500 │ ESP │ ENCRYPTED(packet)  │
   └────────┴──────────┴─────┴────────────────────┘
```

IKE detects NAT during the exchange — by comparing hashes of the addresses it sees with what
the peer claims — and switches to UDP encapsulation automatically.

**Two consequences:**

8 more bytes of overhead (§61.1's MTU table).

**And keepalives become necessary.** The NAT mapping expires if idle (Chapter 33 §33.2), so
NAT-T sends periodic keepalives — typically every 20 seconds — to hold it open. A
tunnel that drops after a few minutes of idleness has a keepalive interval longer than the NAT
device's timeout.

## Rekeying, and the flap it causes

SAs have lifetimes, in time and in volume, and both ends rekey when theirs expires.

> The commonest IPsec fault after initial configuration is a tunnel that drops briefly at
> regular intervals, and **the interval matches a lifetime.**

**The causes:**

**Mismatched lifetimes.** One end at 3,600 s and the other at 28,800 s. IKEv2 negotiates,
IKEv1 frequently does not, and the shorter end rekeys while the longer does not expect it.

**Rekey collisions.** Both ends initiate simultaneously, producing duplicate SAs. IKEv2
handles this; IKEv1's behaviour is implementation-specific and was a genuine interoperability
problem.

**Volume-based lifetimes reached quickly.** A 100 GB lifetime on a busy tunnel rekeys every few
minutes, which is fine if it is clean and disruptive if it is not.

**And the diagnosis is arithmetic:** note the interval between drops, and compare it with the
configured lifetimes. A drop every 3,600 seconds is not a coincidence.

## Authentication: PSK versus certificates

| | **Pre-shared key** | **Certificates** |
|---|---|---|
| Setup | **trivial** | **requires a PKI** (Chapter 58 §58.4) |
| Scale | **one key per pair — $n(n-1)/2$** | **one certificate per device** |
| Rotation | **coordinated, so it never happens** | **automated renewal** |
| **Compromise** | **the key is in every configuration backup** | the private key stays on the device |
| **Peer identification** | by address, usually | **by name, cryptographically** |
| Dynamic-address peers | **awkward (aggressive mode)** | **straightforward** |

> Pre-shared keys are fine for a handful of tunnels and do not scale. The failure is
> Chapter 58 §58.1's key distribution arithmetic, and it arrives at about twenty sites.

And the PSK's presence in configuration backups is a real exposure (Chapter 55 §55.4): a
repository of network configurations contains every tunnel's key, and rotating them requires
coordinated changes at both ends of every tunnel — which is why they are typically the
original values from the day of installation.

## What breaks here

**A tunnel that will not establish.** Read the log and identify the phase. Phase 1 failures
are authentication or proposal mismatch; phase 2 failures are usually traffic selectors or a
transform mismatch.

**A tunnel up and no traffic passing.** Traffic selectors, or routing (§61.1). Check the
SA's packet counters — zero encrypted packets means traffic is not entering the tunnel.

**A tunnel that drops every hour.** **Rekeying.** Compare the interval with the lifetimes.

A tunnel that drops after a few minutes of idleness. The NAT mapping expired. Reduce the
keepalive interval.

**AH configured and nothing works through NAT.** AH cannot traverse NAT. ESP.

**Aggressive mode with a pre-shared key.** The identity is in the clear and the PSK is subject
to offline attack. IKEv2.

A routing protocol that will not form an adjacency. **Policy-based tunnel.** Use a VTI, or
GRE over IPsec.

**Large packets dropped.** §61.1's MTU. **MSS clamping.**

Twenty sites and a pre-shared key management problem. **Expected.** Certificates.

**Interoperability failure between two vendors' IKEv1 implementations.** Historically common
and mostly resolved by IKEv2, which has far fewer options to disagree about.

> **Network+ note.** Objective 4.4. Over-learn: IPsec provides confidentiality, integrity and
> authentication at Layer 3; AH provides authentication without encryption and ESP provides
> both; transport mode protects the payload and tunnel mode the whole packet; **IKE
> negotiates the security association**; and **IPsec is commonly used for site-to-site VPNs.**
> The AH/ESP and transport/tunnel distinctions are examined in every form.
