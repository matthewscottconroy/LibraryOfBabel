# 18.3 The ARP Cache and Its Failures

Resolving an address costs two frames. Doing it before every packet would double the
frame count on the network and add a round trip to every transmission. So every host
caches results — and the cache, being a small store of remotely-supplied information
with no authentication, is where all of ARP's interesting failures live.

## Looking at it

Every operating system exposes the cache, and the first thing to do is look at yours.

**Linux** (modern):

```
$ ip neigh
192.168.10.1    dev eth0 lladdr rr:rr:rr:rr:rr:rr REACHABLE
192.168.10.55   dev eth0 lladdr 00:1b:21:3c:4d:5e STALE
192.168.10.99   dev eth0  FAILED
```

**Linux/macOS/Windows** (the portable command):

```
$ arp -a
? (192.168.10.1) at rr:rr:rr:rr:rr:rr [ether] on eth0
```

**Windows**:

```
> arp -a
Interface: 192.168.10.70 --- 0xc
  Internet Address      Physical Address      Type
  192.168.10.1          rr-rr-rr-rr-rr-rr     dynamic
  192.168.10.255        ff-ff-ff-ff-ff-ff     static
  224.0.0.251           01-00-5e-00-00-fb     static
```

The static entries at the bottom are not configured mappings — they are the
**derived** ones: the subnet broadcast address maps to the Ethernet broadcast, and
multicast IP addresses map algorithmically to multicast MAC addresses. No resolution
is needed for those, because the mapping is defined by a rule.

## The state machine

Linux's neighbour table implements RFC 4861's states, which are more informative than
the simple present/absent view the `arp` command offers:

| State | Meaning |
|---|---|
| `INCOMPLETE` | Request sent, no reply yet |
| `REACHABLE` | Confirmed recently; use it |
| `STALE` | Timer expired; still usable, will be revalidated on next use |
| `DELAY` | Waiting briefly for upper-layer confirmation before probing |
| `PROBE` | Actively re-resolving |
| `FAILED` | Resolution failed |

The `STALE` design is worth noticing. Rather than discarding an expired entry and
paying a round trip on the next packet, the host **uses the stale entry immediately
and revalidates in parallel**. If the mapping was still correct — which it almost
always is — nothing was lost. This is the same optimism that underlies DNS's
serve-stale behaviour (Chapter 39 §39.4) and HTTP's stale-while-revalidate.

An entry stuck at `INCOMPLETE` is a diagnostic: **the request went out and nothing
answered**. The target is off, absent, in another VLAN, firewalled, or the address is
simply unassigned.

## Ageing

Cache entries expire, for the same reason switch table entries do (Chapter 17
§17.2): the mapping may change.

Typical lifetimes:

| System | Base timeout |
|---|---|
| Linux | 30 s (`base_reachable_time`, randomised 0.5×–1.5×) |
| Windows | 15–45 s, randomised |
| Cisco IOS | **4 hours** |
| Most hosts, historically | 20 minutes |

The randomisation prevents synchronisation — without it, a room full of machines
booted together would re-resolve simultaneously, producing a periodic broadcast
spike. **Deliberate desynchronisation is a recurring technique**; it appears again in
DHCP lease renewal (Chapter 40 §40.3) and in routing protocol timers.

Note the size of Cisco's four-hour default versus a host's thirty seconds. That gap
matters: a router may hold a mapping long after every host has re-resolved. It is
also **longer than the default switch MAC-table ageing of five minutes**, which
produces a specific and confusing symptom — the router still knows the MAC address,
so it sends unicast; the switch has forgotten the MAC address, so it floods. Traffic
works, and every port sees it. This is the most common benign cause of persistent
unicast flooding.

## Gratuitous ARP

An ARP message a host sends **unprompted**, announcing its own mapping. Formally: a
request in which the sender and target protocol addresses are the same.

> *Who has 192.168.10.1? Tell 192.168.10.1.*

Nobody is expected to reply. The point is the broadcast itself, and it serves three
legitimate purposes:

**Duplicate address detection.** On acquiring an address, a host sends a gratuitous
ARP for it. If something replies, the address is already in use, and the host reports
the conflict rather than causing chaos. IPv4 formalised this as **ACD** (RFC 5227);
IPv6 makes it mandatory as **Duplicate Address Detection** (§18.4).

**Cache update after a MAC change.** When a failover happens — a clustered pair
swapping roles, a virtual machine migrating between hosts, a NIC replaced — the IP
address moves to a new MAC. Every cached mapping on the segment is now wrong, and
would stay wrong until it aged out. A gratuitous ARP updates everyone in a single
frame.

This is how **VRRP and HSRP failover** (Chapter 56 §56.2) achieve sub-second recovery:
the new active router announces the virtual MAC, and the switches update their
forwarding tables. Without it, failover would take as long as the caches.

**Switch table population.** The broadcast reaches every switch on the path, which
learns the sender's port from the source address. A silent device that comes up and
announces itself is immediately locatable.

And then the illegitimate purpose.

## ARP spoofing

**ARP has no authentication.** None. A reply is believed because it arrived.

There is no field for a signature, no shared secret, no verification that the
responder holds the address it claims. RFC 826 in 1982 was written for a research
network where every station was administered by colleagues, and the omission was not
an oversight so much as an absence of the threat.

So: **anything on the segment can claim any address.**

