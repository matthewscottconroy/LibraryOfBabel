# 34.4 Path MTU Discovery and Black Holes

This section is the payoff of the chapter. It describes one failure mode, that failure
mode is common, its symptom points at the wrong layer, and engineers lose days to it
routinely.

**If you retain one thing from Unit VII, retain this.**

## The mechanism

Chapter 24 §24.3 covered it. Restated as a protocol exchange:

```
   1. Sender transmits a full-size packet with the DF flag SET
   2. A router whose next link has a smaller MTU cannot fragment (DF)
   3. It DROPS the packet
   4. It returns  ICMP Type 3, Code 4 — "Fragmentation Needed and DF Set"
      including the MTU it could not exceed
   5. The sender caches a smaller path MTU for that destination and retransmits
   6. Repeat until packets get through
```

**The whole mechanism depends on step 4 arriving.**

**And DF is set on essentially all modern TCP traffic**, because IPv4 fragmentation is
considered harmful (RFC 8900) and IPv6 forbids router fragmentation outright. **So this is
not an edge case — it is how every large transfer works.**

## The black hole

**Somebody blocks ICMP.**

Usually deliberately, on the belief that ICMP is a security risk (§34.1); sometimes as a
side effect of a default-deny rule nobody examined.

**The sequence:**

```
   Sender:   1500-byte packets, DF set   ──▶
   Router:   cannot forward, MTU is 1400
   Router:   drops, sends ICMP 3/4       ──✂── BLOCKED
   Sender:   hears nothing
   Sender:   retransmits 1500-byte packets, DF set   ──▶
   Router:   drops
   ...
```

**Nobody is ever told anything.** The sender's retransmission timer fires, it resends the
same oversized packet, and the cycle repeats until the connection times out.

## The symptom

**This is why it costs so much time**, and it is worth learning as a pattern:

| Works | Fails |
|---|---|
| `ping` | **large file transfers** |
| **SSH login** | **`scp` of a large file** |
| Small web pages | pages with images |
| **The TCP handshake** | **the first full-size data segment** |
| DNS queries over UDP | **DNS responses over ~1400 bytes (DNSSEC)** |
| Sending email | **sending an attachment** |
| A VPN connecting | **transferring anything over the VPN** |

> **Small things work. Large things hang.**

**And the connection establishes**, because SYN, SYN-ACK and ACK are all small. So the
application reports a successful connection followed by a stall — **which looks like an
application fault, or a server fault, or a "slow network", and looks nothing like a
firewall problem.**

**People check the application. They check the server. They restart things. They open a
ticket with the wrong team.** The fault is a firewall rule two networks away, blocking one
ICMP code.

## Diagnosing it

**Two minutes, if you know the technique.**

```bash
# Binary search for the largest packet that gets through.
# -M do sets DF; -s is the PAYLOAD size; add 28 for ICMP+IP headers.

ping -M do -s 1472 host      # = 1500 total.  Fails?
ping -M do -s 1372 host      # = 1400.        Works?
ping -M do -s 1422 host      # = 1450.        Narrow it down.
```

**On Windows:**

```
ping -f -l 1472 host
```

**Or let a tool do it:**

```bash
tracepath host               # reports the path MTU, and where it changes
tracepath -n 8.8.8.8
```

**The diagnosis:** if 1472 fails and 1372 succeeds, **the path MTU is between 1400 and
1500, and the ICMP that should have told you is being blocked.**

**Confirming it is PMTUD and not general loss:** small pings succeed. If they did not, you
have a different problem.

## The fixes, in order of preference

### 1. Stop blocking ICMP type 3 code 4

**The correct fix.** It is one rule, on one firewall, and it makes the mechanism work as
designed.

```
# iptables
iptables -A INPUT -p icmp --icmp-type fragmentation-needed -j ACCEPT
iptables -A FORWARD -p icmp --icmp-type fragmentation-needed -j ACCEPT

# IPv6 — RFC 4890 lists what must pass
ip6tables -A INPUT -p icmpv6 --icmpv6-type packet-too-big -j ACCEPT
```

**The objection — "ICMP is a security risk" — does not apply to this code.** Type 3 code 4
cannot be used for reconnaissance in any meaningful way, and blocking it breaks a core
mechanism of the protocol.

**If you control the firewall, do this.** The difficulty is that you frequently do not:
the blocking device may be in a provider's network, a partner's, or a customer's.

### 2. TCP MSS clamping

**What you do when you cannot fix the firewall**, and it is what every VPN gateway does.

```
# Cisco
interface Tunnel0
 ip tcp adjust-mss 1360

# Linux
iptables -t mangle -A FORWARD -p tcp --tcp-flags SYN,RST SYN \
         -j TCPMSS --clamp-mss-to-pmtu
```

