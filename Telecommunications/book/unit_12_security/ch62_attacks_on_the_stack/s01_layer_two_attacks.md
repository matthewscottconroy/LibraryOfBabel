# 62.1 Layer Two Attacks

Every Layer 2 protocol in this book was designed for a cooperative environment and
authenticates nothing (Chapter 57 §57.4). This section is what follows from that, and
every mitigation is a bolt-on added later.

## MAC flooding

Chapter 17 §17.2's CAM table is finite. Fill it, and the switch fails open.

```
   1. Attacker sends frames with thousands of random source MAC addresses
   2. The CAM table fills — typically 8,000 to 128,000 entries
   3. Legitimate entries age out and cannot be relearned
   4. The switch has nowhere to record destinations
   5. It floods unknown-destination frames to every port
   6. The attacker now receives traffic for the whole VLAN
```

> **The switch is behaving correctly.** Flooding unknown destinations is what a switch is
> specified to do, and it is what makes the network work when a device is silent. The attack
> exploits the correct behaviour by exhausting the state that prevents it.

**The arithmetic is unfavourable:**

| CAM size | **Filled at 100,000 frames/s in** |
|---|---|
| 8,000 | **0.08 s** |
| 32,000 | **0.32 s** |
| 128,000 | **1.3 s** |

And `macof` — a tool from 1999 — generates that rate on a laptop.

**The mitigation is port security:**

```
   switchport port-security
   switchport port-security maximum 3
   switchport port-security violation restrict
   switchport port-security aging time 5
   switchport port-security aging type inactivity
```

| Violation action | Behaviour |
|---|---|
| `protect` | **drop the offending frames silently** — no log, and it hides the attack |
| **`restrict`** | **drop and log and increment a counter** — usually correct |
| `shutdown` | **err-disable the port** — secure, and a denial of service against the user |

**The practical difficulty is the maximum.** A port with a phone and a PC behind it needs at
least two; a port with a hypervisor needs many; a port with a small unmanaged switch needs
however many devices are on it. A maximum that is too low produces support calls; one that is
too high does nothing. Sticky learning — learn the first N and keep them — is the usual
compromise, and it must be paired with a process for when a device is legitimately replaced.

## VLAN hopping

Two distinct attacks with the same name and different mitigations.

### Switch spoofing

**The attacker's device negotiates a trunk.**

> DTP — Dynamic Trunking Protocol — negotiates whether a link is a trunk. A port left at
> its default `dynamic auto` will become a trunk if the device at the other end asks, and
> **then the attacker receives every VLAN.**

The mitigation is one line and it is universally recommended and frequently absent:

```
   switchport mode access
   switchport nonegotiate
```

**Set access ports explicitly. Disable DTP.** There is no legitimate reason for an access port
to negotiate anything.

### Double tagging

The subtler one, and it exploits the native VLAN.

```
   Attacker sends:  ┌──────┬──────┬─────────────┐
                    │ tag  │ tag  │  payload    │
                    │ 1    │ 20   │             │
                    └──────┴──────┴─────────────┘
                       ▲
                  the NATIVE VLAN of the trunk

   Switch 1: this frame is on the native VLAN, so it strips the outer tag
             and forwards over the trunk — untagged? no: the inner tag remains
   Switch 2: sees a frame tagged VLAN 20, delivers it to VLAN 20
```

The attacker, on VLAN 1, sends a frame that arrives in VLAN 20.

> **It is one-way only** — **the reply cannot be double-tagged back** — **which limits it to
> injection rather than conversation.** **Injection is sufficient for several attacks**, and it
> requires the attacker to be on the trunk's native VLAN.

The mitigations, and all three should be applied:

- Never use VLAN 1 for anything, including as the native VLAN
- Set the native VLAN to an unused, black-holed VLAN on every trunk
- `vlan dot1q tag native` where supported — tag the native VLAN too, which removes the
  mechanism entirely

## Rogue DHCP

**Chapter 40 §40.4's failure, weaponised.**

> **A DHCP client accepts the first answer it receives.** **There is no authentication and no
> tie-break** — **the fastest server wins.**

**Which lets an attacker supply:**

| Field | Attack |
|---|---|
| **Router** | **their own address — every packet passes through them** |
| **DNS server** | **their own — every name resolves where they choose** (§62.2) |
| **NTP server** | **clock manipulation, which breaks Kerberos and certificate validation** |
| Domain, WPAD, TFTP server | **credential theft, and boot-image substitution** |

And most rogue DHCP servers are not attacks. A misconfigured home router plugged into a
wall socket, or a hypervisor's default virtual network bridged to production, causes exactly
the same symptoms, and is far more common.

**The mitigation is DHCP snooping:**

```
   ip dhcp snooping
   ip dhcp snooping vlan 10,20,30
   !
   interface Gi1/0/1              ← towards the real DHCP server
    ip dhcp snooping trust
   !
   interface range Gi1/0/2 - 48   ← towards users
    ip dhcp snooping limit rate 15
```

Untrusted ports may send DHCP requests and not offers. An offer arriving on an untrusted
port is dropped and logged, which stops both the attack and the accident.

And snooping builds a binding table — MAC, IP, VLAN, port, lease time — which is the input
to the next two mitigations, and is why it must be deployed first.

## ARP spoofing

**Chapter 18 §18.3's central weakness.**