**The attack.** Attacker M, on the same broadcast domain as victim A and gateway R,
sends A an unsolicited reply:

> *192.168.10.1 (the gateway) is at mm:mm:mm:mm:mm:mm.*

A caches it. Every packet A sends toward the Internet now goes to M's MAC address.

M simultaneously tells R that A's address is at M's MAC. Now **both directions**
traverse M, which forwards them onward so that everything appears to work. M is a
**man in the middle**: reading, modifying, injecting, or simply recording.

Nothing about this is exotic. The tooling (`ettercap`, `bettercap`, `arpspoof`) is
mature, packaged, and needs no privileges beyond being on the segment. **Any device
on your broadcast domain can do this to any other device on that broadcast domain.**

That sentence is the argument for Chapter 20's segmentation, made concretely: a VLAN
boundary is not a bureaucratic convenience but a limit on who can attack whom.

**What defends against it:**

| Defence | Where | Effect |
|---|---|---|
| **Dynamic ARP Inspection (DAI)** | switch | Validates ARP against the DHCP snooping binding table; drops replies that lie. **The real answer.** |
| **DHCP snooping** | switch | Builds the binding table DAI needs. A prerequisite. |
| **Port security** | switch | Limits which MAC addresses may appear. Partial. |
| **Static ARP entries** | host | Effective for a handful of critical mappings, unmanageable in general |
| **Segmentation** | design | Reduces who is in a position to try |
| **Encryption (TLS)** | application | **Does not prevent the attack**, but limits it to metadata |

The last row deserves emphasis because it changes what the attack is worth. TLS means
the attacker sees *that* A talked to a bank and *how much* data flowed, not *what was
said*. This is not nothing — traffic analysis is real (Chapter 61 §61.4) — but it is
much less than the plaintext era's total compromise. **Ubiquitous encryption
substantially devalued a whole class of local attacks**, which is why it was worth the
decade of effort.

## Proxy ARP

A router answering ARP requests for addresses that are not its own, on behalf of hosts
elsewhere.

Introduced in RFC 1027 (1987) for a specific problem: hosts whose implementations did
not understand subnetting. Such a host believes its entire class A/B/C network is
local, so it ARPs for addresses that are in fact on the other side of a router. Proxy
ARP lets the router answer with its own MAC, and the host — none the wiser — sends
frames to the router, which forwards them.

It works, and it should be off.

| Problem | Why |
|---|---|
| Hides topology | Everything looks local; traceroute and troubleshooting mislead |
| Bloats ARP caches | One MAC address for many IP addresses, many entries |
| Enlarges the failure domain | Broadcast issues extend past their apparent boundary |
| Security | The router will answer for **anything**, which makes enumeration easy |

It survives in two current uses that are worth recognising rather than removing:
**some VPN implementations** use it to make a remote client appear local, and **some
wireless controllers** use it to reduce broadcast traffic over the air.

Otherwise: `no ip proxy-arp`. It has been Cisco's default-on for historical reasons
and is the source of a distinctive symptom — **connectivity that works but whose path
makes no sense**, with hosts reaching things they should not be able to reach
directly.

## The commands

```
ip neigh                      # Linux, show
ip neigh flush all            # Linux, clear (requires privilege)
ip neigh show dev eth0        # Linux, one interface
arp -a                        # portable, show
arp -d 192.168.10.1           # delete one entry
arp -s 192.168.10.1 rr:rr:…   # add a static entry
netsh interface ip delete arpcache   # Windows, clear
show ip arp                   # Cisco
clear arp-cache               # Cisco
arping -I eth0 192.168.10.1   # send an ARP request explicitly
```

**`arping` is underused.** It tests reachability at Layer 2 only, without IP,
without ICMP, and without anything a firewall is likely to filter. When `ping` fails,
`arping` distinguishes *"the host is not there"* from *"the host is there and
something above Layer 2 is broken"* — which is a large fork in a troubleshooting tree
(Chapter 63 §63.2).

It also detects duplicate addresses directly: `arping -D` reports every distinct MAC
that answers.

## What breaks here

**Duplicate IP addresses.** Two hosts, one address; both reply to requests. Caches
across the segment hold different mappings, some correct, some not. The symptom is
**intermittent, host-dependent connectivity that changes when caches expire** — one
of the most confusing failures in networking, and one `arping -D` identifies in
seconds.

**A stale cache after a failover.** The gratuitous ARP was lost, filtered, or never
sent. Hosts keep sending to the old MAC. Clearing the cache confirms the diagnosis
immediately; if connectivity returns, the failover's announcement is the fault.

**`INCOMPLETE` entries.** The target is not answering. Not a cache problem — a
reachability problem, and the cache is reporting it accurately.

**ARP storms.** Something is broadcasting continuously — a loop (Chapter 19), a
misconfigured device, a scan. Every host on the segment burns CPU.

**Proxy ARP surprises.** A host reaching something it has no route to. Check for
proxy ARP on the router before doubting the routing table.

> **Network+ note.** Objective 4.2 expects **ARP spoofing / ARP poisoning** as an
> attack and **Dynamic ARP Inspection** as the mitigation, and objective 5.5 expects
> the `arp` command. Two things to over-learn: **ARP has no authentication**, and
> **DAI depends on DHCP snooping** — the second is examined because people configure
> DAI alone and wonder why it drops everything.