**The router rewrites the MSS option in the TCP handshake**, so the two endpoints agree on
a segment size that fits the path — and never send anything too large in the first place.

**It is a layer violation** (Chapter 21 §21.4): a router modifying a transport-layer
option in transit. **And it works without touching either endpoint**, which is why it is
universal despite being ugly.

**Its limitation:** it only helps **TCP**. UDP-based applications — VPNs, QUIC, video —
get no benefit, and must handle path MTU themselves.

### 3. Lower the MTU on the endpoints

```bash
ip link set dev eth0 mtu 1400
```

**Works, and penalises everything** — including traffic on paths that had no problem. A
blunt instrument, and acceptable on a host that only ever uses one constrained path.

### 4. PLPMTUD

**RFC 4821** — Packetization Layer Path MTU Discovery. The transport probes for the path
MTU itself, using its own traffic and its own acknowledgements, **without depending on
ICMP at all.**

```bash
sysctl net.ipv4.tcp_mtu_probing=1
```

**This is the robust answer** — it cannot be broken by a filtering firewall, because it
uses no ICMP. It has been available since 2007 and is under-deployed, though it is now on
by default in some stacks.

**QUIC does its own** (RFC 9000 §14), for exactly this reason: having watched TCP's PMTUD
be broken by middleboxes for twenty years, QUIC's designers built discovery that does not
depend on anything in the middle behaving well.

## Where the small MTUs come from

**Almost always a tunnel** (Chapter 24 §24.3):

| Encapsulation | Overhead | Resulting MTU |
|---|---|---|
| PPPoE (DSL) | 8 | **1492** |
| GRE | 24 | 1476 |
| IPsec transport | ~30–40 | ~1460 |
| **IPsec tunnel (ESP)** | ~50–60 | **~1440** |
| **VXLAN** | 50 | **1450** |
| WireGuard | 60 | **1420** |
| GRE over IPsec | ~75 | ~1425 |

**And they compound.** A VXLAN inside an IPsec tunnel over PPPoE leaves under 1400 bytes,
with each layer configured by a different team at a different time, none aware of the
others.

**The data-centre answer is jumbo frames on the underlay** — 9000 bytes — so that
encapsulation never reduces the effective MTU below 1500. **This is the operationally
important reason for jumbo frames**, more so than large-transfer efficiency.

## The IPv6 difference

**Worse, and better.**

**Worse:** IPv6 routers **must not fragment** (Chapter 24 §24.3). PMTUD is not an
optimisation — **it is the only mechanism**, and there is no fallback.

**And ICMPv6 Packet Too Big is type 2, a top-level type** rather than a code, which
reflects how essential it is.

**Better:** RFC 4890 exists and is widely known, so **the case for permitting ICMPv6 is
better understood** than the case for permitting ICMPv4 ever was. Most competent IPv6
firewall policies get this right because the consequence of getting it wrong is total
failure rather than a subtle black hole.

**Blocking ICMPv6 does not produce a black hole. It produces no IPv6 at all**, which is at
least easy to diagnose.

## The checklist

**When you meet "small things work, large things hang":**

1. `ping -M do -s 1472 <host>` — does a full-size packet get through?
2. Binary search downward to find the actual path MTU
3. `tracepath <host>` — where does it change?
4. Identify the tunnel or link responsible
5. Fix the ICMP filtering if you control it
6. Otherwise clamp MSS at the boundary you do control
7. Consider `tcp_mtu_probing=1` on the endpoints as insurance

**Steps 1 and 2 take two minutes and identify the fault with certainty.** The remaining
steps are about who can fix it.

## What breaks here

**Large transfers hanging, small ones working.** The signature. Test with
`ping -M do -s`.

**SSH connecting and `scp` hanging.** The same fault, and one of its most common
presentations.

**A VPN that connects and carries nothing.** MSS clamping missing on the tunnel.

**DNSSEC failing while ordinary DNS works.** Responses exceed the path MTU.

**Email sending and attachments failing.** Same.

**It works from one office and not another.** Different paths, different MTUs.

**Everything working after someone set the MTU to 1400 everywhere.** The symptom is gone
and the cause remains — and the whole network now pays for one path's constraint.

> **Network+ note.** Objective 5.2 expects MTU issues as a troubleshooting scenario, and
> **this exact scenario appears on the exam**. Over-learn: **DF triggers PMTUD**;
> **blocking ICMP breaks it**; **the symptom is small-works-large-hangs**; **`ping -M do
> -s` diagnoses it**; and **MSS clamping is the standard workaround.** If you learn one
> troubleshooting pattern from this book, learn this one.