> **ARP has no authentication at all.** A host that receives an ARP reply believes it, and
> many implementations accept unsolicited replies — gratuitous ARP — and update the cache.

```
   Attacker → Victim:  "10.20.0.1 is at aa:bb:cc:dd:ee:ff"   (the gateway's IP, my MAC)
   Attacker → Gateway: "10.20.0.50 is at aa:bb:cc:dd:ee:ff"  (the victim's IP, my MAC)

   Both now send to the attacker, who forwards on — reading and modifying in passing.
```

This is the classic on-path attack, and `ettercap`, `bettercap` and `arpspoof` make it a
single command.

The mitigation is Dynamic ARP Inspection, which uses the DHCP snooping binding table:

```
   ip arp inspection vlan 10,20,30
   interface Gi1/0/1
    ip arp inspection trust          ← uplinks
```

Every ARP packet on an untrusted port is checked against the binding table. An ARP claiming
an address that the table says belongs to a different port is dropped.

And for statically addressed hosts — servers, printers — an ARP ACL supplies the bindings
that DHCP snooping cannot learn.

**IPv6's equivalent** is **RA Guard** and **ND Inspection**: a Router Advertisement arriving on
a user port is an attack or an accident (Chapter 28 §28.3), and RA Guard drops it.
Deploying IPv6 without RA Guard is deploying the rogue-DHCP problem with no snooping.

## Spanning tree attacks

**Chapter 19 §19.3's election has no authentication.**

> **The lowest bridge ID wins, and anyone may claim to have it.**

An attacker sending BPDUs with a bridge priority of 0 becomes the root, and the topology
recalculates so that traffic flows through them — or simply collapses while it converges,
repeatedly.

Even without malice, the same thing happens accidentally: someone plugs a small unmanaged
switch into two wall sockets, and the network reconverges around a device in a cupboard.

Three guards, and each addresses a different case:

| Guard | Applied to | Prevents |
|---|---|---|
| **BPDU Guard** | **access ports** | **any BPDU at all** — err-disables the port |
| **Root Guard** | **towards other switches** | **a superior BPDU** — a neighbour becoming root |
| **Loop Guard** | **non-designated ports** | **a port transitioning to forwarding when BPDUs stop** |

BPDU Guard plus PortFast on every access port is the baseline, and it is the single most
valuable Layer 2 hardening measure — it stops both the attack and the far more common
accident.

## Discovery protocol disclosure

**Small, and worth doing.**

CDP and LLDP announce the device model, software version, port identifier, VLAN, IP address
and duplex — **to anyone connected.** Which is a reconnaissance gift (Chapter 57 §57.4):
an attacker plugging into a wall socket learns the switch's model and software version before
doing anything else.

Disable them towards users; keep them towards infrastructure and IP phones, which use LLDP-MED
for voice VLAN and PoE negotiation and genuinely need it.

## The baseline configuration

**A hardened access port, complete:**

```
   interface range GigabitEthernet1/0/2 - 48
    description ACCESS
    switchport mode access                    ! no DTP
    switchport nonegotiate                    ! no DTP, explicitly
    switchport access vlan 20
    switchport voice vlan 240
    switchport port-security
    switchport port-security maximum 3
    switchport port-security violation restrict
    switchport port-security mac-address sticky
    spanning-tree portfast
    spanning-tree bpduguard enable            ! err-disable on any BPDU
    ip dhcp snooping limit rate 15            ! untrusted by default
    no cdp enable
    storm-control broadcast level 1.00        ! and multicast, and unicast
    errdisable recovery cause bpduguard       ! recover after 5 min, globally
```

> Every line prevents a specific attack in this section, and the whole thing takes ten
> minutes to deploy across an estate with any automation (Chapter 70). The reason it is
> frequently absent is not difficulty; it is that nothing breaks when it is missing.

## What breaks here

A switch flooding all traffic to all ports. **CAM exhaustion**, or a legitimate topology
change. Check the MAC address table size and the port security counters.

Port security shutting a port when a user docks a laptop. The maximum is too low, or
sticky learning has retained a previous device. Process, not configuration.

**A user in the wrong VLAN entirely.** Switch spoofing, or a misconfigured trunk. Check
whether the port negotiated.

Traffic appearing in a VLAN it should not reach, one way only. **Double tagging.** Check the
native VLAN.

**Everyone getting the wrong gateway.** **Rogue DHCP** — and it is usually a home router
someone plugged in.

Intermittent connectivity for two hosts, with ARP entries changing. ARP spoofing, or a
duplicate address (Chapter 53 §53.3). The MAC table distinguishes them.

The network reconverging around a device in a cupboard. A switch became root. BPDU
Guard.

IPv6 hosts acquiring addresses from an unexpected router. **No RA Guard.** The IPv6
equivalent of rogue DHCP, and it is enabled by default on any Linux machine with forwarding on.

An attacker who learned the switch model from a wall socket. **CDP towards users.**

> **Network+ note.** Objective 4.2 and 4.3 cover these directly. Over-learn: MAC flooding
> fills the CAM table and forces flooding, mitigated by port security; VLAN hopping uses
> switch spoofing or double tagging, mitigated by disabling DTP and changing the native VLAN;
> **rogue DHCP is mitigated by DHCP snooping**; **ARP spoofing by dynamic ARP inspection**; and
> **STP attacks by BPDU guard and root guard.** This mapping is examined in almost every form
> and is worth memorising as pairs.
